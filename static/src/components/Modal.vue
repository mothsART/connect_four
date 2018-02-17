<template>
    <div class="modal-mask" v-if="modalIsShow">
      <div class="modal-wrapper">
        <div class="modal-container">
            <template v-if="!server.enabled">
                <connexion-error>
                  <slot name="connexion-error"></slot>
                </connexion-error>
            </template>
            <template v-if="entryIsShow">
                <entry v-on:selectOpponent="getOpponent" v-on:selectRandomOpponent="getRandomOpponent" :users_nb="server.users_nb">
                  <slot></slot>
                </entry>
            </template>
            <template v-if="showUserList">
                <user-list :users="server.users">
                    <slot name="user-list"></slot>
                </user-list>
            </template>
            <template v-if="agreeQuestion">
                <div>
                    <p><strong>{{ user.opponent_nick }}</strong> souhaites jouer avec vous ?</p>
                    <div>
                       <button @click="refuse()">
                        Refuser
                      </button>
                      <button @click="accept()">
                        Accepter
                      </button>
                    </div>
                </div>
            </template>
            <template v-if="waitAgree">
                <div>En attente d'accord de <strong>{{ user.opponent_nick }}</strong></div>
            </template>
            <template v-if="waitPlaying">
                <div>En attente du jeu adverse ...</div>
            </template>
            <template v-if="hasWin">
                <div>Vous avez gagné, bravo !</div>
                <div>
                   <button @click="findOpponent()">
                    Rechercher un autre adversaire
                  </button>
                  <button @click="replaySameOpponent()">
                    Rejouer contre le même adversaire
                  </button>
                </div>
            </template>
            <template v-if="user.hasLoose">
                <div>Vous avez perdu.</div>
                <div>
                   <button @click="findOpponent()">
                    Rechercher un autre adversaire
                  </button>
                  <button @click="replaySameOpponent()">
                    Rejouer contre le même adversaire
                  </button>
                </div>
            </template>
        </div>
      </div>
    </div>
</template>

<script>
import { user }       from '../js/user'
import server         from '../js/server'
import { ws }         from '../js/ws'
import ConnexionError from './ConnexionError.vue'
import Entry          from './Entry.vue'
import UserList       from './UserList.vue'

export default {
  name: 'modal',
  components: { ConnexionError, Entry, UserList },
  data: function () {
    return {
      user: user,
      server: server,
      showEntry: true,
      showUserList: false
    }
  },
  computed: {
    modalIsShow: function() {
      return user.wait_opponent || user.wait_playing
    },
    entryIsShow: function () {
      return this.server.enabled && this.showEntry
    },
    agreeQuestion: function () {
      this.showUserList = false
      return this.user.agree_question
    },
    waitAgree: function () {
      this.showUserList = false
      return this.user.wait_agree
    },
    waitPlaying: function () {
      return this.user.wait_playing
    },
    hasWin: function () {
	  return this.user.hasWin
	}  
  },
  methods: {
    getOpponent: function () {
      this.showEntry = false
      this.showUserList = true
    },
    getRandomOpponent: function() {
      this.showEntry = false
    },
    refuse() {
         ws.agree(false)
    },
    accept() {
        ws.agree(true)
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
