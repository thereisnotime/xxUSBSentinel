## ADDED Requirements

### Requirement: Key device mapping
The application MUST allow any USB device (identified by VID:PID) to be designated as a kill-switch key via the device list or the Map Device flow.

#### Scenario: Map device via list
- **WHEN** the user clicks "Set Key" next to a device in the device list
- **THEN** that device's VID:PID is stored as the key device in config and SharedState

#### Scenario: Map device via unplug detection
- **WHEN** the user activates Map Device mode and unplugs a USB device
- **THEN** the VID:PID of the unplugged device is recorded as the key device

### Requirement: Armed state and trigger
The application MUST monitor for key device removal while armed and execute the shutdown sequence immediately.

#### Scenario: Key removed while armed
- **WHEN** the key device is disconnected while the sentinel is armed
- **THEN** GuiEvent::ShutdownTriggered is sent, wipe steps run if enabled, and shutdown::execute() is called after the BSOD overlay renders for at least 3 frames

#### Scenario: Key removed while disarmed
- **WHEN** the key device is disconnected while the sentinel is disarmed
- **THEN** the event is logged and no shutdown is triggered

### Requirement: Test mode
The application MUST provide a test mode that shows a popup instead of executing a real shutdown.

#### Scenario: Trigger in test mode
- **WHEN** the key device is removed while armed with test mode enabled
- **THEN** GuiEvent::TestTriggered is sent and a popup is shown; shutdown::execute() is NOT called

### Requirement: Shutdown on close
The application MUST optionally trigger the kill-switch when the main window is closed while armed.

#### Scenario: Close while armed with shutdown-on-close enabled
- **WHEN** the user closes the main window while armed and shutdown_on_close is true
- **THEN** the shutdown sequence (or test popup) is triggered

#### Scenario: Close while armed without shutdown-on-close
- **WHEN** the user closes the main window while armed and shutdown_on_close is false
- **THEN** the window minimises to the system tray; no shutdown occurs
