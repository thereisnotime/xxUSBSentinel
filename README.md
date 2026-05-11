# xxUSBSentinel

<!-- CI / Release -->
[![CI](https://github.com/thereisnotime/xxUSBSentinel/actions/workflows/ci.yml/badge.svg)](https://github.com/thereisnotime/xxUSBSentinel/actions/workflows/ci.yml)
[![Release](https://github.com/thereisnotime/xxUSBSentinel/actions/workflows/release.yml/badge.svg)](https://github.com/thereisnotime/xxUSBSentinel/actions/workflows/release.yml)
[![Latest release](https://img.shields.io/github/v/release/thereisnotime/xxUSBSentinel)](https://github.com/thereisnotime/xxUSBSentinel/releases/latest)
[![License](https://img.shields.io/badge/license-PolyForm%20Noncommercial-blue)](LICENSE)

<!-- Quality / Security -->
[![codecov](https://codecov.io/gh/thereisnotime/xxUSBSentinel/branch/master/graph/badge.svg)](https://codecov.io/gh/thereisnotime/xxUSBSentinel)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/thereisnotime/xxUSBSentinel/badge)](https://scorecard.dev/viewer/?uri=github.com/thereisnotime/xxUSBSentinel)
[![deps.rs](https://deps.rs/repo/github/thereisnotime/xxUSBSentinel/status.svg)](https://deps.rs/repo/github/thereisnotime/xxUSBSentinel)

<!-- Stack -->
![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=flat&logo=linux&logoColor=black)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=flat&logo=windows&logoColor=white)


USB kill-switch for Linux and Windows. Map any USB device as a key. When it is removed while the sentinel is armed, the machine shuts down immediately. Designed to make recovering encrypted drive keys as hard as possible if someone physically seizes your machine.

> ⚠️ **Warning:** This tool does not encrypt your data. Use full-disk encryption (LUKS, VeraCrypt, BitLocker) alongside it.

---

## How it works

1. Plug in the USB device you want to use as your key (mouse, keyboard, flash drive, anything).
2. Click **Set Key** next to it in the device list, or use **Map Device** and physically unplug it to auto-detect.
3. Click **Arm Sentinel**.
4. If the key device is removed while armed, the machine shuts down immediately.

---

## Features

- Monitor and log all USB connect and disconnect events in real time
- Map any USB device as the kill-switch key by VID:PID
- Immediate forced shutdown on key removal
- Test mode for safe dry-runs without triggering a real shutdown
- Shutdown on close option
- Desktop notifications on trigger
- Per-device comments
- Event log with timestamps, copy and export
- System tray icon with arm/disarm and toggle controls
- Autostart on login
- Persistent config across restarts
- Permissions warning if the current user cannot shut down

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

[PolyForm Noncommercial License 1.0.0](LICENSE) - free for personal and non-commercial use. Commercial use is prohibited.
