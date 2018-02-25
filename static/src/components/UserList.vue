<template>
  <div class="userListContainer">
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
    <button class="right" @click="selectRandomOpponent">
        Choisir un adversaire au hasard
    </button>
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
    },
    selectRandomOpponent () {
      this.$emit('selectRandomOpponent')
    }
  }
}
</script>

<style lang="scss">
  .userListContainer {
    position: relative;
    table {
      border-collapse: collapse;
      border: solid 1px grey;
      td {
        border: solid 1px grey;
      }
    }
    
    button.right {
      position: absolute;
      top: 0;
      right: 0;
    }
  }
</style>
