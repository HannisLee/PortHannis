<div align="center">

# PortHannis

**轻量级端口转发管理工具**

Rust 核心 · CLI 优先 · 可选 WebUI · 单文件部署

[![CI](https://github.com/HannisLee/PortHannis/actions/workflows/ci.yml/badge.svg)](https://github.com/HannisLee/PortHannis/actions/workflows/ci.yml)
[![Release](https://github.com/HannisLee/PortHannis/actions/workflows/release.yml/badge.svg)](https://github.com/HannisLee/PortHannis/releases)

</div>

---

## 安装

从 [Releases](https://github.com/HannisLee/PortHannis/releases) 下载对应平台的二进制文件：

| 平台 | 文件 | 说明 |
|------|------|------|
| Windows | `porthannis.exe` | 终端命令行版本 |
| Windows | `PortHannis-windows-portable.exe` | Tauri GUI 桌面版，免安装 |
| Linux | `porthannis` | 终端命令行版本 |

或从源码构建：

```bash
git clone https://github.com/HannisLee/PortHannis.git
cd PortHannis
cargo build --release -p porthannis-server
```

## 使用

`port.json` 配置文件会在 exe 同目录自动创建，无需手动新建。

### list — 查看转发条目

```bash
porthannis list
```

```
┌──────────────────────────┬──────────┬──────────┬────────┬───────────┬──────────┬──────┐
│ ID                       ┆ 名称     ┆ 监听地址 ┆ 源端口 ┆ 目标地址  ┆ 目标端口 ┆ 启用 │
╞══════════════════════════╪══════════╪══════════╪════════╪═══════════╪══════════╪══════╡
│ 8f2f3a7e-0059-4cdb-...   ┆ MySQL    ┆ 0.0.0.0  ┆ 3306   ┆ 10.0.0.1  ┆ 3306     ┆ 是   │
└──────────────────────────┴──────────┴──────────┴────────┴───────────┴──────────┴──────┘
```

### add — 添加转发条目

```bash
# 基本用法（监听所有网卡的 3306 端口，转发到 10.0.0.1:3306）
porthannis add -n "MySQL转发" -s 3306 -a 10.0.0.1 -t 3306

# 指定监听地址（只监听 192.168.1.1 上的 8080 端口）
porthannis add -n "Web转发" -s 8080 --source-address 192.168.1.1 -a 10.0.0.2 -t 80
```

| 参数 | 缩写 | 必填 | 说明 | 默认值 |
|------|------|------|------|--------|
| `--name` | `-n` | 是 | 条目名称 | — |
| `--source-port` | `-s` | 是 | 监听端口 | — |
| `--source-address` | — | 否 | 监听地址 | `0.0.0.0`（所有网卡） |
| `--target-address` | `-a` | 是 | 转发目标地址 | — |
| `--target-port` | `-t` | 是 | 转发目标端口 | — |

### modify — 修改转发条目

通过 ID 指定条目（从 `list` 命令获取 ID），支持部分更新：

```bash
porthannis modify <ID> --name "新名称"
porthannis modify <ID> --enabled false
porthannis modify <ID> -s 8080 -a 10.0.0.3 -t 443
```

### serve — 启动 Web 管理界面

```bash
# 默认启动在 127.0.0.1:7777，自动打开浏览器
porthannis serve

# 只改端口
porthannis serve -p 9000

# 监听所有网卡（允许局域网访问）
porthannis serve --host 0.0.0.0 -p 9000

# 不自动打开浏览器
porthannis serve --no-open
```

WebUI 启动后提供 REST API：

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/api/entries` | 列出所有条目 |
| `POST` | `/api/entries` | 创建条目 |
| `PUT` | `/api/entries/{id}` | 更新条目 |
| `DELETE` | `/api/entries/{id}` | 删除条目 |
| `POST` | `/api/entries/{id}/start` | 启动转发 |
| `POST` | `/api/entries/{id}/stop` | 停止转发 |
| `GET` | `/api/entries/{id}/status` | 查询状态 |
| `GET` | `/api/entries/{id}/logs` | 查看日志 |

## 配置文件

`port.json`（与 exe 同目录）：

```json
{
  "entries": [
    {
      "id": "auto-generated-uuid",
      "name": "MySQL 转发",
      "source_address": "0.0.0.0",
      "source_port": 3306,
      "target_address": "10.0.0.1",
      "target_port": 3306,
      "enabled": true,
      "log_directory": "logs/mysql_forward"
    }
  ]
}
```

可以直接编辑此文件，也可以通过 CLI 或 WebUI 管理。

## 技术栈

**Rust** · Axum · Tokio · clap · comfy-table · Tauri 2

## License

MIT
