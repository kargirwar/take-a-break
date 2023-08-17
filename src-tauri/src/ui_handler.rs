mod alarm_manager;
mod ui_handler {

    use super::alarm_manager::*;
    use tauri::AppHandle;
    use tauri::Wry;
    use tokio::select;
    use tokio::sync::broadcast;
    use tokio::sync::broadcast::Sender as BcastSender;
    use tokio::sync::broadcast::Receiver as BcastReceiver;
    use tokio::sync::mpsc::{Receiver};

    #[derive(Clone, Debug, PartialEq)]
    pub enum CommandName {
        //from the UI
        UpdateRules,
        //For alarm module
        Shutdown,
        //For alarm manager
        UpdateAlarms,
        PlayAlarm, //received from alarm module
        //For the UI handler
        NextAlarm, //from alarm manager to ui handler
        PlayAlarm1, //received from alarm module
        PlayAlarm2, //received from alarm module
    }

    impl CommandName {
        pub fn from_str(name: &str) -> Option<Self> {
            match name {
                "update-rules" => Some(CommandName::UpdateRules),
                _ => None,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Command {
        pub name: CommandName,
        pub rules: Option<Vec<Rule>>,
    }

    pub struct UiHandler {
        ui_rx: Receiver<String>,
        win_handle: AppHandle<Wry>,

        //for managing communication with alarm manager
        am_tx: BcastSender<Command>,
        am_rx: BcastReceiver<Command>,
    }

    impl UiHandler {
        pub fn new(ui_rx: Receiver<String>, win_handle: AppHandle<Wry>) -> Self {
            let (am_tx, am_rx): (BcastSender<Command>, BcastReceiver<Command>) = broadcast::channel(500);
            let am = AlarmManager::new(am_tx.clone(), am_tx.subscribe());
            am.run();

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
                                    self.handle_command(msg);
                                },
                                None => println!("Channel 1 closed"),
                            }
                        }
                        message2 = self.am_rx.recv() => {
                            match message2 {
                                Ok(msg) => println!("Received from AM: {:?}", msg),
                                Err(e) => println!("{}", e),
                            }
                        }
                    }
                }
            });
        }

        fn handle_command(&self, cmd: String) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&cmd) {
                if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                    match CommandName::from_str(name) {
                        Some(CommandName::UpdateRules) => {
                            self.handle_update_rules(json);
                        }
                        _ => println!("ui_handler::Unknown command"),
                    }
                } else {
                    println!("ui_handler: No 'name' field or not a string in the JSON object.");
                }
            } else {
                eprintln!("ui_handler: Error while parsing JSON");
            }
        }

        fn handle_update_rules(&self, json: serde_json::Value) {
            let mut rule_objects: Vec<Rule> = Vec::new();

            if let Some(rules) = json.get("rules").and_then(serde_json::Value::as_array) {
                for rule_json in rules {
                    let rule: Rule =
                        serde_json::from_value(rule_json.clone()).expect("ui_handler:Rule deserialization error");
                    rule_objects.push(rule);
                }

                println!("ui_handler:{:#?}", rule_objects);
            }

            let c = Command{name: CommandName::UpdateAlarms, rules: Some(rule_objects)};
            self.am_tx.send(c).unwrap();
        }
    }
}

pub use ui_handler::*;
