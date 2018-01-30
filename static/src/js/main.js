import Vue from 'vue'
import server from '../js/server'
import write from './ws'
import scss from '../scss/app.scss'
import App from '../components/App.vue'


let vueStartDialog = new Vue({
  el: '#app',
  render: h => h(App)
})

console.log(vueStartDialog)
