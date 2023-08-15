mod ui_handler {

    enum CommandName {
        RulesUpdated,
    }

    impl CommandName {
        fn from_str(name: &str) -> Option<Self> {
            match name {
                "rules-updated" => Some(CommandName::RulesUpdated),
                _ => None,
            }
        }
    }

    use serde_json::Value;
    use tauri::AppHandle;
    use tauri::Wry;
    use tokio::sync::mpsc::Receiver;
    //use tauri::Manager;

    pub fn run(mut rx: Receiver<String>, _handle: AppHandle<Wry>) {
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    //Some(i) => {},
                    Some(i) => handle_command(i),
                    None => (),
                }
            }
        });
    }

    fn handle_command(cmd: String) {
        println!("{:?}", cmd);

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&cmd) {
            println!("Parsed JSON: {:#?}", json);

            if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                match CommandName::from_str(name) {
                    Some(CommandName::RulesUpdated) => handle_rules_updated(json),
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
        if let Some(_payload) = json.get("rules").and_then(Value::as_array) {
            //update_alarms(payload);
        } else {
            println!("Invalid payload");
        }
    }

    fn update_alarms(_payload: Vec<serde_json::Value>) {
        //println!("{}", "Updating alarms with".to_owned() + &payload);
    }
}

pub use ui_handler::*;
