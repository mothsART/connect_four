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
  </div>
  <div class="modal-footer">
      <button class="left" :disabled="isDisabled" v-on:click="selectOpponent">
        Sélectionner un adversaire
      </button>
      <button class="right" :disabled="isDisabled" @click="selectRandomOpponent">
        Choisir un adversaire au hasard
      </button>
  </div>
</div>
</template>

<script>
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
    selectOpponent () {
      this.$emit('selectOpponent')
    },
    selectRandomOpponent () {
      this.$emit('selectRandomOpponent')
    },
    chooseLogin: function (e) {
      if (e.target.value)
        this.isDisabled = null
      else
        this.isDisabled = 'disabled';
    }
  }
}
</script>
