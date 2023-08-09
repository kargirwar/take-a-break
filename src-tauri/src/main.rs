// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use soloud::*;

//fn main() {
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sl = Soloud::default()?;
    sl.set_global_volume(3.0);

    let mut wav = audio::Wav::default();

    wav
        .load_mem(include_bytes!("jara-si-dil-alto.ogg"))
        .unwrap();

    //let bytes = include_bytes!("jara-si-dil-alto.ogg");
//
    //unsafe {
        //wav.load_raw_wav_8(bytes)?;
    //}
    //wav.load(&std::path::Path::new("./jara-si-dil-alto.ogg"))?;
    //wav.load(&std::path::Path::new("/Users/pankaj/tauri/soloud-rs"))?;
    //wav.load("jara-si-dil.ogg")?;

    sl.play(&wav);


    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
