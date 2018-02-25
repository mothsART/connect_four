<template>
  <div v-bind:class="{ play: isPlaying }" class="header">
    <p v-if="playerIsSelected" v-bind:class="user.color" class="user">{{ user.nick }}</p>
    <p v-if="gameInProgress" class="middle">contre</p>
    <p v-if="gameInProgress" v-bind:class="user.opponent_color" class="opponent">{{ user.opponent_nick }}</p>
  </div>
</template>

<script>
import { user } from '../js/user'
import server from '../js/server'
import { ws } from '../js/ws'

export default {
  name: 'appheader',
  data: function () {
    return {
      user: user
    }
  },
  computed: {
    playerIsSelected: function() {
       return !user.showEntry
    },
    isPlaying: function () {
      return user.game_in_progress && !user.wait_playing
    },
    gameInProgress: function () {
      return user.game_in_progress
    }
  }
}
</script>

<style lang="scss">
  $yellow: #ffb300;
  $red:    #dd2c00;
  $lblue:  #d7e0e2;
  
  .header {
    height: 2.5rem;
    display: flex;
    justify-content: space-between;
    box-sizing: border-box;
    position: relative;
    padding: 0 .5rem;
    &.play {
      border-bottom : solid 1px $lblue;
    }
    p {
      margin-top: .5rem;
      &.user {
        font-weight: bold;
      }
      &.user.Yellow, &.opponent.Yellow {
        padding-left: 2.5rem;
        color: $yellow;
      }
      &.user.Yellow:before, &.opponent.Yellow:after {
        content: "";
        height: 2rem;
        width: 2rem;
        background-color: $yellow;
        position: absolute;
        margin-top: -.5rem;
        margin-left: -2.5rem;
        border-radius: 2rem;
      }
      &.user.Red, &.opponent.Red {
        padding-left: 2.5rem;
        color: $red;
      }
      &.user.Red:before, &.opponent.Red:after {
        content: "";
        height: 2rem;
        width: 2rem;
        background-color: $red;
        position: absolute;
        margin-top: -.5rem;
        margin-left: -2.5rem;
        border-radius: 2rem;
      }
      &.opponent.Yellow, &.opponent.Red {
        padding-right: 2.5rem;
        padding-left: 0;
        &:after {
          right: .5rem;
        }
      }
    }
  }
</style>
