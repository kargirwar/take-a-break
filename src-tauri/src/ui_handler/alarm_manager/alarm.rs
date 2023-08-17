mod alarm {
    use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    use tokio::sync::broadcast::Sender as BcastSender;
    use tokio::sync::broadcast::Receiver as BcastReceiver;
    use crate::ui_handler::alarm_manager::Rule;
    use crate::Command;
    use crate::CommandName;

    pub struct AlarmTime {
        pub day: String,
        pub hours: usize,
        pub minutes: usize
    }

    pub struct Alarm {
        t: AlarmTime,
        tx: BcastSender<Command>,
        rx: BcastReceiver<Command>,
        shutdown: Arc<AtomicBool>,
    }

    impl Alarm {
        pub fn new(t: AlarmTime, tx: BcastSender<Command>, rx: BcastReceiver<Command>) -> Self {
            let shutdown = Arc::new(AtomicBool::new(false));

            let s = tx.clone();
            let shutdown_flag = shutdown.clone();

            tokio::spawn(async move {
                loop {
                    if shutdown_flag.load(Ordering::Relaxed) {
                        println!("alarm: stopping alarm thread");
                        break; // Exit the loop if shutdown flag is set
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    s.send(Command{name: CommandName::PlayAlarm, rules: None}).unwrap();
                }
            });


            Self {t, tx, rx, shutdown: Arc::clone(&shutdown),}
        }

        pub fn run(mut self) {
            println!("alarm:Running alarm");

            let shutdown_flag = self.shutdown.clone();
            tokio::spawn(async move {
                loop {
                    if shutdown_flag.load(Ordering::Relaxed) {
                        println!("alarm: stopping command thread");
                        break; // Exit the loop if shutdown flag is set
                    }

                    match self.rx.recv().await {
                        Ok(i) => self.handle_command(i),
                        Err(e) => println!("{}", e)
                    };
                }
            });
        }

        fn handle_command(&self, cmd: Command) {
            println!("alarm: handle_command: {:?}", cmd);
            match cmd.name {
                CommandName::Shutdown => self.shutdown.store(true, Ordering::Relaxed),
                CommandName::PlayAlarm => println!("Playing alarm"),
                _ => ()
            }
        }
    }
}

pub use alarm::*;
