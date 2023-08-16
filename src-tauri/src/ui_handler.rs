mod alarm_manager;
mod ui_handler {

    use super::alarm_manager::*;
    use serde::Deserialize;
    use tauri::AppHandle;
    use tauri::Wry;
    use tokio::sync::mpsc;
    use tokio::sync::mpsc::{Receiver, Sender};
    use tokio::select;

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
        am_tx: Sender<String>,
        am_rx: Receiver<String>,
    }

    impl UiHandler {
        pub async fn new(ui_rx: Receiver<String>, win_handle: AppHandle<Wry>) -> Self {
            //ui will transmit on am_tx and receive on am_rx
            //am will receive on rx and transmit on tx
            let (am_tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel(1);
            let (tx, am_rx): (Sender<String>, Receiver<String>) = mpsc::channel(1);

            let am = AlarmManager::new(tx, rx);
            am.run().await;

            Self {
                ui_rx,
                win_handle,
                am_tx,
                am_rx,
            }
        }

        pub fn run(mut self) {
            tokio::spawn(async move {
                loop {
                    select! {
                        message1 = self.ui_rx.recv() => {
                            match message1 {
                                Some(msg) => {
                                    println!("Received from UI: {}", msg);
                                    self.handle_command(msg).await;
                                },
                                None => println!("Channel 1 closed"),
                            }
                        }
                        message2 = self.am_rx.recv() => {
                            match message2 {
                                Some(msg) => println!("Received from AM: {}", msg),
                                None => println!("Channel 2 closed"),
                            }
                        }
                    }
                }
            });
        }

        async fn handle_command(&self, cmd: String) {
            println!("{:?}", cmd);

            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&cmd) {
                println!("Parsed JSON: {:#?}", json);

                if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                    match CommandName::from_str(name) {
                        //Some(CommandName::UpdateRules) => handle_update_rules(json),
                        Some(CommandName::UpdateRules) => {
                            println!("handle_update_rules");
                            self.am_tx.send(cmd).await.unwrap();
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
