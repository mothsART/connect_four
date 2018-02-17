<template>
  <div>
    <table>
      <thead>
        <tr>
          <th>Utilisateur(s) connecté(s)</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="user in users">
          <td>
            {{ user.login }}
          </td>
          <td>
              <button @click="play_with(user.ws_id, user.login)">Jouer</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import { user } from '../js/user'
import server from '../js/server'
import { ws } from '../js/ws'

export default {
  name: 'user-list',
  props: {
    users: Array
  },
  mounted: function() {
     ws.join(user.nick)
  },
  methods: {
    play_with (id, opponent_nick) {
      ws.play_with(id, user.nick, opponent_nick)
    }
  }
}
</script>

<style lang="scss">
  table {
    border-collapse: collapse;
    border: solid 1px grey;
    td {
    border: solid 1px grey;
    }
  }
</style>
