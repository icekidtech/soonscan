use reqwest::Error;
use serde::{Deserialize, Serialize};

// Define the data structures for Block and Transaction
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}

// Function to fetch blockchain data from the SOON network
async fn fetch_blockchain_data(url: &str) -> Result<Vec<Block>, Error> {
    let response = reqwest::get(url).await?.json::<Vec<Block>>().await?;
    Ok(response)
}

// Function to display the fetched blockchain data
fn display_blocks(blocks: &Vec<Block>) {
    for block in blocks {
        println!("Block Index: {}", block.index);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Timestamp: {}", block.timestamp);
        println!("Transactions: {:?}", block.transactions);
        println!("Hash: {}", block.hash);
        println!("-----------------------------------");
    }
}

// Function to validate individual transactions
fn validate_transaction(transaction: &Transaction) -> bool {
    transaction.amount > 0.0 // Example validation: amount must be positive
}

// Function to validate all transactions in a block
fn validate_block_transactions(block: &Block) -> bool {
    for transaction in &block.transactions {
        if !validate_transaction(transaction) {
            return false; // Invalid transaction found
        }
    }
    true // All transactions are valid
}

// Main function to run the explorer
#[tokio::main]
async fn main() {
    let url = "https://explorer.solana.com/address/Hm4BnUnY6H6qLend3YxjWG3XFr4jJiaNPH2UC7fgXGNB?cluster=testnet"; // Replace with the actual SOON network URL
    match fetch_blockchain_data(url).await {
        Ok(blocks) => {
            display_blocks(&blocks); // Pass a reference to blocks
            for block in &blocks { // This works now because blocks is still accessible
                if validate_block_transactions(block) {
                    println!("Block {} is valid.", block.index);
                } else {
                    println!("Block {} is invalid.", block.index);
                }
            }
        }
        Err(e) => println!("Error fetching blockchain data: {}", e),
    }
}