import user from './user'
import server from './server'

function on_join(nickname) {
  console.log("user : " + nickname + " joined game")
}

function on_game_start(data) {
  console.log("game start ! user play : " + !data.begin)
  user.color          = data.color;
  user.opponent_color = data.opponent_color
  user.opponent_nick  = data.opponent
  user.wait_opponent  = false
  user.wait_playing   = !data.begin
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

class WS {
  constructor (port) {
    let websocket = new WebSocket('ws:/' + location.hostname + ':' + port)
    this.websocket = websocket
    websocket.onopen = function (evt) {
      server.enabled = true
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
    server.users_nb = users_nb
  }

  join (nickname) {
    let join    = {'join_nick': nickname}
    let wrapper = JSON.stringify({'path': 'joined', 'content': join})
    this.websocket.send(wrapper)
  }
}

let ws = new WS(3012)
