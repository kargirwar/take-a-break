//commands from JS
//const JS_COMMAND_RULES_UPDATED: &str = "rules-updated";

mod ui_handler {
    use tokio::sync::mpsc::{Receiver};
    use tauri::AppHandle;
    use tauri::Wry;
    //use tauri::Manager;

    pub fn run(mut rx: Receiver<String>, _handle: AppHandle<Wry>) {
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    //Some(i) => {},
                    Some(i) => {handle_command(i)},
                    None => ()
                }
            }
        });
    }

    fn handle_command(cmd: String) {
        println!("{:?}", cmd);

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&cmd) {
            println!("Parsed JSON: {:#?}", json);

            if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                match name {
                    "rules-updated" => handle_rules_updated(json),
                    _ => println!("none"),
                }
            } else {
                println!("No 'name' field or not a string in the JSON object.");
            }
        } else {
            eprintln!("Error while parsing JSON");
        }
    }

    fn handle_rules_updated(json: serde_json::Value) {
        if let Some(payload) = json.get("payload").and_then(|p| p.as_str()) {
            update_alarms(payload.to_string());
        } else {
            println!("Invalid payload");
        }
    }

    fn update_alarms(payload: String) {
        println!("{}", "Updating alarms with".to_owned() + &payload);
    }
}

pub use ui_handler::*;
