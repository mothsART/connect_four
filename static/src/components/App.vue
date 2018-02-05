<template>
  <div>
    <modal></modal>
  </div>
</template>

<script>
import Modal from './Modal.vue'

export default {
  name: 'app',
  components: { Modal }
}
</script>

<style lang="scss">
  body {
      overflow: hidden;
      font-family: sans-serif;
      color: #0f3c4b;
      padding: 0;
  }
</style>
