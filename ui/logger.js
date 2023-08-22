import { Constants } from './constants.js'
const DISABLED = [
]

//workers do not support console.log. How to debug ? 
// We send a message to the module that initiated worker and 
// have it print the debug log
// But sending message requires port which is available only in 
// worker. How to use a common logger for entire system?
// We create static "Log" method which can use used for all code that 
// does not get directly called from worker. For any code that gets
// called from worker we use the "log" method.

class Logger {
    constructor(port = null) {
        this.port = port
    }

    log(tag, str) {
        if (DISABLED.includes(tag)) {
            return;
        }

        if (this.port) {
            this.port.postMessage({
                type: Constants.DEBUG_LOG,
                payload: `${tag}: ${str}`
            })
            return
        }

        Logger.print(tag, str);
    }

    static Log(tag, str) {
        if (DISABLED.includes(tag)) {
            return;
        }

        Logger.print(tag, str ?? "{}");
    }

    static print(tag, str) {
        let [month, date, year]    = new Date().toLocaleDateString("en-US").split("/")
        let [hour, minute, second] = new Date().toLocaleTimeString("en-US").split(/:| /)

        let o = `${date}-${month}-${year} ${hour}:${minute}:${second}:::${tag}: ${str}`;
        console.log(o)
    }
}

export { Logger }
