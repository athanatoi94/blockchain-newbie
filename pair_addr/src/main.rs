use ethers::{
    providers::{Provider, Http},
    types::{H160, U256},
    contract::Contract,
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from("https://arb1.arbitrum.io/rpc")?;

    // PancakeSwap 合约地址
    let factory_address: H160 = "0x02a84c1b3BBD7401a5f7fa98a384EBC70bB5749E".parse()?;

    let factory_abi = ethers::contract::Abigen::new("Factory", r#"
        [
            {
                "inputs": [],
                "name": "allPairsLength",
                "outputs": [{"internalType": "uint256", "name": "", "type": "uint256"}],
                "stateMutability": "view",
                "type": "function"
            },
            {
                "inputs": [{"internalType": "uint256", "name": "", "type": "uint256"}],
                "name": "allPairs",
                "outputs": [{"internalType": "address", "name": "", "type": "address"}],
                "stateMutability": "view",
                "type": "function"
            }
        ]
    "#)?.build()?;

    let factory = Contract::new(factory_address, factory_abi, provider);

    // 获取总pair数量
    let total_pairs: U256 = factory.method::<_, U256>("allPairsLength", ())?.call().await?;
    println!("Total pairs: {}", total_pairs);

    // 获取所有pair地址
    for i in 0..total_pairs.as_u64() {
        let pair_address: H160 = factory.method::<_, H160>("allPairs", i)?.call().await?;
        println!("Pair {}: {:?}", i, pair_address);
    }

    Ok(())
}