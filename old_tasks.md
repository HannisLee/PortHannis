# PortHannis 任务列表

## Phase 1: 项目初始化

- [x] 1.1 创建 Go 模块
  ```bash
  cd d:/Dev/PortHannis
  go mod init github.com/HannisLee/PortHannis
  ```

- [x] 1.2 安装 Wails CLI
  ```bash
  go install github.com/wailsapp/wails/v2/cmd/wails@latest
  ```

- [x] 1.3 初始化 Wails + Svelte 项目
  ```bash
  wails init -n porthannis -t svelte-ts
  ```

- [x] 1.4 配置 wails.json（设置应用名称、图标路径等）

- [x] 1.5 创建项目目录结构
  ```bash
  mkdir -p forwarder config tray
  ```

## Phase 2: 核心数据结构

- [x] 2.1 创建 `config/types.go`
  - [x] 定义 `ForwardRule` 结构体
  - [x] 定义 `LogEntry` 结构体
  - [x] 定义 `Config` 结构体（包含规则列表）

- [x] 2.2 创建 `config/manager.go`
  - [x] 实现 `LoadConfig()` - 从 JSON 文件加载配置
  - [x] 实现 `SaveConfig()` - 保存配置到 JSON 文件
  - [x] 实现 `GetConfigDir()` - 获取平台相关配置目录
  - [x] 实现 `GetLogsDir()` - 获取日志目录

## Phase 3: 端口转发引擎

- [x] 3.1 创建 `forwarder/rule.go`
  - [x] 定义 `RuleState` 结构体（运行时状态）
  - [x] 实现 `NewRuleState()` - 创建规则状态
  - [x] 实现 `Start()` - 启动转发监听
  - [x] 实现 `Stop()` - 停止转发监听

- [x] 3.2 创建 `forwarder/logger.go`
  - [x] 实现 `CircularLogger` - 循环缓冲日志器
  - [x] 实现 `Write()` - 检查文件大小并写入
  - [x] 实现 `ReadLogs()` - 读取日志条目
  - [x] 实现 `Clear()` - 清空日志文件

- [x] 3.3 创建 `forwarder/engine.go`
  - [x] 定义 `Engine` 结构体
  - [x] 实现 `NewEngine()` - 创建转发引擎
  - [x] 实现 `StartRule()` - 启动单个规则
  - [x] 实现 `StopRule()` - 停止单个规则
  - [x] 实现 `RestartRule()` - 重启单个规则
  - [x] 实现 `handleConnection()` - 处理单个 TCP 连接转发

## Phase 4: 系统托盘

- [x] 4.1 准备托盘图标资源
  - [x] 创建或获取图标（16x16, 32x32, 48x48）
  - [x] 放入 `build/icon/` 目录

- [x] 4.2 创建 `tray/tray.go`
  - [x] 实现 `NewTray()` - 创建系统托盘
  - [x] 添加 "显示/隐藏" 菜单项
  - [x] 添加 "退出" 菜单项
  - [x] 实现双击打开窗口
  - [x] 实现托盘图标状态切换（运行中/已停止）

## Phase 5: 主应用逻辑

- [x] 5.1 创建 `app.go`
  - [x] 定义 `App` 结构体
  - [x] 实现 `NewApp()` - 初始化应用
  - [x] 实现 `Startup()` - Wails 启动时调用
  - [x] 实现 `Shutdown()` - Wails 关闭时调用（最小化到托盘）

- [x] 5.2 实现 Wails Bindings - 规则管理
  - [x] `GetRules() []ForwardRule`
  - [x] `AddRule(localPort, targetHost, targetPort) error`
  - [x] `DeleteRule(id) error`
  - [x] `ToggleRule(id, enabled) error`

- [x] 5.3 实现 Wails Bindings - 日志和状态
  - [x] `GetLogs(ruleID, limit) []LogEntry`
  - [x] `ClearLogs(ruleID) error`
  - [x] `GetStatus() map[string]bool`

## Phase 6: 前端 UI

- [x] 6.1 设置 Svelte 项目结构
  - [x] 安装必要依赖（tailwindcss 或其他 UI 库）

- [x] 6.2 创建 `src/lib/api.ts`
  - [x] 封装所有 Wails Go 方法调用

- [x] 6.3 创建 `src/App.svelte` - 主界面
  - [x] 页面布局设计
  - [x] 规则列表容器
  - [x] 添加规则按钮

- [x] 6.4 创建 `src/components/RuleForm.svelte`
  - [x] 表单：本地端口、目标 IP、目标端口
  - [x] 输入验证（端口范围、IP 格式）
  - [x] 提交和取消按钮

- [x] 6.5 创建 `src/components/RuleItem.svelte`
  - [x] 单条规则显示卡片
  - [x] 状态指示器（🟢 运行中 / ⚫ 已停止）
  - [x] 查看日志按钮
  - [x] 启用/禁用开关
  - [x] 删除按钮

