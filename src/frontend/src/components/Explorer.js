import React, { useState } from 'react';
import { fetchTransaction } from '../api/explorer';

function Explorer() {
  const [txid, setTxid] = useState('');
  const [transaction, setTransaction] = useState(null);

  const handleSearch = async () => {
    const tx = await fetchTransaction(txid);
    setTransaction(tx);
  };

  return (
    <div>
      <h2>Transaction Explorer</h2>
      <input type="text" value={txid} onChange={(e) => setTxid(e.target.value)} placeholder="Enter transaction ID" />
      <button onClick={handleSearch}>Search</button>
      {transaction && <div>{JSON.stringify(transaction)}</div>}
    </div>
  );
}

export default Explorer;
