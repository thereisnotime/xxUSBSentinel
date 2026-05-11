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

> ⚠️ **Warning:** This tool does not encrypt your data. Use full-disk encryption (LUKS, VeraCrypt, ~~BitLocker~~) alongside it.

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

| Main window (armed) | System tray |
|---|---|
| ![Armed with context menu](Screenshots/Screenshot_1.jpg) | ![Tray menu](Screenshots/Screenshot_2.jpg) |

![Advanced settings](Screenshots/Screenshot_3.jpg)

---

## Installation

### Linux — build from source

```sh
# Install system dependencies (Debian/Ubuntu)
sudo apt install libusb-1.0-0-dev libxdo-dev pkg-config \
  libgtk-3-dev libxkbcommon-dev libgles2-mesa-dev \
  libwayland-dev libxrandr-dev libxi-dev libxcursor-dev

# Clone and install
git clone git@github.com:thereisnotime/xxUSBSentinel.git
cd xxUSBSentinel
just install   # builds release and copies to ~/.local/bin
```

### Linux — download latest binary

```sh
curl -fsSL https://github.com/thereisnotime/xxUSBSentinel/releases/latest/download/xxusbsentinel-$(curl -fsSL https://api.github.com/repos/thereisnotime/xxUSBSentinel/releases/latest | grep -o '"tag_name":"[^"]*"' | cut -d'"' -f4)-linux-x86_64.tar.gz | tar -xz --strip-components=1 -C ~/.local/bin xxusbsentinel-*/xxusbsentinel
```

Or grab the tarball manually from the [Releases](https://github.com/thereisnotime/xxUSBSentinel/releases/latest) page.

### Windows — download binary

Download `xxusbsentinel-vX.Y.Z-windows-x86_64.zip` from the [Releases](https://github.com/thereisnotime/xxUSBSentinel/releases/latest) page, extract, and run `xxusbsentinel.exe`.

**PowerShell one-liner:**
```powershell
$tag = (Invoke-RestMethod https://api.github.com/repos/thereisnotime/xxUSBSentinel/releases/latest).tag_name
Invoke-WebRequest "https://github.com/thereisnotime/xxUSBSentinel/releases/download/$tag/xxusbsentinel-$tag-windows-x86_64.zip" -OutFile xxusbsentinel.zip
Expand-Archive xxusbsentinel.zip -DestinationPath .
```

**Package managers** — Chocolatey and WinGet support coming soon.

---

## Documentation

| | |
|---|---|
| [Configuration](docs/configuration.md) | Config file reference — all fields and defaults |
| [Hooks](docs/hooks.md) | Run scripts on USB events |
| [Advanced](docs/advanced.md) | Autostart, systemd service, headless usage |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, common tasks, and PR guidelines.

## Roadmap

See [ROADMAP.md](ROADMAP.md) for planned features and ideas.

---

## Compatibility

| Platform | Status |
|----------|--------|
| Linux (X11 / Wayland) | Supported |
| Windows 10/11 | Supported |

---

## License

[PolyForm Noncommercial License 1.0.0](LICENSE) - free for personal and non-commercial use. Commercial use is prohibited.
