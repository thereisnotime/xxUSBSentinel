# Configuration

xxUSBSentinel stores its config in a TOML file:

| Platform | Path |
|----------|------|
| Linux    | `~/.config/xxusbsentinel/config.toml` |
| Windows  | `%APPDATA%\xxusbsentinel\config.toml` |

The file is created automatically on first run. All fields are optional and default to safe values.

---

## Full config reference

```toml
# VID:PID of the mapped key device (e.g. "046D:C52B").
# Set via the GUI — editing this directly also works.
key_device = ""

# Start in test mode (shows a popup instead of real shutdown).
test_mode = false

# Treat closing the main window as a trigger while armed.
shutdown_on_close = false

# Launch automatically on login.
autostart = false

# Trigger wipe_swap before shutdown.
wipe_swap = false

# Trigger wipe_hiberfil before shutdown.
wipe_hiberfil = false

# Show a fake crash screen overlay when the trigger fires.
fake_bsod = false

# Which crash screen to show: "win10", "win11", "linux", "blank"
# Defaults to "linux" on Linux, "win10" on Windows.
bsod_style = "linux"

# Per-device notes shown in the device list.
[device_comments]
"046D:C52B" = "Logitech receiver"

# Script hooks — see docs/hooks.md
[[hooks]]
device  = "*"
event   = "connected"
script  = "/path/to/script"
enabled = true
```

---

## Field details

### `key_device`
The VID:PID string of the device that arms as the kill-switch key. Format is `XXXX:XXXX` (uppercase hex, zero-padded to 4 digits). You can find a device's VID:PID in the device list — it's in the first column.

### `wipe_swap` (Linux / Windows)
On trigger:
- **Linux** — runs `swapoff -a` to flush all swap back to RAM before the machine powers off, making forensic recovery from the swap partition significantly harder.
- **Windows** — sets `ClearPageFileAtShutdown` in the registry so Windows zeros the pagefile during the forced shutdown that follows.

### `wipe_hiberfil` (Linux / Windows)
On trigger:
- **Linux** — writes `0` to `/sys/power/image_size` and masks `hibernate.target` so the kernel cannot write a new hibernation image.
- **Windows** — runs `powercfg /h off` which deletes `hiberfil.sys` immediately.

### `fake_bsod`
Displays a full-screen crash overlay before the shutdown executes. The overlay persists until the OS powers off — it does not time out. Useful to discourage an attacker from noticing that the machine is shutting down intentionally.

### `bsod_style`
Controls which overlay is shown when `fake_bsod = true`:

| Value | Description |
|-------|-------------|
| `"win10"` | Windows 10 BSOD |
| `"win11"` | Windows 11 BSOD |
| `"linux"` | Linux kernel panic |
| `"blank"` | Plain black screen |
