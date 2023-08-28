//! Implements core timer functionality. Starts a thread which 
//! wakes up every minute and checks if an alarm is to be played.
//! CPU usage (on Mac) is not significant
//! The tread runs throughout the life of the app. No need to handle
//! shutdown

mod alarm_manager {

    use crate::player::play;
    use crate::{BcastReceiver, BcastSender, Message, MessageType, Payload, Rule};
    use chrono::{offset::Local, Datelike, Timelike, Weekday};
    use std::time::Duration;

    use log::debug;
    use std::collections::HashMap;

    pub struct AlarmManager {
        tx: BcastSender<Message>,
        rx: BcastReceiver<Message>,
        alarms: HashMap<Weekday, HashMap<usize, Vec<usize>>>,
    }

    impl AlarmManager {
        pub fn new(tx: BcastSender<Message>, rx: BcastReceiver<Message>) -> Self {
            let alarms: HashMap<Weekday, HashMap<usize, Vec<usize>>> = HashMap::new();
            Self { tx, rx, alarms }
        }

        pub fn run(mut self) {
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        msg = self.rx.recv() => {
                            match msg {
                                Ok(i) => self.handle_message(i),
                                Err(e) => debug!("{}", e),
                            }
                        }

                        _ = tokio::time::sleep(Duration::from_secs(60)) => {
                            self.handle_timer_expiry();
                        }
                    }
                }
            });
        }

        /// Recurring 1 minute timer
        fn handle_timer_expiry(&self) {
            debug!("alarm_manager: timer expiry");
            let now = chrono::offset::Local::now();

            let now = Local::now();
            let current_weekday: Weekday = now.weekday();
            let current_hour: usize = now.hour() as usize;
            let current_minute: usize = now.minute() as usize;

            if let Some(hour_map) = self.alarms.get(&current_weekday) {
                if let Some(minute_vec) = hour_map.get(&current_hour) {
                    if minute_vec.contains(&current_minute) {
                        debug!("alarm_manager: playing alarm");
                        play();
                    }
                }
            }
        }

        /// Handles message from ui_handlers
        fn handle_message(&mut self, msg: Message) {
            debug!("alarm_manager:{:?}", msg);
            match msg.typ {
                MessageType::CmdUpdateAlarms => {
                    self.update_alarms(msg.payload);
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

            self.alarms = get_alarms(&rules);
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

    /// For a set of rules, finds the hours and minutes for each day at which 
    /// alarm should be played. Assumes that rules are not overlapping
    fn get_alarms(rules: &[Rule]) -> HashMap<Weekday, HashMap<usize, Vec<usize>>> {
        let mut alarms: HashMap<Weekday, HashMap<usize, Vec<usize>>> = HashMap::new();

        for r in rules {
            for d in &r.days {
                let weekday = get_weekday(d).unwrap();
                let hours = alarms.entry(weekday).or_insert_with(HashMap::new);

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

    fn get_weekday(d: &str) -> Result<Weekday, String> {
        match d {
            "Mon" => Ok(Weekday::Mon),
            "Tue" => Ok(Weekday::Tue),
            "Wed" => Ok(Weekday::Wed),
            "Thu" => Ok(Weekday::Thu),
            "Fri" => Ok(Weekday::Fri),
            "Sat" => Ok(Weekday::Sat),
            "Sun" => Ok(Weekday::Sun),
            _ => Err("Invalid weekday input".to_string()),
        }
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
            assert!(alarms.contains_key(&Weekday::Tue));
            assert!(alarms.contains_key(&Weekday::Wed));
        }
    }
}

pub use alarm_manager::*;
