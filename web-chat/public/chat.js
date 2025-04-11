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

class Bus {
  listen(event, callback) {
    document.addEventListener(event, (e) =>{
      console.log(event, e.detail);
      callback(e.detail);
    });
  }

  emit(event, data) {
    document.dispatchEvent(new CustomEvent(event, {detail: data}));
  }
}


class UI {
  constructor(bus) {
    this.bus = bus;
    this.chat = document.getElementById('chat');
    this.channels = document.getElementById('channels');
    this.currentChannelName = document.getElementById('current-channel-name');

    this.bus.listen('message', this.addMessage.bind(this));
    this.bus.listen('channels_updated', this.renderChannelsList.bind(this));
    this.bus.listen('channel_switched', this.switchChannel.bind(this));

    this.init();
  }

  init() {
    // Set up message form
    let form = document.getElementById('messagesend');
    let message = document.getElementById('message');
    form.addEventListener('submit', (e) => {
      e.preventDefault();
      // console.log(`Sending message ${message.value}`);
      this.bus.emit('send_message', { message: message.value });
    });


    // Set up new channel form
    let newChannelForm = document.getElementById('newchannel');
    let channel = document.querySelector('input#channel').value;
    newChannelForm.addEventListener('submit', (e) => {
      e.preventDefault();
      this.bus.emit('create_channel', { channel: channel });
    });
  }

  addMessage({ nick, content }) {
    this.chat.innerHTML += `<p><strong>${nick}</strong>: ${content}</p>`;
    this.chat.scrollTop = this.chat.scrollHeight;
  }

  switchChannel(e) {
    const { channel } = e.detail;
    this.currentChannelName.textContent = `#${channel}`;
    this.chat.innerHTML = '';
    this.renderChannelsList();
  }

  renderChannelsList(e) {
    const { channels, activeChannel } = e.detail;
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
        fragment.appendChild(this.createChannelListItem(channel, activeChannel));
      });
    
    channelsList.appendChild(fragment);
  }

  createChannelListItem(channel, activeChannel) {
    const li = document.createElement('li');
    const a = document.createElement('a');
    
    a.href = '#';
    a.textContent = `#${channel}`;
    a.dataset.channel = channel;
    
    // Highlight current channel
    if (channel === activeChannel) {
      a.classList.add('active-channel');
    }
    
    a.addEventListener('click', (e) => {
      e.preventDefault();
      if (channel !== activeChannel) {
        this.bus.emit('switch_channel', { channel: channel });
      }
    });
    
    li.appendChild(a);
    return li;
  }
  
}

const URL = new URLConfig();

class Client {
  constructor(bus) {
    this.bus = bus;
    this.urlConfig = new URLConfig();
    this.currentChannel = "general";
    this.socket = null;
    this.nick = randomName(namedata);
    this.channels = new Set(["general"]); // Track available channels

    this.bus.listen('send_message', this.sendMessage.bind(this));
    this.bus.listen('switch_channel', this.switchChannel.bind(this));
    this.bus.listen('create_channel', this.createChannel.bind(this));
  }

  connect() {
    const wsURL = URL.getWS(`/chatroom?channel=general`);
    this.socket = new WebSocket(wsURL);
    this.setupSocketHandlers();
  }

  setupSocketHandlers() {
    this.socket.onopen = e => console.log("[open] Connection established");

    this.socket.onmessage = e => {
      let data = JSON.parse(e.data);
      if (data.type === 'message' && data.channel === this.currentChannel) {
        this.bus.emit('message', data);
      }
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
  sendMessage(e) {
    const { message } = e.detail;
    this.socket.send(JSON.stringify({nick: this.nick, content: message, channel: this.currentChannel}));
  }

  
  fecthChannels() {
    fetch(this.urlConfig.getHTTP('/channels'))
      .then(response => response.json())
      .then(channels => {
        this.channels = new Set(channels); // Update channels set
        let detail = { channels: channels, activeChannel: this.currentChannel };
        this.bus.emit('channels_updated', detail);
      })
      .catch(error => console.error('Error fetching channels:', error));
  }

  switchChannel(e) {
    const { channel } = e.detail;
    const oldChannel = this.currentChannel;
    this.currentChannel = channel;
    
    // Send channel switch message if socket is open
    if (this.socket && this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(JSON.stringify({
        type: 'system',
        content: 'switch_channel',
        channel: channel
      }));
    }

    // Dispatch custom event for channel switch
    this.bus.emit('channel_switched', {
      detail: { channels: this.channels, oldChannel, newChannel: channel }
    });
  }

  // Add method to create a new channel
  createChannel(e) {
    let { channel } = e.detail;
    channel = channel.toLowerCase().trim();
    if (!channel) return;
    
    if (!this.channels.has(channel)) {
      this.channels.add(channel);
    }
    this.switchChannel({ channel });
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

// function setupNewChannelForm(chatClient) {
//   const form = document.getElementById('newchannel');
//   const input = document.getElementById('channel');
  
//   form.addEventListener('submit', (e) => {
//     e.preventDefault();
//     e.stopPropagation();
//     const channelName = input.value.trim();
//     if (channelName) {
//       chatClient.createChannel({ channel: channelName });
//       input.value = '';
//     }
//   });
// }


window.addEventListener("DOMContentLoaded", (event) => {
  console.log("DOMContentLoaded");
  const bus = new Bus();
  let nick = randomName(namedata);
  const client = new Client(bus);
  const ui = new UI(bus);

  // setupNick((newNick) => chatClient.setNick(newNick), nick);
  // setupRandomizeButton((newNick) => chatClient.setNick(newNick));
  
  // setupNewChannelForm(chatClient);
  const channelname = document.getElementById('current-channel-name');
  channelname.textContent = `#${client.currentChannel}`;
  
  client.fecthChannels();
  client.connect();
});