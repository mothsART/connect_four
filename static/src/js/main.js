import Vue from 'vue'
import write from './ws'
import scss from '../scss/app.scss'
import App from '../components/App.vue'

class WS {
  constructor (port) {
    let websocket = new WebSocket('ws:/' + location.hostname + ':' + port)
    this.websocket = websocket
    websocket.onopen = function (evt) {
      vueStartDialog.$children[0].server.enabled = true
      //vueStartDialog.server.enabled = true
      let wrapper = JSON.stringify({'path': 'connected'})
      websocket.send(wrapper)
    }
    this.result = websocket.onmessage = this.onmessage
    websocket.onerror = function (evt) {
      write(evt)
    }
    this.websocket.onclose = function (evt) {
      write(evt)
    }
  }
  onmessage(evt) {
    try {
      let data = JSON.parse(evt.data)
      if (data.path === 'connected')
          WS.on_connected(data.users_nb)
    }
    catch (e) {
      write('Le serveur envoie (' + evt.data + ')')
    }
  }
  
  static on_connected (users_nb) {
    vueStartDialog.$children[0].setUserNb(user_nb)
    //vueStartDialog.setUserNb(users_nb)
  }

  join (nickname) {
    let join    = {'join_nick': nickname}
    let wrapper = JSON.stringify({'path': 'joined', 'content': join})
    this.websocket.send(wrapper)
  }
}

new WS(3012)

let vueStartDialog = new Vue({
  el: '#app',
  render: h => h(App)
})
