# Roadmap

Items are roughly prioritised. Nothing here is a commitment.

---

## Planned

### ARM builds
Add `aarch64-unknown-linux-gnu` and `armv7-unknown-linux-gnueabihf` targets to the release workflow via cross-compilation. ARM Windows (`aarch64-pc-windows-msvc`) when GitHub Actions supports it natively.

### CLI mode
Headless operation without the GUI — run as a background daemon or one-shot from the terminal. Useful for servers and environments without a display. Config via the same TOML file used by the GUI.

### Improve GUI
- Device list sorting and filtering
- Dark / light theme toggle
- Per-device notes inline editing
- Better tray menu with armed-state indicator
- Keyboard shortcuts for arm/disarm

### PPA (Ubuntu/Debian package)
Publish to a Launchpad PPA so users can install and update via `apt`. Triggered from the same release workflow.

---

## On request

### macOS support
macOS is architecturally similar to Linux for this use case (IOKit for USB events, `shutdown` for forced power-off). Will be added if at least one person opens an issue requesting it. Open one if you need it.

---

## Ideas / Under consideration

- Windows service mode (no tray, runs as a service)
- Multiple key devices (any-of or all-of trigger)
- Panic key: keyboard shortcut to trigger shutdown even without USB removal
- Audit log: signed, append-only log of arm/disarm/trigger events
- Remote arming via local network API (LAN-only, no cloud)
