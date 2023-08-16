mod alarm_manager {
    use crate::Rule;
    use crate::CommandName;
    use std::collections::HashMap;
    use tokio::sync::mpsc::{Receiver, Sender};

    pub struct AlarmManager {
        tx: Sender<String>,
        rx: Receiver<String>,
    }

    impl AlarmManager {
        pub fn new(tx: Sender<String>, rx: Receiver<String>) -> Self {
            Self {tx, rx}
        }

        pub async fn run(mut self) {
            println!("alarm_manager:Running AlarmManager");
            tokio::spawn(async move {
                loop {
                    match self.rx.recv().await {
                        Some(i) => {
                            self.handle_command(i).await;
                        },
                        None => print!("e"),
                    }
                }
            });
        }

        async fn handle_command(&self, cmd: String) {
            println!("alarm_manager:{:?}", cmd);

            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&cmd) {
                println!("alarm_manager:Parsed JSON: {:#?}", json);

                if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                    match CommandName::from_str(name) {
                        Some(CommandName::UpdateRules) => {
                            handle_update_rules(json);
                            self.tx.send("Ack".to_string()).await.unwrap();
                        }
                        _ => println!("alarm_manager:none"),
                    }
                } else {
                    println!("alarm_manager:No 'name' field or not a string in the JSON object.");
                }
            } else {
                eprintln!("alarm_manager:Error while parsing JSON");
            }
        }
    }

    fn handle_update_rules(json: serde_json::Value) {
        if let Some(rules) = json.get("rules").and_then(serde_json::Value::as_array) {
            let mut rule_objects: Vec<Rule> = Vec::new();

            for rule_json in rules {
                let rule: Rule =
                    serde_json::from_value(rule_json.clone()).expect("alarm_manager:Rule deserialization error");
                rule_objects.push(rule);
            }

            println!("alarm_manager:{:#?}", rule_objects);
            update_alarms(rule_objects);
        }
    }

    fn update_alarms(rules: Vec<Rule>) {
        let alarms = get_alarms(&rules);
        println!("alarm_manager:alarms: {:?}", alarms);
    }

    fn get_hours(s: usize, e: usize) -> Vec<usize> {
        let mut hrs = Vec::new();

        let mut current_hour = s;
        while current_hour <= e {
            hrs.push(current_hour);
            current_hour += 1;
        }

        hrs
    }

    fn get_alarms(rules: &[Rule]) -> HashMap<String, HashMap<usize, Vec<usize>>> {
        let mut alarms: HashMap<String, HashMap<usize, Vec<usize>>> = HashMap::new();

        for r in rules {
            let mut hours: HashMap<usize, Vec<usize>> = HashMap::new();

            for d in &r.days {
                alarms.insert(d.clone(), hours.clone());

                let s = r.from;
                let e = r.to;
                let f = r.interval;
                let hrs = get_hours(s, e);

                let mut i = 0;
                let mut m = f;
                let mut h = 0;

                loop {
                    let mut mins = Vec::new();

                    loop {
                        mins.push(m % 60);
                        m += f;

                        if m - (60 * i) >= 60 {
                            h = hrs[i];

                            if f == 60 && h == s {
                                i += 1;
                                break;
                            }

                            alarms.get_mut(d).unwrap().insert(h, mins.clone());
                            println!("{} h: {} mins: {:?}", d, h, mins);

                            i += 1;
                            break;
                        }
                    }

                    if e == hrs[i] {
                        if m % 60 == 0 {
                            alarms
                                .get_mut(d)
                                .unwrap()
                                .insert(e, vec![0]);
                            println!("{} h: {} mins: {:?}", d, e, alarms[d].get(&e).unwrap());
                        }
                        break;
                    }
                }
            }
        }

        alarms
    }
}

pub use alarm_manager::*;
