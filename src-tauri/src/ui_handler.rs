mod alarm_manager;
mod ui_handler {

    use super::alarm_manager::*;
    use serde::Deserialize;
    use tauri::AppHandle;
    use tauri::Wry;
    use tokio::select;
    use tokio::sync::mpsc;
    use tokio::sync::mpsc::{Receiver, Sender};

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
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&cmd) {
                if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                    match CommandName::from_str(name) {
                        Some(CommandName::UpdateRules) => {
                            self.am_tx.send(cmd).await.unwrap();
                        }
                        _ => println!("none"),
                    }
                } else {
                    println!("ui_handler: No 'name' field or not a string in the JSON object.");
                }
            } else {
                eprintln!("ui_handler: Error while parsing JSON");
            }
        }
    }
}

pub use ui_handler::*;
