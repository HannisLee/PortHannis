# PortHannis — 端口转发管理器

轻量级跨平台端口转发管理工具，支持 GUI（Windows/Linux）和 Headless WebUI 两种部署模式。

## 特性

- **TCP 端口转发** — 高性能异步 TCP 代理，支持双向数据转发
- **JSON 配置管理** — 单文件配置，人工可编辑，原子写入
- **日志轮转** — 每条目独立日志，5MB 上限，自动循环删除最旧记录
- **REST API** — 完整的 CRUD + 启停控制 API
- **GUI 桌面应用** — 基于 Tauri v2，Windows/Linux 原生体验
- **WebUI** — Headless 模式下通过浏览器管理，适合服务器部署
- **单文件部署** — Windows 仅需一个 exe + 一个 json 配置文件
- **体积小巧** — 二进制文件 < 5 MB（含嵌入式前端）

## 快速开始

### Windows

1. 下载 `porthannis-windows.zip`
2. 解压到任意目录
3. 双击运行 `porthannis.exe` 或命令行启动：
   ```powershell
   .\porthannis.exe -b 0.0.0.0 -P 25879
   ```
4. 浏览器打开 `http://localhost:25879`

### Linux Headless

```bash
tar xzf porthannis-linux-headless.tar.gz
./porthannis -b 0.0.0.0 -P 25879
# 浏览器打开 http://<server-ip>:25879
```

### Linux GUI

```bash
# .deb 安装
sudo dpkg -i porthannis-gui_*.deb

# 或 .AppImage
chmod +x porthannis-gui_*.AppImage
./porthannis-gui_*.AppImage
```

## 命令行参数

| 参数 | 环境变量 | 默认值 | 说明 |
|------|----------|--------|------|
| `-c, --config` | `PORTHANNIS_CONFIG` | 平台数据目录 | JSON 配置文件路径 |
| `-b, --bind` | `PORTHANNIS_BIND` | `127.0.0.1` | API 绑定地址 |
| `-P, --port` | `PORTHANNIS_PORT` | `25879` | API 绑定端口 |

## API 文档

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/api/health` | 健康检查 |
| `GET` | `/api/entries` | 列出所有转发条目 |
| `POST` | `/api/entries` | 创建新条目 |
| `GET` | `/api/entries/{id}` | 获取单个条目 |
| `PUT` | `/api/entries/{id}` | 更新条目 |
| `DELETE` | `/api/entries/{id}` | 删除条目 |
| `POST` | `/api/entries/{id}/start` | 启动转发 |
| `POST` | `/api/entries/{id}/stop` | 停止转发 |
| `GET` | `/api/entries/{id}/status` | 查询转发状态 |
| `GET` | `/api/entries/{id}/logs` | 分页查询日志 |

### 创建条目示例

```bash
curl -X POST http://localhost:25879/api/entries \
  -H "Content-Type: application/json" \
  -d '{
    "name": "MySQL 转发",
    "source_address": "0.0.0.0",
    "source_port": 3306,
    "target_address": "192.168.1.100",
    "target_port": 3306,
    "enabled": true
  }'
```

## 配置文件格式

```json
{
  "entries": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "MySQL 转发",
      "source_address": "0.0.0.0",
      "source_port": 3306,
      "target_address": "192.168.1.100",
      "target_port": 3306,
      "log_directory": "/home/user/.config/porthannis/logs/mysql_forward",
      "enabled": true,
      "created_at": "2026-01-01T00:00:00Z",
      "updated_at": "2026-01-01T00:00:00Z"
    }
  ]
}
```

## 从源码构建

### 前置条件

- Rust 1.85+
- Node.js 22+
- (仅 GUI) Tauri 系统依赖: `libwebkit2gtk-4.1-dev` 等

### Headless 服务器

```bash
cd frontend && npm ci && npm run build
cd ..
cargo build --release -p porthannis-server
# 二进制位于: target/release/porthannis
```

### Tauri GUI

```bash
cd frontend && npm ci && npm run build
cd ../gui
cargo tauri build
```

## 技术栈

- **Rust** — 核心后端 + Axum HTTP 框架 + Tokio 异步
- **Tauri v2** — 跨平台桌面应用
- **React + TypeScript + Vite** — Web 前端
- **JSON** — 配置持久化（无数据库依赖）

## 架构概览

```
┌─────────────────────────────────────┐
│           前端 (React)              │
│   (GUI 内嵌 / Headless 浏览器)       │
└─────────────┬───────────────────────┘
              │ REST API (HTTP)
┌─────────────▼───────────────────────┐
│       Axum API Server (Rust)       │
│  ├─ CRUD 条目管理                    │
│  ├─ 启动/停止 转发                   │
│  └─ 日志查询                         │
└─────────────┬───────────────────────┘
              │
┌─────────────▼───────────────────────┐
│      ForwardingManager (Rust)       │
│  ├─ ConfigStore (JSON 读写)          │
│  ├─ TcpProxy × N (端口转发)          │
│  └─ EntryLogger × N (日志轮转)       │
└─────────────────────────────────────┘
```

## License

MIT
