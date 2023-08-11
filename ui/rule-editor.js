import { Utils } from './utils.js'
import { PubSub } from './pubsub.js'
import { Constants } from './constants.js'

class RuleEditor {
    constructor($root) {
        this.$root = $root;
        this.template = document.getElementById('rule-editor-template').innerHTML;
    }

    load() {
        this.$root.replaceChildren(Utils.generateNode(this.template, {}));
        this.$root.querySelector('#save-rule').addEventListener('click', () => {
            PubSub.publish(Constants.PAGE_CHANGE, {
                page: Constants.PAGE_RULES
            });
        });
    }
}

export { RuleEditor }
