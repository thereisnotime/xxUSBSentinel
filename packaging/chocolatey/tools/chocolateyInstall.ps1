$ErrorActionPreference = 'Stop'

$version  = $env:ChocolateyPackageVersion
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

$packageArgs = @{
  packageName    = $env:ChocolateyPackageName
  unzipLocation  = $toolsDir
  url64bit       = "https://github.com/thereisnotime/xxUSBSentinel/releases/download/v$version/xxusbsentinel-v$version-windows-x86_64.zip"
  checksum64     = 'REPLACE_CHECKSUM'
  checksumType64 = 'sha256'
}

Install-ChocolateyZipPackage @packageArgs

# Add the extracted binary directory to PATH for this session
$binDir = Join-Path $toolsDir "xxusbsentinel-v$version-windows-x86_64"
Install-ChocolateyPath $binDir 'Machine'
