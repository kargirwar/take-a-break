import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Logger } from './logger.js'
import { Tabs } from './tabs.js'
import { Rules } from './rules.js'
import { Status } from './status.js'

const TAG = "app";

class App {
    constructor($root) {
        this.$root = $root;
        this.rootTemplate = document.getElementById('app-template').innerHTML;
    }

    load() {
        this.$root.replaceChildren(Utils.generateNode(this.rootTemplate, {}));

        this.$root.querySelector('.add-rule').addEventListener('click', () => {
            PubSub.publish(Constants.EVENT_NEW_RULE, {});
        });

        //new Tabs();

        let rules = new Rules(document.querySelector('.tab-content.rules'));
        rules.load();

        let status = new Status(document.querySelector('.tab-content.status'));
    }
}

export { App }
