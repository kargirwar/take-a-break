import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Constants } from './constants.js'

class Index {
    constructor($root) {
        this.$root = $root;
        this.template = document.getElementById('index-template').innerHTML;
    }

    load() {
        this.$root.replaceChildren(Utils.generateNode(this.template, {}));

        this.$root.querySelector('#start').addEventListener('click', () => {
            PubSub.publish(Constants.PAGE_CHANGE, {
                page: Constants.PAGE_RULE_EDITOR
            });
        });
    }
}

export { Index }
