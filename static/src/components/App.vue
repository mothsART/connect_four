<template>
  <modal></modal>
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
