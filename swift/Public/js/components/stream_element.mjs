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
        this.state = { items: [] }
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
        sheet.insertRule(`* { font-family: 'Source Code Pro', monospace; }`)
        this.shadowRoot.adoptedStyleSheets.push(sheet)
    }

    /** DOM Lifecycle hook */
    connectedCallback() {
        if (this.shadowRoot == null) {
            return;
        }
        this.shadowRoot.innerHTML = ''
        this.shadowRoot.appendChild(StreamElement.render(this.state));
    }

    /**
     * Invented paradigm, implement rendering with static function so it
     * is not bound to anything but the passed in state
     * @param {StreamState} state
     *
     * @return {Element}
     */
    static render(state) {
        if (state == null) {
            const p = document.createElement('p')
            p.innerText = `no state, this is an error`
            return p
        }
        if (state.items.length === 0) {
            const p = document.createElement('p')
            p.innerText = `no message`
            return p
        }

        function createTree(container, data) {
            const ul = container.appendChild(document.createElement('ul'));
            if (data != null && typeof data === 'object') {
                for (const [key, val] of Object.entries(data)) {
                    const li = ul.appendChild(document.createElement('li'));
                    if (data != null && typeof data === 'object') {
                        li.textContent = `${key}`;
                        createTree(li, val);
                    }
                }
            } else {
                const li = ul.appendChild(document.createElement('li'));
                li.textContent += `↪ ${data}`
            }
        }

        const root = document.createElement('ul')
        // Object.fromEntries(Array.from(state.items.entries()).map(([index, val]) => [`Change ${index}`, val])))

        for (const [index, value] of state.items.entries()) {
            const li = root.appendChild(document.createElement('li'))
            li.textContent = `Change ${index}`
            const summary = li.appendChild(document.createElement('summary'))
            const details = summary.appendChild(document.createElement('details'))
            createTree(details, value)
        }


        return root
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
        this.shadowRoot.innerHTML = ''
        this.shadowRoot.appendChild(StreamElement.render(this.state));
    }
}

const customElementRegistry = window.customElements;
customElementRegistry.define('app-stream', StreamElement);
