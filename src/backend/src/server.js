const express = require('express');
const bodyParser = require('body-parser');
const { validateTransaction } = require('./validator');
const { fetchLatestBlock, fetchTransaction } = require('./explorer');

const app = express();
const PORT = process.env.PORT || 3000;

app.use(bodyParser.json());

// Endpoint to validate a transaction
app.post('/api/validate-transaction', async (req, res) => {
    const { sender, recipient, amount } = req.body;
    try {
        const signature = await validateTransaction(sender, recipient, amount, PROGRAM_ID);
        res.json({ message: 'Transaction validated', signature });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Endpoint to fetch latest block data
app.get('/api/latest-block', async (req, res) => {
    try {
        const block = await fetchLatestBlock();
        res.json(block);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Endpoint to fetch a transaction by signature
app.get('/api/transaction/:signature', async (req, res) => {
    const { signature } = req.params;
    try {
        const transaction = await fetchTransaction(signature);
        res.json(transaction);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});
