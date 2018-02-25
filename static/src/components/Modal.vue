<template>
    <div v-bind:class="{ active: isActive }"
         class="modal-mask"
         v-if="modalIsShow">
      <div class="modal-wrapper">
        <div class="modal-container">
            <template v-if="!server.enabled">
                <connexion-error>
                  <slot name="connexion-error"></slot>
                </connexion-error>
            </template>
            <template v-if="entryIsShow">
                <entry v-on:connexion="connexion"
                       :users_nb="server.users_nb">
                  <slot></slot>
                </entry>
            </template>
            <template v-if="user.showUserList">
                <user-list :users="server.users"
                           v-on:selectRandomOpponent="selectRandomOpponent">
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
                    Rejouer contre {{ user.opponent_nick }}
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
                    Rejouer contre {{ user.opponent_nick }}
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
      server: server
    }
  },
  computed: {
    modalIsShow: function() {
      return user.wait_opponent || user.wait_playing
    },
    entryIsShow: function () {
      return this.server.enabled && user.showEntry
    },
    agreeQuestion: function () {
      user.showUserList = false
      return this.user.agree_question
    },
    waitAgree: function () {
      user.showUserList = false
      return this.user.wait_agree
    },
    waitPlaying: function () {
      return this.user.wait_playing
    },
    hasWin: function () {
      return this.user.hasWin
    },
    isActive: function () {
      return !user.showEntry
    }
  },
  methods: {
    connexion: function () {
      user.showEntry = false
      user.showUserList = true
    },
    selectRandomOpponent: function() {
      ws.random_opponent()
      user.showEntry = false
    },
    refuse() {
      ws.agree(false)
    },
    accept() {
      ws.agree(true)
    },
    findOpponent() {
      user.showUserList = true
      user.hasWin       = false
      user.hasLoose     = false
    },
    replaySameOpponent() {
      ws.play_with(
        user.opponent_id,
        user.nick,
        user.opponent_nick
      )
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
    display: table;
    transition: opacity .3s ease;
    &:before {
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      content: "";
      position:fixed;
      background-color: rgba(0, 0, 0, .5);
    }
    &.active:before {
      top: 3rem;
    } 
  }
  
  .modal-wrapper {
    position: relative;
    display: table-cell;
    vertical-align: middle;
  }
  
  .modal-container {
    width: 500px;
    text-align: center;
    margin: 0px auto;
    padding: 1rem;
    background-color: rgba(255, 255, 255, 0.8);
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
