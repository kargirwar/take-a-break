import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
//pages
import { Index } from './index.js'
import { RuleEditor } from './rule-editor.js'
import { Rules } from './rules.js'

class Router {
    constructor() {
        document.addEventListener('DOMContentLoaded', () => {                                                         
            this.init();
        }) 
    }

    init() {
        this.container = document.getElementById('container');
        this.index = new Index(this.container);
        this.ruleEditor = new RuleEditor(this.container);
        this.rules = new Rules(this.container);

        PubSub.subscribe(Constants.PAGE_CHANGE, (e) => {
            console.log(e);
            this.handlePageChange(e.page);
        });

        this.index.load();
    }

    handlePageChange(page) {
        switch (page) {
            case Constants.PAGE_INDEX:
                this.index.load();
                break;

            case Constants.PAGE_RULE_EDITOR:
                this.ruleEditor.load();
                break;

            case Constants.PAGE_RULES:
                this.rules.load();
                break;
        }
    }
}

new Router()
