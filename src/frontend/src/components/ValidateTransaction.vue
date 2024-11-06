<template>
  <div>
    <h2>Validate Transaction</h2>
    <form @submit.prevent="submitTransaction">
      <label>Sender Public Key:</label>
      <input v-model="sender" required />
      <label>Recipient Public Key:</label>
      <input v-model="recipient" required />
      <label>Amount:</label>
      <input v-model="amount" type="number" required />
      <button type="submit">Validate</button>
    </form>
    <p v-if="responseMessage">{{ responseMessage }}</p>
  </div>
</template>

<script>
import { validateTransaction } from '../api';

export default {
  data() {
    return {
      sender: '',
      recipient: '',
      amount: null,
      responseMessage: ''
    };
  },
  methods: {
    async submitTransaction() {
      try {
        const { data } = await validateTransaction({
          sender: this.sender,
          recipient: this.recipient,
          amount: parseInt(this.amount)
        });
        this.responseMessage = `Transaction Successful: ${data.signature}`;
      } catch (error) {
        this.responseMessage = `Error: ${error.response.data.error}`;
      }
    }
  }
};
</script>
