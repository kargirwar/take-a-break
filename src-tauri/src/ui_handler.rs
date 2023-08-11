mod ui_handler {
    use tokio::sync::mpsc::{Receiver};
    use tauri::AppHandle;
    use tauri::Wry;
    use tauri::Manager;

    pub fn run(mut rx: Receiver<u8>, handle: AppHandle<Wry>) {
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(i) => {println!("{:?}", i); handle.emit_all("some_event", "message").unwrap();},
                    None => println!("None"),
                }
            }
        });
    }
}

pub use ui_handler::*;
