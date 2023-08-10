import { Utils } from './utils.js'

class Index {
    constructor() {
        document.addEventListener('DOMContentLoaded', () => {                                                         
            this.init();
        }) 
    }

    init() {
        this.container = document.getElementById('container');
        let tmpl = document.getElementById('index-template').innerHTML;
        let n = Utils.generateNode(tmpl, {});
        this.container.append(n);

        document.getElementById('header').innerHTML = "Header";
    }
}

new Index()
