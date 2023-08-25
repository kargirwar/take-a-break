import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Logger } from './logger.js'

const TAG = "help";

class Help {
    constructor($root) {
        this.$root = $root;
        this.rootTemplate = document.getElementById('help-template').innerHTML;
    }

    load() {
        this.$root.replaceChildren(Utils.generateNode(this.rootTemplate, {}));
        this.$root.querySelector('.back').addEventListener('click', () => {
            PubSub.publish(Constants.PAGE_APP, {});
        });
    }
}

export { Help }
