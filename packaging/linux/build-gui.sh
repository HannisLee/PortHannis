#!/bin/bash
# PortHannis Linux GUI 构建脚本
# 生成: .deb + .AppImage
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"

echo "=== PortHannis Linux GUI 构建 ==="

# 1. 安装 Tauri CLI (如需要)
if ! command -v cargo-tauri &> /dev/null; then
    echo "[0/4] 安装 cargo-tauri..."
    cargo install tauri-cli
fi

# 2. 构建前端
echo "[1/4] 构建前端..."
cd "$PROJECT_ROOT/frontend"
npm ci
npm run build

# 3. Tauri 构建
echo "[2/4] Tauri 构建..."
cd "$PROJECT_ROOT/gui"
cargo tauri build

echo ""
echo "构建完成! 产物位于: $PROJECT_ROOT/gui/target/release/bundle/"
