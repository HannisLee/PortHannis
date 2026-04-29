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

- [ ] 6.1 设置 Svelte 项目结构
  - [ ] 安装必要依赖（tailwindcss 或其他 UI 库）

- [ ] 6.2 创建 `src/lib/api.ts`
  - [ ] 封装所有 Wails Go 方法调用

- [ ] 6.3 创建 `src/App.svelte` - 主界面
  - [ ] 页面布局设计
  - [ ] 规则列表容器
  - [ ] 添加规则按钮

- [ ] 6.4 创建 `src/components/RuleForm.svelte`
  - [ ] 表单：本地端口、目标 IP、目标端口
  - [ ] 输入验证（端口范围、IP 格式）
  - [ ] 提交和取消按钮

- [ ] 6.5 创建 `src/components/RuleItem.svelte`
  - [ ] 单条规则显示卡片
  - [ ] 状态指示器（🟢 运行中 / ⚫ 已停止）
  - [ ] 查看日志按钮
  - [ ] 启用/禁用开关
  - [ ] 删除按钮

- [ ] 6.6 创建 `src/components/LogViewer.svelte`
  - [ ] 日志列表显示
  - [ ] 时间戳、来源、流量、状态
  - [ ] 清空日志按钮
  - [ ] 自动滚动到底部

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
