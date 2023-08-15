// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
mod ui_handler;

use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::ui_handler::*;

#[tokio::main]
async fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel(10);

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let app = tauri::Builder::default()
        .manage(tx)
        .invoke_handler(tauri::generate_handler![command])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");
    
    let handle = app.handle();
    run(rx, handle);

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { .. } => {
            //api.prevent_exit();
        }
        _ => {}
    });
}

#[tauri::command]
async fn command(payload: &str, tx: tauri::State<'_, Sender<String>>) -> Result<(), ()> {
    match tx.send(String::from(payload)).await {
        Ok(_) => return Ok(()),
        Err(_) => return Err(()),
    }
}
