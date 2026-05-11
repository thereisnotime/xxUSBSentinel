# Script Hooks

Hooks let you run an arbitrary script or binary when a USB event fires. They are configured in `config.toml` as a list of `[[hooks]]` entries.

---

## Hook fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `device` | string | `"*"` | VID:PID to match, or `"*"` for any device |
| `event` | string | `"connected"` | `"connected"`, `"disconnected"`, or `"triggered"` |
| `script` | string | — | Absolute path to the script or binary |
| `enabled` | bool | `true` | Set to `false` to disable without deleting the entry |

---

## Events

| Event | When it fires |
|-------|---------------|
| `connected` | A USB device is plugged in |
| `disconnected` | A USB device is removed |
| `triggered` | The kill-switch fires (key device removed while armed) |

The `device` filter is matched against the VID:PID string (e.g. `"046D:C52B"`). Use `"*"` to match any device.

For the `triggered` event, the `device` field is ignored — the trigger always fires for the mapped key device.

---

## Examples

### Lock the screen when any device is disconnected

```toml
[[hooks]]
device  = "*"
event   = "disconnected"
script  = "/usr/bin/loginctl"
enabled = true
```

> The script is executed with no arguments. Wrap complex commands in a shell script.

### Run a custom script when the specific key device connects

```toml
[[hooks]]
device  = "046D:C52B"
event   = "connected"
script  = "/home/user/.local/bin/notify-key-present.sh"
enabled = true
```

### Send a notification when the kill-switch triggers

```toml
[[hooks]]
device  = "*"
event   = "triggered"
script  = "/home/user/.local/bin/alert.sh"
enabled = true
```

---

## Script environment

Scripts are executed directly (no shell wrapper). The script must be executable (`chmod +x`). No environment variables are injected — if you need the VID:PID or other context, write a wrapper script that reads them from the config file or the system.

Scripts run asynchronously and their output is discarded. Failures are silently ignored — the shutdown sequence does not wait for hook scripts to complete.

---

## Adding hooks

Hooks can only be added by editing the config file directly — there is no GUI editor yet (planned). The app reloads the config on next start.

```toml
[[hooks]]
device  = "*"
event   = "connected"
script  = "/path/to/your/script"
enabled = true

[[hooks]]
device  = "046D:C52B"
event   = "disconnected"
script  = "/path/to/another/script"
enabled = false
```
