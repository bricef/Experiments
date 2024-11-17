


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


let nick = "Anon"

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
    console.log(`New name ${nick}`);
  });
  nickfield.placeholder = nick;
}

function setupMessageForm(socket){
  let form = document.getElementById('messagesend');
  let message = document.getElementById('message');
  form.addEventListener('submit', (e) => {
    e.preventDefault();
    console.log(`Sending message ${message.value}`);
    socket.send(JSON.stringify({nick: nick, content: message.value}));
    message.value = '';
  });
}

window.addEventListener("DOMContentLoaded", (event) => {
  let socket = new WebSocket("ws://localhost:1323/chatroom");
  setupNick("Anon");
  setupSocketHandlers(socket);
  setupMessageForm(socket);

});