<template>
  <div>
    <h2>Latest Block Information</h2>
    <button @click="getLatestBlock">Fetch Latest Block</button>
    <pre v-if="block">{{ block }}</pre>
  </div>
</template>

<script>
import { fetchLatestBlock } from '../api';

export default {
  data() {
    return {
      block: null
    };
  },
  methods: {
    async getLatestBlock() {
      const { data } = await fetchLatestBlock();
      this.block = data;
    }
  }
};
</script>
