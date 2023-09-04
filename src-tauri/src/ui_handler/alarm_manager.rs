//! Implements core timer functionality. Starts a thread which
//! wakes up every minute and checks if an alarm is to be played.
//! CPU usage (on Mac) is not significant
//! The tread runs throughout the life of the app. No need to handle
//! shutdown

mod alarm_utils;
mod alarm_manager {

    use super::alarm_utils::*;
    use crate::player::play;
    use crate::Alarm;
    use crate::{BcastReceiver, BcastSender, Message, MessageType, Payload, Rule};
    use chrono::{offset::Local, Datelike, Timelike, Weekday};
    use std::time::Duration;

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

                        let c = Message {
                            typ: MessageType::EvtPlayingAlarm,
                            payload: Payload::Alarm(Some(Alarm{
                                day: current_weekday,
                                hour: current_hour,
                                min: current_minute
                            })),
                        };

                        self.tx.send(c).unwrap();
                    }
                }
            }

            self.notify_next_alarm();
        }

        /// Handles message from ui_handlers
        fn handle_message(&mut self, msg: Message) {
            debug!("alarm_manager:{:?}", msg);
            match msg.typ {
                MessageType::CmdUpdateAlarms => {
                    self.update_alarms(msg.payload);
                    self.notify_next_alarm();
                }
                _ => debug!("alarm_manager::Unknown command"),
            }
        }

        fn notify_next_alarm(&self) {
            let now = Local::now();
            let current_weekday: Weekday = now.weekday();
            let current_hour: usize = now.hour() as usize;
            let current_minute: usize = now.minute() as usize;

            let c = Message {
                typ: MessageType::EvtNextAlarm,
                payload: Payload::Alarm(find_next_alarm(
                        &self.alarms,
                        current_weekday,
                        current_hour,
                        current_minute,
                )),
            };

            self.tx.send(c).unwrap();
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
        fn test_next_1() {
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

            assert_eq!(
                find_next_alarm(&alarms, Weekday::Fri, 17, 58),
                Some(Alarm {
                    day: Weekday::Fri,
                    hour: 18,
                    min: 0
                })
            );
            assert_eq!(
                find_next_alarm(&alarms, Weekday::Sat, 18, 0),
                Some(Alarm {
                    day: Weekday::Sun,
                    hour: 19,
                    min: 30
                })
            );
            assert_eq!(
                find_next_alarm(&alarms, Weekday::Sun, 18, 0),
                Some(Alarm {
                    day: Weekday::Sun,
                    hour: 19,
                    min: 30
                })
            );
            assert_eq!(
                find_next_alarm(&alarms, Weekday::Sun, 19, 0),
                Some(Alarm {
                    day: Weekday::Sun,
                    hour: 19,
                    min: 30
                })
            );
            assert_eq!(
                find_next_alarm(&alarms, Weekday::Sun, 19, 20),
                Some(Alarm {
                    day: Weekday::Sun,
                    hour: 19,
                    min: 30
                })
            );
            assert_eq!(
                find_next_alarm(&alarms, Weekday::Sun, 19, 31),
                Some(Alarm {
                    day: Weekday::Sun,
                    hour: 20,
                    min: 0
                })
            );
            assert_eq!(
                find_next_alarm(&alarms, Weekday::Sun, 20, 31),
                Some(Alarm {
                    day: Weekday::Fri,
                    hour: 17,
                    min: 30
                })
            );
        }

        #[test]
        fn test_next_2() {
            setup_logger();
            let rule1 = Rule {
                serial: 1,
                days: vec!["Sat".to_string()],
                interval: 2,
                from: 12,
                to: 13,
            };

            let rules = vec![rule1];
            let alarms = get_alarms(&rules);

            let mins: Vec<usize> = (2..=58).step_by(2).collect();
            let expected = hashmap! {
                Weekday::Sat => hashmap! {
                    12 => mins,
                    13 => vec![0],
                },
            };

            assert_eq!(alarms, expected);

            assert_eq!(
                find_next_alarm(&alarms, Weekday::Sat, 13, 12),
                Some(Alarm {
                    day: Weekday::Sat,
                    hour: 12,
                    min: 2
                })
            );
        }
    }
}

pub use alarm_manager::*;
