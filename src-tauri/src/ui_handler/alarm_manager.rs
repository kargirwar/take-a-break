mod alarm;
mod alarm_manager {

    use super::alarm::*;
    use crate::player::play;
    use crate::AlarmTime;
    use crate::Command;
    use crate::CommandName;
    use crate::Payload;

    use log::debug;
    use serde::Deserialize;
    use std::collections::HashMap;
    use tokio::sync::broadcast::Receiver as BcastReceiver;
    use tokio::sync::broadcast::Sender as BcastSender;

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
        alarms: Vec<(u64, AlarmTime)>,
        index: u32,
    }

    impl AlarmManager {
        pub fn new(tx: BcastSender<Command>, rx: BcastReceiver<Command>) -> Self {
            let alarms: Vec<(u64, AlarmTime)> = Vec::new();
            let index = 0;
            Self {
                tx,
                rx,
                alarms,
                index,
            }
        }

        pub fn run(mut self) {
            debug!("alarm_manager:Running AlarmManager");
            tokio::spawn(async move {
                loop {
                    match self.rx.recv().await {
                        Ok(i) => self.handle_command(i),
                        Err(e) => debug!("{}", e),
                    };
                }
            });
        }

        fn handle_command(&mut self, cmd: Command) {
            debug!("alarm_manager:{:?}", cmd);
            match cmd.name {
                CommandName::UpdateAlarms => {
                    self.update_alarms(cmd.payload);
                    self.notify_next_alarm();
                }
                CommandName::PlayAlarm => {
                    play();
                    self.notify_next_alarm();
                }
                _ => debug!("alarm_manager::Unknown command"),
            }
        }

        fn update_alarms(&mut self, payload: Payload) {
            let rules;
            if let Payload::Rules(temp) = payload {
                rules = temp;
            } else {
                return;
            }

            self.alarms.clear();
            self.index = 0;

            //first shut down alarms if any
            let c = Command {
                name: CommandName::Shutdown,
                payload: Payload::None,
            };
            self.tx.send(c).unwrap();

            let alarms = get_alarms(&rules);
            for (day, hours_map) in &alarms {
                for (hours, minutes_vec) in hours_map {
                    for minutes in minutes_vec {
                        debug!("Day Hours Minutes: {} {} {}", day, hours, minutes);

                        let t = AlarmTime {
                            day: day.to_string(),
                            hours: *hours,
                            minutes: *minutes,
                        };

                        let a = Alarm::new(t.clone(), self.tx.clone(), self.tx.subscribe());

                        self.alarms.push((a.seconds.try_into().unwrap(), t));
                        a.run();
                    }
                }
            }

            //we are sorting the alarms by time from current instance.
            //By moving through these we will get the next alarmtime
            //we will wrap around to first after reachin end of vector
            self.alarms.sort_by_key(|tuple| tuple.0);
        }

        fn notify_next_alarm(&mut self) {
            if self.alarms.len() == 0 {
                return;
            }

            debug!("alarm_manager::alarms{:?}", self.alarms);
            debug!("alarm_manager::index{:?}", self.index);

            let alarms = self.get_next_prev_alarms();
            let c = Command {
                name: CommandName::NextAlarm,
                payload: Payload::Alarms(alarms),
            };

            self.tx.send(c).unwrap();
            self.index = (self.index + 1) % self.alarms.len() as u32;
        }

        fn get_next_prev_alarms(&self) -> (AlarmTime, AlarmTime) {
            let mut p;
            if self.index == 0 {
                p = self.alarms.len() as u32 - 1;
            } else {
                p = self.index - 1;
            }

            let a_p = &self.alarms[p as usize].1;
            let a_n = &self.alarms[self.index as usize].1;

            return (a_p.clone(), a_n.clone());
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
            let hours: HashMap<usize, Vec<usize>> = HashMap::new();

            for d in &r.days {
                alarms.insert(d.clone(), hours.clone());

                let s = r.from;
                let e = r.to;
                let f = r.interval;
                let hrs = get_hours(s, e);

                let mut i = 0;
                let mut m = f;
                let mut h;

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
                            alarms.get_mut(d).unwrap().insert(e, vec![0]);
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
        fn test_get_alarms() {
            let rule = Rule {
                serial: 1,
                days: vec!["Mon".to_string()],
                interval: 10,
                from: 9,
                to: 10,
            };
            let rules = vec![rule];
            let alarms = get_alarms(&rules);
            println!("{:?}", alarms);
            assert!(alarms.contains_key("Sun"));
        }
    }
}

pub use alarm_manager::*;
