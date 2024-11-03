use solana_client::rpc_client::RpcClient;
use crate::models::blocks::Block;

pub async fn fetch_recent_blocks() -> Vec<Block> {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");
    // Fetch blocks, convert into Block struct, and return
    vec![]
}
