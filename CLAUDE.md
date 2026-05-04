# CLAUDE.md

PortHannis — 轻量级端口转发管理器，Rust 核心 + React WebUI。

## 项目结构

```
port-hannis/
├── port.json                # 配置文件（所有端口转发信息）
├── Cargo.toml               # Rust workspace
├── server/                  # Rust HTTP 服务器 + TCP 转发核心
│   ├── core.rs              # TCP 转发核心（单文件，所有核心逻辑）
│   ├── src/main.rs          # Axum HTTP API
│   └── Cargo.toml
├── frontend/                # React Web UI（开发中）
├── gui/                     # Tauri 桌面应用（开发中）
└── logs/                    # 日志目录（运行时生成）
```

## 核心文件说明

### server/core.rs
包含所有 TCP 转发核心逻辑的单文件（~800 行）：
- **数据结构**：`ForwardingEntry`, `EntryStatus`, `LogMessage`
- **TCP 转发**：`TcpProxy` - 基于 tokio 的异步 TCP 双向转发
- **配置管理**：`ConfigStore` - 读写 `port.json`
- **日志轮转**：`EntryLogger` - 1MB × 5 文件轮转
- **生命周期**：`ProxyManager` - 启动/停止/查询转发条目
- **API 处理函数**：所有 HTTP 端点的实现

### server/src/main.rs
HTTP API 服务器：
- Axum 路由配置
- 静态文件服务（rust-embed 嵌入 frontend/dist）
- 浏览器自动打开（7777 端口）

## 快速开始

### 运行服务器
```bash
# 直接运行
run.bat

# 或使用 cargo
cargo run -p porthannis-server
```

服务器将在 `http://127.0.0.1:7777` 启动，浏览器会自动打开。

### 构建发布版本
```bash
cargo build --release -p porthannis-server
# 可执行文件: target/release/porthannis.exe
```

## API 端点

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | `/api/health` | 健康检查 |
| GET | `/api/entries` | 列出所有条目 |
| POST | `/api/entries` | 创建新条目 |
| GET | `/api/entries/{id}` | 获取单个条目 |
| PUT | `/api/entries/{id}` | 更新条目 |
| DELETE | `/api/entries/{id}` | 删除条目 |
| POST | `/api/entries/{id}/start` | 启动转发 |
| POST | `/api/entries/{id}/stop` | 停止转发 |
| GET | `/api/entries/{id}/status` | 查询状态 |
| GET | `/api/entries/{id}/logs` | 获取日志 |

## port.json 格式

```json
{
  "entries": [
    {
      "id": "uuid-v4",
      "name": "示例转发",
      "source_address": "0.0.0.0",
      "source_port": 8080,
      "target_address": "192.168.3.11",
      "target_port": 80,
      "enabled": true,
      "log_directory": "logs/example"
    }
  ]
}
```

## 技术栈

| 组件 | 技术 |
|------|------|
| HTTP API | Axum 0.8 |
| TCP 转发 | tokio + tokio-util |
| 序列化 | serde + serde_json |
| 日志 | tracing |
| 嵌入前端 | rust-embed |

## 日志系统

每个转发条目都有独立的日志目录：
- `logs/{entry_name}/current.log` - 当前日志
- `logs/{entry_name}/current.log.1` - 历史日志 1
- `logs/{entry_name}/current.log.5` - 历史日志 5（最旧）

单个文件最大 1MB，最多保留 5 个历史文件。

## 开发状态

- ✅ **server/core.rs** - TCP 转发核心完成
- ✅ **server/src/main.rs** - HTTP API 完成
- ✅ **port.json** - 配置管理完成
- ⏳ **frontend/** - Web UI 开发中
- ⏳ **gui/** - Tauri GUI 开发中

## 常见问题

### 端口被占用
如果 7777 端口被占用，可以修改 `server/src/main.rs` 中的端口号。

### 日志位置
日志文件存储在 `logs/{条目名称}/` 目录下。

### Windows 防火墙
首次运行时，Windows 可能会询问是否允许端口监听，请点击"允许"。
