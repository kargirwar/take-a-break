mod player {
    use soloud::*;
    use std::thread;
    use crate::utils::*;

    pub fn play() {
        thread::spawn(|| {
            println!("{:?}", is_locked());
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
