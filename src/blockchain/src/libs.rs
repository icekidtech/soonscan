// Import required Solana program libraries and Borsh serialization
use borsh::{BorshDeserialize, BorshSerialize}; // Serialization for Solana-compatible types
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define the Transaction data structure, which is serializable for blockchain storage
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Transaction {
    pub sender: Pubkey,     // Public key of the transaction sender
    pub recipient: Pubkey,  // Public key of the transaction recipient
    pub amount: u64,        // Amount to be transferred
}

// Define the structure for transaction validation results
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TransactionValidation {
    pub is_valid: bool,
    pub message: String,
}

// Define the Block data structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Block {
    pub index: u64,                       // Block index
    pub previous_hash: [u8; 32],          // Hash of the previous block
    pub timestamp: i64,                   // Timestamp for the block creation
    pub transactions: Vec<Transaction>,   // List of transactions in the block
    pub hash: [u8; 32],                   // Hash of the current block
}

// Core function to validate a single transaction
fn validate_transaction(transaction: &Transaction) -> TransactionValidation {
    if transaction.amount > 0 {
        TransactionValidation {
            is_valid: true,
            message: "Transaction is valid.".to_string(),
        }
    } else {
        TransactionValidation {
            is_valid: false,
            message: "Transaction amount must be greater than zero.".to_string(),
        }
    }
}

// Main function to process transaction validation on the blockchain
pub fn process_transaction_validation(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input_data: &[u8],
) -> ProgramResult {
    msg!("Transaction Validator Invoked");

    // Deserialize the incoming transaction data
    let transaction = Transaction::try_from_slice(input_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Validate the transaction
    let validation_result = validate_transaction(&transaction);

    // Log the validation result
    msg!("Validation Result: {:?}", validation_result);

    // Further steps could involve storing the validation result or updating on-chain data
    Ok(())
}

// Solana's entry point to link with the Rust program
entrypoint!(process_transaction_validation);
