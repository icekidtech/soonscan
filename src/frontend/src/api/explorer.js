export async function fetchTransaction(txid) {
    const response = await fetch(`/api/explorer/transaction/${txid}`);
    return await response.json();
  }
  