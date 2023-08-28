// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod player;
mod ui_handler;
mod utils;

use crate::ui_handler::*;
use log::debug;
use tauri::{AboutMetadata, Menu, MenuItem};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

#[tokio::main]
async fn main() {
    utils::setup_logger();
    debug!("Starting up!");
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel(10);

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    //authors: None,
    //comments: None,
    //copyright: None,
    //license: None,
    //website: None,
    //website_label: None,
    //};
    let mut meta_data = AboutMetadata::default();
    meta_data.version = Some("0.1".to_string());

    let menu = Menu::new()
        .add_native_item(MenuItem::Quit)
        .add_native_item(MenuItem::About("Take a break!".to_string(), meta_data));

    let app = tauri::Builder::default()
        .menu(menu)
        .manage(tx.clone())
        .invoke_handler(tauri::generate_handler![command])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    let handle = app.handle();
    let ui = UiHandler::new(rx, handle);
    ui.run();

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
