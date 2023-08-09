// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use soloud::*;
use std::process::Command;
use std::io::{Cursor};
use plist::Value;
use std::fmt;
use std::thread;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
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
    let _handle = thread::spawn(|| {
        println!("{:?}", is_locked());
        let mut sl = Soloud::default().unwrap();
        sl.set_global_volume(3.0);

        let mut wav = audio::Wav::default();

        wav
            .load_mem(include_bytes!("beep.mp3"))
            .unwrap();

        sl.play(&wav);

        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    format!("Hello, {}!", name)
}
