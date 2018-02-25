<template>
<div slot="entry">
<div class="modal-header">
    <slot name="header">
      {{ users_nb }} inscrit(s) en ligne
    </slot>
  </div>
  <div class="modal-body">
      <label for="login">Identifiant : </label>
      <input id="login" type="text" @input="chooseLogin" />
      <button :disabled="isDisabled" v-on:click="connexion">
        Connexion
      </button>
  </div>
</div>
</template>

<script>
import { user } from '../js/user'

export default {
  name: 'entry',
  props: {
    users_nb: Number
  },
  data () {
    return {
      isDisabled: 'disabled'
    }
  },
  methods: {
    connexion () {
      this.$emit('connexion')
    },
    chooseLogin: function (e) {
      user.nick = e.target.value
      if (e.target.value)
        this.isDisabled = null
      else
        this.isDisabled = 'disabled'
    }
  }
}
</script>
