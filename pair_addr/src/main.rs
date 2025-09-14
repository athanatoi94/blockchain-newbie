use anyhow::{anyhow, Result};
use ethers::{
    prelude::*,
    types::{Address, U256},
};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Semaphore;
use ethers::abi::Abi;

// PancakeSwap在Arbitrum上的Factory合约地址
const FACTORY_ADDRESS: &str = "0x02a84c1b3BBD7401a5f7fa98a384EBC70bB5749E";
// Arbitrum RPC节点URL
const ARBITRUM_RPC_URL: &str = "https://rpc.ankr.com/arbitrum/46af0d8bca1783a0b8e8504c6700a5a911e75499e9abbf9ec0da142d4de7e843";
// 最大并发请求数
const MAX_CONCURRENT_REQUESTS: usize = 5;

// Factory合约ABI片段
const FACTORY_ABI: &str = r#"[
    {
        "constant": true,
        "inputs": [],
        "name": "allPairsLength",
        "outputs": [{"name": "", "type": "uint256"}],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [{"name": "", "type": "uint256"}],
        "name": "allPairs",
        "outputs": [{"name": "", "type": "address"}],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    }
]"#;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Connecting to Arbitrum RPC...");

    // 连接到Arbitrum RPC节点
    let provider = Provider::<Http>::try_from(ARBITRUM_RPC_URL)?;
    let client = Arc::new(provider);

    // 解析Factory合约地址
    let factory_address = Address::from_str(FACTORY_ADDRESS)
        .map_err(|e| anyhow!("Invalid factory address: {}", e))?;

    println!("Connected to Arbitrum. Fetching pair count...");

    // 创建Factory合约实例
    let abi: Abi = serde_json::from_str(FACTORY_ABI)?;
    let factory = Contract::new(factory_address, abi, client.clone());

    // 获取交易对总数
    let pairs_length: U256 = factory
        .method::<_, U256>("allPairsLength", ())?
        .call()
        .await?;

    let total_pairs = pairs_length.as_u64();
    println!("Total pairs found: {}", total_pairs);

    if total_pairs == 0 {
        println!("No pairs found");
        return Ok(());
    }

    // 使用信号量控制并发
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));
    let mut handles = Vec::new();

    println!("Fetching all pair addresses...");

    for i in 0..total_pairs {
        let factory_clone = factory.clone();
        let semaphore_clone = semaphore.clone();

        let handle = tokio::spawn(async move {
            let _permit = semaphore_clone.acquire().await.unwrap();

            match get_pair_address(&factory_clone, i).await {
                Ok(address) => {
                    println!("Pair {}: {:?}", i, address);
                    Some(address)
                }
                Err(e) => {
                    eprintln!("Error fetching pair {}: {}", i, e);
                    None
                }
            }
        });

        handles.push(handle);

        // 添加延迟以避免请求过于频繁
        if i % 10 == 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    // 收集所有结果
    let mut pair_addresses = Vec::new();
    for handle in handles {
        if let Ok(Some(address)) = handle.await {
            pair_addresses.push(address);
        }
    }

    println!("\n=== Results ===");
    println!("Successfully fetched {} out of {} pairs", pair_addresses.len(), total_pairs);

    // 保存到文件
    save_to_file(&pair_addresses).await?;

    Ok(())
}

async fn get_pair_address(factory: &Contract<Provider<Http>>, index: u64) -> Result<Address> {
    let pair_address: Address = factory
        .method::<_, Address>("allPairs", U256::from(index))?
        .call()
        .await?;

    Ok(pair_address)
}

async fn save_to_file(pair_addresses: &[Address]) -> Result<()> {
    use std::fs::File;
    use std::io::Write;

    let addresses: Vec<String> = pair_addresses
        .iter()
        .map(|addr| format!("{:?}", addr))
        .collect();

    let json = serde_json::to_string_pretty(&addresses)?;

    let mut file = File::create("pancakeswap_pairs_arbitrum.json")?;
    file.write_all(json.as_bytes())?;

    println!("Results saved to pancakeswap_pairs_arbitrum.json");
    Ok(())
}