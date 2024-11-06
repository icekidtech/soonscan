const solanaWeb3 = require('@solana/web3.js');

// Connect to Solana network (using Devnet for development)
const connection = new solanaWeb3.Connection(
    solanaWeb3.clusterApiUrl('devnet'),
    'confirmed'
);

module.exports = connection;
