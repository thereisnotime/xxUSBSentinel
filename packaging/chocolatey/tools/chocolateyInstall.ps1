$ErrorActionPreference = 'Stop'

$version  = $env:ChocolateyPackageVersion
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

$packageArgs = @{
  packageName   = $env:ChocolateyPackageName
  fileType      = 'exe'
  url64bit      = "https://github.com/thereisnotime/xxUSBSentinel/releases/download/v$version/xxusbsentinel-v$version-windows-x86_64.exe"
  checksum64    = 'REPLACE_CHECKSUM'
  checksumType64 = 'sha256'
  silentArgs    = ''
  validExitCodes = @(0)
}

Get-ChocolateyWebFile @packageArgs -file (Join-Path $toolsDir 'xxusbsentinel.exe')
Install-ChocolateyPath $toolsDir 'Machine'
