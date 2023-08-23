import { Logger } from './logger.js'

const TAG = "tabs"

class Tabs {
    constructor() {
        this.$tabs = document.querySelector('.tabs');
        this.$contents = document.querySelectorAll('.tab-content');
        this.init();
        Logger.Log(TAG, "init done");
    }

    init() {
        let list = this.$tabs.querySelectorAll('li');
        list.forEach((t) => {
            t.addEventListener('click', (e) => {
                let li = e.target.parentElement;
                if (li.classList.contains('is-active')) {
                    return;
                }

                //disable currently active tab
                this.$tabs.querySelector('.is-active').classList.remove('is-active');
                this.$contents.forEach((e) => {
                    e.style.display = "none";
                });

                //activate current tab
                li.classList.add('is-active');

                //and the content
                let target = e.target;
                Logger.Log(TAG, target.className);
                this.$contents.forEach(($c) => {
                    if ($c.classList.contains(`${target.className}`)) {
                        $c.style.display = "block";
                    }
                });
            });
        });
    }
}

export { Tabs }
