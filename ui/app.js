import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Logger } from './logger.js'
import { Tabs } from './tabs.js'
import { Rules } from './rules.js'

const TAG = "app";

class App {
    constructor($root) {
        this.$root = $root;
        this.rootTemplate = document.getElementById('app-template').innerHTML;
    }

    load() {
        this.$root.replaceChildren(Utils.generateNode(this.rootTemplate, {}));
        new Tabs();

        let rules = new Rules(document.querySelector('.tab-content.rules'));
        rules.load();
    }
}

export { App }
