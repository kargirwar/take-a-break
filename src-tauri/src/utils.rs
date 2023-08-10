mod utils {
    use plist::Value;
    use std::fmt;
    use std::io::Cursor;
    use std::process::Command;

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
}

pub use utils::*;
