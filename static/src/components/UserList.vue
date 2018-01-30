<template>
  <table slot="user-list">
    <thead>
      <tr>
        <th>Utilisateur(s) connecté(s)</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="ligne in logins">
        <td>
          {{ ligne }}
        </td>
      </tr>
    </tbody>
  </table>
</template>

<script>
import server from '../js/server'

export default {
  name: 'user-list',
  data: function () {
    return {
      logins: [
        "Déborah",
        "Papounet"
      ]      
    }
  },
  mounted: {
  
  }
}
</script>
