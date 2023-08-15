mod alarm_manager;
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

    #[derive(Debug, Deserialize)]
    pub struct Rule {
        pub days: Vec<String>,
        pub from: usize,
        pub interval: usize,
        pub serial: usize,
        pub to: usize,
    }

    use serde::Deserialize;
    use serde_json::Value;
    use tauri::AppHandle;
    use tauri::Wry;
    use tokio::sync::mpsc::Receiver;
    use super::alarm_manager::*;
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
        if let Some(rules) = json.get("rules").and_then(serde_json::Value::as_array) {
            let mut rule_objects: Vec<Rule> = Vec::new();

            for rule_json in rules {
                let rule: Rule =
                    serde_json::from_value(rule_json.clone()).expect("Rule deserialization error");
                rule_objects.push(rule);
            }

            println!("{:#?}", rule_objects);
            update_alarms(rule_objects);
        }
    }

    fn update_alarms(rules: Vec<Rule>) {
        let alarms = get_alarms(&rules);
        println!("alarms: {:?}", alarms);
    }
}

pub use ui_handler::*;
