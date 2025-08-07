# Copilot 指令

<!-- 使用此文件为 Copilot 提供工作区特定的自定义指令。更多详情请访问 https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

## 项目概述
这是一个名为 `tpmgr`（TeX 包管理器）的 Rust 命令行工具，用于 LaTeX 包管理，灵感来源于 Python 的 uv 和 TeXLive 的 tlmgr。

## 核心功能
- LaTeX 包依赖管理
- 项目本地包安装
- 包版本解析
- 快速包安装和更新
- 与现有 LaTeX 发行版集成

## 代码风格指南
- 遵循 Rust 最佳实践和习惯用法
- 使用 `clap` 进行命令行参数解析
- 使用 `serde` 进行配置序列化
- 使用 `anyhow` 或 `thiserror` 实现适当的错误处理
- 使用 `tokio` 进行异步网络操作
- 遵循语义化版本控制规范

## 架构说明
- 采用模块化设计，分别包含以下模块：
  - 包解析模块
  - 依赖管理模块
  - 配置处理模块
  - LaTeX 集成模块
  - 网络操作模块
