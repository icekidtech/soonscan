import Vue from 'vue';
import Router from 'vue-router';
import Home from '../views/Home.vue';
import Transaction from '../views/Transaction.vue';

Vue.use(Router);

export default new Router({
  mode: 'history',
  routes: [
    { path: '/', name: 'Home', component: Home },
    { path: '/transaction', name: 'Transaction', component: Transaction },
  ]
});
