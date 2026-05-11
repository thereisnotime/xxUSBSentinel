# xxUSBSentinel Context

## Metadata
- Domain: xxUSBSentinel — USB kill-switch for Linux and Windows
- Primary audience: LLM agents working on this codebase, human contributors
- Last updated: 2026-05-12
- Status: Active
- Stability: sections marked `[STABLE]` change rarely; `[VOLATILE]` change often

---

## 0. Context Maintenance Protocol [STABLE]

This file is the primary working context for the project.

- LLM agents should treat it as a living document and update it whenever meaningful behaviour changes.
- If code and this file diverge, update this file immediately so future work stays reliable.
- Temporary or branch-specific behaviour should be noted here with a cleanup marker.

### Quick update checklist
- Refresh `Last updated` date
- Review config fields if any were added/removed
- Validate Critical Invariants still hold
- Update module responsibilities if new files were added

---

## 1. Summary [STABLE]

> USB kill-switch — maps any USB device as a key that triggers immediate forced shutdown when removed.

A single-binary desktop application (egui GUI) for Linux and Windows. The user maps any USB device (mouse, keyboard, flash drive, anything identifiable by VID:PID) as a "key". While armed, if the key device is removed, the machine shuts down immediately. Designed to make encrypted drive key recovery as hard as possible if the machine is physically seized.

**Secondary features:** system tray, desktop notifications, event log, per-device comments, script hooks on USB events, optional fake crash-screen overlay before shutdown, optional swap/pagefile wipe before shutdown, test mode (popup instead of real shutdown), shutdown-on-close.

---

## 2. Architecture [STABLE]

Single Rust crate (`xxusbsentinel`). No workspace. All modules are in `src/`.

```
main.rs        — entry point: loads config, wires threads, launches eframe
app.rs         — egui app (SentinelApp): all GUI rendering and user interaction
config.rs      — Config + Hook structs, TOML load/save, autostart management
sentinel.rs    — SharedState (Arc<Mutex>), GuiEvent enum, LogEntry, UsbDevice
usb.rs         — USB monitor background thread (rusb), hotplug detection, sends GuiEvents
shutdown.rs    — can_shutdown(), notify(), execute()
wipe.rs        — wipe_swap(), wipe_hiberfil()
tray.rs        — system tray icon + menu (Linux: ksni, Windows: tray-icon)
```

### Thread model

```
main thread (eframe/egui)
  └─ SentinelApp::update() called every frame
       reads: mpsc::Receiver<GuiEvent>
       reads/writes: Arc<Mutex<SharedState>>

USB monitor thread (std::thread::spawn in usb::start_monitor)
  └─ rusb hotplug loop
       writes: Arc<Mutex<SharedState>> (key_device, waiting flag)
       sends: mpsc::Sender<GuiEvent>

tray thread (platform-specific)
  └─ reads/writes: Arc<Mutex<SharedState>> (armed, test_mode, shutdown_on_close)
```

### Event flow (trigger path)

```
1. USB monitor detects key device removed while armed
2. Sends GuiEvent::ShutdownTriggered { wipe_swap, wipe_hiberfil, fake_bsod, bsod_style }
3. GUI thread receives it in update(), sets self.bsod_active = true, resets self.bsod_frames = 0
4. Each subsequent frame: if bsod_active && not preview → bsod_frames += 1
5. After 3 frames (overlay is visible on screen): run wipe_swap/wipe_hiberfil if enabled, then shutdown::execute()
```

The 3-frame delay exists so the BSOD overlay is actually rendered before the process is killed. Do not reduce it — 1-2 frames is not reliable across all compositors and GPU drivers.

---

## 3. Key Data Structures [STABLE]

### Config (`config.rs`)

All fields have serde defaults. The TOML file is written on every GUI action that changes a field. Fields that also exist in SharedState must be kept in sync — the GUI writes both.

| Field | Type | Default | Notes |
|-------|------|---------|-------|
| `key_device` | String | `""` | VID:PID e.g. `"046D:C52B"` |
| `test_mode` | bool | false | popup instead of real shutdown |
| `shutdown_on_close` | bool | false | close window = trigger while armed |
| `autostart` | bool | false | launch on login |
| `device_comments` | HashMap<String,String> | {} | keyed by VID:PID |
| `hooks` | Vec<Hook> | [] | script hooks on USB events |
| `wipe_swap` | bool | false | swapoff -a / ClearPageFileAtShutdown before shutdown |
| `wipe_hiberfil` | bool | false | mask hibernate / powercfg /h off before shutdown |
| `fake_bsod` | bool | false | show overlay before shutdown |
| `bsod_style` | String | platform | `"win10"`, `"win11"`, `"linux"`, `"blank"` |

