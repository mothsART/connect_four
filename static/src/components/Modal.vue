<template>
    <div class="modal-mask">
      <div class="modal-wrapper">
        <div class="modal-container">
            <template v-if="!server.enabled">
                <connexion-error>
                  <slot name="connexion-error"></slot>
                </connexion-error>
            </template>
            <template v-if="isShow">
                <entry v-on:selectOpponent="getOpponent" v-on:selectRandomOpponent="getRandomOpponent" :users_nb="server.users_nb">
                  <slot></slot>
                </entry>
            </template>
            <template v-if="showUserList">
                <user-list>
                    <slot name="user-list"></slot>
                </user-list>
            </template>
        </div>
      </div>
    </div>
</template>

<script>
import server from '../js/server'
import ConnexionError from './ConnexionError.vue'
import Entry from './Entry.vue'
import UserList from './UserList.vue'

export default {
  name: 'modal',
  components: { ConnexionError, Entry, UserList },
  data: function () {
    return {
      server: server,
      showEntry: true,
      showUserList: false
    }
  },
  computed: {
    isShow: function () {
      return this.server.enabled && this.showEntry
    }
  },
  methods: {
    getOpponent: function () {
      this.showEntry = false
      this.showUserList = true
    },
    getRandomOpponent: function() {
      this.showEntry = false
    }
  }
}
</script>

<style lang="scss">
  .modal-mask {
    position: fixed;
    z-index: 9998;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, .5);
    display: table;
    transition: opacity .3s ease;
  }
  
  .modal-wrapper {
    display: table-cell;
    vertical-align: middle;
  }
  
  .modal-container {
    width: 500px;
    text-align: center;
    margin: 0px auto;
    padding: 1rem;
    background-color: #fff;
    border-radius: 2px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, .33);
    transition: all .3s ease;
    font-family: Helvetica, Arial, sans-serif;
  }
  
  .modal-body {
    margin: 20px 0;
  }
  
  .modal-footer {
    padding-bottom: 1rem;
  }
  
  button.left {
    float: left;
  }
  
  button.right {
    float: right;
  }
  
  .modal-enter {
    opacity: 0;
  }
  
  .modal-leave-active {
    opacity: 0;
  }
  
  .modal-enter .modal-container,
  .modal-leave-active .modal-container {
    transform: scale(1.1);
  }
</style>