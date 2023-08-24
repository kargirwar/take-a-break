import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Logger } from './logger.js'

const TAG = "status";

class Status {
    constructor($root) {
        this.$root = $root;
        this.rootTemplate = document.getElementById('status-template').innerHTML;
        this.$root.replaceChildren(Utils.generateNode(this.rootTemplate, {}));

        this.$n = this.$root.querySelector('.next-break');
        this.$p = this.$root.querySelector('.prev-break');

        PubSub.subscribe(Constants.EVENT_NEXT_ALARM, (e) => {
            Logger.Log(TAG, JSON.stringify(e));
            this.$n.innerHTML = e.alarms['next-alarm'];
            this.$p.innerHTML = e.alarms['prev-alarm'];
        });
    }
}

export { Status }
