mod ethereum_l1;
mod mev_boost;
mod node;
mod p2p_network;
mod taiko;
mod utils;

use anyhow::Error;
use tokio::sync::mpsc;

const MESSAGE_QUEUE_SIZE: usize = 100;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_logging();
    let config = utils::config::Config::read_env_variables();

    let (avs_p2p_tx, avs_p2p_rx) = mpsc::channel(MESSAGE_QUEUE_SIZE);
    let (node_tx, node_rx) = mpsc::channel(MESSAGE_QUEUE_SIZE);
    let p2p = p2p_network::AVSp2p::new(node_tx.clone(), avs_p2p_rx);
    p2p.start();
    let taiko = taiko::Taiko::new(&config.taiko_proposer_url, &config.taiko_driver_url);
    let ethereum_l1 = ethereum_l1::EthereumL1::new(
        &config.mev_boost_url,
        &config.ethereum_private_key,
        &config.taiko_preconfirming_address,
    )?;
    let mev_boost = mev_boost::MevBoost::new(&config.mev_boost_url);
    let node = node::Node::new(node_rx, avs_p2p_tx, taiko, ethereum_l1, mev_boost);
    node.entrypoint().await?;
    Ok(())
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}
