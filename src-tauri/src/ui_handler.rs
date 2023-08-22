mod alarm_manager;
mod ui_handler {

    use super::alarm_manager::*;
    use crate::utils::*;
    use log::debug;
    use serde_json::json;
    use std::fmt;
    use tauri::AppHandle;
    use tauri::Manager;
    use tauri::Wry;
    use tokio::select;
    use tokio::sync::broadcast;
    use tokio::sync::broadcast::Receiver as BcastReceiver;
    use tokio::sync::broadcast::Sender as BcastSender;
    use tokio::sync::mpsc::Receiver;
    use std::fs::File;
    use std::io::Write;

    const BCAST_CHANNEL_SIZE: usize = 10;

    #[derive(Clone, Debug)]
    pub struct AlarmTime {
        pub day: String,
        pub hours: usize,
        pub minutes: usize,
    }

    impl fmt::Display for AlarmTime {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}-{}-{}", self.day, self.hours, self.minutes)
        }
    }

    #[derive(Clone, Debug)]
    pub enum Payload {
        Rules(Vec<Rule>),
        Alarms((AlarmTime, AlarmTime)),
        None,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum CommandName {
        //from the UI
        UpdateRules,
        //For alarm module
        Shutdown,
        //For alarm manager
        UpdateAlarms,
        //For the UI handler
        NextAlarm, //from alarm manager to ui handler
        PlayAlarm, //received from alarm module
    }

    impl CommandName {
        pub fn from_str(name: &str) -> Option<Self> {
            match name {
                "update-rules" => Some(CommandName::UpdateRules),
                _ => None,
            }
        }
    }

    impl fmt::Display for CommandName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CommandName::NextAlarm => write!(f, "event-next-alarm"),
                _ => write!(f, "not-implemented"),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Command {
        pub name: CommandName,
        pub payload: Payload,
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
            let (am_tx, am_rx): (BcastSender<Command>, BcastReceiver<Command>) =
                broadcast::channel(BCAST_CHANNEL_SIZE);
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
                                    debug!("Received from UI: {}", msg);
                                    self.handle_ui_command(msg);
                                },
                                None => debug!("Channel 1 closed"),
                            }
                        }
                        message2 = self.am_rx.recv() => {
                            match message2 {
                                Ok(msg) => self.handle_am_command(msg),
                                Err(e) => debug!("{}", e),
                            }
                        }
                    }
                }
            });
        }

        fn handle_am_command(&self, cmd: Command) {
            let payload;
            if let CommandName::NextAlarm = cmd.name {
                payload = cmd.payload;
            } else {
                return;
            }

            debug!("handle_am_command: {:?}", payload);
            match payload {
                Payload::Alarms(i) => {
                    let json = json!({
                        "prev-alarm": i.0.to_string(),
                        "next-alarm": i.1.to_string()
                    });
                    self.win_handle
                        .emit_all(&CommandName::NextAlarm.to_string(), json.to_string())
                        .unwrap();
                }
                _ => debug!("ui_handler: invalid"),
            };
        }

        fn handle_ui_command(&self, cmd: String) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&cmd) {
                if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                    match CommandName::from_str(name) {
                        Some(CommandName::UpdateRules) => {
                            self.handle_update_rules(json);
                        }
                        _ => debug!("ui_handler::Unknown command"),
                    }
                } else {
                    debug!("ui_handler: No 'name' field or not a string in the JSON object.");
                }
            } else {
                debug!("ui_handler: Error while parsing JSON");
            }
        }

        fn handle_update_rules(&self, json: serde_json::Value) {
            debug!("rules: {}", json);
            let mut rule_objects: Vec<Rule> = Vec::new();

            if let Some(rules) = json.get("rules").and_then(serde_json::Value::as_array) {
                for rule_json in rules {
                    let rule: Rule = serde_json::from_value(rule_json.clone())
                        .expect("ui_handler:Rule deserialization error");
                    rule_objects.push(rule);
                }

                debug!("ui_handler:{:#?}", rule_objects);
            }

            Self::save_rules(&rule_objects);

            let c = Command {
                name: CommandName::UpdateAlarms,
                payload: Payload::Rules(rule_objects),
            };
            self.am_tx.send(c).unwrap();
        }

        fn save_rules(rules: &Vec<Rule>) {
            let serialized_rules = serde_json::to_string(&rules).unwrap();
            let mut file = File::create(get_settings_file_name()).unwrap();
            file.write_all(serialized_rules.as_bytes()).unwrap();
        }
    }
}

pub use ui_handler::*;
