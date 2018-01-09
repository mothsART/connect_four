var websocket = null;

var grid = [
  [null, null, null, null, null, null],
  [null, null, null, null, null, null],
  [null, null, null, null, null, null],
  [null, null, null, null, null, null],
  [null, null, null, null, null, null],
  [null, null, null, null, null, null],
  [null, null, null, null, null, null]
]

var user = {
  nick: null,
  opponent_nick: null,
  color: null,
  opponent_color: null,
  wait_opponent: true,
  wait_playing: true
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

function ws(nick) {
    "use strict";
    websocket = new WebSocket('ws:/' + location.hostname + ':3012');
    websocket.onopen = function(evt) {
      var join    = {'join_nick': nick};
      var wrapper = JSON.stringify({'path': 'joined', 'content': join});
      websocket.send(wrapper);
    };
    websocket.onmessage = function(evt) {
      try {
        var data = JSON.parse(evt.data);
        window["on_" + data.path](data);
      }
      catch (e) {
        write('Le serveur envoie (' + evt.data + ')');
      }
    };
    websocket.onerror = function(evt){
      console.log(evt);
    };
    websocket.onclose = function (evt) {
      console.log(evt);
    };
}

function col_over(element) {
  "use strict";
  var col_index = parseInt(element.id.substr(4));
  document.getElementById('arrow').style.opacity = 1;
  var arrow_x = 112 + 104 * (col_index - 2);
  if (col_index == 1)
    arrow_x = 0;
  document.getElementById('arrow').style.transform = "translate(" + arrow_x + "px, 0)";
}

function col_out() {
  "use strict";
  document.getElementById('arrow').style.opacity = 0;
}

function play(color, col_index) {
  "use strict";
  var col      = grid[col_index - 1].slice(0);
  var parent   = document.getElementById('discs');
  var disc     = document.getElementById('disc-template').cloneNode(true);
  disc.getElementsByClassName('disc')[0].classList.add(color.toLowerCase());
  var deplac_x = 103.833333333;
  var deplac_y = 104.4;
  var new_y    = 0;
  var nb_col   = col.length;
  col.reverse().every(function(element, index) {
    if(element != null)
      return true;
    new_y = nb_col - index - 1;
  });
  grid[col_index - 1][new_y] = color;
  disc.setAttribute(
    'style',
    'opacity: 1;'
    + 'transform: translate('
      + (col_index - 1) * deplac_x + 'px,'
      + deplac_y * new_y + 'px);'
  );
  parent.appendChild(disc);
  return new_y;
}

function col_clic(element) {
  "use strict";
  if (user.wait_playing)
    return;
  var col_index = parseInt(element.id.substr(4));
  var new_y = play(user.color, col_index);
  user.wait_playing = true;
  websocket.send(JSON.stringify({
    'path': 'play',
    'content': {
      'id': 0,
      'disc_x': col_index,
      'disc_y': new_y 
    }
  }));
}

function write(t) {
  "use strict";
  console.log(t);
};
