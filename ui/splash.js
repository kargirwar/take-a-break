import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Logger } from './logger.js'
import { Tabs } from './tabs.js'
import { Rules } from './rules.js'
import { Status } from './status.js'

const TAG = "splash";

class Splash {
    constructor($root) {
        this.$root = $root;
        this.rootTemplate = document.getElementById('splash-template').innerHTML;
    }

    load() {
        this.$root.replaceChildren(Utils.generateNode(this.rootTemplate, {}));
    }
}

export { Splash }
