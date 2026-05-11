use std::process::Command;

/// Disable / clear swap.
///
/// Linux  — `swapoff -a`: flushes all swap back to RAM instantly, preventing
///           forensic recovery from the swap partition or file after shutdown.
/// Windows — sets the ClearPageFileAtShutdown registry key so the pagefile is
///           zeroed during the forced shutdown that follows immediately.
pub fn wipe_swap() {
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("swapoff").arg("-a").status();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("reg")
            .args([
                "add",
                r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "/v",
                "ClearPageFileAtShutdown",
                "/t",
                "REG_DWORD",
                "/d",
                "1",
                "/f",
            ])
            .status();
    }
}

/// Remove / zero the hibernation image.
///
/// Linux  — sets `/sys/power/image_size` to 0 (tells the kernel to write an
///           empty hibernation image) and masks the hibernate target so it
///           cannot be re-enabled before the shutdown completes.
/// Windows — runs `powercfg /h off` which deletes hiberfil.sys on the spot.
pub fn wipe_hiberfil() {
    #[cfg(target_os = "linux")]
    {
        let _ = std::fs::write("/sys/power/image_size", "0");
        let _ = Command::new("systemctl")
            .args(["mask", "--runtime", "hibernate.target"])
            .spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("powercfg").args(["/h", "off"]).status();
    }
}
