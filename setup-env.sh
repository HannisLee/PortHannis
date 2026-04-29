#!/bin/bash
# PortHannis 开发环境配置脚本
# 使用方法: source setup-env.sh

export GOROOT=/d/Code/Go/go
export GOPATH=/d/Code/Go/go-packages
export WAILS_CACHE_DIR=/d/Code/Wails/cache

export PATH="$GOROOT/bin:$GOPATH/bin:$PATH"

echo "PortHannis 开发环境已加载:"
echo "  GOROOT=$GOROOT"
echo "  GOPATH=$GOPATH"
echo "  Go版本: $(go version)"
echo "  Wails版本: $(wails version)"