### Hook (`config.rs`)

| Field | Type | Default | Notes |
|-------|------|---------|-------|
| `device` | String | `"*"` | VID:PID to match, or `"*"` |
| `event` | String | `"connected"` | `"connected"`, `"disconnected"`, `"triggered"` |
| `script` | String | `""` | absolute path to script/binary |
| `enabled` | bool | true | |

Scripts receive 3 positional args: `VID:PID device-name event-type`.

### SharedState (`sentinel.rs`)

The bridge between the USB monitor thread and the GUI thread. Wrapped in `Arc<Mutex<SharedState>>`. Fields mirror a subset of Config plus runtime state. Kept in sync with Config by the GUI thread — when the user changes a setting, both `cfg.field` and `state.lock().field` are updated.

| Field | Notes |
|-------|-------|
| `armed` | set by GUI arm/disarm button and tray |
| `test_mode` | mirrors cfg.test_mode |
| `waiting` | true while Map Device is listening for the next disconnect |
| `key_device` | mirrors cfg.key_device |
| `shutdown_on_close` | mirrors cfg.shutdown_on_close |
| `wipe_swap` | mirrors cfg.wipe_swap |
| `wipe_hiberfil` | mirrors cfg.wipe_hiberfil |
| `fake_bsod` | mirrors cfg.fake_bsod |
| `bsod_style` | mirrors cfg.bsod_style |

---

## 4. GUI Layout [VOLATILE]

### Main window (780×560 default)

```
┌─ header ──────────────────────────────────────────────────────┐
│  Logo  [ARMED]/[DISARMED]  key_device  version                │
├─ toolbar ─────────────────────────────────────────────────────┤
│  Map Device  [Arm/Disarm]  □ Test mode  Advanced  Report/Help │
├─ left panel (device list) ────┬─ right panel (event log) ─────┤
│  VID:PID  Name  Comment  Btn  │  [Copy All] [Clear] [Export]  │
│  ...                          │  timestamped events           │
└───────────────────────────────┴───────────────────────────────┘
```

Right-click on VID:PID or Name column opens context menu: Copy VID:PID, Copy VID, Copy PID, Copy device info, Copy last event, Resolve online.

### Advanced Settings (separate window)

- General: Shutdown on close, Autostart on login
- Actions on trigger: Disable swap/pagefile, Remove hibernation file, Show fake crash screen (with style dropdown and Preview button)
- Script hooks: per-rule table with device/event/script/enabled fields, Add rule / delete button

### System tray menu

- Show xxUSBSentinel
- Arm / Disarm
- Test mode (toggle)
- Shutdown on close (toggle)
- Exit

---

## 5. Critical Invariants [STABLE]

1. **Never call `shutdown::execute()` from the USB thread.** The USB thread sends `GuiEvent::ShutdownTriggered`. The GUI thread handles wipe + BSOD + shutdown. This is required because the BSOD overlay must be rendered before the process exits.

2. **`wipe_*` functions run before `shutdown::execute()`, never after.** The order in `app.rs` is: run hooks → wipe_swap → wipe_hiberfil → execute(). Reversing this would mean the wipes never complete.

3. **`bsod_frames >= 3` before shutdown fires.** Do not reduce this threshold — the overlay needs to reach the screen before the process is killed. On slow compositors one frame is not enough.

4. **Config::save() on every user action that mutates a field.** Settings must survive a crash or forced restart. Never batch saves.

5. **Both `cfg.field` and `state.lock().field` must be updated together** for any field that lives in both. The USB monitor thread reads from SharedState, not Config. If they diverge, armed behaviour and GUI state will be inconsistent.

6. **`bsod_panel()` is marked `#[allow(deprecated)]` intentionally.** `CentralPanel::show()` is deprecated in egui 0.34 but is the only valid API inside a viewport callback where only `&Context` is available (no `&mut Ui`). Do not remove the `allow` or switch to a different API without verifying it works in the viewport context.

