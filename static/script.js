function write(t) {
  "use strict";
  console.log(t);
};

var websocket = null;

function ws() {
    "use strict";
    websocket = new WebSocket('ws://127.0.0.1:3012');
    websocket.onopen = function(evt) {
      write('Connected');
      var message = {'nick': 'mothsart', 'message': 'coucou'};
      var join    = {'join_nick': 'mothsart'};
      var wrapper = JSON.stringify({'path': 'joined', 'content': message});
      write(wrapper);
      websocket.send(wrapper);
    };
    websocket.onmessage = function(evt) {
      write('Le serveur envoie (' + evt.data + ')');
    };
    websocket.onerror = function(evt){
      console.log(evt);
    };

    websocket.onclose = function (evt) {
      console.log(evt);
    };
}