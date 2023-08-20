mod alarm;
mod alarm_manager {

    use crate::player::play;
    use super::alarm::*;
    use crate::CommandName;
    use crate::Command;
    use serde::Deserialize;
    use std::collections::HashMap;
    use tokio::sync::broadcast::Sender as BcastSender;
    use tokio::sync::broadcast::Receiver as BcastReceiver;
    use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};

    #[derive(Debug, Deserialize, Clone)]
    pub struct Rule {
        pub days: Vec<String>,
        pub from: usize,
        pub interval: usize,
        pub serial: usize,
        pub to: usize,
    }

    pub struct AlarmManager {
        tx: BcastSender<Command>,
        rx: BcastReceiver<Command>,
    }

    impl AlarmManager {
        pub fn new(tx: BcastSender<Command>, rx: BcastReceiver<Command>) -> Self {
            Self {tx, rx}
        }

        pub fn run(mut self) {
            debug!("alarm_manager:Running AlarmManager");
            tokio::spawn(async move {
                loop {
                    match self.rx.recv().await {
                        Ok(i) => self.handle_command(i),
                        Err(e) => debug!("{}", e)
                    };
                }
            });
        }

        fn handle_command(&mut self, cmd: Command) {
            debug!("alarm_manager:{:?}", cmd);
            match cmd.name {
                CommandName::UpdateAlarms => self.update_alarms(cmd.rules),
                CommandName::PlayAlarm => {
                    play();
                }
                _ => debug!("alarm_manager::Unknown command")
            }
        }

        fn update_alarms(&mut self, rules: Option<Vec<Rule>>) {
            if rules.is_none() {
                return;
            }

            let rules = rules.unwrap().to_vec();

            //first shut down alarms if any
            let c = Command{name: CommandName::Shutdown, rules: None};
            self.tx.send(c).unwrap();

            let alarms = get_alarms(&rules);
            for (day, hours_map) in &alarms {
                for (hours, minutes_vec) in hours_map {
                    for minutes in minutes_vec {
                        debug!("day hours Minute: {} {} {}", day, hours, minutes);
                        let a = Alarm::new(AlarmTime{
                            day: day.to_string(),
                            hours: *hours,
                            minutes: *minutes
                        }, self.tx.clone(), self.tx.subscribe());

                        a.run();
                    }
                }
            }
        }
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
                            debug!("{} h: {} mins: {:?}", d, h, mins);

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
                            debug!("{} h: {} mins: {:?}", d, e, alarms[d].get(&e).unwrap());
                        }
                        break;
                    }
                }
            }
        }

        alarms
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test() {
            let rule = Rule{serial: 1, days: vec!["Sun".to_string()], interval: 10, from: 16, to: 17};
            let rules = vec![rule];
            let alarms = get_alarms(&rules);
            assert!(alarms.contains_key("Sun"));
        }
    }
}

pub use alarm_manager::*;