7. **Test mode must never call `shutdown::execute()`.** In test mode, `GuiEvent::TestTriggered` is sent instead of `ShutdownTriggered`. The GUI shows a popup. The path must stay separate.

---

## 6. Platform-Specific Notes [STABLE]

### Linux
- Tray: `ksni` with `async-io` + `blocking` features (avoids tokio/zbus conflict with accesskit_unix).
- Autostart: writes `~/.config/autostart/xxusbsentinel.desktop`.
- Shutdown: `systemctl poweroff -i`, fallback `shutdown -h now`.
- Wipe swap: `swapoff -a`. Wipe hiberfil: write `0` to `/sys/power/image_size`, mask `hibernate.target`.
- `can_shutdown()`: queries logind via `busctl CanPowerOff`; falls back to uid == 0 check.

### Windows
- Tray: `tray-icon` crate.
- Autostart: `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
- Shutdown: `shutdown /p /f`, fallback `shutdown /s /t 0 /f`.
- Wipe swap: sets `ClearPageFileAtShutdown` registry DWORD. Wipe hiberfil: `powercfg /h off`.
- `can_shutdown()`: always returns `true`.

---

## 7. Test Strategy [STABLE]

Tests live in `#[cfg(test)] mod tests` blocks inside the source files they test. No separate `tests/` directory.

**What is tested:**
- `config.rs` — Config/Hook serde roundtrip, defaults, save/load, device_comments roundtrip (7 tests)
- `usb.rs` — `vid_pid_str` formatting (2 tests)
- `sentinel.rs` — SharedState field mirroring, LogEntry/UsbDevice construction (3 tests)
- `shutdown.rs` — `can_shutdown()` no-panic, `notify()` no-panic (2 tests)

**What is not tested (and why):**
- `app.rs` — 1200+ lines of egui GUI code; requires a real display or headless renderer. Not worth the cost for the current scope.
- `tray.rs` — platform tray bindings; requires a desktop session.
- `wipe.rs` — all calls have OS-level side effects (swapoff, powercfg) that cannot be run safely in tests.
- `shutdown::execute()` — would shut down the machine.

Coverage ceiling is ~10-15% due to the GUI-heavy architecture. This is expected and acceptable.

---

## 8. Release Process [STABLE]

```sh
just release-patch   # bumps Cargo.toml version, commits, tags vX.Y.Z, pushes
```

Pushing a `v*` tag triggers `.github/workflows/release.yml` which:
1. Builds Linux binary + AppImage (linuxdeploy + gtk plugin)
2. Builds Windows binary + Chocolatey nupkg (SHA256 injected at build time)
3. Signs all artifacts with cosign (keyless, GitHub OIDC)
4. Attests SLSA provenance via `actions/attest-build-provenance`
5. Generates `checksums.sha256`
6. Creates a GitHub release with all artifacts
7. Publishes to Chocolatey (if `CHOCOLATEY_API_KEY` secret is set)
8. Opens a WinGet PR (if `WINGET_TOKEN` secret is set)

---

## 9. File Tree [VOLATILE]

```
src/
  app.rs           egui SentinelApp — all rendering and event handling
  config.rs        Config + Hook structs, TOML I/O, autostart
  main.rs          entry point
  sentinel.rs      SharedState, GuiEvent, LogEntry, UsbDevice
  shutdown.rs      can_shutdown(), notify(), execute()
  tray.rs          system tray (ksni on Linux, tray-icon on Windows)
  usb.rs           USB monitor thread (rusb hotplug)
  wipe.rs          wipe_swap(), wipe_hiberfil()
resources/
  guard-on.png     app icon + tray icon (armed)
  guard-off.png    tray icon (disarmed)
  bsod-win10.png   Windows 10 crash screen overlay
  bsod-win11.png   Windows 11 crash screen overlay
  bsod-linux.png   Linux kernel panic overlay
packaging/
  appimage/        .desktop file for AppImage
  chocolatey/      nuspec + install/uninstall scripts
docs/
  configuration.md full config reference
  hooks.md         script hook reference
  advanced.md      autostart, systemd service, headless usage
.github/
  workflows/
    ci.yml         check + lint + coverage + build (Linux + Windows)
    release.yml    tag-triggered: build + sign + attest + publish
    codeql.yml     CodeQL SAST (Rust, build-mode: none)
    scorecard.yml  OpenSSF Scorecard weekly + on push
  dependabot.yml   weekly Cargo + Actions updates
```
