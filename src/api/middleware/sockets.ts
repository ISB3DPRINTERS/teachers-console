import WebSocket from 'ws';

export default function(message:number) {
    const wss = new WebSocket.Server({ port: 8010 });
    wss.on('connection', function connection(ws) {
        ws.on('error', console.error);
        
        ws.on('message', function message(data) {
            console.log('received: %s', data);
        });
        
        ws.send(message);
    })
}
