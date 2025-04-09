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
  constructor() {
    this.urlConfig = new URLConfig();
    this.currentChannel = "general";
    this.socket = null;
    this.nick = randomName(namedata);
    this.channels = new Set(["general"]); // Track available channels
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

  updateChannelsList() {
    fetch(this.urlConfig.getHTTP('/channels'))
      .then(response => response.json())
      .then(channels => {
        this.channels = new Set(channels); // Update channels set
        this.renderChannelsList();
      })
      .catch(error => console.error('Error fetching channels:', error));
  }

  renderChannelsList() {
    const channelsList = document.getElementById('channels').querySelector('ul');
    channelsList.innerHTML = '';
    
    const fragment = document.createDocumentFragment();
    
    // Always show general channel first
    fragment.appendChild(this.createChannelListItem("general"));
    
    // Add other channels sorted alphabetically
    Array.from(this.channels)
      .filter(channel => channel !== "general")
      .sort()
      .forEach(channel => {
        fragment.appendChild(this.createChannelListItem(channel));
      });
    
    channelsList.appendChild(fragment);
  }

  createChannelListItem(channel) {
    const li = document.createElement('li');
    const a = document.createElement('a');
    
    a.href = '#';
    a.textContent = `#${channel}`;
    a.dataset.channel = channel;
    
    // Highlight current channel
    if (channel === this.currentChannel) {
      a.classList.add('active-channel');
    }
    
    a.addEventListener('click', (e) => {
      e.preventDefault();
      if (channel !== this.currentChannel) {
        this.switchChannel(channel);
      }
    });
    
    li.appendChild(a);
    return li;
  }

  switchChannel(channel) {
    const oldChannel = this.currentChannel;
    this.currentChannel = channel;
    
    // Update UI
    document.getElementById('current-channel-name').textContent = `#${channel}`;
    document.getElementById('chat').innerHTML = '';
    this.renderChannelsList(); // Re-render to update active channel
    
    // Send channel switch message if socket is open
    if (this.socket && this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(JSON.stringify({
        type: 'system',
        content: 'switch_channel',
        channel: channel
      }));
    }

    // Dispatch custom event for channel switch
    window.dispatchEvent(new CustomEvent('channelSwitch', {
      detail: { oldChannel, newChannel: channel }
    }));
  }

  // Add method to create a new channel
  createChannel(channelName) {
    channelName = channelName.toLowerCase().trim();
    if (!channelName) return;
    
    if (!this.channels.has(channelName)) {
      this.channels.add(channelName);
    }
    this.switchChannel(channelName);
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

function setupNewChannelForm(chatClient) {
  const form = document.getElementById('newchannel');
  const input = document.getElementById('channel');
  
  form.addEventListener('submit', (e) => {
    e.preventDefault();
    e.stopPropagation();
    const channelName = input.value.trim();
    if (channelName) {
      chatClient.createChannel(channelName);
      input.value = '';
    }
  });
}


window.addEventListener("DOMContentLoaded", (event) => {
  console.log("DOMContentLoaded");
  let nick = randomName(namedata);
  const chatClient = new ChatClient();

  setupNick((newNick) => chatClient.setNick(newNick), nick);
  setupRandomizeButton((newNick) => chatClient.setNick(newNick));
  
  setupNewChannelForm(chatClient);
  const channelname = document.getElementById('current-channel-name');
  channelname.textContent = `#${chatClient.currentChannel}`;
  
  chatClient.updateChannelsList();
  chatClient.connect();
});