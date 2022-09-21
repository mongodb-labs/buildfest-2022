import { AutoConnectWebSocket } from 'utils/socket.mjs';

const dateFmt = Intl.DateTimeFormat('en-US', { dateStyle: 'full', timeStyle: 'long' });

const stream = document.getElementById('mta-stream')
const liveness = document.getElementById('liveness')

const websocket = new AutoConnectWebSocket()

websocket.addEventListener('open', () => liveness.setAttribute('class', 'online'))

// data / error are mutually exclusive
for await (const { data, error } of websocket) {
    console.log(`async iterator iterated!`)

    if (error != null) {
        liveness.setAttribute('class', 'offline')
        continue
    }

    if (data != null) {
        liveness.setAttribute('class', 'online')
        stream.setState({
            items: [
                ...stream.state.items,
                { created: dateFmt.format(new Date()), ...data }
            ]
        })
        continue
    }
}
