const solanaWeb3 = require('@solana/web3.js');
const connection = require('./networkConfig');

// Fetch latest block details
async function fetchLatestBlock() {
    const slot = await connection.getSlot();
    const block = await connection.getBlock(slot);
    return block;
}

// Fetch transaction details by signature
async function fetchTransaction(signature) {
    const transaction = await connection.getTransaction(signature);
    return transaction;
}

module.exports = { fetchLatestBlock, fetchTransaction };
