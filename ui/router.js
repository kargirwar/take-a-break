import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
//pages
import { Index } from './index.js'
import { Rules } from './rules.js'
import { BackendHandler } from './backend-handler.js'

class Router {
    constructor() {
        document.addEventListener('DOMContentLoaded', () => {                                                         
            this.init();
        }) 
    }

    init() {
        this.container = document.getElementById('container');
        this.index = new Index(this.container);
        this.rules = new Rules(this.container);

        new BackendHandler();
        this.rules.load();
    }
}

new Router()
