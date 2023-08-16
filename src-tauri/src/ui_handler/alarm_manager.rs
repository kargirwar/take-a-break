mod alarm_manager {
    use crate::CommandName;
    use crate::Command;
    use serde::Deserialize;
    use std::collections::HashMap;
    use tokio::sync::broadcast::{Receiver, Sender};

    #[derive(Debug, Deserialize, Clone)]
    pub struct Rule {
        pub days: Vec<String>,
        pub from: usize,
        pub interval: usize,
        pub serial: usize,
        pub to: usize,
    }

    pub struct AlarmManager {
        tx: Sender<Command>,
        rx: Receiver<Command>,
    }

    impl AlarmManager {
        pub fn new(tx: Sender<Command>, rx: Receiver<Command>) -> Self {
            Self {tx, rx}
        }

        pub async fn run(mut self) {
            println!("alarm_manager:Running AlarmManager");
            tokio::spawn(async move {
                loop {
                    match self.rx.recv().await {
                        Ok(i) => self.handle_command(i),
                        Err(e) => println!("{}", e)
                    };
                }
            });
        }

        fn handle_command(&self, cmd: Command) {
            println!("alarm_manager:{:?}", cmd);
            match cmd.name {
                CommandName::UpdateAlarms => update_alarms(cmd.rules),
                _ => println!("alarm_manager::Unknown command")
            }
        }
    }

    fn update_alarms(rules: Vec<Rule>) {
        let alarms = get_alarms(&rules);
        println!("alarm_manager:updating alarms: {:?}", alarms);
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