- [x] 6.6 创建 `src/components/LogViewer.svelte`
  - [x] 日志列表显示
  - [x] 时间戳、来源、流量、状态
  - [x] 清空日志按钮
  - [x] 自动滚动到底部

## Phase 7: 集成与测试

- [ ] 7.1 整合所有模块
  - [ ] 确保 main.go 正确初始化所有组件
  - [ ] 测试应用启动流程

- [ ] 7.2 功能测试
  - [ ] 添加规则并启动转发
  - [ ] 测试 TCP 连接转发
  - [ ] 测试日志记录和查看
  - [ ] 测试多规则同时运行
  - [ ] 测试启用/禁用规则
  - [ ] 测试删除规则
  - [ ] 测试配置持久化（重启后保留）

- [ ] 7.3 跨平台编译测试
  - [ ] Windows 编译
  - [ ] Linux 编译
  - [ ] macOS 编译

## Phase 8: 打包发布

- [ ] 8.1 生成各平台安装包
  ```bash
  wails build -platform windows/amd64
  wails build -platform linux/amd64
  wails build -platform darwin/amd64
  wails build -platform darwin/arm64
  ```

- [ ] 8.2 编写 README.md
  - [ ] 功能介绍
  - [ ] 下载链接
  - [ ] 使用说明
  - [ ] 配置文件位置说明

## 待办事项

- [ ] 准备托盘图标资源

---

# PortHannis 任务列表 — WebUI 功能开发（已完成）

## 背景

PortHannis 当前通过 Wails 桌面 GUI 提供规则管理。在 Linux 服务器场景下，用户通常没有 GUI 桌面环境，只能通过 SSH 远程操作。因此需要内嵌一个 WebUI HTTP 服务，用户通过 SSH 端口转发后即可在浏览器中管理转发规则。

**已有依赖（go.mod 中已存在但未使用）：**
- `github.com/labstack/echo/v4` — HTTP 框架
- `github.com/gorilla/websocket` — WebSocket 支持

---

## Phase 1: 配置扩展

- [x] 1.1 扩展 `config/types.go` — 添加 WebUI 配置结构
  ```go
  type WebUIConfig struct {
      Enabled  bool   `json:"enabled"`   // 是否启用 WebUI，默认 true
      Port     int    `json:"port"`      // 监听端口，默认 18080
      Password string `json:"password"`  // 访问密码（空则无需认证）
  }
  ```
  在 `Config` 结构体中新增 `WebUI WebUIConfig` 字段。

- [x] 1.2 更新 `config/manager.go`
  - [x] 新增 `GetWebUIConfig() WebUIConfig` 方法
  - [x] 新增 `UpdateWebUIConfig(cfg WebUIConfig) error` 方法
  - [x] 在 `LoadConfig()` 中设置 WebUI 默认值（enabled: true, port: 18080）

**修改文件：**
- `config/types.go`
- `config/manager.go`

---

## Phase 2: WebUI HTTP 服务器

- [x] 2.1 创建 `webui/server.go` — HTTP 服务器核心
  - [x] 定义 `Server` 结构体，持有 `*echo.Echo`、`*App` 引用
  - [x] 实现 `NewServer(app *App) *Server` — 初始化 Echo 实例
  - [x] 实现 `Start()` — 启动 HTTP 服务（非阻塞，goroutine）
  - [x] 实现 `Stop()` — 优雅关闭 HTTP 服务
  - [x] 嵌入前端静态文件（`//go:embed`）

- [x] 2.2 创建 `webui/auth.go` — 认证中间件
  - [x] 基于 session cookie 的简单密码认证
  - [x] `POST /api/login` — 验证密码，设置 cookie
  - [x] `POST /api/logout` — 清除 cookie
  - [x] 中间件：未认证请求返回 401
  - [x] 密码为空时跳过认证

- [x] 2.3 创建 `webui/api.go` — REST API 路由
  复用 `App` 中已有的业务方法，映射为 REST 端点：

  | 方法 | 路径 | 对应 App 方法 |
  |------|------|--------------|
  | GET | `/api/rules` | `GetRules()` |
  | POST | `/api/rules` | `AddRule(localPort, targetHost, targetPort)` |
  | DELETE | `/api/rules/:id` | `DeleteRule(id)` |
  | PUT | `/api/rules/:id/toggle` | `ToggleRule(id, enabled)` |
  | GET | `/api/rules/:id/logs?limit=500` | `GetLogs(ruleID, limit)` |
  | DELETE | `/api/rules/:id/logs` | `ClearLogs(ruleID)` |
  | GET | `/api/status` | `GetStatus()` |
  | GET/PUT | `/api/webui-config` | WebUI 配置读写 |

- [x] 2.4 创建 `webui/ws.go` — WebSocket 实时状态推送
  - [x] `GET /api/ws/status` — WebSocket 端点
  - [x] 定时推送规则运行状态（每 2 秒）
  - [x] 处理连接断开

**新建文件：**
- `webui/server.go`
- `webui/auth.go`
- `webui/api.go`
- `webui/ws.go`

