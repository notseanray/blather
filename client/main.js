const WebSocket = require("ws");


const ws = new WebSocket('ws://127.0.0.1:7930/ws')

ws.on('message', msg => console.log(msg.toString()));

ws.on('open', function open() {
  ws.send('password hello');
});
