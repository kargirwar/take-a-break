mod player {
    use crate::utils::*;
    use soloud::*;
    use std::thread;
    use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};

    pub fn play() {
        thread::spawn(|| {
            debug!("{:?}", is_locked());
            match is_locked() {
                LockedState::Locked => return,
                _ => ()
            }

            let mut sl = Soloud::default().unwrap();
            sl.set_global_volume(3.0);

            let mut wav = audio::Wav::default();

            wav.load_mem(include_bytes!("beep.mp3")).unwrap();

            sl.play(&wav);

            while sl.voice_count() > 0 {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });
    }
}

pub use player::*;
