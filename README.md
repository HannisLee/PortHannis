# PortHannis — 端口转发管理器

轻量级端口转发管理工具，Rust 核心 + 内嵌 WebUI，单二进制部署。

## 特性

- **CLI 命令行管理** — 终端直接操作转发条目（list / add / modify）
- **Web 管理界面** — 按需启动，浏览器可视化管理（`porthannis serve`）
- **TCP 端口转发** — 高性能异步 TCP 代理，支持双向数据转发
- **JSON 配置管理** — 单文件配置（port.json），人工可编辑，首次运行自动创建
- **日志轮转** — 每条目独立日志，1MB × 5 文件轮转
- **REST API** — 完整的 CRUD + 启停控制 API
- **Tauri 桌面应用** — Windows GUI 便携版，免安装直接运行

## 快速开始

### Windows

从 [Releases](https://github.com/HannisLee/PortHannis/releases) 下载 `porthannis.exe`，放入任意目录即可使用。

```powershell
# 查看帮助
porthannis.exe

# 添加一条转发规则
porthannis.exe add --name "MySQL转发" -s 3306 -a 192.168.1.100 -t 3306

# 查看所有转发条目
porthannis.exe list

# 启动 Web 管理界面（浏览器自动打开）
porthannis.exe serve
```

`port.json` 会在 exe 同目录自动创建。

### Ubuntu / Debian

**方式一：下载预编译二进制**

从 [Releases](https://github.com/HannisLee/PortHannis/releases) 下载 `porthannis`：

```bash
chmod +x porthannis
./porthannis list
```

**方式二：从源码运行**

```bash
# 安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆并构建
git clone https://github.com/HannisLee/PortHannis.git
cd PortHannis
cargo build --release -p porthannis-server

# 使用
./target/release/porthannis list
./target/release/porthannis serve
```

### macOS / 其他平台

```bash
git clone https://github.com/HannisLee/PortHannis.git
cd PortHannis
cargo run --release -p porthannis-server -- serve
```

## CLI 命令

### porthannis list

列出所有转发条目：

```
┌──────────────────────────────────────┬──────────┬────────┬───────────┬──────────┬──────┐
│ ID                                   ┆ 名称     ┆ 源端口 ┆ 目标地址  ┆ 目标端口 ┆ 启用 │
╞══════════════════════════════════════╪══════════╪════════╪═══════════╪══════════╪══════╡
│ 8f2f3a7e-0059-4cdb-919b-7a282bd319af ┆ 测试转发 ┆ 9999   ┆ 127.0.0.1 ┆ 80       ┆ 是   │
└──────────────────────────────────────┴──────────┴────────┴───────────┴──────────┴──────┘
```

### porthannis add

添加新的转发条目：

```bash
porthannis add --name "MySQL转发" --source-port 3306 --target-address 192.168.1.100 --target-port 3306
# 短参数形式：
porthannis add -n "MySQL转发" -s 3306 -a 192.168.1.100 -t 3306
```

### porthannis modify

修改已有转发条目（按 ID，支持部分更新）：

```bash
porthannis modify <ID> --name "新名称"
porthannis modify <ID> --enabled false
porthannis modify <ID> -s 8080 -a 10.0.0.1 -t 80
```

### porthannis serve

启动 Web 管理界面：

```bash
# 默认 127.0.0.1:7777，自动打开浏览器
porthannis serve

# 自定义监听地址
porthannis serve --addr 0.0.0.0:9000

# 不自动打开浏览器
porthannis serve --no-open
```

## API 文档

Web 管理界面启动后（`porthannis serve`），提供以下 REST API：

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

`port.json` 位于 exe 同目录，首次运行自动创建：

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
- **clap** — CLI 命令行解析
- **comfy-table** — 终端表格输出
- **Tauri 2** — Windows 桌面应用（WebView 包装）
- **vanilla HTML/CSS/JS** — WebUI 内嵌于二进制（无前端构建步骤）
- **JSON** — 配置持久化（无数据库依赖）

## 项目结构

```
PortHannis/
├── port.json              # 配置文件（运行时自动创建于 exe 同目录）
├── Cargo.toml             # Rust workspace
├── server/
│   ├── core.rs            # TCP 转发核心（单文件，所有核心逻辑）
│   ├── web.html           # 内嵌 WebUI（单文件）
│   └── src/main.rs        # CLI 入口 + HTTP API 服务器
├── gui/
│   ├── src/main.rs        # Tauri 桌面应用入口
│   ├── src/lib.rs         # Tauri 库
│   ├── tauri.conf.json    # Tauri 配置
│   ├── dist/              # Tauri 前端占位
│   └── icons/             # 应用图标
└── logs/                  # 日志目录（运行时生成）
```

## 从源码构建

### 前置条件

- Rust 1.85+

### 构建命令行版本

```bash
cargo build --release -p porthannis-server
# 二进制位于: target/release/porthannis (或 .exe)
```

### 构建 Windows GUI 桌面应用

```bash
cargo build --release -p porthannis-gui
# 二进制位于: target/release/porthannis-gui.exe
```

## License

MIT
