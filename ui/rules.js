import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Logger } from './logger.js'

const TAG = "rules";

class Rules {
    constructor($root) {
        this.$root = $root;
        this.rootTemplate = document.getElementById('rules-template').innerHTML;
        this.ruleTemplate = document.getElementById('rule-template').innerHTML;
    }

    load() {
        this.$root.replaceChildren(Utils.generateNode(this.rootTemplate, {}));
        
        let $list = this.$root.querySelector('#rules-content');
        this.setupTabs();
        this.$root.querySelector('#add-rule').addEventListener('click', () => {
            let $n = Utils.generateNode(this.ruleTemplate, {});
            $list.append($n);
        });

        this.$root.addEventListener('click', (e) => {
            let $n = e.target;
            if ($n.classList.contains('del-rule')) {
                $n.closest('.rule').remove();
            }
        });

        this.$root.addEventListener('click', (e) => {
            let $n = e.target;
            if ($n.classList.contains('save-rule')) {
                this.saveRule($n.parentElement);
            }
        });

        //Track changes
        this.$root.addEventListener('change', (e) => {
            let $p = e.target.closest('.rule');
            $p.style.borderColor = 'yellow';
            $p.querySelector('.save-rule').style.display = 'block';
        });

        //debug
        document.querySelector('#add-rule').dispatchEvent(new Event('click'));
    }

    saveRule($n) {
        let from = parseInt($n.querySelector('.from').value);
        let to = parseInt($n.querySelector('.to').value);

        let days;
        $n.querySelectorAll('input[name="days"]').forEach((r) => {
            if (r.checked) {
                days = r.value;
            }
        });

        Logger.Log(TAG, days);

        if (days == undefined) {
            Utils.alert("Please select days", 3000);
            return
        }

        if (to <= from) {
            Utils.alert("To hours must be greater than from hours", 3000);
            return;
        }

        Utils.info("Saved", 3000);
        $n.style.borderColor = 'grey';
        $n.querySelector('.save-rule').style.display = 'none';
    }

    setupTabs() {
        let tabs = this.$root.querySelectorAll('[id^=tab-]');
        tabs.forEach((t) => {
            t.addEventListener('click', (e) => {
                let p = e.target.closest('[id^=tab-]');
                Logger.Log(TAG, p.classList);

                if (p.classList.contains('active')) {
                    //already selected. nop
                    return;
                }

                //TODO: works with only two tabs
                let sibling = p.nextElementSibling ?? p.previousElementSibling;
                sibling.classList.remove('active');
                p.classList.add('active'); 

                //show/hide tab content
                let n = p.id.replace(/tab-/, '');
                let content = this.$root.querySelector(`#${n}-content`);
                content.style.display = 'block';
                sibling = content.nextElementSibling ?? content.previousElementSibling;
                sibling.style.display = 'none';
            });
        });
    }
}

export { Rules }
