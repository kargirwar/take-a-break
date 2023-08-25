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

        this.$root.replaceChildren(Utils.generateNode(this.rootTemplate, {}));

        this.rules = [];

        //this event is raised when backend applies already saved rules
        PubSub.subscribe(Constants.EVENT_RULES_APPLIED, (e) => {
            Logger.Log(TAG, JSON.stringify(e));
            this.rules = e.rules;
            let rules = new Rules(this.$root.querySelector('.tab-content.rules'));
            rules.load(this.rules);
        });

        //this event is raised when front end changes any rules
        PubSub.subscribe(Constants.EVENT_RULES_UPDATED, (e) => {
            this.rules = e.rules;
        });

        this.$root.addEventListener('click', (e) => {
            if (e.target.classList.contains('add-rule')) {
                Logger.Log(TAG, "add rule");
                PubSub.publish(Constants.EVENT_NEW_RULE, {});
            }
        });
    }

    load() {
        this.$root.replaceChildren(Utils.generateNode(this.rootTemplate, {}));

        let rules = new Rules(this.$root.querySelector('.tab-content.rules'));
        rules.load(this.rules);
    }
}

export { App }
