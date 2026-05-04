# PortHannis Windows 构建脚本
# 生成: porthannis.zip (包含 porthannis.exe + config.json)
param(
    [string]$OutputDir = "dist"
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)

Write-Host "=== PortHannis Windows 构建 ==="

# 1. 构建前端
Write-Host "[1/4] 构建前端..."
Push-Location "$projectRoot/frontend"
npm ci
npm run build
Pop-Location

# 2. 构建 Release 二进制
Write-Host "[2/4] 构建 release 二进制..."
Push-Location "$projectRoot"
cargo build --release -p porthannis-server
Pop-Location

# 3. 可选: UPX 压缩
$exe = "$projectRoot/target/release/porthannis.exe"
if (Get-Command upx -ErrorAction SilentlyContinue) {
    Write-Host "[3/4] UPX 压缩..."
    upx --best $exe
} else {
    Write-Host "[3/4] 跳过 UPX (未安装)"
}

# 4. 打包
Write-Host "[4/4] 打包..."
$distDir = "$projectRoot/$OutputDir"
New-Item -ItemType Directory -Force -Path $distDir | Out-Null
Copy-Item $exe "$distDir/"
Copy-Item "$PSScriptRoot/../default-config.json" "$distDir/config.json"

$zipPath = "$projectRoot/$OutputDir/porthannis-windows.zip"
Compress-Archive -Path "$distDir/porthannis.exe", "$distDir/config.json" -DestinationPath $zipPath -Force

Write-Host ""
Write-Host "构建完成: $zipPath"
Write-Host "解压后运行: .\porthannis.exe --help"
