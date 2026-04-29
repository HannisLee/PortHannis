# PortHannis

一个轻量级的跨平台端口转发工具，通过系统托盘 UI 管理转发规则。

A lightweight cross-platform port forwarding tool with a system tray UI.

## 功能特性

- 单文件可执行程序，无需安装
- 系统托盘常驻，点击打开配置窗口
- 简单的 TCP 端口转发：本地端口 → 目标地址:端口
- 每个转发规则独立日志记录，支持循环缓冲（单文件最大 10MB）
- 跨平台支持：Windows、Linux、macOS

## 界面预览

```
┌─────────────────────────────────────────────┐
│  PortHannis                    [─][□][×]    │
├─────────────────────────────────────────────┤
│                                             │
│  [+ 添加转发规则]                           │
│                                             │
│  ┌───────────────────────────────────────┐ │
│  │ 🟢 本地:8080 → 192.168.1.100:3000    │ │
│  │    [查看日志] [禁用] [删除]           │ │
│  └───────────────────────────────────────┘ │
│                                             │
│  ┌───────────────────────────────────────┐ │
│  │ ⚫ 本地:9000 → 10.0.0.5:22            │ │
│  │    [查看日志] [启用] [删除]           │ │
│  └───────────────────────────────────────┘ │
│                                             │
└─────────────────────────────────────────────┘
```

## 下载

前往 [Releases](https://github.com/HannisLee/PortHannis/releases) 下载对应平台的可执行文件。

## 快速开始

1. 下载对应平台的可执行文件
2. 运行程序 — 应用将出现在系统托盘中
3. 点击托盘图标打开配置窗口
4. 添加转发规则：本地端口 → 目标地址 + 目标端口
5. 启用规则即可开始转发

## 配置文件

配置和日志文件按平台存储在以下位置：

| 平台 | 路径 |
|------|------|
| Windows | `%APPDATA%\porthannis\` |
| Linux | `~/.config/porthannis/` |
| macOS | `~/Library/Application Support/porthannis/` |

目录结构：

```
porthannis/
├── rules.json          # 转发规则配置
└── logs/
    └── {ruleID}.log    # 每条规则独立的日志文件
```

## 技术栈

| 组件 | 选择 | 说明 |
|------|------|------|
| 后端语言 | Go 1.22+ | 高性能、跨平台编译、单文件输出 |
| UI 框架 | Wails v2 | Go + Web 前端，使用系统 WebView |
| 前端框架 | Svelte + TypeScript | 轻量、编译后体积小 |
| 配置存储 | JSON 文件 | 简单、人类可读 |
| 日志存储 | 循环缓冲文件 | 限制磁盘占用，每规则独立文件 |

## 项目结构

```
PortHannis/
├── main.go                 # Wails 入口
├── app.go                  # 主应用逻辑和 Wails bindings
├── config/                 # 配置管理
│   ├── types.go            # 数据结构定义
│   └── manager.go          # 配置文件读写
├── forwarder/              # 端口转发引擎
│   ├── engine.go           # 转发引擎核心
│   ├── rule.go             # 转发规则运行时状态
│   └── logger.go           # 循环缓冲日志记录器
├── tray/                   # 系统托盘（开发中）
├── frontend/               # Wails 前端
│   ├── src/
│   │   ├── App.svelte      # 主界面
│   │   └── components/     # UI 组件
│   └── package.json
├── wails.json              # Wails 配置
├── go.mod
└── README.md
```

## 开发

### 环境要求

- [Go 1.22+](https://go.dev/dl/)
- [Wails CLI v2](https://wails.io/docs/gettingstarted/installation)
- [Node.js](https://nodejs.org/)（前端工具链）

### 构建

```bash
# 克隆仓库
git clone https://github.com/HannisLee/PortHannis.git
cd PortHannis

# 安装前端依赖
cd frontend && npm install && cd ..

# 开发模式
wails dev

# 生产构建
wails build
```

### 跨平台编译

```bash
wails build -platform windows/amd64
wails build -platform linux/amd64
wails build -platform darwin/amd64
wails build -platform darwin/arm64
```

## 开发进度

- [x] Phase 1: 项目初始化
- [x] Phase 2: 核心数据结构与配置管理
- [ ] Phase 3: 端口转发引擎（进行中）
- [ ] Phase 4: 系统托盘
- [ ] Phase 5: 主应用逻辑
- [ ] Phase 6: 前端 UI
- [ ] Phase 7: 集成与测试
- [ ] Phase 8: 打包发布

## License

MIT
