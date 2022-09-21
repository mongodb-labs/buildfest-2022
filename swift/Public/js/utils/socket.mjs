// @ts-check
import { EJSON } from 'bson'
import { List } from 'utils/list.mjs'

const makeNotifier = () => {
    /** @type {() => void} */
    let resolve
    /** @type {(error: Error) => void} */
    let reject
    /** @type {Promise<void>} */
    const p = new Promise((pRes, pRej) => {
        resolve = pRes;
        reject = pRej
    })
    // @ts-ignore
    return { p, resolve, reject }
}

/**
 * I am a thin event handling wrapper around a WebSocket
 * Supporting auto reconnect on close event. (Maybe we're just in a tunnel!)
 */
export class AutoConnectWebSocket extends EventTarget {
    constructor(url = `ws://${window.location.host}/feed`) {
        super();
        this.url = url
        this.ws = new WebSocket(this.url)
        this.reconnectInterval = null;
        this.messages = List.newEmptyList();
        this.notify = makeNotifier()
        this.isConnected = false;
        this.addListeners();
    }

    addListeners() {
        this.ws.addEventListener('close', this.onClose)
        this.ws.addEventListener('open', this.onOpen)
        this.ws.addEventListener('message', this.onMessage)
        this.ws.addEventListener('error', this.onError)
    }

    removeListeners() {
        this.ws.removeEventListener('close', this.onClose)
        this.ws.removeEventListener('open', this.onOpen)
        this.ws.removeEventListener('message', this.onMessage)
        this.ws.removeEventListener('error', this.onError)
    }

    onClose = (closeEvent) => {
        console.log(`[ws] onClose`)
        this.isConnected = false;
        this.removeListeners();
        this.reconnectInterval = setInterval(() => {
            console.log('reconnecting...')
            this.ws = new WebSocket(this.url)
            this.addListeners()
        }, 5000)
        this.dispatchEvent(new class extends Event { }('close'))
    }

    onOpen = () => {
        console.log('[ws] onOpen')
        this.isConnected = true;
        if (this.reconnectInterval != null) {
            console.log('reconnected!!')
            // We got a fresh socket all hooked up!
            clearInterval(this.reconnectInterval)
        }
        this.dispatchEvent(new class extends Event { }('open'))
    }

    onMessage = (message) => {
        console.log(`[ws] onMessage`)
        const data = EJSON.parse(message.data)
        List.push(this.messages, { data })
        this.notify.resolve()
        this.dispatchEvent(new class extends Event { data = data }('message'))
    }

    onError = (error) => {
        this.removeListeners();
        console.log(`[ws] onError ${error}`)
        List.push(this.messages, { error })
        this.notify.resolve();
        this.dispatchEvent(new class extends Event { error = error }('error'))
    }

    async *[Symbol.asyncIterator]() {
        while (true) {
            await this.notify.p;
            this.notify = makeNotifier();
            yield List.shift(this.messages).value
        }
    }
}