---

## Phase 3: 集成到主应用

- [x] 3.1 修改 `app.go` — 集成 WebUI 服务器
  - [x] `App` 结构体新增 `webui *webui.Server` 字段
  - [x] `startup()` 中根据配置启动 WebUI 服务器
  - [x] `shutdown()` 中停止 WebUI 服务器
  - [x] 新增 Wails Binding：`GetWebUIConfig()` 和 `UpdateWebUIConfig()`

- [x] 3.2 修改 `main.go` — 确保生命周期正确
  - [x] 确认 WebUI 随 Wails 应用启停

**修改文件：**
- `app.go`
- `main.go`

---

## Phase 4: 前端适配 — 双模式 API 层

当前前端 `api.ts` 直接调用 Wails IPC（`window['go']['main']['App']`）。需要改造为同时支持 Wails 模式和 HTTP 模式。

- [x] 4.1 改造 `frontend/src/lib/api.ts`
  - [x] 检测运行环境：`window.__WAILS__` 存在 → Wails 模式，否则 → HTTP 模式
  - [x] HTTP 模式下使用 `fetch` 调用 `/api/*` 端点
  - [x] 保持相同的 TypeScript 接口，对外透明
  - [x] HTTP 模式下自动处理认证（登录跳转或 401 拦截）

- [x] 4.2 新增 `frontend/src/components/LoginForm.svelte` — WebUI 登录页
  - [x] 简单密码输入框
  - [x] 调用 `POST /api/login`
  - [x] 仅在 HTTP 模式 + 需要认证时显示

- [x] 4.3 修改 `frontend/src/App.svelte`
  - [x] 添加认证状态管理（HTTP 模式下）
  - [x] 未登录时显示 LoginForm
  - [x] WebSocket 连接状态实时更新

- [x] 4.4 修改 `frontend/src/components/RuleItem.svelte`
  - [x] 无逻辑变更，仅确认双模式兼容

- [x] 4.5 修改 `frontend/src/components/RuleForm.svelte`
  - [x] 无逻辑变更，仅确认双模式兼容

- [x] 4.6 修改 `frontend/src/components/LogViewer.svelte`
  - [x] 无逻辑变更，仅确认双模式兼容

**修改文件：**
- `frontend/src/lib/api.ts`（核心改造）
- `frontend/src/App.svelte`
- `frontend/src/components/LoginForm.svelte`（新建）

---

## Phase 5: 构建与嵌入

- [x] 5.1 修改 `main.go` 的 embed 指令
  - [x] Wails 模式：`//go:embed all:frontend/dist`（现有）
  - [x] WebUI 模式：同样嵌入 `frontend/dist`，由 Echo 提供静态文件服务
  - [x] 确保两套 embed 不冲突（使用相同的构建产物）

- [x] 5.2 更新构建流程
  - [x] `wails build` 产出的单文件可执行程序同时包含 GUI 和 WebUI
  - [x] 确认构建产物体积增长在可接受范围内

**修改文件：**
- `main.go`

---

## Phase 6: 测试验证

- [x] 6.1 WebUI 功能测试
  - [x] 启动应用，浏览器访问 `http://localhost:18080`
  - [x] 测试添加/删除/切换规则
  - [x] 测试日志查看和清空
  - [x] 测试 WebSocket 状态实时更新

- [x] 6.2 认证测试
  - [x] 设置密码后，未登录无法访问 API
  - [x] 登录后可正常操作
  - [x] 密码为空时无需认证

- [x] 6.3 SSH 端口转发测试
  - [x] Linux 上启动应用
  - [x] 从本地 SSH 转发：`ssh -L 18080:localhost:18080 user@server`
  - [x] 本地浏览器访问 `http://localhost:18080` 操作正常

- [x] 6.4 双模式共存测试
  - [x] Wails GUI 和 WebUI 同时操作，状态一致
  - [x] 通过 WebUI 添加的规则在 GUI 中可见
  - [x] 通过 GUI 添加的规则在 WebUI 中可见

---

## 文件变更总览

| 操作 | 文件 | 说明 |
|------|------|------|
| 新建 | `webui/server.go` | HTTP 服务器核心 |
| 新建 | `webui/auth.go` | 认证中间件 |
| 新建 | `webui/api.go` | REST API 路由 |
| 新建 | `webui/ws.go` | WebSocket 状态推送 |
| 新建 | `frontend/src/components/LoginForm.svelte` | 登录组件 |
| 修改 | `config/types.go` | 新增 WebUIConfig |
| 修改 | `config/manager.go` | 新增 WebUI 配置读写 |
| 修改 | `app.go` | 集成 WebUI 服务器 |
| 修改 | `main.go` | embed 和生命周期 |
| 修改 | `frontend/src/lib/api.ts` | 双模式 API 层 |
| 修改 | `frontend/src/App.svelte` | 认证状态管理 |
| 复制 | `old_tasks.md` | 旧任务归档 |
