class Utils {
    static processTemplate(templ, data) {
        var re = new RegExp(/{(.*?)}/g);
        templ = templ.replace(re, function(match, p1) {
            if (data[p1] || data[p1] == 0 || data[p1] == '') {
                return data[p1];
            } else {
                return match;
            }
        });
        return templ;
    }

    //https://stackoverflow.com/questions/494143/creating-a-new-dom-element-from-an-html-string-using-built-in-dom-methods-or-pro
    static generateNode(templ, data) {
        templ = Utils.processTemplate(templ, data);	
        let template = document.createElement('template');
        template.innerHTML = templ.trim()
        return template.content
    }

    //TODO: alert and info are not DRY
    static alert(msg, time) {
        let $root = document.querySelector('#dialog-container');
        $root.style.display = 'flex';
        let $dialog = $root.querySelector('#dialog'); 
        $dialog.innerHTML = msg;
        $dialog.classList.remove('is-info');
        $dialog.classList.add('is-warning');

        setTimeout(() => {
            $root.style.display = 'none';
        }, time);
    }

    static info(msg, time) {
        let $root = document.querySelector('#dialog-container');
        $root.style.display = 'flex';
        let $dialog = $root.querySelector('#dialog'); 
        $dialog.innerHTML = msg;
        $dialog.classList.remove('is-warning');
        $dialog.classList.add('is-info');
        //$dialog.style.background = 'blue'

        setTimeout(() => {
            $root.style.display = 'none';
        }, time);
    }

    static range(min, max, step = 1) {
        let arr = []
        for (let i = min; i <= max; i = i + step) {
            arr.push(i)
        }
        return arr
    }

    static isEmpty(obj) {
        for (const prop in obj) {
            if (Object.hasOwn(obj, prop)) {
                return false;
            }
        }

        return true;
    }
}

export { Utils }
