//! Miscellaneous utilities
mod utils {
    use plist::Value;
    use std::fmt;
    use std::io::Cursor;
    use std::process::Command;

    use dirs;
    use log::LevelFilter;
    use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
    use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
    use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
    use log4rs::append::rolling_file::RollingFileAppender;
    use log4rs::{
        config::{Appender, Config, Root},
        encode::pattern::PatternEncoder,
    };
    use std::path::PathBuf;

    pub enum LockedState {
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

    /// Checks if screen is locked across macOS, Linux, and Windows
    pub fn is_locked() -> LockedState {
        #[cfg(target_os = "macos")]
        {
            is_locked_macos()
        }

        #[cfg(target_os = "linux")]
        {
            is_locked_linux()
        }

        #[cfg(target_os = "windows")]
        {
            is_locked_windows()
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            LockedState::Unknown
        }
    }

    #[cfg(target_os = "macos")]
    fn is_locked_macos() -> LockedState {
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
                LockedState::Unknown
            }
            None => LockedState::Unlocked, // If key doesn't exist, screen is unlocked
        }
    }

    #[cfg(target_os = "linux")]
    fn is_locked_linux() -> LockedState {
        // Method 1: Try systemd-logind (most reliable, DE-agnostic)
        if let Ok(state) = check_loginctl() {
            return state;
        }

        // Method 2: Try D-Bus screensaver interface
        if let Ok(state) = check_dbus_screensaver() {
            return state;
        }

        // Method 3: Try GNOME-specific screensaver
        if let Ok(state) = check_gnome_screensaver() {
            return state;
        }

        // Method 4: Try Cinnamon-specific screensaver
        if let Ok(state) = check_cinnamon_screensaver() {
            return state;
        }

        // Method 5: Try KDE/Plasma screensaver
        if let Ok(state) = check_kde_screensaver() {
            return state;
        }

        LockedState::Unknown
    }

    #[cfg(target_os = "linux")]
    fn check_loginctl() -> Result<LockedState, ()> {
        let output = Command::new("loginctl")
            .args(&["show-session", "self", "-p", "LockedHint"])
            .output()
            .map_err(|_| ())?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("LockedHint=yes") {
                Ok(LockedState::Locked)
            } else if stdout.contains("LockedHint=no") {
                Ok(LockedState::Unlocked)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    #[cfg(target_os = "linux")]
    fn check_dbus_screensaver() -> Result<LockedState, ()> {
        // Try using busctl (part of systemd)
        let output = Command::new("busctl")
            .args(&[
                "call",
                "org.freedesktop.ScreenSaver",
                "/org/freedesktop/ScreenSaver",
                "org.freedesktop.ScreenSaver",
                "GetActive",
            ])
            .output()
            .map_err(|_| ())?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // busctl returns "b true" or "b false"
            if stdout.contains("true") {
                Ok(LockedState::Locked)
            } else if stdout.contains("false") {
                Ok(LockedState::Unlocked)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    #[cfg(target_os = "linux")]
    fn check_gnome_screensaver() -> Result<LockedState, ()> {
        let output = Command::new("gnome-screensaver-command")
            .arg("-q")
            .output()
            .map_err(|_| ())?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("is active") {
                Ok(LockedState::Locked)
            } else if stdout.contains("is inactive") {
                Ok(LockedState::Unlocked)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    #[cfg(target_os = "linux")]
    fn check_cinnamon_screensaver() -> Result<LockedState, ()> {
        let output = Command::new("cinnamon-screensaver-command")
            .arg("-q")
            .output()
            .map_err(|_| ())?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("is active") {
                Ok(LockedState::Locked)
            } else if stdout.contains("is inactive") {
                Ok(LockedState::Unlocked)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    #[cfg(target_os = "linux")]
    fn check_kde_screensaver() -> Result<LockedState, ()> {
        let output = Command::new("qdbus")
            .args(&[
                "org.freedesktop.ScreenSaver",
                "/ScreenSaver",
                "org.freedesktop.ScreenSaver.GetActive",
            ])
            .output()
            .map_err(|_| ())?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if stdout == "true" {
                Ok(LockedState::Locked)
            } else if stdout == "false" {
                Ok(LockedState::Unlocked)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    #[cfg(target_os = "windows")]
    fn is_locked_windows() -> LockedState {
        // Method 1: Try PowerShell approach (most reliable)
        if let Ok(state) = check_windows_powershell() {
            return state;
        }

        // Method 2: Try WMI query
        if let Ok(state) = check_windows_wmi() {
            return state;
        }

        LockedState::Unknown
    }

    #[cfg(target_os = "windows")]
    fn check_windows_powershell() -> Result<LockedState, ()> {
        let script = r#"
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class SessionInfo {
    [DllImport("user32.dll")]
    public static extern bool GetLastInputInfo(ref LASTINPUTINFO plii);
    [StructLayout(LayoutKind.Sequential)]
    public struct LASTINPUTINFO {
        public uint cbSize;
        public uint dwTime;
    }
}
"@
$sessionInfo = Get-Process -IncludeUserName | Where-Object {$_.Name -eq 'LogonUI'} | Select-Object -First 1
if ($sessionInfo) { Write-Output 'locked' } else { Write-Output 'unlocked' }
"#;

        let output = Command::new("powershell")
            .args(&["-NoProfile", "-Command", script])
            .output()
            .map_err(|_| ())?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
            if stdout.contains("locked") {
                Ok(LockedState::Locked)
            } else if stdout.contains("unlocked") {
                Ok(LockedState::Unlocked)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    #[cfg(target_os = "windows")]
    fn check_windows_wmi() -> Result<LockedState, ()> {
        // Check if LogonUI.exe is running (indicates lock screen)
        let output = Command::new("tasklist")
            .args(&["/FI", "IMAGENAME eq LogonUI.exe"])
            .output()
            .map_err(|_| ())?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("LogonUI.exe") {
                Ok(LockedState::Locked)
            } else {
                Ok(LockedState::Unlocked)
            }
        } else {
            Err(())
        }
    }

    // Logger settings. Sends all debug prints to <app_dir>/debug.log
    // Log file rotated after 1M.
    pub fn setup_logger() {
        //setup rotation
        let file_name = get_log_file_name();
        println!("file_name: {}", file_name);
        let window_size = 1; // log0, log1, log2
        let fixed_window_roller = FixedWindowRoller::builder()
            .build(&(file_name.clone() + "{}"), window_size)
            .unwrap();
        let size_limit = 1 * 1024 * 1024; //1mb
        let size_trigger = SizeTrigger::new(size_limit);
        let compound_policy =
            CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

        let logfile = RollingFileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S%.3f)} [{l}] - {m}{n}",
            )))
            .build(file_name, Box::new(compound_policy))
            .unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(
                Root::builder()
                    .appender("logfile")
                    .build(LevelFilter::Debug),
            )
            .unwrap();

        log4rs::init_config(config).unwrap();
    }

    pub fn get_log_file_name() -> String {
        let mut path = get_app_dir().expect("get_app_dir() returned None");

        std::fs::create_dir_all(&path).expect("failed to create app dir");

        path.push("debug.log");
        path.to_string_lossy().to_string()
    }

    pub fn get_settings_file_name() -> String {
        if let Some(mut path) = get_app_dir() {
            path.push("settings.json");
            return path.to_string_lossy().to_string();
        }

        return "".to_string();
    }

    fn get_app_dir() -> Option<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            if let Some(home_dir) = dirs::home_dir() {
                let mut path = PathBuf::new();
                path.push(home_dir);
                path.push("Library");
                
                #[cfg(feature = "debug")]
                path.push("com.68kilo.tab_debug");
                
                #[cfg(not(feature = "debug"))]
                path.push("com.68kilo.tab");
                
                return Some(path);
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(config_dir) = dirs::config_dir() {
                let mut path = PathBuf::new();
                path.push(config_dir);
                
                #[cfg(feature = "debug")]
                path.push("tab_debug");
                
                #[cfg(not(feature = "debug"))]
                path.push("tab");
                
                return Some(path);
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(data_dir) = dirs::data_local_dir() {
                let mut path = PathBuf::new();
                path.push(data_dir);
                
                #[cfg(feature = "debug")]
                path.push("68kilo\\tab_debug");
                
                #[cfg(not(feature = "debug"))]
                path.push("68kilo\\tab");
                
                return Some(path);
            }
        }

        None
    }
}

pub use utils::*;