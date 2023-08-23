import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Logger } from './logger.js'
import { App } from './app.js'
//pages
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
        new BackendHandler();

        this.$container = document.getElementById('container');
        this.app = new App(this.$container);
        this.app.load();
    }
}

new Router()
