<template>
  <div id="app">
    <modal v-if="showModal" @close="showModal = false" :server="server">
	  <connexion-error></connexion-error>
	  <entry></entry>
	  <user-list></user-list>
    </modal>
  </div>
</template>

<script>
import server from '../js/server'
import Modal from './Modal.vue'

export default {
  name: 'app',
  components: { Modal },
  data () {
    return {
      server: server,
      users_nb: 0,
      showModal: true      
    }
  },
  methods: {
    setUserNb: function(users_nb) {
      this.server.users_nb = users_nb
    }
  }
}

</script>

<style lang="scss">
body {
    overflow: hidden;
    font-family: sans-serif;
    color: #0f3c4b;
    padding: 0;
}

#content {
    display: flex;
    flex-direction: column;
    margin: 5px;
    height: calc(100vh - 10px);
}

svg {
    height: calc(100vh - 20px);
}

#arrow, #disc-template {
    opacity: 0;
}

#arrow polygon {
    fill: gold;
}

#masks rect {
    fill: transparent;
}

.disc.yellow {
   fill: #ffae00;
}

.disc.red {
   fill: red;
}

</style>
