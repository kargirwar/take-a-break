mod player {
    use crate::utils::*;
    //use log::debug;
    use rodio::{source::Source, Decoder, OutputStream};
    use std::io::Cursor;
    use std::thread;

    const BEEP_INTERVAL: u64 = 1000; //milliseconds
    const PLAY_DURATION: u64 = 1000;

    #[cfg(feature = "debug")]
    const MAX_TIMES: u64 = 1;

    #[cfg(not(feature = "debug"))]
    const MAX_TIMES: u64 = 5;

    pub fn play() {
        match is_locked() {
            LockedState::Locked => return,
            _ => (),
        }

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mp3_data = include_bytes!("beep.mp3");
        let cursor = Cursor::new(mp3_data);

        let source = Decoder::new(cursor).unwrap();
        let buffered = source.buffered();

        for _ in 0..MAX_TIMES {
            let sh = stream_handle.clone();
            let src = buffered.clone();

            thread::spawn(move || {
                sh.play_raw(src.convert_samples()).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(PLAY_DURATION));
            });

            std::thread::sleep(std::time::Duration::from_millis(BEEP_INTERVAL));
        }
    }
}

pub use player::*;
