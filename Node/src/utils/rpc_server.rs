#[cfg(test)]
pub mod test {
    use jsonrpsee::server::{ServerBuilder, ServerHandle};
    use jsonrpsee::RpcModule;
    use lazy_static::lazy_static;
    use serde_json::json;
    use std::net::SocketAddr;
    use tracing::info;

    pub struct RpcServer {
        handle: Option<ServerHandle>,
    }

    impl RpcServer {
        pub fn new() -> Self {
            RpcServer {
                handle: None::<ServerHandle>,
            }
        }

        #[cfg(test)]
        pub async fn start_test_responses(
            &mut self,
            addr: SocketAddr,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let server = ServerBuilder::default().build(addr).await?;
            let mut module = RpcModule::new(());

            module.register_async_method("RPC.GetL2TxLists", |_, _, _| async {
                TX_LISTS_RESPONSE.clone()
            })?;
            module.register_async_method(
                "RPC.AdvanceL2ChainHeadWithNewBlocks",
                |_, _, _| async {
                    json!({
                        "result": "Request received and processed successfully",
                        "error": null,
                        "id": 1
                    })
                },
            )?;

            let handle = server.start(module);
            tokio::spawn(handle.clone().stopped());

            self.handle = Some(handle);
            Ok(())
        }

        pub async fn stop(&mut self) {
            if let Some(handle) = self.handle.take() {
                handle.stop().unwrap();
            }
            info!("Server stopped");
        }
    }

    lazy_static! {
        pub static ref TX_LISTS_RESPONSE: serde_json::Value = json!({
            "TxLists": [
                [
                    {
                        "type": "0x0",
                        "chainId": "0x28c61",
                        "nonce": "0x1",
                        "to": "0xbfadd5365bb2890ad832038837115e60b71f7cbb",
                        "gas": "0x267ac",
                        "gasPrice": "0x5e76e0800",
                        "maxPriorityFeePerGas": null,
                        "maxFeePerGas": null,
                        "value": "0x0",
                        "input": "0x40d097c30000000000000000000000004cea2c7d358e313f5d0287c475f9ae943fe1a913",
                        "v": "0x518e6",
                        "r": "0xb22da5cdc4c091ec85d2dda9054aa497088e55bd9f0335f39864ae1c598dd35",
                        "s": "0x6eee1bcfe6a1855e89dd23d40942c90a036f273159b4c4fd217d58169493f055",
                        "hash": "0x7c76b9906579e54df54fe77ad1706c47aca706b3eb5cfd8a30ccc3c5a19e8ecd"
                    },
                    {
                        "type": "0x2",
                        "chainId": "0x28c61",
                        "nonce": "0x3f",
                        "to": "0x380a5ba81efe70fe98ab56613ebf9244a2f3d4c9",
                        "gas": "0x2c2c8",
                        "gasPrice": null,
                        "maxPriorityFeePerGas": "0x1",
                        "maxFeePerGas": "0x3",
                        "value": "0x5af3107a4000",
                        "input": "0x3593564c000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000006672d0a400000000000000000000000000000000000000000000000000000000000000020b000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000005af3107a40000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000005af3107a400000000000000000000000000000000000000000000000000000000353ca3e629a00000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002bae2c46ddb314b9ba743c6dee4878f151881333d9000bb8ebf1f662bf092ff0d913a9fe9d7179b0efef1611000000000000000000000000000000000000000000",
                        "accessList": [],
                        "v": "0x1",
                        "r": "0x36517a175a60d3026380318917976fa32c82e542850357a611af05d2212ab9a4",
                        "s": "0x32d89dce30d76287ddba907b0c662cd09dc30891b1c9c2ef644edfc53160b298",
                        "yParity": "0x1",
                        "hash": "0xece2a3c6ca097cfe5d97aad4e79393240f63865210f9c763703d1136f065298b"
                    },
                    {
                        "type": "0x2",
                        "chainId": "0x28c61",
                        "nonce": "0x39",
                        "to": "0x380a5ba81efe70fe98ab56613ebf9244a2f3d4c9",
                        "gas": "0x2c2c8",
                        "gasPrice": null,
                        "maxPriorityFeePerGas": "0x1",
                        "maxFeePerGas": "0x3",
                        "value": "0x5af3107a4000",
                        "input": "0x3593564c000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000006672d0d400000000000000000000000000000000000000000000000000000000000000020b000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000005af3107a40000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000005af3107a400000000000000000000000000000000000000000000000000000000353ca3e629a00000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002bae2c46ddb314b9ba743c6dee4878f151881333d9000bb8ebf1f662bf092ff0d913a9fe9d7179b0efef1611000000000000000000000000000000000000000000",
                        "accessList": [],
                        "v": "0x0",
                        "r": "0xc779421d1ee81dbd3dfbfad5fd632b45303b4513ea1b8ac0bc647f5430cd97b9",
                        "s": "0x13cedef844bf5a954183182992ffbf9b8b23331de255157528be7da6614618b2",
                        "yParity": "0x0",
                        "hash": "0xb105d9f16e8fb913093c8a2c595bf4257328d256f218a05be8dcc626ddeb4193"
                    }
                ]
            ]
        });
    }
}
