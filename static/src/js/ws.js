import { user } from './user'
import server from './server'

function on_join(nickname) {
  console.log("user : " + nickname + " joined game")
}

function on_play(data) {
  "use strict"
  console.log("play!")
  play(user.opponent_color, data.x)
  user.wait_playing  = false
}

function on_wait(data) {
  console.log("wait !")
}

function on_win(data) {
  console.log("you win!")
  user.wait_opponent = true
}

function on_game_over(data) {
  console.log("game over!")
  play(user.opponent_color, data.x)
  user.wait_opponent = true
}

function on_has_played(data) {
  console.log("has played!")
}

export default function write(t) {
  console.log(t)
};

function on_connected(nickname) {
  console.log("user : " + nickname + " joined game")
}

function on_get_users() {
    server.users = [ "dfhdhg"]
}

class WS {
  constructor (port) {
    let websocket = new WebSocket('ws:/' + location.hostname + ':' + port)
    this.websocket = websocket
    websocket.onopen = function (evt) {
      server.enabled = true   
      let wrapper = JSON.stringify({'path': 'connected', 'content': {}})
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
        return WS.on_connected(data.user_id, data.users_nb)
      else if (data.path === 'has_joined') {
        return WS.get_list_users(this)
      }
      else if (data.path === 'user_list') {
        return WS.on_user_list(data.users)
      }
      else if (data.path === 'game_start') {   
        return WS.on_game_start(data)
      }
    }
    catch (e) {
      write('Le serveur envoie (' + evt.data + ')')
    }
  }
  
  static on_connected (user_id, users_nb) {
    user.id = user_id
    server.users_nb = users_nb
  }
  
  static get_list_users (websocket) {
    let wrapper = JSON.stringify({'path': 'user_list', 'content': {}})
    websocket.send(wrapper)
  }
  
  static on_user_list (users) {
      let index = users.findIndex(function (element) {
        if (element.ws_id == user.id) {
          return element
        }
      })
      server.users = users.splice(0, index)
  }
  
  static on_game_start(data) {
      console.log("game start ! user play : " + !data.begin)
      user.color          = data.color
      user.opponent_color = data.opponent_color
      user.opponent_nick  = data.opponent
      user.wait_opponent  = false
      user.wait_playing   = false
      console.log(user)
      debugger
  }

  join (nickname) {
    let join    = { 'join_nick': nickname }
    let wrapper = JSON.stringify({'path': 'join', 'content': join})
    this.websocket.send(wrapper)
  }
  
  play_with (id, nick, opponent_nick) {
    let user    = { 'user_id': id, 'nick': nick, 'opponent_nick': opponent_nick }
    let wrapper = JSON.stringify({'path': 'play_with', 'content': user})
    this.websocket.send(wrapper) 
  }
  
  play_random_user () {
    let wrapper = JSON.stringify({'path': 'play_random_user', 'content': {}})
    this.websocket.send(wrapper) 
  }
}

let ws = new WS(3012)

export { ws }
