# portcli

`portcli` 是一个用 Rust 编写的 TCP 端口转发命令行工具。它通过 CLI 管理转发规则，通过后台守护进程执行实际转发，适合把本机端口映射到局域网、远端机器或本机开发服务。

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/HannisLee/PortCLI/actions/workflows/ci.yml/badge.svg)](https://github.com/HannisLee/PortCLI/actions/workflows/ci.yml)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20windows-blue.svg)](#)

## 功能特性

- 用命令添加、修改、删除、启用和禁用 TCP 转发规则
- 后台守护进程读取配置并启动所有已启用规则
- 支持 `status`、`stop`、`reload` 等本机控制命令
- 守护进程日志和每条规则的转发日志分开保存
- 遵循 Linux XDG 和 Windows AppData 目录规范
- 普通高位端口通常不需要 root 或管理员权限

## 安装

### 一行安装 Linux x86_64

在 Linux x86_64 上可以用安装脚本下载最新 GitHub Release：

```bash
curl -fsSL https://raw.githubusercontent.com/HannisLee/PortCLI/main/install.sh | sh
```

脚本默认行为：

- 默认仓库：`HannisLee/PortCLI`
- 默认版本：最新 release
- 默认安装目录：`$HOME/.local/bin`
- 默认二进制：`$HOME/.local/bin/portcli`
- 支持平台：Linux x86_64

安装后验证：

```bash
portcli --version
```

如果提示 `portcli: command not found`，说明安装目录不在 `PATH` 中。把下面这一行加入你的 shell 配置文件，例如 `~/.bashrc`、`~/.zshrc` 或对应 profile：

```bash
export PATH="$HOME/.local/bin:$PATH"
```

然后重新打开终端，或执行：

```bash
source ~/.bashrc
```

### 指定版本或安装目录

安装指定版本：

```bash
curl -fsSL https://raw.githubusercontent.com/HannisLee/PortCLI/main/install.sh | PORTCLI_VERSION=<version> sh
```

`PORTCLI_VERSION` 可以写 `0.4.3` 或 `v0.4.3`，脚本会自动兼容。

安装到指定目录：

```bash
curl -fsSL https://raw.githubusercontent.com/HannisLee/PortCLI/main/install.sh | PORTCLI_INSTALL_DIR="$HOME/bin" sh
```

同时指定版本和目录：

```bash
curl -fsSL https://raw.githubusercontent.com/HannisLee/PortCLI/main/install.sh | PORTCLI_VERSION=<version> PORTCLI_INSTALL_DIR="$HOME/bin" sh
```

### 使用自定义下载地址

如果你已经下载了 release 压缩包，或需要从镜像地址安装，可以指定 `PORTCLI_DOWNLOAD_URL`：

```bash
curl -fsSL https://raw.githubusercontent.com/HannisLee/PortCLI/main/install.sh | PORTCLI_DOWNLOAD_URL="https://example.com/portcli.tar.gz" sh
```

该压缩包中必须包含名为 `portcli` 的可执行文件。

### 手动安装 Linux

从 [GitHub Releases](https://github.com/HannisLee/PortCLI/releases) 下载对应版本的 Linux 静态链接压缩包：

```bash
wget https://github.com/HannisLee/PortCLI/releases/download/v<version>/portcli-v<version>-x86_64-unknown-linux-musl.tar.gz
tar -xzf portcli-v<version>-x86_64-unknown-linux-musl.tar.gz
install -m 0755 portcli "$HOME/.local/bin/portcli"
portcli --version
```

musl 版本通常可以在 Ubuntu、Debian、CentOS、Rocky、Alpine 等 Linux 发行版上运行。

### 手动安装 Windows

1. 从 [GitHub Releases](https://github.com/HannisLee/PortCLI/releases) 下载 `portcli-v<version>-x86_64-pc-windows-msvc.zip`。
2. 解压 `portcli.exe` 到固定目录，例如 `C:\Tools\portcli\`。
3. 将该目录加入系统 `PATH`。
4. 打开 PowerShell 验证：

```powershell
portcli --version
```

### 从源码编译

需要 Rust 1.70 或更高版本。

```bash
git clone https://github.com/HannisLee/PortCLI.git
cd PortCLI
cargo build --release
```

编译产物位于：

- Linux：`target/release/portcli`
- Windows：`target\release\portcli.exe`

## 快速开始

下面的例子把本机 `127.0.0.1:9999` 转发到本机已有服务 `127.0.0.1:8080`。

```bash
portcli add local-web --source 127.0.0.1:9999 --target 127.0.0.1:8080
portcli enable local-web
portcli run
portcli status
curl http://127.0.0.1:9999
```

停止转发：

```bash
portcli disable local-web
portcli stop
```

规则添加后默认是禁用状态，必须执行 `portcli enable <name>` 后，守护进程才会启动这条规则。

## 常用场景

### 暴露本机开发服务到局域网

假设本机开发服务监听在 `127.0.0.1:3000`，希望局域网设备访问你的电脑 `8080` 端口：

```bash
portcli add dev-web --source 0.0.0.0:8080 --target 127.0.0.1:3000
portcli enable dev-web
portcli run
```

局域网设备访问：

```text
http://你的电脑局域网IP:8080
```

如果端口或目标服务变了，可以直接修改规则：

```bash
portcli modify dev-web --source 0.0.0.0:8081
portcli modify dev-web --target 127.0.0.1:5173
```

守护进程运行时，`modify` 会自动触发配置重载。

### 转发到局域网内其他机器

例如把本机 `13000` 端口转发到局域网机器 `10.10.2.45:3000`：

```bash
portcli add gpu-web --source 0.0.0.0:13000 --target 10.10.2.45:3000
portcli enable gpu-web
portcli run
portcli status
```

本机访问：

```bash
curl http://127.0.0.1:13000
```

局域网设备访问：

```text
http://你的电脑局域网IP:13000
```

### 转发 SSH

```bash
portcli add ssh-box --source 127.0.0.1:2222 --target 192.168.31.20:22
portcli enable ssh-box
portcli run
ssh -p 2222 user@127.0.0.1
```

只允许本机访问时，`--source` 使用 `127.0.0.1:<端口>`；需要局域网访问时，使用 `0.0.0.0:<端口>`。

### 同时管理多条规则

```bash
portcli add web --source 0.0.0.0:8080 --target 192.168.31.10:80
portcli add api --source 0.0.0.0:9000 --target 192.168.31.11:9000
portcli add ssh --source 127.0.0.1:2222 --target 192.168.31.20:22

portcli enable web
portcli enable api
portcli enable ssh
portcli run

portcli list
portcli status
```

临时停掉一条规则：

```bash
portcli disable api
```

彻底删除规则：

```bash
portcli remove api
```

## 命令参考

### 全局选项

| 用法 | 说明 |
| --- | --- |
| `portcli --version` / `portcli -V` | 显示当前版本号 |
| `portcli --help` / `portcli -h` | 显示顶层帮助、常用示例和子命令列表 |
| `portcli <command> --help` | 查看子命令说明和示例 |

### 规则管理

| 命令 | 说明 |
| --- | --- |
| `portcli list` | 列出所有规则，守护进程运行时尽量显示运行状态 |
| `portcli add <name> --source <addr> --target <addr>` | 添加规则，新规则默认禁用 |
| `portcli modify <name> [--source <addr>] [--target <addr>]` | 修改规则，至少传一个字段 |
| `portcli remove <name>` | 删除规则 |
| `portcli enable <name>` | 启用规则 |
| `portcli disable <name>` | 禁用规则 |

`<addr>` 必须是 `host:port` 格式，例如 `127.0.0.1:9999`、`0.0.0.0:8080`、`192.168.31.10:80`。

### 守护进程

| 命令 | 说明 |
| --- | --- |
| `portcli run` | 后台启动守护进程 |
| `portcli run --foreground` | 前台运行，适合调试端口占用或启动失败 |
| `portcli status` | 查看守护进程和规则状态 |
| `portcli stop` | 停止守护进程 |
| `portcli reload` | 手动通知守护进程重新读取配置 |

`enable`、`disable`、`modify`、`remove` 在守护进程运行时会自动触发 `reload`。只有手动编辑配置文件后，才通常需要执行 `portcli reload`。

### 日志

| 用法 | 说明 |
| --- | --- |
| `portcli logs` | 查看守护进程日志最后 100 行 |
| `portcli logs <name>` | 查看指定规则日志最后 100 行 |
| `portcli logs -n 20` | 查看守护进程日志最后 20 行 |
| `portcli logs web -n 20` | 查看 `web` 规则日志最后 20 行 |
| `portcli logs web -f` | 持续跟踪 `web` 规则日志 |
| `portcli logs --dir` | 输出日志目录 |
| `portcli logs web --clear` | 清空 `web` 规则日志 |
| `portcli logs --clear` | 清空守护进程日志 |

## 配置和数据目录

### 配置文件

| 平台 | 路径 |
| --- | --- |
| Linux | `~/.config/portcli/config.toml` |
| Windows | `%APPDATA%\portcli\config\config.toml` |

配置示例：

```toml
[[rules]]
name = "web"
source = "0.0.0.0:8080"
target = "192.168.31.10:80"
enabled = true

[[rules]]
name = "ssh"
source = "127.0.0.1:2222"
target = "192.168.31.20:22"
enabled = false
```

### 日志和运行时状态

| 平台 | 路径 |
| --- | --- |
| Linux 日志 | `~/.local/share/portcli/logs/` |
| Linux 状态文件 | `~/.local/share/portcli/state.json` |
| Windows 日志 | `%LOCALAPPDATA%\portcli\data\logs\` |
| Windows 状态文件 | `%LOCALAPPDATA%\portcli\data\state.json` |

日志目录结构：

```text
logs/
├── daemon.log
└── rules/
    ├── web.log
    └── ssh.log
```

`state.json` 保存守护进程 PID、本机控制端口和随机 token。守护进程停止时会自动删除该文件。

## 工作机制

```text
CLI
 │
 ├── 读写配置 ───────────────► config.toml
 │
 ├── 控制命令 ───────────────► Daemon
 │    (TCP JSON, 127.0.0.1)    │
 │                             ├── Control Server
 │                             ├── Forward Tasks
 │                             └── Log Writer
 │
 └── 查看日志 ───────────────► log files
```

1. CLI 把规则保存到 TOML 配置文件。
2. `portcli run` 启动守护进程。
3. 守护进程读取配置，为所有 `enabled = true` 的规则启动 TCP listener。
4. 入站连接进入 `source` 后，守护进程连接 `target` 并进行双向复制。
5. CLI 的 `status`、`stop`、`reload` 通过本机 TCP JSON 控制协议发送给守护进程。
6. 守护进程日志和规则日志分别写入数据目录。

## 排障

### 安装后找不到命令

确认二进制是否已经安装：

```bash
ls -l "$HOME/.local/bin/portcli"
```

如果文件存在，把 `$HOME/.local/bin` 加入 `PATH`：

```bash
export PATH="$HOME/.local/bin:$PATH"
portcli --version
```

### 端口被占用

如果规则状态是 `failed`，并且错误类似 `Address already in use`，说明 `--source` 端口已经被其他进程占用。

```bash
portcli status
portcli modify web --source 0.0.0.0:8081
```

### 目标服务连不上

如果规则能启动，但访问转发端口失败，检查规则日志：

```bash
portcli logs web -n 50
```

如果出现 `connect target failed`，说明守护进程无法连接 `--target` 指向的服务。

### 守护进程状态异常

```bash
portcli status
portcli stop
portcli run
```

如果手动删除或修改了状态文件，也可以直接重新启动守护进程。

## 当前限制

- 仅支持 TCP，不支持 UDP
- 不内置 TLS
- 未集成 systemd 或 Windows Service
- 没有图形界面
- 手动编辑配置后需要执行 `portcli reload`
- 控制协议只监听本机回环地址，并使用本地状态文件中的随机 token 认证
- 日志跟踪采用 500ms 轮询

## 测试

运行 Rust 测试：

```bash
cargo test
```

当前仓库保留了 Linux 验证脚本：

- [test_linux.sh](test_linux.sh)
- [test_linux_forward.sh](test_linux_forward.sh)

这些脚本可能访问或清理用户 home 下的 `~/.config/portcli` 和 `~/.local/share/portcli`，运行前请确认影响。
