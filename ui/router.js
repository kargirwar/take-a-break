import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Logger } from './logger.js'
//pages
import { Rules } from './rules.js'
import { BackendHandler } from './backend-handler.js'

const TAG = "router";

class Router {
    constructor() {
        document.addEventListener('DOMContentLoaded', () => {                                                         
            this.init();
            Logger.Log(TAG, "DOMContentLoaded");
            PubSub.publish(Constants.EVENT_DOM_LOADED, {});
        }) 
    }

    init() {
        this.container = document.getElementById('container');
        this.rules = new Rules(this.container);

        new BackendHandler();
        this.rules.load();
    }
}

new Router()
