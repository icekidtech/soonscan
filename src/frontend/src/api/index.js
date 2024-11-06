import axios from 'axios';

const api = axios.create({
  baseURL: 'http://localhost:3000/api' // Adjust to backend URL if different
});

export const validateTransaction = (data) => api.post('/validate-transaction', data);
export const fetchLatestBlock = () => api.get('/latest-block');
export const fetchTransaction = (signature) => api.get(`/transaction/${signature}`);
