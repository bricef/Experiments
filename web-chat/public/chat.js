let socket = new WebSocket("ws://localhost:1323/chatroom");

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
    consol.log('[close] Connection died');
  }
};

socket.onerror = function(error) {
  console.log(`[error]: ${error.message}`);
};