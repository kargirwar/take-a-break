import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Logger } from './logger.js'
import { App } from './app.js'
import { Splash } from './splash.js'
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
        this.$container = document.getElementById('container');

        PubSub.subscribe(Constants.EVENT_RULES_APPLIED, (e) => {
            Logger.Log(TAG, "EVENT_RULES_APPLIED");
            //if (e.rules.length == 0) {
                //Logger.Log(TAG, "Loading splash");
                //let splash = new Splash(this.$container);
                //splash.load();
                //return;
            //}

            Logger.Log(TAG, "Loading app");
            this.app = new App(this.$container);
            this.app.load();
        });

        new BackendHandler();
    }
}

new Router()
