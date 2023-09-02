mod alarm_utils {
    use std::collections::HashMap;
    use log::debug;
    use chrono::Weekday;
    use crate::Rule;

    /// Given a set of alarms and current hour and minutes, determine next scheduled alarm
    pub fn find_next_alarm(
        alarms: &HashMap<Weekday, HashMap<usize, Vec<usize>>>,
        current_day: Weekday,
        current_hour: usize,
        current_minute: usize,
        ) -> Option<(Weekday, usize, usize)> {
        let mut current_day_to_check;
        let mut next_alarm: Option<(Weekday, usize, usize)> = None;

        // Start with current_day, check if alarms are scheduled after current time.
        // Repeat for successive days until we wrap around to current_day
        if let Some(hour_map) = alarms.get(&current_day) {
            if let Some(hour_mins) = find_next_for_today(hour_map, current_hour, current_minute) {
                next_alarm = Some((current_day, hour_mins.0, hour_mins.1));
            }
        }

        if next_alarm.is_some() {
            //we found our next alarm on the same day
            return next_alarm;
        }

        //try searching other days
        current_day_to_check = current_day.succ();
        loop {
            if let Some(hour_map) = alarms.get(&current_day_to_check) {
                let mut sorted_keys: Vec<usize> = hour_map.keys().cloned().collect();
                sorted_keys.sort();

                for hour in &sorted_keys {
                    if let Some(mins) = hour_map.get(hour) {
                        for min in mins.iter() {
                            //mins are sorted. just pick up the first
                            return Some((current_day_to_check, *hour, *min));
                        }
                    }
                }
            }

            current_day_to_check = current_day_to_check.succ();

            if current_day_to_check == current_day {
                //wrapped around
                break;
            }
        }

        return next_alarm;
    }

    fn find_next_for_today(
        hour_map: &HashMap<usize, Vec<usize>>,
        current_hour: usize,
        current_minute: usize,
        ) -> Option<(usize, usize)> {
        let mut next_alarm: Option<(usize, usize)> = None;
        //hashmaps are not sorted. So first get sorted hours
        let mut sorted_keys: Vec<usize> = hour_map.keys().cloned().collect();
        sorted_keys.sort();

        'hour_loop: for hour in &sorted_keys {
            if let Some(mins) = hour_map.get(hour) {
                for min in mins.iter() {
                    if *hour > current_hour {
                        //mins are sorted. just pick up the first
                        next_alarm = Some((*hour, *min));
                        break 'hour_loop;
                    } else if *hour == current_hour {
                        if *min > current_minute {
                            next_alarm = Some((*hour, *min));
                            break 'hour_loop;
                        }
                    }
                }
            }
        }

        return next_alarm;
    }

    /// For a set of rules, finds the hours and minutes for each day at which
    /// alarm should be played. Assumes that rules are not overlapping
    pub fn get_alarms(rules: &[Rule]) -> HashMap<Weekday, HashMap<usize, Vec<usize>>> {
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

    fn get_hours(s: usize, e: usize) -> Vec<usize> {
        let mut hrs = Vec::new();

        let mut current_hour = s;
        while current_hour <= e {
            hrs.push(current_hour);
            current_hour += 1;
        }

        hrs
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

}
pub use alarm_utils::*;
