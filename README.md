# PortHannis — 端口转发管理器

轻量级端口转发管理工具，Rust 核心 + Web UI。

## 特性

- **TCP 端口转发** — 高性能异步 TCP 代理，支持双向数据转发
- **JSON 配置管理** — 单文件配置（port.json），人工可编辑
- **日志轮转** — 每条目独立日志，1MB × 5 文件轮转
- **REST API** — 完整的 CRUD + 启停控制 API
- **WebUI** — 通过浏览器管理，支持 7777 端口访问
- **单文件核心** — server/core.rs 包含所有核心逻辑

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
curl -X POST http://127.0.0.1:7777/api/entries \
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

`port.json` 位于项目根目录：

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
      "log_directory": "logs/mysql_forward",
      "enabled": true,
      "created_at": "2026-01-01T00:00:00Z",
      "updated_at": "2026-01-01T00:00:00Z"
    }
  ]
}
```

## 技术栈

- **Rust** — 核心后端 + Axum HTTP 框架 + Tokio 异步
- **React + TypeScript** — Web 前端（开发中）
- **JSON** — 配置持久化（无数据库依赖）

## 项目结构

```
port-hannis/
├── port.json          # 配置文件
├── server/
│   ├── core.rs        # TCP 转发核心（~800 行）
│   └── src/main.rs    # HTTP API 服务器
├── frontend/          # Web UI（开发中）
└── logs/              # 日志目录
```

## 从源码构建

### 前置条件

- Rust 1.85+
- Node.js 22+（构建前端）

### 构建服务器

```bash
cargo build --release -p porthannis-server
# 二进制位于: target/release/porthannis.exe
```

## License

MIT
