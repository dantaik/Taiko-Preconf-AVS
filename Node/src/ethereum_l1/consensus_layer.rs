#![allow(dead_code)] // TODO: remove
use super::validator::Validator;
use crate::utils::types::*;
use anyhow::Error;
use beacon_api_client::{
    mainnet::MainnetClientTypes, Client, GenesisDetails, ProposerDuty, PublicKeyOrIndex, StateId,
};
use ethereum_consensus::{
    crypto::bls::PublicKey as EthereumConsensusBlsPublicKey,
    phase0::validator::Validator as EthereumConsensusValidator,
};
use reqwest;

pub struct ConsensusLayer {
    client: Client<MainnetClientTypes>,
}

impl ConsensusLayer {
    pub fn new(rpc_url: &str) -> Result<Self, Error> {
        let client = Client::new(reqwest::Url::parse(rpc_url)?);
        Ok(Self { client })
    }

    pub async fn get_lookahead(&self, epoch: u64) -> Result<Vec<ProposerDuty>, Error> {
        let (_, duties) = self.client.get_proposer_duties(epoch).await?;
        tracing::debug!("got duties len: {}", duties.len());
        Ok(duties)
    }

    pub async fn get_genesis_details(&self) -> Result<GenesisDetails, Error> {
        self.client.get_genesis_details().await.map_err(Error::new)
    }

    // pub async fn get_validator_inclusion_proof(&self, validator_index: u64, epoch: u64) -> Result<Vec<u8>, Error> {
    //     self.client.get_validator(state_id, validator_id)
    // }


    pub async fn get_validators(
        &self,
        public_keys: &[EthereumConsensusBlsPublicKey],
    ) -> Result<Vec<Validator>, Error> {
        let public_keys_or_indices = public_keys
            .iter()
            .map(|k| PublicKeyOrIndex::PublicKey(k.clone()))
            .collect::<Vec<PublicKeyOrIndex>>();
        let validators = self
            .client
            .get_validators(StateId::Head, &public_keys_or_indices, &vec![])
            .await?;
        let validators_mapped = validators
            .iter()
            .map(|v| Validator::try_from(v.validator.clone()))
            .collect::<Result<Vec<_>, _>>();

        validators_mapped.map_err(|e| anyhow::anyhow!("Failed to convert validator: {e}"))
    }

    pub async fn get_all_head_validators(&self) -> Result<Vec<Validator>, Error> {
        let validators = self
            .client
            .get_validators(StateId::Head, &vec![], &vec![])
            .await?;
        let validators_mapped = validators
            .iter()
            .map(|v| Validator::try_from(v.validator.clone()))
            .collect::<Result<Vec<_>, _>>();

        validators_mapped.map_err(|e| anyhow::anyhow!("Failed to convert validator: {e}"))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_lookahead() {
        let server = setup_server().await;
        let cl = ConsensusLayer::new(server.url().as_str()).unwrap();
        let duties = cl.get_lookahead(1).await.unwrap();

        assert_eq!(duties.len(), 32);
        assert_eq!(duties[0].slot, 32);
    }

    #[tokio::test]
    async fn test_get_genesis_data() {
        let server = setup_server().await;
        let cl = ConsensusLayer::new(server.url().as_str()).unwrap();
        let genesis_data = cl.get_genesis_details().await.unwrap();

        assert_eq!(genesis_data.genesis_time, 1590832934);
        assert_eq!(
            genesis_data.genesis_validators_root.to_string(),
            "0xcf8e0d4e9587369b2301d0790347320302cc0943d5a1884560367e8208d920f2"
        );
        assert_eq!(genesis_data.genesis_fork_version, [0; 4]);
    }

    pub async fn setup_server() -> mockito::ServerGuard {
        let mut server = mockito::Server::new_async().await;
        server
            .mock("GET", "/eth/v1/beacon/genesis")
            .with_body(r#"{
                "data": {
                  "genesis_time": "1590832934",
                  "genesis_validators_root": "0xcf8e0d4e9587369b2301d0790347320302cc0943d5a1884560367e8208d920f2",
                  "genesis_fork_version": "0x00000000"
                }
              }"#)
            .create();
        server
            .mock("GET", "/eth/v1/validator/duties/proposer/1")
            .with_body(include_str!("lookahead_test_response.json"))
            .create();
        server
    }
}
