# PortHannis 任务列表 — WebUI 功能开发

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
  - [ ] 新增 `GetWebUIConfig() WebUIConfig` 方法
  - [ ] 新增 `UpdateWebUIConfig(cfg WebUIConfig) error` 方法
  - [ ] 在 `LoadConfig()` 中设置 WebUI 默认值（enabled: true, port: 18080）

**修改文件：**
- `config/types.go`
- `config/manager.go`

---

## Phase 2: WebUI HTTP 服务器

- [x] 2.1 创建 `webui/server.go` — HTTP 服务器核心
  - [ ] 定义 `Server` 结构体，持有 `*echo.Echo`、`*App` 引用
  - [ ] 实现 `NewServer(app *App) *Server` — 初始化 Echo 实例
  - [ ] 实现 `Start()` — 启动 HTTP 服务（非阻塞，goroutine）
  - [ ] 实现 `Stop()` — 优雅关闭 HTTP 服务
  - [ ] 嵌入前端静态文件（`//go:embed`）

- [x] 2.2 创建 `webui/auth.go` — 认证中间件
  - [ ] 基于 session cookie 的简单密码认证
  - [ ] `POST /api/login` — 验证密码，设置 cookie
  - [ ] `POST /api/logout` — 清除 cookie
  - [ ] 中间件：未认证请求返回 401
  - [ ] 密码为空时跳过认证

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
  - [ ] 处理连接断开

**新建文件：**
- `webui/server.go`
- `webui/auth.go`
- `webui/api.go`
- `webui/ws.go`

---

## Phase 3: 集成到主应用

- [x] 3.1 修改 `app.go` — 集成 WebUI 服务器
  - [ ] `App` 结构体新增 `webui *webui.Server` 字段
  - [ ] `startup()` 中根据配置启动 WebUI 服务器
  - [ ] `shutdown()` 中停止 WebUI 服务器
  - [ ] 新增 Wails Binding：`GetWebUIConfig()` 和 `UpdateWebUIConfig()`

- [x] 3.2 修改 `main.go` — 确保生命周期正确
  - [ ] 确认 WebUI 随 Wails 应用启停

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

- [ ] 6.1 WebUI 功能测试
  - [ ] 启动应用，浏览器访问 `http://localhost:18080`
  - [ ] 测试添加/删除/切换规则
  - [ ] 测试日志查看和清空
  - [ ] 测试 WebSocket 状态实时更新

- [ ] 6.2 认证测试
  - [ ] 设置密码后，未登录无法访问 API
  - [ ] 登录后可正常操作
  - [ ] 密码为空时无需认证

- [ ] 6.3 SSH 端口转发测试
  - [ ] Linux 上启动应用
  - [ ] 从本地 SSH 转发：`ssh -L 18080:localhost:18080 user@server`
  - [ ] 本地浏览器访问 `http://localhost:18080` 操作正常

- [ ] 6.4 双模式共存测试
  - [ ] Wails GUI 和 WebUI 同时操作，状态一致
  - [ ] 通过 WebUI 添加的规则在 GUI 中可见
  - [ ] 通过 GUI 添加的规则在 WebUI 中可见

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
