import { Constants } from './constants.js'
import { PubSub } from './pubsub.js'
import { Utils } from './utils.js'
import { Logger } from './logger.js'

const TAG = "backend-handler";
const appWindow = window.__TAURI__.window.appWindow;
const { invoke } = window.__TAURI__.tauri;

class BackendHandler {
    constructor() {
        PubSub.subscribe(Constants.EVENT_RULES_SAVED, (e) => {
            Logger.Log(TAG, JSON.stringify(e.rules));
            let rules = [
                {
                    serial: 1,
                    days: ["Sun"],
                    interval: 20,
                    from: 16,
                    to: 17,
                }
            ];

            //invoke('command', {
                //"payload": JSON.stringify({
                    //name: "update-rules",
                    //rules: e.rules})
            invoke('command', {
                "payload": JSON.stringify({
                    name: "update-rules",
                    rules: rules})
                //"payload": JSON.stringify({
                    //name: "update-rules",
                    //rules: e.rules})
            }).then((response) => {
                //console.log(response);
            });
        });

        appWindow.listen('some_event', (e) => {
            Logger.Log(TAG, JSON.stringify(e.payload));
        });
    }
}

export { BackendHandler }
