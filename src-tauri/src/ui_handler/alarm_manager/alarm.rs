mod alarm {
    use crate::{AlarmTime, BcastReceiver, BcastSender, Message, MessageType, Payload};

    use chrono::Datelike;
    use chrono::Days;
    use chrono::Duration;
    use chrono::NaiveDate;
    use chrono::Timelike;
    use chrono::Weekday;
    use log::debug;
    use tokio::select;
    use tokio_util::sync::CancellationToken;

    pub struct Alarm {
        t: AlarmTime,
        tx: BcastSender<Message>,
        rx: BcastReceiver<Message>,
        token: CancellationToken,
        pub seconds: i64,
    }

    impl Alarm {
        pub fn new(t: AlarmTime, tx: BcastSender<Message>, rx: BcastReceiver<Message>) -> Self {
            let s = tx.clone();
            let token = CancellationToken::new();
            let cloned_token = token.clone();
            let alarm_time = t.clone();
            let seconds = seconds_till_first_alarm(&alarm_time);

            tokio::spawn(async move {
                //if we are before the alarm time sleep till that
                //alarm time.
                //if we are after the alarm time sleep until the alarm time
                //next week.
                //
                //In both cases the loop will work on a weekly basis
                debug!("{:?}:Sleeping initially for {}", alarm_time, seconds);

                let initial_duration = std::time::Duration::from_secs(seconds.try_into().unwrap());

                select! {
                    _ = cloned_token.cancelled() => {
                        debug!("{:?}:Cancelled initial", alarm_time);
                        return;
                    }
                    _ = tokio::time::sleep(initial_duration) => {
                        s.send(
                            Message{
                                typ: MessageType::CmdPlayAlarm,
                                payload: Payload::None
                            }).unwrap();
                    }
                }

                let week_seconds = 7 * 24 * 60 * 60;
                let weekly_duration =
                    std::time::Duration::from_secs(week_seconds.try_into().unwrap());
                //weekly alarm
                loop {
                    select! {
                        _ = cloned_token.cancelled() => {
                            debug!("{:?}:Cancelled weekly", alarm_time);
                            break;
                        }
                        _ = tokio::time::sleep(weekly_duration) => {
                            s.send(Message{
                                typ: MessageType::CmdPlayAlarm,
                                payload: Payload::None
                            }).unwrap();
                        }
                    }
                }
            });

            Self {
                t,
                tx,
                rx,
                token,
                seconds,
            }
        }

        pub fn run(mut self) {
            tokio::spawn(async move {
                loop {
                    match self.rx.recv().await {
                        Ok(i) => {
                            if i.typ == MessageType::CmdShutdown {
                                self.token.cancel();
                                break;
                            }
                        }
                        Err(e) => debug!("{}", e),
                    };
                }
            });
        }
    }

    fn seconds_till_first_alarm(a: &AlarmTime) -> i64 {
        let now = chrono::offset::Local::now();
        let today = now.weekday();

        //today's timestamp with the alarm time
        let target = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
            .unwrap()
            .and_hms_opt(
                a.hours.try_into().unwrap(),
                a.minutes.try_into().unwrap(),
                0,
            )
            .unwrap();

        if today == get_weekday(&a.day).unwrap() {
            let d = target - now.naive_local();
            if d > Duration::zero() {
                //we are still before alarm time today
                return d.num_seconds();
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
        days_to_advance = days_to_advance % 7;
        days_to_advance += 1;

        debug!("{:?}:days_to_advance : {}", a, days_to_advance);

        //today's timestamp with the alarm time
        let mut target = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
            .unwrap()
            .and_hms_opt(
                a.hours.try_into().unwrap(),
                a.minutes.try_into().unwrap(),
                0,
            )
            .unwrap();

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
        use crate::utils::*;

        #[test]
        fn test_seconds_to_next() {
            setup_logger();
            //TODO: How to test??
            let a = AlarmTime {
                day: "Mon".to_string(),
                hours: 9,
                minutes: 40,
            };
            let seconds = seconds_till_first_alarm(&a);
            println!("seconds: {:?}", seconds);
            assert!(seconds < 600);
        }
    }
}

pub use alarm::*;
