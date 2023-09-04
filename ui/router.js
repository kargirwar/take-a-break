import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Logger } from './logger.js'
import { App } from './app.js'
import { Help } from './help.js'
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

        this.app = new App(this.$container);
        this.app.load();

        this.help = new Help(this.$container);

        PubSub.subscribe(Constants.PAGE_HELP, (e) => {
            this.help.load();
        });

        PubSub.subscribe(Constants.PAGE_APP, (e) => {
            this.app.load();
        });

        new BackendHandler();
    }
}

new Router()
