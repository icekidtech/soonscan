<template>
  <div>
    <h1>Transaction Dashboard</h1>
    <ValidateTransaction />
    <TransactionExplorer />
  </div>
</template>

<script>
import ValidateTransaction from '../components/ValidateTransaction.vue';
import TransactionExplorer from '../components/TransactionExplorer.vue';

export default {
  components: { ValidateTransaction, TransactionExplorer }
};
</script>
