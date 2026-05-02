# PortHannis - 跨平台端口转发工具

## 项目概述

PortHannis 是一个轻量级的跨平台端口转发工具，支持 Linux、Windows 和 macOS。

**核心特性：**
- 单文件可执行程序，无需安装
- 系统托盘常驻，点击打开配置窗口
- 简单的端口转发：本地端口 → 内网目标端口
- 每个转发规则独立日志记录和查看

## 技术栈

| 组件 | 技术选择 | 说明 |
|------|---------|------|
| 后端语言 | Go 1.21+ | 高性能、跨平台编译 |
| UI 框架 | Wails v2 | Go + Web frontend，使用系统 WebView |
| 前端框架 | Svelte | 轻量、编译后体积小 |
| 托盘支持 | Wails 内置 systray | 跨平台系统托盘 |
| 配置存储 | JSON 文件 | `~/.config/porthanni/rules.json` |
| 日志存储 | 循环缓冲文本日志 | 每规则独立文件，10MB 循环覆盖 |

## 项目结构

```
porthannis/
├── main.go                 # Wails 入口
├── app.go                  # 主应用逻辑和 Wails bindings
├── forwarder/              # 端口转发引擎
│   ├── engine.go          # 转发引擎核心
│   ├── rule.go            # 转发规则结构和方法
│   └── logger.go          # 循环缓冲日志记录器
├── config/                 # 配置管理
│   ├── manager.go         # 配置文件读写
│   └── types.go           # 数据结构定义
├── tray/                   # 系统托盘
│   └── tray.go            # 托盘图标和菜单
├── frontend/               # Wails 前端
│   ├── src/
│   │   ├── App.svelte     # 主界面
│   │   ├── components/    # UI 组件
│   │   │   ├── RuleItem.svelte    # 单条规则卡片
│   │   │   ├── RuleForm.svelte    # 添加/编辑规则表单
│   │   │   └── LogViewer.svelte   # 日志查看弹窗
│   │   └── lib/
│   │       └── api.ts     # Wails API 调用封装
│   ├── package.json
│   └── ...
├── wails.json             # Wails 配置
├── go.mod
└── README.md
```

## 核心数据结构

```go
// config/types.go
type ForwardRule struct {
    ID          string    `json:"id"`           // UUID
    LocalPort   int       `json:"localPort"`    // 本地监听端口 (1024-65535)
    TargetHost  string    `json:"targetHost"`   // 目标 IP/域名
    TargetPort  int       `json:"targetPort"`   // 目标端口
    Enabled     bool      `json:"enabled"`      // 是否启用
    CreatedAt   time.Time `json:"createdAt"`
}

type LogEntry struct {
    Timestamp   time.Time `json:"timestamp"`
    Source      string    `json:"source"`       // 客户端地址
    BytesIn     int64     `json:"bytesIn"`
    BytesOut    int64     `json:"bytesOut"`
    Status      string    `json:"status"`       // "connected" | "error" | "closed"
}
```

## Wails API 设计

暴露给前端的方法：

```go
// app.go - Wails bindings
type App struct {
    configManager   *config.Manager
    forwarderEngine *forwarder.Engine
    logManager      *forwarder.LogManager
}

// 规则管理
func (a *App) GetRules() []ForwardRule
func (a *App) AddRule(localPort int, targetHost string, targetPort int) error
func (a *App) DeleteRule(id string) error
func (a *App) ToggleRule(id string, enabled bool) error

// 日志查询
func (a *App) GetLogs(ruleID string, limit int) []LogEntry
func (a *App) ClearLogs(ruleID string) error

// 状态查询
func (a *App) GetStatus() map[string]bool  // ruleID -> running
```

## 日志策略

每个转发规则对应一个日志文件：`~/.config/porthanni/logs/{ruleID}.log`

**循环缓冲实现：**
- 单个日志文件最大 10MB
- 超过时从头开始覆盖写入
- 保证始终保留最新的日志条目

## 前端 UI 设计

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
│  配置文件位置: ~/.config/porthannis/        │
└─────────────────────────────────────────────┘
```

## 开发环境

```bash
# 安装 Wails CLI
go install github.com/wailsapp/wails/v2/cmd/wails@latest

# 初始化项目
wails init -n porthannis -t svelte

# 开发模式
wails dev

# 构建单文件
wails build
```

## 构建目标

- **Windows**: `porthannis.exe` (~15MB)
- **Linux**: `porthannis` (~12MB)
- **macOS**: `PortHannis.app` (~14MB)

## 设计决策

| 决策项 | 选择 | 原因 |
|--------|------|------|
| 目标地址配置 | IP 和端口分开输入 | 更清晰，减少输入错误 |
| 日志策略 | 10MB 循环覆盖 | 限制磁盘占用，保留最新信息 |
| 规则数量 | 无限制 | 用户自由控制 |
| 端口范围 | 1024-65535 | 避免需要管理员权限 |

## 开发工作流

### 任务领取与提交规范

1. **领取任务**：从 `tasks.md` 中选择一个待办任务，标记为进行中
2. **完成开发**：编写代码实现功能
3. **完整测试**：
   - 功能测试：验证核心功能正常工作
   - 边界测试：测试边界条件和错误处理
   - 跨平台验证（如适用）
4. **提交代码**：撰写规范的 commit 信息并提交
5. **完成阶段**：修改 `tasks.md` ，标记为已完成

### Commit 信息规范

遵循 Conventional Commits 格式：

### 环境配置

开发工具和缓存文件统一保存在 `D:\Code` 目录：

- **Go 安装路径**: `D:\Code\Go`
- **Go GOPATH**: `D:\Code\Go\go-packages`
- **Wails 缓存**: `D:\Code\Wails\cache`
