// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
mod ui_handler;
mod player;

use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::ui_handler::*;
use crate::player::*;
use tauri::Manager;

//commands from UI
const RULES_UPDATED: &str = "rules-updated";

#[derive(Debug)]
pub struct Command {
    name: String,
    payload: String
}

#[tokio::main]
async fn main() {
    let (tx, rx): (Sender<Command>, Receiver<Command>) = mpsc::channel(10);

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
async fn command(name: &str, payload: &str, tx: tauri::State<'_, Sender<Command>>) -> Result<(), ()> {
    match name {
        RULES_UPDATED => {
            match tx.send(Command{name: String::from(name), payload: String::from(payload)}).await {
                Ok(_) => return Ok(()),
                Err(_) => return Err(()),
            }
        },

        &_ => Ok(())
    }
}
