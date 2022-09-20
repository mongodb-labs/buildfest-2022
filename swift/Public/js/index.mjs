import { EJSON } from 'https://unpkg.com/bson@4.7.0/dist/bson.browser.esm.js'

const dateFmt = Intl.DateTimeFormat('en-US', { dateStyle: 'full', timeStyle: 'long' });
const stream = document.getElementById('mta-stream')

let occurrences = 0;
const interval = setInterval(() => {
    stream.setState({
        items: [
            ...stream.state.items,
            {
                 created: dateFmt.format(new Date()),
                ...Object.fromEntries(Array.from({ length: 4 }, (_, i) => Math.random()).entries())
            }
        ]
    })

    if (occurrences === 5) {
        clearInterval(interval)
    }
    occurrences += 1;
}, 3000)

const websocket = new WebSocket(`ws://${window.location.host}/feed`)

websocket.addEventListener('open', () => console.log('ws => open'))
websocket.addEventListener('close', () => console.log('ws => close'))

websocket.addEventListener('message', message => {
    const data = EJSON.parse(message.data, { relaxed: false })
    stream.setState({
        items: [...stream.state.items, { created: dateFmt.format(new Date()), ...data }]
    })
})

websocket.addEventListener('error', error => {
    stream.setState({
        items: [...stream.state.items, { created: dateFmt.format(new Date()), error }]
    })
})
