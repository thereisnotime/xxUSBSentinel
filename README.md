# xxUSBSentinel

USB kill-switch for Linux and Windows. Map any USB device as a key — when it is removed while the sentinel is armed, the machine shuts down immediately. Designed to make recovering encrypted drive keys as hard as possible if someone physically seizes your machine.

> **Warning:** This tool does not encrypt your data. Use full-disk encryption (LUKS, VeraCrypt, BitLocker) alongside it.

---

## How it works

1. Plug in the USB device you want to use as your key (mouse, keyboard, flash drive, anything).
2. Click **Set Key** next to it in the device list, or use **Map Device** and physically unplug it to auto-detect.
3. Click **Arm Sentinel**.
4. If the key device is removed while armed, the machine shuts down immediately.

---

## Features

- Monitors all USB connect/disconnect events in real time
- Maps any USB device as the kill-switch key by VID:PID
- Immediate forced shutdown on key removal (Linux: systemctl poweroff; Windows: shutdown /s /t 0 /f)
- Test mode — fires a desktop notification instead of a real shutdown so you can verify your setup safely
- Shutdown on close — closing the window while armed triggers shutdown instead of minimising to tray
- Persistent config — key device, test mode, shutdown on close, and per-device comments survive restarts
- Per-device comments stored in config
- Event log with timestamps, colour-coded by event type, selectable text, copy and export
- System tray icon with arm/disarm, test mode, and shutdown on close toggles
- Desktop notifications on trigger (Linux: notify-send; Windows: WScript.Shell popup)
- Autostart on login (Linux: XDG autostart; Windows: registry Run key)
- Permissions check — warns if the current user cannot shut down the system

---

## Screenshots

![Default](Screenshots/Screenshot_1.jpg)
![Armed](Screenshots/Screenshot_2.jpg)

---

## Installation

### Pre-built binary

Download the latest release from the [Releases](https://github.com/thereisnotime/xxUSBSentinel/releases) page and run the binary. No installation needed.

### Build from source

**Requirements (Linux):**
```sh
sudo apt install libusb-1.0-0-dev libxdo-dev pkg-config
```

**Build:**
```sh
just release
# binary at target/release/xxusbsentinel
```

Or without just:
```sh
cargo build --release
```

**Install to `~/.local/bin`:**
```sh
just install
```

---

## Usage

```
just run          # debug build
just run-release  # release build
just install      # install to ~/.local/bin
```

---

## Development

```
just build        # debug build
just check        # type-check only
just clippy       # lint
just fmt          # format
just test         # run tests
just ci           # fmt-check + clippy + test
just clean        # remove build artefacts
just bump-patch   # bump x.y.Z
just bump-minor   # bump x.Y.0
just bump-major   # bump X.0.0
just dist-linux   # package as .tar.gz
```

---

## Compatibility

| Platform | Status |
|----------|--------|
| Linux (X11 / Wayland) | Supported |
| Windows 10/11 | Supported |

---

## License

[PolyForm Noncommercial License 1.0.0](LICENSE) — free for personal and non-commercial use. Commercial use is prohibited.
