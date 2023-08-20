mod alarm {
    use tokio::sync::broadcast::Sender as BcastSender;
    use tokio::sync::broadcast::Receiver as BcastReceiver;
    use crate::ui_handler::alarm_manager::Rule;
    use crate::Command;
    use crate::CommandName;
    use tokio_util::sync::CancellationToken;
    use tokio::select;
    use chrono::Datelike;
    use chrono::NaiveDate;
    use chrono::Weekday;
    use chrono::Duration;
    use chrono::Days;
    use chrono::Timelike;
    use std::fmt;
    use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};


    #[derive(Clone, Debug)]
    pub struct AlarmTime {
        pub day: String,
        pub hours: usize,
        pub minutes: usize
    }

	pub struct Alarm {
		t: AlarmTime,
		tx: BcastSender<Command>,
		rx: BcastReceiver<Command>,
        token: CancellationToken
    }

    impl Alarm {
        pub fn new(t: AlarmTime, tx: BcastSender<Command>, rx: BcastReceiver<Command>) -> Self {

            let s = tx.clone();
            let token = CancellationToken::new();
            let cloned_token = token.clone();
            let alarm_time = t.clone();

            tokio::spawn(async move {

                //if we are before the alarm time sleep till that 
                //alarm time.
                //if we are after the alarm time sleep until the alarm time
                //next week. 
                //
                //In both cases the loop will work on a weekly basis
                let seconds = seconds_till_first_alarm(&alarm_time);
                debug!("{:?}:Sleeping initially for {}", alarm_time, seconds);

                select! {
                    _ = cloned_token.cancelled() => {
                        debug!("{:?}:Cancelled initial", alarm_time);
                        return;
                    }
                    _ = tokio::time::sleep(std::time::Duration::from_secs(seconds.try_into().unwrap())) => {
                        s.send(Command{name: CommandName::PlayAlarm, rules: None}).unwrap();
                    }
                }

                let week_seconds = 7 * 24 * 60 * 60;

                //weekly alarm
                loop {
                    select! {
                        _ = cloned_token.cancelled() => {
                            debug!("{:?}:Cancelled weekly", alarm_time);
                            break;
                        }
                        _ = tokio::time::sleep(std::time::Duration::from_secs(week_seconds.try_into().unwrap())) => {
                            s.send(Command{name: CommandName::PlayAlarm, rules: None}).unwrap();
                        }
                    }
                }
            });

            Self {t, tx, rx, token}
        }

        pub fn run(mut self) {
            tokio::spawn(async move {
                loop {
                    match self.rx.recv().await {
                        Ok(i) => {
                            if i.name == CommandName::Shutdown {
                                self.token.cancel();
                                break;
                            }
                        },
                        Err(e) => debug!("{}", e)
                    };
                }
            });
        }
    }

    fn seconds_till_first_alarm(a: &AlarmTime) -> i64 {
        let now = chrono::offset::Local::now();
        let today = now.weekday();

        //today's timestamp with the alarm time
        let target = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).
            unwrap().
            and_hms_opt(a.hours.try_into().unwrap(), a.minutes.try_into().unwrap(), 0).
            unwrap();

        if today == get_weekday(&a.day).unwrap() {
            let d = target - now.naive_local();
            if d > Duration::zero() {
                //we are still before alarm time today
                return d.num_seconds()
            }

            return seconds_till_next_week_alarm(a);
        } else {
            return seconds_till_next_week_alarm(a);
        }
    }

    fn seconds_till_next_week_alarm(a: &AlarmTime) -> i64 {
        let now = chrono::offset::Local::now();
        let today = now.weekday() as i32;
        let sunday = Weekday::Sun as i32;
        let alarm_day = get_weekday(&a.day).unwrap() as i32;

        let mut days_to_advance = /*days upto sunday*/ (sunday - today) + alarm_day;
        debug!("{:?}:days_to_advance : {}", a, days_to_advance);
        days_to_advance = days_to_advance % 7;
        days_to_advance += 1;

        //today's timestamp with the alarm time
        let mut target = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).
            unwrap().
            and_hms_opt(a.hours.try_into().unwrap(), a.minutes.try_into().unwrap(), 0).
            unwrap();

        //advance to appropriate day
        target = target + Days::new(days_to_advance.try_into().unwrap());
        target = target.with_hour(a.hours.try_into().unwrap()).unwrap();
        target = target.with_minute(a.minutes.try_into().unwrap()).unwrap();

        debug!("{:?}:target: {}", a, target);

        //calculate seconds till the next week alaram time from now
        let d = target - now.naive_local();
        return d.num_seconds();
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

        #[test]
        fn test() {
            //TODO: How to test??
            let a = AlarmTime{day: "Sun".to_string(), hours: 0, minutes: 0};
            assert_eq!(seconds_till_first_alarm(&a), 500);
        }
    }   
}

pub use alarm::*;
