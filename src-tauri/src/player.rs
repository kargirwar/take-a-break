mod player {
    use crate::utils::*;
    //use log::debug;
    use soloud::*;
    use std::thread;

    const BEEP_INTERVAL: u64 = 1000; //milliseconds
    const POLL_INTERVAL: u64 = 100;

    pub fn play() {
        match is_locked() {
            LockedState::Locked => return,
            _ => (),
        }

        for _ in 0..10 {
            thread::spawn(|| {
                let mut sl = Soloud::default().unwrap();
                sl.set_global_volume(3.0);

                let mut wav = audio::Wav::default();

                wav.load_mem(include_bytes!("beep.mp3")).unwrap();

                sl.play(&wav);

                while sl.voice_count() > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(POLL_INTERVAL));
                }
            });

            std::thread::sleep(std::time::Duration::from_millis(BEEP_INTERVAL));
        }
    }
}

pub use player::*;
