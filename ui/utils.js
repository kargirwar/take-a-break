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
}

export { Utils }
