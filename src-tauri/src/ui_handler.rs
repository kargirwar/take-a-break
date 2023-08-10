mod ui_handler {
    use tokio::sync::mpsc::{Receiver};
    pub fn run(mut rx: Receiver<u8>) {
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(i) => println!("{:?}", i),
                    None => println!("None"),
                }
            }
        });
    }
}

pub use ui_handler::*;
