# tpmgr Examples 测试指南

*[English](README.md)*

本目录包含用于测试 `tpmgr` 功能的示例项目和自动化测试脚本。

## 📁 测试项目

### 1. basic-project

**描述**: 基础 LaTeX 项目，测试基本包安装功能。  
**用到的包**: `geometry`, `graphicx`, `lipsum`  
**特点**: 简单的文档结构，适合测试基础功能。

### 2. multi-package-test

**描述**: 多包测试项目，包含数学、颜色、代码高亮等功能。  
**用到的包**: `amsmath`, `amsfonts`, `amssymb`, `xcolor`, `listings`, `hyperref`  
**特点**: 测试多种类型包的检测和安装。

### 3. complex-compile-chain

**描述**: 复杂编译链项目，支持 BibTeX 引用。  
**用到的包**: `amsmath`, `natbib`, `hyperref`  
**特点**: 多步编译流程，测试引用和编译链功能。

### 4. presentation

**描述**: Beamer 演示文稿项目。  
**用到的包**: `tikz`, `subcaption`  
**特点**: 测试 Beamer 文档类和图形包。

## 🚀 测试脚本

### PowerShell 版本 (`test_examples.ps1`)

适用于 Windows 系统，功能最完整。

#### 基本用法

```powershell
# 测试所有项目
.\test_examples.ps1

# 测试单个项目
.\test_examples.ps1 -ProjectName basic-project

# 详细模式
.\test_examples.ps1 -ProjectName basic-project -Verbose

# 跳过清理步骤
.\test_examples.ps1 -ProjectName basic-project -SkipClean

# 仅测试编译（跳过包安装测试）
.\test_examples.ps1 -ProjectName basic-project -CompileOnly
```

#### 参数说明

- `-ProjectName`: 指定要测试的项目名称（可选）
- `-Verbose`: 显示详细输出
- `-SkipClean`: 跳过项目重置步骤
- `-CompileOnly`: 仅测试编译功能

### Bash 版本 (`test_examples.sh`)

适用于 Linux/macOS 系统，功能与 PowerShell 版本相同。

#### 基本用法

```bash
# 给脚本执行权限
chmod +x test_examples.sh

# 测试所有项目
./test_examples.sh

# 测试单个项目
./test_examples.sh basic-project

# 详细模式
./test_examples.sh -v basic-project

# 跳过清理步骤
./test_examples.sh -s basic-project

# 仅测试编译
./test_examples.sh -c basic-project

# 显示帮助
./test_examples.sh -h
```

## 🔄 测试流程

测试脚本会自动执行以下步骤：

### 1. 项目重置

- 删除所有生成的文件（PDF、辅助文件等）
- 保留源文件（`.tex`, `.bib`, `.cls`, `.sty`, `.md`）
- 删除配置文件 (`tpmgr.toml`)

### 2. 项目初始化

- 运行 `tpmgr init` 在当前目录初始化项目
- 检查配置文件是否正确生成

### 3. 正则表达式包检测测试

- 运行 `tpmgr analyze --verbose` 分析依赖
- 运行 `tpmgr install` 安装缺失包
- 尝试编译项目验证包安装

### 4. 编译错误包检测测试

- 重置项目状态
- 重新初始化项目
- 运行 `tpmgr analyze --compile --verbose` 通过编译错误分析依赖
- 运行 `tpmgr install --compile` 基于编译错误安装包

### 5. 最终编译验证

- 运行 `tpmgr compile --verbose` 编译项目
- 检查是否生成了 PDF 输出文件

## 📋 测试结果解读

### 成功标识

- ✅ 测试步骤成功完成
- 🔄 正在执行的步骤  
- ℹ️ 信息性输出

### 警告和错误

- ⚠️ 警告：步骤可能失败但测试继续
- ❌ 错误：严重问题导致测试停止

### 总结报告

脚本结束时会显示所有项目的测试结果总结。

## 🛠️ 前置条件

1. **构建 tpmgr**:

   ```bash
   cargo build
   ```

2. **安装 LaTeX 发行版**:

   - Windows: TeX Live 或 MiKTeX
   - Linux: TeX Live (`sudo apt install texlive-full`)
   - macOS: MacTeX

3. **确保工具在 PATH 中**:

   - `pdflatex`
   - `bibtex` (用于 complex-compile-chain)

## 🐛 故障排除

### 常见问题

1. **"tpmgr binary not found"**

   - 解决方案: 运行 `cargo build` 构建项目

2. **"pdflatex: command not found"**

   - 解决方案: 安装 LaTeX 发行版并确保在 PATH 中

3. **包安装失败**

   - 某些包可能不在标准仓库中，这是预期行为
   - 脚本会继续测试编译以验证已安装的包

4. **权限错误**

   - Linux/macOS: 确保脚本有执行权限 (`chmod +x test_examples.sh`)
   - Windows: 确保 PowerShell 执行策略允许脚本运行

### 调试技巧

1. **使用详细模式**:

   ```bash
   ./test_examples.sh -v basic-project
   ```

2. **检查临时日志**:

   - `/tmp/tpmgr_*.log` (Linux/macOS)
   - 或在脚本中查看详细输出

3. **手动测试单个项目**:

   ```bash
   cd examples/basic-project
   ../../target/debug/tpmgr init
   ../../target/debug/tpmgr analyze --verbose
   ../../target/debug/tpmgr install
   ../../target/debug/tpmgr compile
   ```

## 📝 添加新测试项目

要添加新的测试项目：

1. 在 `examples/` 目录下创建新文件夹
2. 添加 `.tex` 和其他必要的源文件  
3. 在测试脚本的 `TEST_PROJECTS` 数组中添加项目名称
4. 运行测试验证新项目正常工作

## 🔗 相关文档

- [tpmgr 用户指南](../README.md)
- [配置文件文档](../docs/configuration.md)
- [编译链配置](../docs/compilation.md)
