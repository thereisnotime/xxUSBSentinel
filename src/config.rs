use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub key_device:        String,
    #[serde(default)]
    pub test_mode:         bool,
    #[serde(default)]
    pub shutdown_on_close: bool,
    #[serde(default)]
    pub autostart:         bool,
    /// User notes keyed by VID:PID.
    #[serde(default)]
    pub device_comments:   HashMap<String, String>,
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let path = config_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(text) = toml::to_string_pretty(self) {
            let _ = std::fs::write(&path, text);
        }
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("xxusbsentinel")
        .join("config.toml")
}

// ── Autostart ────────────────────────────────────────────────────────────────

pub fn autostart_enabled() -> bool {
    #[cfg(target_os = "linux")]
    { desktop_file_path().exists() }
    #[cfg(target_os = "windows")]
    { windows_autostart_get() }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    { false }
}

pub fn set_autostart(enable: bool) {
    #[cfg(target_os = "linux")]
    {
        let path = desktop_file_path();
        if enable {
            if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
            if let Ok(exe) = std::env::current_exe() {
                let content = format!(
                    "[Desktop Entry]\nType=Application\nName=xxUSBSentinel\nExec={}\nNoDisplay=true\n",
                    exe.display()
                );
                let _ = std::fs::write(&path, content);
            }
        } else {
            let _ = std::fs::remove_file(&path);
        }
    }
    #[cfg(target_os = "windows")]
    { windows_autostart_set(enable); }
}

#[cfg(target_os = "linux")]
fn desktop_file_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("autostart")
        .join("xxusbsentinel.desktop")
}

#[cfg(target_os = "windows")]
fn windows_autostart_get() -> bool {
    use winreg::{RegKey, enums::HKEY_CURRENT_USER};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu.open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run")
        .and_then(|k| k.get_value::<String, _>("xxUSBSentinel"))
        .is_ok()
}

#[cfg(target_os = "windows")]
fn windows_autostart_set(enable: bool) {
    use winreg::{RegKey, enums::{HKEY_CURRENT_USER, KEY_WRITE}};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let Ok(key) = hkcu.open_subkey_with_flags(
        r"Software\Microsoft\Windows\CurrentVersion\Run",
        KEY_WRITE,
    ) else { return };
    if enable {
        if let Ok(exe) = std::env::current_exe() {
            let _ = key.set_value("xxUSBSentinel", &exe.to_string_lossy().to_string());
        }
    } else {
        let _ = key.delete_value("xxUSBSentinel");
    }
}
