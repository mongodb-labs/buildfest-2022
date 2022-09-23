// export type List<T = unknown> = {
//     value: null;
//     length: number;
//     next: Node<T>;
//     prev: Node<T>;
//     [Symbol.iterator]: () => IterableIterator<Node<T>>;
//     readonly [Symbol.toStringTag]: 'List';
//   };

//   export type Node<T = unknown> = {
//     value: T;
//     next: Node<T>;
//     prev: Node<T>;
//   };

const newEmptyList = () => {
    const sentinel = {
        next: null,
        prev: null,
        value: null,
        length: 0,
        *[Symbol.iterator]() {
            if (isEmpty(this)) {
                return;
            }

            let ptr = this.next;
            // @ts-expect-error: TS does not know what it is saying.
            while (ptr !== this) {
                const { next } = ptr; // Save next before yielding so that we make remove within iteration safe
                yield ptr;
                ptr = next;
            }
        },
        get [Symbol.toStringTag]() {
            return 'List';
        }
    };
    sentinel.next = sentinel;
    sentinel.prev = sentinel;
    return sentinel;
};

const fromArray = (array) => {
    const head = newEmptyList();
    for (const item of array) {
        push(head, item);
    }
    return head;
};

const isEmpty = (head) => {
    return head.next === head;
};

const first = (head) => {
    if (isEmpty(head)) return null;
    return head.next;
};

const last = (head) => {
    if (isEmpty(head)) return null;
    return head.prev;
};

/** Puts `value` at the front of the list */
const unshift = (head, value) => {
    head.length += 1;
    const newNode = {
        next: head.next,
        prev: head,
        value
    };
    head.next.prev = newNode;
    head.next = newNode;
};

/** Puts `value` at the back of the list */
const push = (head, value) => {
    head.length += 1;
    const newNode = {
        next: head,
        prev: head.prev,
        value: value
    };
    head.prev.next = newNode;
    head.prev = newNode;
};

const remove = (head, node) => {
    if (node == null) return null;
    if (isEmpty(head)) return null;
    head.length -= 1;
    const nodeBefore = node.prev;
    const nodeAfter = node.next;
    nodeBefore.next = nodeAfter;
    nodeAfter.prev = nodeBefore;
    node.next = null;
    node.prev = null;
    return node;
};

/** Removes the first node at the front of the list */
const shift = (head) => {
    if (isEmpty(head)) return null;
    return List.remove(head, List.first(head));
};

/** Removes the last node at the end of the list */
const pop = (head) => {
    if (isEmpty(head)) return null;
    return List.remove(head, List.last(head));
};

const clear = (head) => {
    head.next = head;
    head.prev = head;
    head.length = 0;
};

function* each(head) {
    if (isEmpty(head)) {
        return;
    }

    const gen = head[Symbol.iterator]();
    let genRes = gen.next();
    while (!genRes.done) {
        yield genRes.value.value;
        genRes = gen.next();
    }
}

const toArray = (head) => {
    return Array.from(each(head));
};

const toString = (head) => {
    return `head => [ ${toArray(head).join(' => ')} ] => head`;
};

export const List = Object.freeze({
    newEmptyList,
    fromArray,
    isEmpty,
    first,
    last,
    unshift,
    shift,
    push,
    pop,
    remove,
    clear,
    each,
    toArray,
    toString
});
