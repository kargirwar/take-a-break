mod alarm_manager;
mod ui_handler {

    use std::sync::{Arc, Mutex};
    use super::alarm_manager::*;
    use crossbeam_channel::bounded;
    use serde::Deserialize;
    use serde_json::Value;
    use tauri::AppHandle;
    use tauri::Wry;
    use tokio::sync::mpsc;
    use tokio::sync::mpsc::{Receiver, Sender};
    use std::thread;

    pub enum CommandName {
        UpdateRules,
    }

    impl CommandName {
        pub fn from_str(name: &str) -> Option<Self> {
            match name {
                "update-rules" => Some(CommandName::UpdateRules),
                _ => None,
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Rule {
        pub days: Vec<String>,
        pub from: usize,
        pub interval: usize,
        pub serial: usize,
        pub to: usize,
    }

    pub struct UiHandler {
        ui_rx: Receiver<String>,
        win_handle: AppHandle<Wry>,

        //for managing communication with alarm manager
        tx: crossbeam_channel::Sender<String>,
        rx: crossbeam_channel::Receiver<String>,
    }

    impl UiHandler {
        pub fn new(ui_rx: Receiver<String>, win_handle: AppHandle<Wry>) -> Self {
            let (tx, rx) = bounded(1);

            let am = AlarmManager::new(tx.clone(), rx.clone());
            am.run();

            Self {
                ui_rx,
                win_handle,
                tx,
                rx,
            }
        }

        pub fn run(mut self) {
            tokio::spawn(async move {
                loop {
                    match self.ui_rx.recv().await {
                        //Some(i) => {},
                        Some(i) => self.handle_command(i),
                        None => (),
                    }
                }
            });
        }

        fn handle_command(&self, cmd: String) {
            println!("{:?}", cmd);

            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&cmd) {
                println!("Parsed JSON: {:#?}", json);

                if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                    match CommandName::from_str(name) {
                        //Some(CommandName::UpdateRules) => handle_update_rules(json),
                        Some(CommandName::UpdateRules) => {
                            println!("handle_update_rules");
                            self.tx.send(cmd).unwrap();
                        }
                        _ => println!("none"),
                    }
                } else {
                    println!("No 'name' field or not a string in the JSON object.");
                }
            } else {
                eprintln!("Error while parsing JSON");
            }
        }

        //
        //

        //fn handle_rules_updated(json: serde_json::Value) {
        //if let Some(rules) = json.get("rules").and_then(serde_json::Value::as_array) {
        //let mut rule_objects: Vec<Rule> = Vec::new();
        //
        //for rule_json in rules {
        //let rule: Rule =
        //serde_json::from_value(rule_json.clone()).expect("Rule deserialization error");
        //rule_objects.push(rule);
        //}
        //
        //println!("{:#?}", rule_objects);
        //update_alarms(rule_objects);
        //}
        //}
        //
        //fn update_alarms(rules: Vec<Rule>) {
        //let alarms = get_alarms(&rules);
        //println!("alarms: {:?}", alarms);
        //}
    }
}

pub use ui_handler::*;
