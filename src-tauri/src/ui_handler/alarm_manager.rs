//! Implements core timer functionality. Starts a thread which
//! wakes up every minute and checks if an alarm is to be played.
//! CPU usage (on Mac) is not significant
//! The tread runs throughout the life of the app. No need to handle
//! shutdown

mod alarm_utils;
mod alarm_manager {

    use crate::player::play;
    use crate::{BcastReceiver, BcastSender, Message, MessageType, Payload, Rule};
    use chrono::{offset::Local, Datelike, Timelike, Weekday};
    use std::time::Duration;
    use super::alarm_utils::*;

    use log::debug;
    use std::collections::HashMap;

    #[allow(dead_code)]
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

            if let Some(alarm) =
                find_next_alarm(&self.alarms, current_weekday, current_hour, current_minute)
            {
                debug!("Next alarm at: {:?}", alarm);
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::utils::*;
        use maplit::hashmap;

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

            let mins: Vec<usize> = (1..=59).collect();
            let expected = hashmap! {
                Weekday::Tue => hashmap! {
                    18 => mins.clone(),
                    19 => vec![0, 30],
                    20 => vec![0],
                },
                Weekday::Wed => hashmap! {
                    18 => mins.clone(),
                    19 => vec![0],
                },
            };

            assert!(alarms.contains_key(&Weekday::Tue));
            assert!(alarms.contains_key(&Weekday::Wed));
            assert_eq!(alarms, expected);
        }

        #[test]
        fn test_next_alarm() {
            setup_logger();
            let rule1 = Rule {
                serial: 1,
                days: vec!["Fri".to_string()],
                interval: 30,
                from: 17,
                to: 18,
            };

            let rule2 = Rule {
                serial: 2,
                days: vec!["Sun".to_string()],
                interval: 30,
                from: 19,
                to: 20,
            };

            let rules = vec![rule1, rule2];
            let alarms = get_alarms(&rules);

            let expected = hashmap! {
                Weekday::Fri => hashmap! {
                    17 => vec![30],
                    18 => vec![0],
                },
                // Add more weekdays and alarms as needed
                Weekday::Sun => hashmap! {
                    19 => vec![30],
                    20 => vec![0],
                },
            };

            assert_eq!(alarms, expected);

            let mut next = find_next_alarm(&alarms, Weekday::Fri, 17, 58);
            assert_eq!(next, Some((Weekday::Fri, 18, 0)));

            next = find_next_alarm(&alarms, Weekday::Sat, 18, 0);
            assert_eq!(next, Some((Weekday::Sun, 19, 30)));

            next = find_next_alarm(&alarms, Weekday::Sun, 18, 0);
            assert_eq!(next, Some((Weekday::Sun, 19, 30)));

            next = find_next_alarm(&alarms, Weekday::Sun, 19, 0);
            assert_eq!(next, Some((Weekday::Sun, 19, 30)));

            next = find_next_alarm(&alarms, Weekday::Sun, 19, 20);
            assert_eq!(next, Some((Weekday::Sun, 19, 30)));

            next = find_next_alarm(&alarms, Weekday::Sun, 19, 31);
            assert_eq!(next, Some((Weekday::Sun, 20, 0)));

            next = find_next_alarm(&alarms, Weekday::Sun, 20, 31);
            assert_eq!(next, Some((Weekday::Fri, 17, 30)));
        }
    }
}

pub use alarm_manager::*;
