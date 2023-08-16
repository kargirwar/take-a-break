module alarm {
    use tokio::sync::broadcast::{Receiver, Sender};
    struct Alarm {
        tx: Sender<String>,
        rx: Receiver<String>,
    }

    impl Alarm {
        pub fn new(tx: Sender<String>, rx: Receiver<String>) -> Self {
            Self {tx, rx}
        }

        pub async fn run(mut self) {
            println!("alarm:Running AlarmManager");
            tokio::spawn(async move {
                loop {
                    match self.rx.recv().await {
                        Ok(i) => self.handle_command(i),
                        Err(e) => println!("{}", e)
                    };
                }
            });
        }

        fn handle_command(&self, cmd: String) {
            println!("alarm: {:?}", cmd);
        }
    }
}

pub use alarm::*;
