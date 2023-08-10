// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
mod ui_handler;
mod player;

use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::ui_handler::*;
use crate::player::*;

#[tokio::main]
async fn main() {
    let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel(10);
    run(rx);

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(tx)
        .invoke_handler(tauri::generate_handler![greet, my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Greet called");
    play();
    format!("Hello, {}!", name)
}

#[tauri::command]
async fn my_custom_command(tx: tauri::State<'_, Sender<u8>>) -> Result<(), ()> {
    match tx.send(100).await {
        Ok(_) => return Ok(()),
        Err(_) => return Err(()),
    }
}
