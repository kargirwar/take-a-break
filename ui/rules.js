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
        this.rules = [];
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

                //just re-read everything so we don't have to worry about serial
                this.rules = this.getRules();
                //this is necessary so that dom is in step with the object in memory
                this.updateSerial();
            }
        });

        this.$root.addEventListener('click', (e) => {
            let $n = e.target;
            if ($n.classList.contains('save-rule')) {
                this.saveRule($n.parentElement, parseInt(e.target.dataset.serial));
                PubSub.publish(Constants.EVENT_RULES_SAVED, {
                    rules: this.rules
                });
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

    updateSerial() {
        let serial = 1;
        [...document.querySelectorAll('.save-rule')].forEach((e) => {
            e.dataset.serial = serial++;
        });
    }

    saveRule($r, serial = null) {
        let from = parseInt($r.querySelector('.from').value);
        let to = parseInt($r.querySelector('.to').value);

        let days = [];
        $r.querySelectorAll('input[name="days"]').forEach((r) => {
            if (r.checked) {
                days.push(r.value);
            }
        });

        Logger.Log(TAG, days);

        if (days.length == 0) {
            Utils.alert("Please select days", 3000);
            return;
        }

        if (to <= from) {
            Utils.alert("To hours must be greater than from hours", 3000);
            return;
        }

        if (this.isDuplicate(this.getRule($r), serial)) {
            Utils.alert("Duplicate rule", 3000);
            return;
        }

        Logger.Log(TAG, JSON.stringify(this.getRules()));

        Utils.info("Saved", 2000);
        $r.style.borderColor = 'grey';
        $r.querySelector('.save-rule').style.display = 'none';

        this.rules = this.getRules();

        return;
    }

    isDuplicate(r, serial = null) {
        //for any given day there can be only rule for a set of from-to
        for (let i = 0; i < this.rules.length; i++) {
            let o = this.rules[i];
            if (serial === o.serial) {
                //don't compare with self
                continue;
            }
            //let commonDays = _.intersection(r.days, o.days);
            let commonDays = r.days.filter(x => o.days.includes(x));
            if (commonDays.length == 0) {
                continue;
            }

            let range1 = Utils.range(r.from, r.to);
            let range2 = Utils.range(o.from, o.to);

            let commonHours = range1.filter(x => range2.includes(x));
            Logger.Log(TAG, `commonHours: ${commonHours}`);

            if (commonHours.length > 1) {
                return true;
            }
        }

        return false;
    }

    getRules() {
        let rules = [];
        let serial = 1;
        [...document.querySelectorAll('.rule')].forEach(($r) => {
            let r = {};
            r.serial = serial;
            Object.assign(r, this.getRule($r));
            rules.push(r);
            serial++;
        });

        return rules;
    }

    getRule($r) {
        let r = {};
        r.days = [];
        [...$r.querySelectorAll('[name=days]')].forEach(($d) => {
            if ($d.checked) {
                r.days.push($d.value);
            }

            r.interval = parseInt($r.querySelector('.interval').value);
            r.from = parseInt($r.querySelector('.from').value);
            r.to = parseInt($r.querySelector('.to').value);
        });

        return r;
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
