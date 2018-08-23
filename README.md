# xxUSBSentinel

### Description ###
Windows anti-forensics USB monitoring tool. First you map a Key USB device - it can be a mouse, keyboard, flash drive etc. It will ask you to plug and unplug your device. Then the program remembers the device ID (VID:PID). After that you can arm the Sentinel and it would listen for device disconnect events and if your Key USB device is unplugged - xxUSBSentinel will shutdown your computer the fast way. The whole purpose of that is to make recovering your encrypted drive keys almost impossible.

* WARNING: This software will not encrypt or protect your data/drives, its only aim is to help you improve your operation security.
 
### Features ###
- Monitor and log all types of USB devices connecting and disconnecting
- Export logs to file
- Resolve device VID:PID 
- Map a device to be a Key USB device
- Fast shutdown on Key USB device unplug
- Control everything from GUI or from the traybar icon
- Test mode option for practice

### Screenshots ###
![Screenshot1](/Screenshots/Screenshot_1.jpg?raw=true "Default mode.")
![Screenshot2](/Screenshots/Screenshot_2.jpg?raw=true "Armed and ready.")
![Screenshot3](/Screenshots/Screenshot_3.jpg?raw=true "Resolving a device.")
![Screenshot4](/Screenshots/Screenshot_4.jpg?raw=true "Tray icon functionality.")

### Installation ###
No installation is needed, download release or build it yourself.

### Dependencies ###
Currently it depends on LibUsbDotNet, but in future releases the library will be packed with the executable:
```sh
LibUsbDotNet.dll
````

### Compatability ###
The should work on most Windows versions and has been tested on the following:
```sh
Windows 10 Home x64
``` 

### Todo ###
Add RAM, hiberfil, pagefile and swapfile secure wiping.
Add fake BSOD.
Add configuration saving.
Add option for custom commands instead of shutdown.
Pack libraries with release executable.

### Uninstall ###
Delete the executable.

