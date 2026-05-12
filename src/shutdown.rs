use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

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
        // Use the WinRT toast API so the notification appears in the Action
        // Center rather than as a blocking WScript modal dialog.
        let script = format!(
            "[void][Windows.UI.Notifications.ToastNotificationManager,\
Windows.UI.Notifications,ContentType=WindowsRuntime]; \
[void][Windows.Data.Xml.Dom.XmlDocument,Windows.Data.Xml.Dom.XmlDocument,ContentType=WindowsRuntime]; \
$x=[Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent(\
[Windows.UI.Notifications.ToastTemplateType]::ToastText02); \
$x.GetElementsByTagName('text').Item(0).AppendChild($x.CreateTextNode('{}')) | Out-Null; \
$x.GetElementsByTagName('text').Item(1).AppendChild($x.CreateTextNode('{}')) | Out-Null; \
[Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('xxUSBSentinel')\
.Show([Windows.UI.Notifications.ToastNotification]::new($x))",
            summary.replace('\'', ""),
            body.replace('\'', "")
        );
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_shutdown_returns_without_panic() {
        // Just verify it doesn't panic; result depends on the environment.
        let _ = can_shutdown();
    }

    #[test]
    fn notify_does_not_panic() {
        // Spawns notify-send / PowerShell which will fail silently outside a desktop session.
        notify("test summary", "test body");
    }
}

pub fn execute() {
    #[cfg(target_os = "windows")]
    {
        if Command::new("shutdown")
            .args(["/p", "/f"])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .is_err()
        {
            let _ = Command::new("shutdown")
                .args(["/s", "/t", "0", "/f"])
                .creation_flags(CREATE_NO_WINDOW)
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
