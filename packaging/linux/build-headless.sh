#!/bin/bash
# PortHannis Linux Headless 构建脚本
# 生成: porthannis-linux-headless.tar.gz
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
OUTPUT_DIR="${PROJECT_ROOT}/dist"

echo "=== PortHannis Linux Headless 构建 ==="

# 1. 构建前端
echo "[1/4] 构建前端..."
cd "$PROJECT_ROOT/frontend"
npm ci
npm run build

# 2. 构建 Release 二进制
echo "[2/4] 构建 release 二进制..."
cd "$PROJECT_ROOT"
cargo build --release -p porthannis-server
BIN="$PROJECT_ROOT/target/release/porthannis"

# 3. Strip + 可选 UPX
echo "[3/4] 优化二进制..."
strip "$BIN" 2>/dev/null || true
if command -v upx &> /dev/null; then
    upx --best "$BIN"
fi

# 4. 打包
echo "[4/4] 打包..."
mkdir -p "$OUTPUT_DIR"
cp "$BIN" "$OUTPUT_DIR/"
cp "$SCRIPT_DIR/../default-config.json" "$OUTPUT_DIR/config.json"

cd "$OUTPUT_DIR"
tar czf "porthannis-linux-headless.tar.gz" porthannis config.json
rm porthannis config.json

echo ""
echo "构建完成: $OUTPUT_DIR/porthannis-linux-headless.tar.gz"
echo "解压后运行: ./porthannis --help"
