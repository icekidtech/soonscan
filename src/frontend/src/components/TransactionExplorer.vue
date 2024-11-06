<template>
  <div>
    <h2>Fetch Transaction Details</h2>
    <input v-model="signature" placeholder="Enter Transaction Signature" />
    <button @click="getTransaction">Fetch Transaction</button>
    <pre v-if="transaction">{{ transaction }}</pre>
  </div>
</template>

<script>
import { fetchTransaction } from '../api';

export default {
  data() {
    return {
      signature: '',
      transaction: null
    };
  },
  methods: {
    async getTransaction() {
      const { data } = await fetchTransaction(this.signature);
      this.transaction = data;
    }
  }
};
</script>
