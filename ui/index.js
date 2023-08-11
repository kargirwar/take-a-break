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

        this.start = document.getElementById('start');

        this.start.addEventListener('click', () => {
            const { invoke } = window.__TAURI__.tauri

            invoke('greet', { name: 'Pankaj' }).then((response) => {
                console.log(response);
            });

            invoke('my_custom_command', {}).then((response) => {
                //console.log(response);
            });
        });

        const appWindow = window.__TAURI__.window.appWindow

        appWindow.listen('some_event', (e) => {
            console.log(e.payload);
        });
    }
}

new Index()
