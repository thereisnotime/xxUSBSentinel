# xxUSBSentinel

### Description ###
Windows anti-forensics USB monitoring tool. First you map a Key USB device - it can be a mouse, keyboard, flash drive etc. It will ask you to plug and unplug your device. Then the program remembers the device ID (VID:PID). After that you can arm the Sentinel and it would listen for device disconnect events and if your Key USB device is unplugged - xxUSBSentinel will shutdown your computer the fast way. The whole purpose of that is to make recovering your encrypted drive keys almost impossible.

* WARNING: This software will not encrypt or protect your data/drives, its only aim is to help you improve your operation security.
 

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

