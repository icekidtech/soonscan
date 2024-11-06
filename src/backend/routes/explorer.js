const express = require('express');
const { getTransaction, validateTransaction } = require('../services/validator');
const router = express.Router();

// Route to fetch a transaction by ID
router.get('/transaction/:txid', async (req, res) => {
  try {
    const transaction = await getTransaction(req.params.txid);
    res.json(transaction);
  } catch (error) {
    res.status(500).json({ error: 'Error fetching transaction' });
  }
});

// Route to validate a transaction
router.post('/transaction/validate', async (req, res) => {
  try {
    const isValid = await validateTransaction(req.body.transaction);
    res.json({ isValid });
  } catch (error) {
    res.status(500).json({ error: 'Error validating transaction' });
  }
});

module.exports = router;
