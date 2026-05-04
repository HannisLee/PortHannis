# CLAUDE.md

PortHannis — 轻量级跨平台端口转发管理器，Rust 核心 + Tauri GUI + React WebUI。

## 项目结构

```
PortHannis/
├── Cargo.toml                     # workspace root
├── core/                          # 核心库 (porthannis-core)
│   └── src/
│       ├── models.rs              # ForwardingEntry, EntryStatus, LogLine 等
│       ├── config.rs              # ConfigStore: JSON 配置文件原子读写
│       ├── logger.rs              # EntryLogger: 分段日志轮转 (5×1MB, O(1))
│       ├── forwarder.rs           # TcpProxy: tokio 异步 TCP 双向转发
│       ├── manager.rs             # ForwardingManager: 总调度器 (CRUD + 生命周期)
│       ├── error.rs               # 统一错误类型
│       └── api/                   # Axum REST 路由 (供 server 和 gui 共用)
├── server/                        # Headless 二进制 (porthannis-server)
│   └── src/main.rs                # CLI + rust-embed 前端 + Axum
├── gui/                           # Tauri v2 桌面应用
│   ├── src/main.rs                # 后台启动 Axum, 查找可用端口, 启动 Tauri
│   ├── src/lib.rs                 # run_tauri + get_api_port command
│   ├── tauri.conf.json            # 窗口配置, frontendDist, bundle
│   └── capabilities/default.json
├── frontend/                      # React + TypeScript + Vite
│   └── src/
│       ├── api/                   # client.ts (fetch 封装, Tauri/Headless 双模式)
│       ├── components/            # Layout, Sidebar, StatusBadge, LogViewer 等
│       ├── pages/                 # Dashboard, EntryList, EntryForm, LogPage
│       └── hooks/                 # useEntries, useLogs (React Query)
└── packaging/                     # 构建与打包脚本
```

## 构建命令

```bash
# 前端
cd frontend && npm ci && npm run build    # 构建到 frontend/dist/

# Headless 服务器 (含嵌入式前端)
cargo build --release -p porthannis-server   # => target/release/porthannis[.exe]

# Tauri GUI
cd gui && cargo tauri build

# 全 workspace 检查
cargo check && cargo clippy --all-targets --all-features && cargo test --all
```

## 架构要点

- **前后端通信**: Tauri 模式下, Rust 在 `127.0.0.1:<随机端口>` 启动 Axum, 通过 `window.__PORTHANNIS_API_PORT__` 把端口传给 React 前端; Headless 模式下前端由 `rust-embed` 嵌入服务
- **日志轮转**: 每个条目独立 `log_directory/<entry_id>/` 目录, `current.log` + `current.log.1..5`, 每段 1MB, 超出后重命名循环删除最旧段
- **TCP 转发**: `tokio::io::copy` 双向拷贝 (split + try_join), 连接日志通过 `mpsc::unbounded_channel` 异步写入, 避免 I/O 路径持锁
- **配置原子写入**: 序列化到 `<config>.tmp`, `fs::rename` 到正式路径
- **前端 API 客户端自动检测**: `window.__PORTHANNIS_API_PORT__` 存在则用 `http://127.0.0.1:{port}`, 否则同源相对路径

## 关键技术栈

| 层面 | 技术 |
|------|------|
| 异步运行时 | Tokio |
| HTTP 框架 | Axum 0.8 |
| 桌面框架 | Tauri v2 |
| 前端 UI | React 19 + TypeScript + Vite |
| 前端状态 | @tanstack/react-query |
| 路由 | react-router-dom v7 |
| 配置 | serde + serde_json (单 JSON 文件) |
| 日志 | tracing + tracing-subscriber |

## 注意事项

- `rust-embed` 的 `#[folder]` 指向 `../frontend/dist/`, 构建 headless 前必须先 `npm run build`
- `core/src/api/mod.rs` 的 `build_router` 被 server 和 gui 共用, 修改路由会影响两者
- `tauri.conf.json` 中的 `frontendDist` 字段名是 Tauri v2 格式, 不要改成 `distDir`
- 前端 `EntryList` 的状态列 (`e.status`) 来自 `useEntries` query 数据中的嵌套字段 — 需要确保 API 返回的条目包含 `status` 字段, 或者在组件层通过 `useEntryStatus` 单独查询
- 日志行格式: `YYYY-MM-DDTHH:MM:SS.sssZ [level] message`, 解析依赖此格式
- Windows 构建使用 `windows_subsystem = "windows"` 属性隐藏控制台窗口
