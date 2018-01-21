function write(t) {
  "use strict";
  console.log(t);
};

class WS {
  constructor(port) {
    let websocket = new WebSocket('ws:/' + location.hostname + ':' + port);
    this.websocket = websocket;
    websocket.onopen = function(evt) {
      let wrapper = JSON.stringify({'path': 'connected'});
      websocket.send(wrapper);
    };
    websocket.onmessage = function(evt) {
      try {
        let data = JSON.parse(evt.data);
        window["on_" + data.path](data);
      }
      catch (e) {
        write('Le serveur envoie (' + evt.data + ')');
      }
    };
    websocket.onerror = function(evt){
      write(evt);
    };
    websocket.onclose = function (evt) {
      write(evt);
    };
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
