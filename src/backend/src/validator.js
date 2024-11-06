const solanaWeb3 = require('@solana/web3.js');
const borsh = require('borsh');
const connection = require('./networkConfig');

// Define the Transaction structure (to match the Rust structure)
class Transaction {
    constructor(sender, recipient, amount) {
        this.sender = sender;
        this.recipient = recipient;
        this.amount = amount;
    }
}

// Borsh schema definition
const transactionSchema = new Map([
    [Transaction, { kind: 'struct', fields: [['sender', [32]], ['recipient', [32]], ['amount', 'u64']] }]
]);

async function validateTransaction(sender, recipient, amount, programId) {
    const transaction = new Transaction(sender, recipient, amount);
    const serializedTransaction = borsh.serialize(transactionSchema, transaction);

    // Create a Solana transaction to send to the Rust program
    let instruction = new solanaWeb3.TransactionInstruction({
        keys: [{ pubkey: sender, isSigner: true, isWritable: true }],
        programId,
        data: Buffer.from(serializedTransaction), // Send serialized data to the program
    });

    let tx = new solanaWeb3.Transaction().add(instruction);
    let signature = await solanaWeb3.sendAndConfirmTransaction(connection, tx, [sender]);

    console.log("Transaction signature", signature);
    return signature;
}

module.exports = { validateTransaction };
