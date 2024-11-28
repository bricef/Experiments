


function setupSocketHandlers(socket){
  socket.onopen = e => console.log("[open] Connection established");

  socket.onmessage = e => {
    let data = JSON.parse(e.data);
    let chat = document.getElementById('chat');
    chat.innerHTML += `<p><strong>${data.nick}</strong>: ${data.content}</p>`;
    chat.scrollTop = chat.scrollHeight;
  }


  socket.onclose = function(event) {
    if (event.wasClean) {
      console.log(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
    } else {
      console.log('[close] Connection died');
    }
  };

  socket.onerror = function(error) {
    console.log(`[error]: ${error.message}`);
  };
}




function setupMessageForm(socket){
  let form = document.getElementById('messagesend');
  let message = document.getElementById('message');
  form.addEventListener('submit', (e) => {
    e.preventDefault();
    // console.log(`Sending message ${message.value}`);
    socket.send(JSON.stringify({nick: nick, content: message.value}));
    message.value = '';
  });
}

function choose(arr){
  return arr[Math.floor(Math.random() * arr.length)];
}

var namedata = {
  animals: ["lion", "bear", "octopus", "goose"],
  adjectives: ["bright", "wise", "silly"]
};

var host = window.location.host;

// Udate namedata when possible
fetch(`https://${host}/namedata.json`)
    .then((e) => e.json())
    .then((nd) => {
      namedata = nd;
    })

function capitalizeFirstLetter(val) {
  return String(val).charAt(0).toUpperCase() + String(val).slice(1);
}

function randomName(namedata){
  let animal = capitalizeFirstLetter(choose(namedata.animals));
  let adjective = capitalizeFirstLetter(choose(namedata.adjectives));
  let number = Math.floor(Math.random() * 99);
  return adjective + animal + number;
}

let nick = randomName(namedata);

function setNick(newNick){
  if (newNick === "") {
    nick = "Anon";
  }else{
    nick = newNick;
  }
}

function setupNick(nick){
  nickfield = document.getElementById('nick')
  nickfield.addEventListener('input', (e) => {
    setNick(e.target.value);
  });
  nickfield.placeholder = nick;
}

function setupRandomiseButton() {
  let button = document.getElementById('randomise');
  button.addEventListener('click', (e) => {
    let nick = randomName(namedata); 
    setNick(nick);
    nickfield = document.getElementById('nick')
    nickfield.value = nick;
    nickfield.placeholder = nick;
  });
}

window.addEventListener("DOMContentLoaded", (event) => {
  let socket = new WebSocket(`wss://${host}/chatroom`);
  
  setupNick(nick);
  setupSocketHandlers(socket);
  setupMessageForm(socket);
  setupRandomiseButton();
});