// @ts-check
import { EJSON } from 'https://unpkg.com/bson@4.7.0/dist/bson.browser.esm.js'

/**
 * @typedef StreamState
 * @property {Record<string, any>[]} items
 */

class StreamElement extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' })
        this.inheritStyles();

        /** @type {StreamState} */
        this.state = { items: [{ hello: 1 }, { bye: 2 }] }
    }

    /**
     * Normally shadow DOM is used to isolate style changes
     * We want to inherit what is defined so far but further
     * programmatic changes should remain confined to this custom element
     */
    inheritStyles() {
        if (this.shadowRoot == null) {
            return;
        }
        const [indexCss] = Array.from(document.styleSheets).filter(sheet => sheet.href?.includes('index.css'))
        const sheet = new CSSStyleSheet()
        for (const rule of indexCss.cssRules) {
            sheet.insertRule(rule.cssText)
        }
        this.shadowRoot.adoptedStyleSheets.push(sheet)
    }

    /** DOM Lifecycle hook */
    connectedCallback() {
        if (this.shadowRoot == null) {
            return;
        }
        this.shadowRoot.innerHTML = StreamElement.render(this.state);
    }

    /**
     * Invented paradigm, implement rendering with static function so it
     * is not bound to anything but the passed in state
     * @param {StreamState} state
     *
     * @return {string}
     */
    static render(state) {
        if (state == null) {
            return `<pre>no state, this is an error</pre>`
        }
        if (state.items.length === 0) {
            return `<pre>no message</pre>`
        }

        let listEl = `<ul>`

        const expandObject = (object) => {
            let str = ''
            for (const [key, value] of Object.entries(object)) {
                str += `<li><pre>${key} => ${EJSON.stringify(value)}</pre></li>`
            }
            return str;
        }

        for (const [index, item] of state.items.entries()) {
            listEl += `<li><pre>Item: ${index.toLocaleString('us')}<pre><ul>`
            listEl += expandObject(item)
            listEl += `</li></ul>`
        }

        listEl += `</ul>`
        return listEl;
    }

    /**
     * This one goes out to all my react fiends
     * @param {StreamState} state
     */
    setState(state) {
        if (this.shadowRoot == null) {
            return;
        }
        this.state = state;
        this.shadowRoot.innerHTML = StreamElement.render(this.state);
    }
}

const customElementRegistry = window.customElements;
customElementRegistry.define('app-stream', StreamElement);
