class URLConfig {
  constructor() {
    this.isDebug = window.location.protocol === 'http:';
    this.host = window.location.host;
    this.wsProtocol = this.isDebug ? 'ws' : 'wss';
    this.httpProtocol = this.isDebug ? 'http' : 'https';
  }

  getWS(path) {
    return `${this.wsProtocol}://${this.host}${path}`;
  }

  getHTTP(path) {
    return `${this.httpProtocol}://${this.host}${path}`;
  }
}

const URL = new URLConfig();

class ChatClient {
  constructor(nick) {
    this.currentChannel = "general";
    this.socket = null;
    this.nick = nick;
  }

  setNick(newNick) {
    this.nick = newNick;
  }

  connect() {
    const wsURL = URL.getWS(`/chatroom?channel=general`);
    this.socket = new WebSocket(wsURL);
    this.setupSocketHandlers();
    this.setupMessageForm();
  }

  setupSocketHandlers() {
    this.socket.onopen = e => console.log("[open] Connection established");

    this.socket.onmessage = e => {
      let data = JSON.parse(e.data);
      let chat = document.getElementById('chat');
      chat.innerHTML += `<p><strong>${data.nick}</strong>: ${data.content}</p>`;
      chat.scrollTop = chat.scrollHeight;
    }

    this.socket.onclose = function(event) {
      if (event.wasClean) {
        console.log(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
      } else {
        console.log('[close] Connection died');
      }
    };

    this.socket.onerror = function(error) {
      console.log(`[error]: ${error.message}`);
    };
  }

  setupMessageForm() {
    let form = document.getElementById('messagesend');
    let message = document.getElementById('message');
    form.addEventListener('submit', (e) => {
      e.preventDefault();
      // console.log(`Sending message ${message.value}`);
      this.socket.send(JSON.stringify({nick: this.nick, content: message.value}));
      message.value = '';
    });
  }

}

function choose(arr){
  return arr[Math.floor(Math.random() * arr.length)];
}

var namedata = {
  animals: ["lion", "bear", "octopus", "goose"],
  adjectives: ["bright", "wise", "silly"]
};

fetch(URL.getHTTP('/namedata.json'))
  .then((e) => e.json())
  .then((nd) => {
    namedata = nd;
  })
  .catch(error => console.error('Error fetching namedata:', error));

function capitalizeFirstLetter(val) {
  return String(val).charAt(0).toUpperCase() + String(val).slice(1);
}

function randomName(namedata){
  let animal = capitalizeFirstLetter(choose(namedata.animals));
  let adjective = capitalizeFirstLetter(choose(namedata.adjectives));
  let number = Math.floor(Math.random() * 99);
  return adjective + animal + number;
}



function setupNick(cb, nick) {
  nickfield = document.getElementById('nick')
  nickfield.addEventListener('input', (e) => {
    cb(e.target.value);
  });
  nickfield.placeholder = nick;
}

function setupRandomizeButton(cb) {
  let button = document.getElementById('randomise');
  button.addEventListener('click', (e) => {
    let nick = randomName(namedata); 
    cb(nick);
    nickfield = document.getElementById('nick')
    nickfield.value = nick;
    nickfield.placeholder = nick;
  });
}

window.addEventListener("DOMContentLoaded", (event) => {
  let nick = randomName(namedata);
  const chatClient = new ChatClient(nick);

  setupNick((newNick) => chatClient.setNick(newNick), nick);
  setupRandomizeButton((newNick) => chatClient.setNick(newNick));
  
  setupNewChannelForm(chatClient);
  chatClient.connect();
});