use std::process::Command;

pub fn execute() {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("shutdown")
            .args(["/s", "/t", "0", "/f"])
            .spawn();
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
