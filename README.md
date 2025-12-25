# RPanel

RPanel 是一款基于 Rust 编写的高性能 Docker 集群管理工具。它采用 Controller-Agent 架构，支持多主机 Docker 环境的统一管理和监控。

## ✨ 功能特性

*   **多主机管理**: 通过 Agent 自动注册，轻松管理多个服务器上的 Docker 环境。
*   **实时监控**: 实时采集并展示服务器的 CPU、内存、磁盘使用率及在线状态。
*   **Web 面板**: 提供直观的 React Web 界面，支持中英文一键切换。
*   **高性能**: 后端采用 Rust (Tokio + Tonic + Axum) 开发，资源占用极低。
*   **易于部署**: 简单的配置和单一二进制文件部署。

## 🏗 架构说明

系统由两个核心组件组成：

*   **rpanel-controller (服务端)**
    *   提供 Web 访问接口 (Port: 5666)
    *   提供 gRPC 服务供 Agent 连接 (Port: 15666)
    *   使用 SQLite 存储数据
    *   包含前端静态资源服务

*   **rpanel-agent (客户端)**
    *   部署在目标 Docker 主机上
    *   采集系统监控数据 (CPU/Mem/Disk)
    *   与本地 Docker Daemon 通信 (默认 localhost:2375)
    *   通过 gRPC 上报数据到 Controller

## 🚀 快速开始

### 1. 编译项目

你需要安装 [Rust](https://www.rust-lang.org/) 和 [Node.js](https://nodejs.org/) 环境。

```bash
# 编译前端
cd rpanel-web
bun install
bun run build
cd ..

# 编译后端 (Controller & Agent)
cargo build --release
```

编译完成后：
*   前端资源位于 `dist/`
*   后端二进制文件位于 `target/release/`

### 2. 运行服务端 (Controller)

首先确保 `dist` 目录位于运行目录或正确配置了静态资源路径。

```bash
# 首次运行会自动生成配置文件 config/controller.toml
./target/release/rpanel-controller
```

*   **Web 面板**: http://127.0.0.1:5666
*   **gRPC 端口**: 15666

### 3. 运行客户端 (Agent)

在需要管理的目标机器上运行 Agent。

```bash
# 首次运行会自动生成配置文件 config/agent.toml
./target/release/rpanel-agent
```

## ⚙️ 配置说明

配置文件默认位于 `config/` 目录下。

### Controller 配置 (`config/controller.toml`)

```toml
port = 15666  # gRPC 监听端口
```

### Agent 配置 (`config/agent.toml`)

```toml
# Agent 唯一标识 (首次运行自动生成)
id = "uuid-..." 

# 本地 Docker Daemon 地址
docker = "http://localhost:2375" 

# Controller 服务端地址
controller = "http://localhost:15666" 
```

## 📂 项目结构

*   `rpanel-controller`: 控制端后端 (Axum + Tonic + SeaORM)
*   `rpanel-agent`: 代理端 (Tonic + SystemStat)
*   `rpanel-web`: 前端面板 (React + Vite + TypeScript)
*   `rpanel-common`: 公共库
*   `rpanel-grpc`: gRPC Proto 定义
*   `migration`: 数据库迁移脚本

## 📝 开发指南

1.  **启动前端开发服务器**:
    ```bash
    cd rpanel-web
    bun run dev
    ```

2.  **启动后端**:
    ```bash
    cargo run -p rpanel-controller
    cargo run -p rpanel-agent
    ```

## 📄 License

Apache License