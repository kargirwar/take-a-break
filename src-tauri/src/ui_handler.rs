mod ui_handler {
    use tokio::sync::mpsc::{Receiver};
    use tauri::AppHandle;
    use tauri::Wry;
    use tauri::Manager;
    use crate::Command;

    pub fn run(mut rx: Receiver<Command>, handle: AppHandle<Wry>) {
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    //Some(i) => {},
                    Some(i) => {handle_command(i, &handle)},
                    None => ()
                }
            }
        });
    }

    fn handle_command(cmd: Command, handle: &AppHandle<Wry>) {
        println!("{:?}", cmd);
        //handle.emit_all("some_event", "message").unwrap();
        match cmd.name {
            RULES_UPDATED => {update_rules(&cmd.payload)},
            _ => ()
        }
    }

    fn update_rules(payload: &str) {
        match serde_json::from_str::<serde_json::Value>(payload) {
            Ok(json) => {
                println!("Parsed JSON: {:#?}", json);
            },
            Err(err) => {
                eprintln!("Error while parsing JSON: {}", err);
            }
        }
    }
}

pub use ui_handler::*;
