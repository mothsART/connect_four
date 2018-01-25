import 'vue'

Vue.component('modal', {
  template: '#modal-template',
  props: {
    server: Object
  },
  data () {
    return {
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
      this.showEntry = false;
      this.showUserList = true;
    },
    getRandomOpponent: function() {
      this.showEntry = false;
    }
  }
})

Vue.component('connexion-error', {
  template: '#connexion-error'
})

Vue.component('entry', {
  template: '#entry-template',
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
      this.$emit('selectOpponent');
    },
    selectRandomOpponent () {
      this.$emit('selectRandomOpponent');
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
    server: server.server,
    users_nb: 0,
    showModal: true
  },
  methods: {
    setUserNb: function(users_nb) {
      this.server.users_nb = users_nb;
    }
  }
})

module.exports = {
  app: app
}
