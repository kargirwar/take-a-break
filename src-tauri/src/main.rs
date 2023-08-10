// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use soloud::*;
use std::process::Command;
use std::io::{Cursor};
use plist::Value;
use std::fmt;
use std::thread;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::mpsc;

enum LockedState {
    Locked,
    Unlocked,
    Unknown,
}

struct MyState(String);
struct MyString(String);

//struct Comm(Sender<&str>, Receiver<&str>);

impl fmt::Debug for LockedState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LockedState::Locked => write!(f, "Locked"),
            LockedState::Unlocked => write!(f, "Unlocked"),
            LockedState::Unknown => write!(f, "Unknown"),
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx): (Sender<u8>, Receiver<u8>) = mpsc::channel(10);

    tokio::spawn(async move  {
        loop {
            match rx.recv().await {
                Some(i) => println!("{:?}", i),
                None => println!("None"),
            }
        }
    });

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(MyState("some state value".into()))
        .manage(MyString("Hello, managed state!".to_string()))
        .manage(tx)
        .invoke_handler(tauri::generate_handler![greet, my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn is_locked() -> LockedState {
    let buf = Command::new("ioreg")
        .args(&["-n", "Root", "-d1", "-a"])
        .output();

    let output;

    match buf {
        Ok(buf) => {
            if buf.status.success() {
                output = buf.stdout;
            } else {
                return LockedState::Unknown;
            }
        },
        Err(_) => return LockedState::Unknown,
    }

    let v: Value;
    if let Ok(result) = Value::from_reader(Cursor::new(output)) {
        v = result;
    } else {
        return LockedState::Unknown;
    }

    let is_locked = v.as_dictionary()
        .and_then(|dict| dict.get("IOConsoleLocked"));

    match is_locked {
        Some(l) => {
            if let Some(b) = l.as_boolean() {
                return if b { LockedState::Locked } else { LockedState::Unlocked };
            }
            return LockedState::Unknown;
        },
        None => LockedState::Unknown,
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Greet called");
    //let _handle = thread::spawn(|| {
        //println!("{:?}", is_locked());
        //let mut sl = Soloud::default().unwrap();
        //sl.set_global_volume(3.0);
//
        //let mut wav = audio::Wav::default();
//
        //wav
            //.load_mem(include_bytes!("beep.mp3"))
            //.unwrap();
//
        //sl.play(&wav);
//
        //while sl.voice_count() > 0 {
            //std::thread::sleep(std::time::Duration::from_millis(100));
        //}
    //});

    format!("Hello, {}!", name)
}

#[tauri::command]
async fn my_custom_command(tx: tauri::State<'_, Sender<u8>>) -> Result<(), ()> {
    tx.send(100).await;
    Ok(())
}
