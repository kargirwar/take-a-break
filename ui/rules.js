import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'

class Rules {
    constructor($root) {
        this.$root = $root;
        this.template = document.getElementById('rules-template').innerHTML;
    }

    load() {
        this.$root.replaceChildren(Utils.generateNode(this.template, {}));
        this.$root.querySelector('#edit-rule').addEventListener('click', () => {
            PubSub.publish(Constants.PAGE_CHANGE, {
                page: Constants.PAGE_RULE_EDITOR
            });
        });
    }
}

export { Rules }
