$ErrorActionPreference = 'Stop'

# Stop the process if running
Stop-Process -Name "xxusbsentinel" -Force -ErrorAction SilentlyContinue

# Remove autostart registry entry if present
$regPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
Remove-ItemProperty -Path $regPath -Name "xxUSBSentinel" -ErrorAction SilentlyContinue

Write-Host "xxUSBSentinel has been uninstalled."
