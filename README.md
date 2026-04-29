# PortHannis

A lightweight cross-platform port forwarding tool with a system tray UI.

## Features

- Single executable, no installation required (~10MB)
- System tray integration — runs in background, click to configure
- Forward local ports to internal network targets
- Per-rule logging with circular buffer (10MB max per rule)
- Cross-platform: Windows, Linux, macOS

## Screenshots

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

## Download

See [Releases](https://github.com/HannisLee/PortHannis/releases) for pre-built binaries.

## Quick Start

1. Download the executable for your platform
2. Run it — the app will appear in your system tray
3. Click the tray icon to open the configuration window
4. Add a forwarding rule: local port → target host + target port
5. Click enable to start forwarding

## Configuration

Config files are stored in:

| Platform | Path |
|----------|------|
| Windows | `%APPDATA%\porthannis\` |
| Linux | `~/.config/porthannis/` |
| macOS | `~/Library/Application Support/porthannis/` |

## Development

### Prerequisites

- [Go 1.21+](https://go.dev/dl/)
- [Wails CLI v2](https://wails.io/docs/gettingstarted/installation)
- [Node.js](https://nodejs.org/) (for frontend tooling)

### Setup

```bash
# Clone the repository
git clone https://github.com/HannisLee/PortHannis.git
cd PortHannis

# Install frontend dependencies
cd frontend && npm install && cd ..

# Run in development mode
wails dev

# Build for production
wails build
```

### Cross-Platform Build

```bash
wails build -platform windows/amd64
wails build -platform linux/amd64
wails build -platform darwin/amd64
wails build -platform darwin/arm64
```

## Tech Stack

| Component | Choice | Reason |
|-----------|--------|--------|
| Backend | Go | Cross-platform, single binary, excellent networking |
| UI Framework | Wails v2 | Native performance, system WebView, small binary |
| Frontend | Svelte + TypeScript | Lightweight, fast compilation |
| Config Storage | JSON | Simple, human-readable |
| Logging | Circular buffer files | Bounded disk usage, per-rule isolation |

## License

MIT
