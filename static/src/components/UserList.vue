<template>
  <table >
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
            <button>Jouer</button>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<script>
import server from '../js/server'
import { ws } from '../js/ws'

export default {
  name: 'user-list',
  props: {
    users: Array
  },
  mounted: function() {
     ws.get_list_users()
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
