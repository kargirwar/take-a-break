//! Module to handle interactions with UI.
//! Receives commands from JS world and sends back events.
mod alarm_manager;
mod ui_handler {

    use super::alarm_manager::*;
    use crate::utils::*;
    use chrono::Weekday;
    use log::debug;
    use serde::ser::SerializeStruct;
    use serde::Deserialize;
    use serde::Serialize;
    use serde::Serializer;
    use serde_json::json;
    use serde_json::Value;
    use std::fmt;
    use std::fs::{File, OpenOptions};
    use std::io::{Read, Write};
    use tauri::AppHandle;
    use tauri::Manager;
    use tauri::Wry;
    use tokio::select;
    use tokio::sync::broadcast;
    use tokio::sync::mpsc::Receiver;

    use tokio::sync::broadcast::Receiver as R;
    pub type BcastReceiver<T> = R<T>;

    use tokio::sync::broadcast::Sender as S;
    pub type BcastSender<T> = S<T>;

    const BCAST_CHANNEL_SIZE: usize = 10;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Rule {
        pub days: Vec<String>,
        pub from: usize,
        pub interval: usize,
        pub serial: usize,
        pub to: usize,
    }

    #[derive(Clone, Debug)]
    pub enum Payload {
        Rules(Vec<Rule>),
        Alarm(Option<Alarm>),
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum MessageType {
        //from the UI
        CmdStartup,
        CmdUpdateRules,
        //For UI
        EvtRulesApplied,

        //For alarm module
        //CmdShutdown,
        //received from alarm module
        //CmdPlayAlarm,

        //For alarm manager
        CmdUpdateAlarms,
        //From alarm manager
        EvtNextAlarm,
        EvtPlayingAlarm,
    }

    impl MessageType {
        pub fn from_str(typ: &str) -> Option<Self> {
            match typ {
                "cmd-update-rules" => Some(MessageType::CmdUpdateRules),
                "cmd-startup" => Some(MessageType::CmdStartup),
                _ => None,
            }
        }
    }

    impl fmt::Display for MessageType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MessageType::EvtNextAlarm => write!(f, "event-next-alarm"),
                MessageType::EvtRulesApplied => write!(f, "event-rules-applied"),
                _ => write!(f, "not-implemented"),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Message {
        pub typ: MessageType,
        pub payload: Payload,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Alarm {
        pub day: Weekday,
        pub hour: usize,
        pub min: usize,
    }

    impl Serialize for Alarm {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("Alarm", 3)?;
            state.serialize_field("day", &format!("{:?}", self.day))?;
            state.serialize_field("hour", &self.hour)?;
            state.serialize_field("min", &self.min)?;
            state.end()
        }
    }

    pub struct UiHandler {
        ui_rx: Receiver<String>,
        win_handle: AppHandle<Wry>,

        //for managing communication with alarm manager
        am_tx: BcastSender<Message>,
        am_rx: BcastReceiver<Message>,
        rules: Vec<Rule>,
        prev_alarm: Option<Alarm>,
    }

    impl UiHandler {
        pub fn new(ui_rx: Receiver<String>, win_handle: AppHandle<Wry>) -> Self {
            let (am_tx, am_rx): (BcastSender<Message>, BcastReceiver<Message>) =
                broadcast::channel(BCAST_CHANNEL_SIZE);

            let am = AlarmManager::new(am_tx.clone(), am_tx.subscribe());
            am.run();

            let rules = Self::read_rules();
            let prev_alarm = None;

            Self {
                ui_rx,
                win_handle,
                am_tx,
                am_rx,
                rules,
                prev_alarm,
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
                                    self.handle_ui_message(msg);
                                },
                                None => debug!("Channel 1 closed"),
                            }
                        }
                        message2 = self.am_rx.recv() => {
                            match message2 {
                                Ok(msg) => self.handle_am_message(msg),
                                Err(e) => debug!("{}", e),
                            }
                        }
                    }
                }
            });
        }

        fn handle_am_message(&mut self, msg: Message) {
            match msg.typ {
                MessageType::EvtNextAlarm => self.handle_next_alarm(msg.payload),
                MessageType::EvtPlayingAlarm => self.handle_playing_alarm(msg.payload),
                _ => (),
            }
        }

        fn handle_next_alarm(&self, payload: Payload) {
            let json: Value = match payload {
                Payload::Alarm(alarm) => match alarm {
                    Some(alarm) => {
                        json!({
                            "next-alarm": alarm,
                            "prev-alarm": self.prev_alarm
                        })
                    }
                    None => json!({
                        "next-alarm": null,
                        "prev-alarm": null
                    }),
                },
                _ => return,
            };

            //inform UI
            self.win_handle
                .emit_all(&MessageType::EvtNextAlarm.to_string(), json.to_string())
                .unwrap();
        }

        fn handle_playing_alarm(&mut self, payload: Payload) {
            match payload {
                Payload::Alarm(alarm) => match alarm {
                    Some(alarm) => {
                        self.prev_alarm = Some(alarm);
                    }
                    None => (),
                },
                _ => return,
            };
        }

        fn handle_ui_message(&self, msg: String) {
            let json = match serde_json::from_str::<serde_json::Value>(&msg) {
                Ok(parsed) => parsed,
                Err(_) => return,
            };

            if let Some(typ) = json.get("type").and_then(|n| n.as_str()) {
                match MessageType::from_str(typ) {
                    Some(MessageType::CmdUpdateRules) => {
                        self.handle_update_rules(json);
                    }

                    Some(MessageType::CmdStartup) => {
                        self.handle_startup();
                    }
                    _ => debug!("ui_handler::Unknown command"),
                }
            }
        }

        fn handle_startup(&self) {
            //startoff timers
            let c = Message {
                typ: MessageType::CmdUpdateAlarms,
                payload: Payload::Rules(self.rules.clone()),
            };

            self.am_tx.send(c).unwrap();

            //inform UI
            let json = json!({
                "rules": serde_json::to_string(&self.rules).unwrap()
            });

            self.win_handle
                .emit_all(&MessageType::EvtRulesApplied.to_string(), json.to_string())
                .unwrap();
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

            let c = Message {
                typ: MessageType::CmdUpdateAlarms,
                payload: Payload::Rules(rule_objects),
            };
            self.am_tx.send(c).unwrap();
        }

        fn save_rules(rules: &Vec<Rule>) {
            let serialized_rules = serde_json::to_string(&rules).unwrap();
            let mut file = File::create(get_settings_file_name()).unwrap();
            file.write_all(serialized_rules.as_bytes()).unwrap();
        }

        fn read_rules() -> Vec<Rule> {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(get_settings_file_name())
                .unwrap();

            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            // Deserialize the content into a Vec<Rule>
            let rules: Vec<Rule> = if contents.is_empty() {
                Vec::new()
            } else {
                serde_json::from_str(&contents).unwrap()
            };

            return rules;
        }
    }
}

pub use ui_handler::*;
