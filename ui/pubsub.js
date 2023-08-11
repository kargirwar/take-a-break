let subscribers = {};

class PubSub {
    static subscribe(evt, cb) {
        if (!subscribers[evt]) {
            subscribers[evt] = new Set();
        }
        subscribers[evt].add(cb);
    }

    static publish(evt, data) {
        let list = subscribers[evt];
        if (!list) {
            return;
        }
        for (let s of list) {
            s(data);
        }
    }
}

export { PubSub }
