// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use soloud::*;
use std::process::Command;
use std::io::{self, Cursor, Write};
use plist::Value;
use std::fmt;

enum LockedState {
    Locked,
    Unlocked,
    Unknown,
}

impl fmt::Debug for LockedState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LockedState::Locked => write!(f, "Locked"),
            LockedState::Unlocked => write!(f, "Unlocked"),
            LockedState::Unknown => write!(f, "Unknown"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("{:?}", is_locked());

    let mut sl = Soloud::default()?;
    sl.set_global_volume(3.0);

    let mut wav = audio::Wav::default();

    wav
        .load_mem(include_bytes!("jara-si-dil-alto.ogg"))
        .unwrap();

    sl.play(&wav);


    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
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
    format!("Hello, {}!", name)
}
