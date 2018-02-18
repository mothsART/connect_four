let grid = [
  [null, null, null, null, null, null],
  [null, null, null, null, null, null],
  [null, null, null, null, null, null],
  [null, null, null, null, null, null],
  [null, null, null, null, null, null],
  [null, null, null, null, null, null],
  [null, null, null, null, null, null]
]

class User {
  constructor () {
    this.id             = 0
    this.ws_id          = 0
    this.nick           = null
    this.opponent_id    = 0
    this.opponent_nick  = null
    this.color          = null
    this.opponent_color = null
    this.agree_question = false
    this.wait_agree     = false
    this.wait_opponent  = true
    this.wait_playing   = false
    this.game_id        = 1
    this.hasWin         = false
    this.hasLoose       = false
    this.grid           = grid
    this.game_id        = 0
  }
}

let play = function (color, col_index) {
  if (!color) {
    throw "color is not defined !"
  }
  var col      = user.grid[col_index - 1].slice(0)
  var parent   = document.getElementById('discs')
  var disc     = document.getElementById('disc-template').cloneNode(true)
  disc.classList.add('playingDisc')
  disc.getElementsByClassName('disc')[0].classList.add(color.toLowerCase())
  var deplac_x = 103.833333333
  var deplac_y = 104.4
  var new_y    = 0
  var nb_col   = col.length
  col.reverse().every(function(element, index) {
    if(element != null)
      return true
    new_y = nb_col - index - 1
  })
  user.grid[col_index - 1][new_y] = color
  disc.setAttribute(
    'style',
    'opacity: 1;'
    + 'transform: translate('
      + (col_index - 1) * deplac_x + 'px,'
      + deplac_y * new_y + 'px);'
  )
  parent.appendChild(disc)
  return new_y
}

let refreshGrid = function () {
  var elements = document.getElementsByClassName('playingDisc')
  while(elements.length > 0) {
    elements[0].parentNode.removeChild(elements[0])
  }
  // initialize grid on cloning
  user.grid = JSON.parse(JSON.stringify(grid))
}

let user = new User()
export { user, play, refreshGrid }
