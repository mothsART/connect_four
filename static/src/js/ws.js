function on_join(nickname) {
  "use strict";
  console.log("user : " + nickname + " joined game");
}

function on_game_start(data) {
  "use strict";
  console.log("game start ! user play : " + !data.begin);
  user.color          = data.color;
  user.opponent_color = data.opponent_color;
  user.opponent_nick  = data.opponent;
  user.wait_opponent  = false;
  user.wait_playing   = !data.begin;
}

function on_play(data) {
  "use strict";
  console.log("play!");
  play(user.opponent_color, data.x);
  user.wait_playing  = false;
}

function on_wait(data) {
  "use strict";
  console.log("wait !");
}

function on_win(data) {
  "use strict";
  console.log("you win!");
  user.wait_opponent  = true;
}

function on_game_over(data) {
  "use strict";
  console.log("game over!");
  play(user.opponent_color, data.x);
  user.wait_opponent  = true;
}

function on_has_played(data) {
  "use strict";
  console.log("has played!");
}

function write(t) {
  "use strict";
  console.log(t);
};

function on_connected(nickname) {
  console.log("user : " + nickname + " joined game");
}

class WS {
  constructor(port) {
    let websocket = new WebSocket('ws:/' + location.hostname + ':' + port);
    this.websocket = websocket;
    websocket.onopen = function(evt) {
      let wrapper = JSON.stringify({'path': 'connected'});
      websocket.send(wrapper);
    }
    this.result = websocket.onmessage = this.onmessage;
    websocket.onerror = function(evt){
      write(evt);
    };
    websocket.onclose = function (evt) {
      write(evt);
    };
  }
  
  onmessage(evt) {
      try {
        let data = JSON.parse(evt.data);
        if (data.path == 'connected')
            WS.on_connected(data.users_nb);
            //ws.WS.on_connected(data.users_nb);
        /*debugger;
        window["this.on_" + data.path](data);
        */
      }
      catch (e) {
        write('Le serveur envoie (' + evt.data + ')');
      }
    }
  static on_connected(users_nb) {
    modal.app.setUserNb(users_nb);
  }

  join(nickname) {
      let join    = {'join_nick': nickname};
      let wrapper = JSON.stringify({'path': 'joined', 'content': join});
      this.websocket.send(wrapper);
  }
}

module.exports = {
  write: write,
  WS: WS
};
