import 'vue'

Vue.component('modal', {
  template: '#modal-template',
  data: function () {
    return {
      showEntry: true,
      showUserList: false
    }
  },
  methods: {
    getOpponent: function () {
      this.showEntry = false;
      this.showUserList = true;
    }
  }
})

Vue.component('entry', {
  template: '#entry-template',
  data: function () {
    return {
      users: 1000,
      isDisabled: 'disabled'
    }
  },
  methods: {
    selectOpponent: function () {
      this.$emit('select');
    },
    chooseLogin: function (e) {
      if (e.target.value)
        this.isDisabled = null;
      else
        this.isDisabled = 'disabled';
    }
  }
})

Vue.component('user-list', {
  template: '#user-list-template',
  data: function () {
    return {
      logins: [
        "Déborah",
        "Papounet"
      ]      
    }
  }
})

// start app
var app = new Vue({
  el: '#app',
  created: function() {
    const port = 3012;
    new ws.WS(port);
  },
  data: {
    showUserList: false,
    showModal: true
  },
  methods: {
    getOpponent: function () {
      debugger;
    }
  }
})

module.exports = {
  app: app
}
