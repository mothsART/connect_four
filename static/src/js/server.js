class Server {
  constructor (port) {
    this.enabled = false
    this.users_nb = 0
    this.users = []
  }
}

let server = new Server()
export default server
