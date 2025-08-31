use anyhow::Result;
use ethers::{
    providers::{Provider, Http},
    core::types::TransactionRequest,
    utils::parse_ether,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://polygon-bor-rpc.publicnode.com";
    let private_key = std::env::var("PRIVATE_KEY")?;
    let to_address = std::env::var("TO_ADDRESS")?;
    let matic_amount = 0.001;

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let wallet = private_key.parse::<ethers::signers::LocalWallet>()?;
    let client = Arc::new(provider).with_signer(wallet);

    let gas_price = client.get_gas_price().await?;
    let low_gas_price = gas_price * 1 / 10;

    let tx = TransactionRequest::new()
        .to(to_address.parse()?)
        .value(parse_ether(matic_amount)?)
        .gas_price(low_gas_price);

    let tx_hash = client.send_transaction(tx, None).await?.tx_hash();

    println!("transaction sent: 0x{:x}, gas_price:{}", tx_hash, low_gas_price);
    Ok(())
}