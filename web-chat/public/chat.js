// Utility functions
function capitalizeFirstLetter(val) {
  return String(val).charAt(0).toUpperCase() + String(val).slice(1);
}

function choose(arr) {
  return arr[Math.floor(Math.random() * arr.length)];
}

function randomName(namedata) {
  let animal = capitalizeFirstLetter(choose(namedata.animals));
  let adjective = capitalizeFirstLetter(choose(namedata.adjectives));
  let number = Math.floor(Math.random() * 99);
  return adjective + animal + number;
}

class ChatClient {
  constructor(host, isDebug) {
    this.host = host;
    this.isDebug = isDebug;
    this.wsProtocol = isDebug ? 'ws' : 'wss';
    this.httpProtocol = isDebug ? 'http' : 'https';
    this.currentChannel = "general";
    this.socket = null;
    this.nick = randomName(namedata);
  }

  connect() {
    this.socket = new WebSocket(`${this.wsProtocol}://${this.host}/chatroom?channel=general`);
    this.setupSocketHandlers();
    this.setupMessageForm();
  }

  setupSocketHandlers() {
    this.socket.onopen = e => console.log("[open] Connection established");

    this.socket.onmessage = e => {
      let data = JSON.parse(e.data);
      switch(data.type) {
        case 'chat':
          let chat = document.getElementById('chat');
          chat.innerHTML += `<p><strong>${data.nick}</strong>: ${data.content}</p>`;
          chat.scrollTop = chat.scrollHeight;
          break;
        case 'channel':
          // Update channel list when server sends channel update
          this.updateChannelList(data.channels);
          break;
        case 'system':
          console.log('System message:', data.content);
          break;
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

  setupMessageForm() {
    let form = document.getElementById('messagesend');
    let message = document.getElementById('message');
    form.addEventListener('submit', (e) => {
      e.preventDefault();
      let msg = JSON.stringify({
        type: 'chat',
        nick: this.nick,
        content: message.value,
        channel: this.currentChannel
      })
      console.log(msg);
      this.socket.send(msg);
      message.value = '';
    });
  }

  switchChannel(channel) {
    this.currentChannel = channel;
    document.getElementById('current-channel').textContent = `#${channel}`;
    document.getElementById('chat').innerHTML = '';
    
    if (this.socket && this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(JSON.stringify({
        type: 'system',
        content: 'switch_channel',
        channel: channel
      }));
    }
  }

  setNick(newNick) {
    this.nick = newNick === "" ? "Anon" : newNick;
  }

  updateChannelList(channels) {
    
        const channelsList = document.getElementById('channels').querySelector('ul');
        channelsList.innerHTML = '';
        
        const fragment = document.createDocumentFragment();
        channels.forEach(channel => {
          const li = this.createChannelListItem(channel);
          fragment.appendChild(li);
        });
        
        channelsList.appendChild(fragment);
      
  }

  createChannelListItem(channel) {
    const li = document.createElement('li');
    const a = document.createElement('a');
    
    a.href = '#';
    a.textContent = `#${channel}`;
    a.dataset.channel = channel;
    
    // Use arrow function to preserve 'this' context
    a.addEventListener('click', (e) => {
      e.preventDefault();
      this.switchChannel(channel);
    });
    
    li.appendChild(a);
    return li;
  }
}

var namedata = {
  animals: ["lion", "bear", "octopus", "goose"],
  adjectives: ["bright", "wise", "silly"]
};

var host = window.location.host;
var isDebug = window.location.protocol === 'http:';

// Update namedata when possible
fetch(`${isDebug ? 'http' : 'https'}://${host}/namedata.json`)
    .then((e) => e.json())
    .then((nd) => {
      namedata = nd;
    })

function setupNick(chatClient) {
  nickfield = document.getElementById('nick')
  nickfield.addEventListener('input', (e) => {
    chatClient.setNick(e.target.value);
  });
  nickfield.placeholder = chatClient.nick;
}

function setupRandomiseButton(chatClient) {
  let button = document.getElementById('randomise');
  button.addEventListener('click', (e) => {
    let nick = randomName(namedata); 
    chatClient.setNick(nick);
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
    const channelName = input.value.trim();
    if (channelName) {
      chatClient.switchChannel(channelName);
      input.value = '';
    }
  });
}

window.addEventListener("DOMContentLoaded", (event) => {
  const chatClient = new ChatClient(host, isDebug);
  setupNick(chatClient);
  setupRandomiseButton(chatClient);
  setupNewChannelForm(chatClient);
  chatClient.connect();
});