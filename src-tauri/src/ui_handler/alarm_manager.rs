mod alarm;
mod types;
mod alarm_manager {

    use super::alarm::*;
    use super::types::*;
    use crate::player::play;
    use crate::{AlarmTime, BcastReceiver, BcastSender, Command, CommandName, Payload, Rule};

    use log::debug;
    use std::collections::HashMap;

    pub struct AlarmManager<T> {
        tx: BcastSender<Command>,
        rx: BcastReceiver<Command>,
        alarms: WrappingVec<T>,
        prev_alarm: AlarmTime,
    }

    impl AlarmManager<(u64, AlarmTime)> {
        pub fn new(tx: BcastSender<Command>, rx: BcastReceiver<Command>) -> Self {
            let alarms = WrappingVec::new();
            let prev_alarm = AlarmTime {
                day: "Xxx".to_string(),
                hours: 0,
                minutes: 0,
            };
            Self {
                tx,
                rx,
                alarms,
                prev_alarm,
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
                    self.update_prev_alarm();
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
            debug!("alarm_manager::alarms{:?}", self.alarms);

            let prev = &self.prev_alarm;

            if let Some(next) = self.alarms.next() {
                let c = Command {
                    name: CommandName::NextAlarm,
                    payload: Payload::Alarms((prev.clone(), next.1.clone())),
                };

                self.tx.send(c).unwrap();
            }
        }

        fn update_prev_alarm(&mut self) {
            if let Some(prev) = self.alarms.prev() {
                self.prev_alarm = prev.1.clone();
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
            for d in &r.days {
                let hours = alarms.entry(d.clone()).or_insert_with(HashMap::new);

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

                            hours.entry(h).or_insert_with(Vec::new).extend(mins.iter());
                            debug!("{} h: {} mins: {:?}", d, h, mins);

                            i += 1;
                            break;
                        }
                    }

                    if e == hrs[i] {
                        if m % 60 == 0 {
                            hours.entry(e).or_insert_with(|| vec![0]);
                            debug!("{} h: {} mins: {:?}", d, e, hours[&e]);
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
        use crate::utils::*;

        #[test]
        fn test_get_alarms() {
            setup_logger();
            let rule1 = Rule {
                serial: 1,
                days: vec!["Tue".to_string(), "Wed".to_string()],
                interval: 1,
                from: 18,
                to: 19,
            };

            let rule2 = Rule {
                serial: 2,
                days: vec!["Tue".to_string()],
                interval: 30,
                from: 19,
                to: 20,
            };

            let rules = vec![rule1, rule2];
            let alarms = get_alarms(&rules);
            debug!("{:?}", alarms);
            assert!(alarms.contains_key("Sun"));
        }
    }
}

pub use alarm_manager::*;
