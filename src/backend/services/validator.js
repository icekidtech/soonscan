const { Connection, PublicKey } = require('@solana/web3.js');

const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

// Fetches a transaction from SOON network
async function getTransaction(txid) {
  const transaction = await connection.getTransaction(txid);
  return transaction;
}

// Mock transaction validation
async function validateTransaction(transaction) {
  // Add specific checks and validation logic
  if (transaction.signatures.length > 0) {
    return true;  // Example validation rule
  }
  return false;
}

module.exports = { getTransaction, validateTransaction };
