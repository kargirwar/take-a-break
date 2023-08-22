mod utils {
    use plist::Value;
    use std::fmt;
    use std::io::Cursor;
    use std::process::Command;

    use dirs;
    use log::LevelFilter;
    use log4rs::{
        append::{
            console::{ConsoleAppender, Target},
            file::FileAppender,
        },
        config::{Appender, Config, Root},
        encode::pattern::PatternEncoder,
        filter::threshold::ThresholdFilter,
    };
    use std::fs;
    use std::path::PathBuf;

    pub enum LockedState {
        Locked,
        Unlocked,
        Unknown,
    }

    pub fn is_locked() -> LockedState {
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
            }
            Err(_) => return LockedState::Unknown,
        }

        let v: Value;
        if let Ok(result) = Value::from_reader(Cursor::new(output)) {
            v = result;
        } else {
            return LockedState::Unknown;
        }

        let is_locked = v
            .as_dictionary()
            .and_then(|dict| dict.get("IOConsoleLocked"));

        match is_locked {
            Some(l) => {
                if let Some(b) = l.as_boolean() {
                    return if b {
                        LockedState::Locked
                    } else {
                        LockedState::Unlocked
                    };
                }
                return LockedState::Unknown;
            }
            None => LockedState::Unknown,
        }
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

    pub fn setup_logger() {
        let level = log::LevelFilter::Info;

        let file_path = get_log_file_name();
        if file_path.is_empty() {
            return;
        }
        // Build a stderr logger.
        let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

        // Logging to log file.
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S%.3f)} [{l}] - {m}{n}",
            )))
            .build(file_path)
            .unwrap();

        // Log Trace level output to file where trace is the default level
        // and the programmatically specified level to stderr.
        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .build("stderr", Box::new(stderr)),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    .appender("stderr")
                    .build(LevelFilter::Debug),
            )
            .unwrap();

        // Use this to change log levels at runtime.
        // This means you can change the default log level to trace
        // if you are trying to debug an issue and need more logs on then turn it off
        // once you are done.
        let _handle = log4rs::init_config(config).unwrap();
    }

    fn dir_exists(path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    fn get_log_file_name() -> String {
        if let Some(mut path) = get_app_dir() {
            path.push("debug.log");
            return path.to_string_lossy().to_string();
        }

        return "".to_string();
    }

    pub fn get_settings_file_name() -> String {
        if let Some(mut path) = get_app_dir() {
            path.push("settings.json");
            return path.to_string_lossy().to_string();
        }

        return "".to_string();
    }

    fn get_app_dir() -> Option<PathBuf> {
        if let Some(home_dir) = dirs::home_dir() {
            let mut path = PathBuf::new();
            path.push(home_dir);
            path.push("Library");
            path.push("TauriTimer");
            return Some(path);
        }

        return None;
    }
}

pub use utils::*;
