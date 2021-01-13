#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'

if ($v) {
  $Version = "v${v}"
}
if ($args.Length -eq 1) {
  $Version = $args.Get(0)
}

$ByteInstall = $env:BYTE_INSTALL
$BinDir = if ($ByteInstall) {
  "$ByteInstall\bin"
} else {
  "$Home\.byte\bin"
}

$ByteZip = "$BinDir\byte.zip"
$ByteExe = "$BinDir\byte.exe"

# GitHub requires TLS 1.2
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$ByteUri = if (!$Version) {
  "https://github.com/denoland/deno/releases/latest/download/deno-${Target}.zip"
} else {
  "https://github.com/denoland/deno/releases/download/${Version}/deno-${Target}.zip"
}

if (!(Test-Path $BinDir)) {
  New-Item $BinDir -ItemType Directory | Out-Null
}

Invoke-WebRequest $ByteUri -OutFile $ByteZip -UseBasicParsing

if (Get-Command Expand-Archive -ErrorAction SilentlyContinue) {
  Expand-Archive $ByteZip -Destination $BinDir -Force
} else {
  if (Test-Path $ByteExe) {
    Remove-Item $ByteExe
  }
  Add-Type -AssemblyName System.IO.Compression.FileSystem
  [IO.Compression.ZipFile]::ExtractToDirectory($ByteZip, $BinDir)
}

Remove-Item $ByteZip

$User = [EnvironmentVariableTarget]::User
$Path = [Environment]::GetEnvironmentVariable('Path', $User)
if (!(";$Path;".ToLower() -like "*;$BinDir;*".ToLower())) {
  [Environment]::SetEnvironmentVariable('Path', "$Path;$BinDir", $User)
  $Env:Path += ";$BinDir"
}

Write-Output "Byte was installed successfully to $ByteExe"
Write-Output "Run 'byte help' to get started"