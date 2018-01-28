import user from './user'

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
