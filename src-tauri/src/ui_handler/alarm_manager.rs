mod alarm_manager {
    //use crate::Rule;
    //use std::collections::HashMap;
    use tokio::sync::mpsc::{Receiver, Sender};

    pub struct AlarmManager {
        tx: Sender<String>,
        rx: Receiver<String>,
    }

    impl AlarmManager {
        pub fn new(tx: Sender<String>, rx: Receiver<String>) -> Self {
            Self {tx, rx}
        }

        pub async fn run(mut self) {
            println!("Running AlarmManager");
            tokio::spawn(async move {
                loop {
                    match self.rx.recv().await {
                        Some(i) => {
                            println!("AlarmManager: {}", i);
                            self.tx.send("Acknowledged".to_string()).await.unwrap();
                        },
                        None => print!("e"),
                    }
                }
            });
        }
    }

    //fn get_hours(s: usize, e: usize) -> Vec<usize> {
        //let mut hrs = Vec::new();
//
        //let mut current_hour = s;
        //while current_hour <= e {
            //hrs.push(current_hour);
            //current_hour += 1;
        //}
//
        //hrs
    //}
//
    //pub fn get_alarms(rules: &[Rule]) -> HashMap<String, HashMap<usize, Vec<usize>>> {
        //let mut alarms: HashMap<String, HashMap<usize, Vec<usize>>> = HashMap::new();
//
        //for r in rules {
            //let hours: HashMap<usize, Vec<usize>> = HashMap::new();
//
            //for d in &r.days {
                //alarms.insert(d.clone(), hours.clone());
//
                //let s = r.from;
                //let e = r.to;
                //let f = r.interval;
                //let hrs = get_hours(s, e);
//
                //let mut i = 0;
                //let mut m = f;
                //let mut h;
//
                //loop {
                    //let mut mins = Vec::new();
//
                    //loop {
                        //mins.push(m % 60);
                        //m += f;
//
                        //if m - (60 * i) >= 60 {
                            //h = hrs[i];
//
                            //if f == 60 && h == s {
                                //i += 1;
                                //break;
                            //}
//
                            //println!("{} h: {} mins: {:?}", d, h, mins);
//
                            //i += 1;
                            //break;
                        //}
                    //}
//
                    //if e == hrs[i] {
                        //if m % 60 == 0 {
                            //alarms.get_mut(d).unwrap().insert(e, vec![0]);
                            //println!("{} h: {} mins: {:?}", d, e, alarms[d].get(&e).unwrap());
                        //}
                        //break;
                    //}
                //}
            //}
        //}
//
        //alarms
    //}
}
//
pub use alarm_manager::*;
