import { user, play, refreshGrid } from './user'
import server from './server'

export default function write(t) {
  console.log(t)
}

class WS {
  constructor (port) {
    let websocket = new WebSocket(
      'ws:/' + location.hostname + ':' + port
    )
    this.websocket = websocket
    websocket.onopen = function (evt) {
      server.enabled = true   
      let wrapper = JSON.stringify({
        'path': 'connected',
        'content': {}
      })
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
    let data = JSON.parse(evt.data)
    if (data.path === 'connected')
      return WS.on_connected(data.user_id, data.users_nb)
    else if (data.path === 'has_joined') {
      if (data.user.ws_id == user.id) {
        return WS.get_list_users(this)
      }
      else {
          return WS.add_on_user_list(data.user)
      }
    }
    else if (data.path === 'user_list') {
      return WS.on_user_list(data.users)
    }
    else if (data.path === 'game_request') {
      return WS.on_game_request(data)
    }
    else if (data.path === 'wait_agreement') {
      return WS.on_wait_agreement(data)
    }
    else if (data.path === 'game_start') {
      return WS.on_game_start(data)
    }
    else if (data.path === 'play') {
      return WS.on_play(data)
    }
    else if (data.path === 'has_played') {
      return WS.on_has_played(data)
    }
    else if (data.path === 'game_over') {
      return WS.on_game_over(data)
    }
    else if (data.path === 'win') {
      return WS.on_win(data)
    }
    else {
      write('Le serveur envoie (' + evt.data + ')')
    }
  }
  
  static on_connected (user_id, users_nb) {
    user.id = user_id
    server.users_nb = users_nb
  }
  
  static get_list_users (websocket) {
    let wrapper = JSON.stringify({
      'path': 'user_list',
      'content': {}
    })
    websocket.send(wrapper)
  }
  
  static on_game_request (data) {
    user.hasWin         = false
    user.hasLoose       = false
    user.opponent_nick  = data.opponent_nick
    user.opponent_id    = data.opponent_id
    user.agree_question = true
  }

  static on_wait_agreement (data) {
    user.hasWin         = false
    user.hasLoose       = false
    user.opponent_nick  = data.opponent_nick
    user.opponent_id    = data.opponent_id
    user.wait_agree = true
  }
  static on_user_list (users) {
    if (users) {  
      server.users = users
    }
  }
  
  static add_on_user_list(user) {
     server.users.push(user)
  }
  
  static on_game_start(data) {
    refreshGrid()
    let second_user = null
    if (user.id == data.user.id) {
      user.color          = data.user.color
      user.opponent_color = data.opponent.color
      user.wait_playing   = false
      second_user         = data.opponent
    }
    if (user.id == data.opponent.id) {
      user.color          = data.opponent.color
      user.opponent_color = data.user.color
      user.wait_playing   = true
      second_user         = data.user
    }
    if (!second_user) {
      return
    }
    user.game_id        = data.game_id
    user.opponent_nick  = second_user.nick
    user.opponent_id    = second_user.id
    user.agree_question = false
    user.wait_agree     = false
    user.wait_opponent  = false
  }
  
  static on_play(data) {
    play(user.opponent_color, data.x)
    user.wait_playing  = false
  }

  static on_has_played(data) {
  }
  
  static on_game_over(data) {
    play(user.opponent_color, data.x)
    user.wait_opponent = true
    user.wait_playing  = false
    user.hasLoose      = true
  }

  static on_win(data) {
    user.wait_opponent = true
    user.wait_playing  = false
    user.hasWin        = true
  }
  
  get_id () {
    let wrapper = JSON.stringify({
      'path': 'get_id',
      'content': {}
    })
    this.websocket.send(wrapper)
  }
  
  join (nickname) {
    let join    = { 'join_nick': nickname }
    let wrapper = JSON.stringify({
      'path': 'join',
      'content': join
    })
    this.websocket.send(wrapper)
  }
  
  play_with (id, nick, opponent_nick) {
    let new_user = {
      'user_id': id, 
      'nick': nick,
      'opponent_nick': opponent_nick 
    }
    let wrapper = JSON.stringify({
      'path': 'play_with',
      'content': new_user
    })
    this.websocket.send(wrapper) 
  }
  
  agree (response) {
    let new_user = {
      'nick': user.nick,
      'opponent_id': user.opponent_id,
      'opponent_nick': user.opponent_nick,
      'response': response
    }
    let wrapper = JSON.stringify({
      'path': 'agree',
      'content': new_user
    })
    this.websocket.send(wrapper) 
  }
  
  play (col_index, new_y, color) {
    let game = {
      'game_id': user.game_id,
      'user_id': user.id,
      'disc_x':  col_index,
      'disc_y':  new_y,
      'color':   color
    }
    let wrapper = JSON.stringify({
      'path': 'play',
      'content': game
    })
    this.websocket.send(wrapper)
  }

  play_random_user () {
    let wrapper = JSON.stringify({
      'path': 'play_random_user',
      'content': {}
    })
    this.websocket.send(wrapper) 
  }
}

let ws = new WS(3012)

export { ws }
