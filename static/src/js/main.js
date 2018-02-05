import Vue from 'vue'
import server from '../js/server'
import scss from '../scss/app.scss'
import App from '../components/App.vue'
import Grid from '../components/Grid.vue'


new Vue({
  el: '#app',
  render: h => h(App)
})

new Vue({
  el: '#grid',
  render: h => h(Grid)
})
