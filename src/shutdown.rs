use std::process::Command;

/// Returns true if the current user has permission to power off the system.
pub fn can_shutdown() -> bool {
    #[cfg(target_os = "linux")]
    {
        // Query logind via busctl (present on all systemd systems)
        if let Ok(out) = Command::new("busctl")
            .args([
                "call",
                "org.freedesktop.login1",
                "/org/freedesktop/login1",
                "org.freedesktop.login1.Manager",
                "CanPowerOff",
            ])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            if s.contains("\"yes\"") {
                return true;
            }
            if s.contains("\"auth") || s.contains("\"challenge") || s.contains("\"no\"") {
                return false;
            }
        }
        // busctl not available — fall back to uid check (root can always shutdown)
        Command::new("id")
            .arg("-u")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false)
    }
    #[cfg(target_os = "windows")]
    {
        true
    }
}

/// Fire a desktop notification. Non-blocking — failure is silently ignored.
pub fn notify(summary: &str, body: &str) {
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("notify-send")
            .args([
                "--urgency=critical",
                "--app-name=xxUSBSentinel",
                summary,
                body,
            ])
            .spawn();
    }
    #[cfg(target_os = "windows")]
    {
        // PowerShell toast via BurntToast or the built-in WScript shell balloon
        let script = format!(
            "$n=New-Object -ComObject WScript.Shell; \
             $n.Popup('{} {}',3,'xxUSBSentinel',48) | Out-Null",
            summary.replace('\'', ""),
            body.replace('\'', "")
        );
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .spawn();
    }
}

pub fn execute() {
    #[cfg(target_os = "windows")]
    {
        // /p = power off immediately (no dialog, no delay).
        // Fall back to /s /t 0 /f if /p is unavailable (pre-Win8).
        if Command::new("shutdown").args(["/p", "/f"]).spawn().is_err() {
            let _ = Command::new("shutdown")
                .args(["/s", "/t", "0", "/f"])
                .spawn();
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Try systemd first, fall back to sysvinit shutdown
        if Command::new("systemctl")
            .args(["poweroff", "-i"])
            .spawn()
            .is_err()
        {
            let _ = Command::new("shutdown").args(["-h", "now"]).spawn();
        }
    }
}
