# tpmgr - 现代 LaTeX 包管理工具 🚀

*[English](README.md) | [文档](docs/) | [示例](examples/)*

使用 Rust 开发的现代 LaTeX 包管理工具，旨在提供项目级的包管理，避免在全局预装过多的包，并减少手动配置依赖包的麻烦。

## ✨ 核心特性

### 📦 核心功能

- **🔍 自动包检测**: 通过正则表达式模式和编译错误检测缺失的 LaTeX 包
- **🎯 智能安装**: 自动安装缺失包，支持全局和项目级安装
- **⚙️ 编译链支持**: 多步骤编译过程（LaTeX → BibTeX → LaTeX）
- **🪄 魔术变量**: 使用 `${PROJECT_ROOT}`, `${CURRENT_DIR}`, `${HOME}` 实现项目可移植性

### 🔧 高级特性

- **🚀 自动配置**: 首次运行时自动检测并设置 TeXLive 路径和最优镜像
- **🌐 镜像管理**: 内置 CTAN 镜像，自动选择最快镜像
- **⚙️ 配置管理**: 全局和项目级配置，支持继承和覆盖
- **🔗 TeXLive 集成**: 与 TeXLive 完美集成，支持 tlmgr 协作
- **📚 多文档支持**: 复杂项目结构和多文档编译
- **🎯 环境隔离**: 项目级包管理，不污染系统环境

## 📥 安装

### Windows 用户

**方式一：远程安装（推荐）**

```powershell
# 一键安装最新版本
iwr -useb https://raw.githubusercontent.com/jiaojiaodubai/tpmgr/master/install-remote.ps1 | iex

# 或下载后运行，支持更多选项
curl -o install-remote.ps1 https://raw.githubusercontent.com/jiaojiaodubai/tpmgr/master/install-remote.ps1
.\install-remote.ps1 -InstallerType "nsis"    # 使用 NSIS 安装程序
.\install-remote.ps1 -InstallerType "inno"    # 使用 Inno Setup 安装程序
.\install-remote.ps1 -InstallerType "portable" # 使用便携版
.\install-remote.ps1 -Help                    # 显示所有选项
```

**方式二：手动下载**

1. 前往 [Releases 页面](https://github.com/jiaojiaodubai/tpmgr/releases)
2. 下载以下任一安装包：
   - `tpmgr-x.x.x-installer.exe` - NSIS 安装程序（体积小，中英双语）
   - `tpmgr-x.x.x-setup.exe` - Inno Setup 安装程序（专业版，支持三种语言）
   - `tpmgr-x.x.x-portable.zip` - 便携版（无需安装）
3. 运行安装程序或解压便携版
4. 重启终端即可使用 `tpmgr` 命令

**方式三：从源码构建**

```powershell
# 克隆仓库并构建
git clone https://github.com/jiaojiaodubai/tpmgr.git
cd tpmgr
cd build
.\build-all.ps1

# 构建的安装包位于 dist/ 目录
```
.\scripts\install.ps1
```

### macOS 用户

```bash
# 从 GitHub Releases 下载二进制文件
curl -L https://github.com/jiaojiaodubai/tpmgr/releases/latest/download/tpmgr-macos.tar.gz | tar xz
cd tpmgr-*-macos
./install.sh
```

### Linux 用户

```bash
# 从 GitHub Releases 下载二进制文件  
curl -L https://github.com/jiaojiaodubai/tpmgr/releases/latest/download/tpmgr-linux.tar.gz | tar xz
cd tpmgr-*-linux
./install.sh
```

### 从源码安装（所有平台）

```bash
# 安装 Rust（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 从源码安装
cargo install --git https://github.com/jiaojiaodubai/tpmgr.git

# 或克隆并构建
git clone https://github.com/jiaojiaodubai/tpmgr.git
cd tpmgr
cargo build --release
cargo install --path .
```

### 卸载

**Windows（MSI版本）：**

- 通过"设置 > 应用和功能"或"控制面板 > 程序和功能"卸载

**Windows（便携版）：**

```powershell
# 运行便携版附带的卸载脚本
.\uninstall.bat
```

**其他平台：**

```bash
# 如果通过 cargo install 安装
cargo uninstall tpmgr

# 手动删除二进制文件（需要根据实际安装位置调整）
sudo rm /usr/local/bin/tpmgr
# 或
rm ~/.local/bin/tpmgr
```

## 🚀 快速开始

### 首次运行自动配置

首次运行tpmgr时，会自动：

- 🔍 **检测您的TeXLive安装** 并将路径保存到全局配置
- 🌐 **测试可用镜像** 并为您的位置选择最快的镜像
- 💾 **全局保存这些设置** 使所有未来项目都能受益于最优配置

### 初始化新的 LaTeX 项目

```bash
tpmgr init my-paper
cd my-paper
```

### 安装包

```bash
# 安装指定包
tpmgr install amsmath geometry hyperref

# 全局安装包
tpmgr install --global tikz pgfplots

# 自动安装缺失包（扫描当前目录）
tpmgr install

# 使用编译检测自动安装
tpmgr install --compile

# 为指定文件自动安装
tpmgr install --path main.tex
```

### 搜索包

```bash
tpmgr search "math"
tpmgr search "graphics"
```

### 列出已安装包

```bash
# 列出本地包
tpmgr list

# 列出全局包
tpmgr list --global
```

### 更新包

```bash
# 更新所有包
tpmgr update

# 更新指定包
tpmgr update amsmath geometry
```

### 删除包

```bash
tpmgr remove old-package
```

### 获取包信息

```bash
tpmgr info tikz
```

### 清理缓存

```bash
tpmgr clean
```

### 镜像管理

```bash
# 列出可用镜像
tpmgr mirror list

# 自动选择最快镜像
tpmgr mirror use --auto

# 手动选择镜像
tpmgr mirror use "镜像名称"
```

### 依赖分析

```bash
# 分析当前项目依赖
tpmgr analyze

# 分析指定文件
tpmgr analyze --path main.tex

# 使用编译检测进行分析
tpmgr analyze --compile

# 显示详细分析
tpmgr analyze --verbose
```

### 编译

```bash
# 执行配置的编译链
tpmgr compile

# 编译指定文件或路径
tpmgr compile --path main.tex

# 编译后自动清理中间文件
tpmgr compile --clean

# 显示详细编译输出
tpmgr compile --verbose

# 组合选项
tpmgr compile --path src/paper.tex --clean --verbose
```

tpmgr 将包安装在项目的 `packages/` 目录中。为了确保编译引擎找到这些包，需要在执行编译命令之前设置 `TEXINPUTS` 环境变量。

#### 调用 `tpmgr compile`（推荐）

你可以直接在第三方工具中调用 `tpmgr compile`：

1. 在 `tpmgr.toml` 中配置编译命令：

   ```toml
   [[project.compile.steps]]
   tool = "xelatex"  # 或你偏好的引擎
   args = ["-interaction=nonstopmode", "${PROJECT_ROOT}/main.tex"]
   ```

2. 运行带自动包检测的编译：

   ```bash
   tpmgr compile
   ```

上面的构建步骤中，我们用到了 `${PROJECT_ROOT}` 这个魔术变量，它会被替换为当前项目的根目录。关于魔术变量，请参考[配置管理](#配置管理)

#### 手动设置环境变量

Windows (PowerShell):

```powershell
$env:TEXINPUTS = ".\packages\;$env:TEXINPUTS"

pdflatex main.tex
```

Linux/macOS (Bash):

```bash
export TEXINPUTS="./packages/:$TEXINPUTS"

pdflatex main.tex
```

### 配置管理

```bash
# 显示当前配置
tpmgr config show

# 设置配置值
tpmgr config set compile "xelatex -interaction=nonstopmode ${PROJECT_ROOT}/main.tex"
tpmgr config set install_global true

# 获取特定配置值
tpmgr config get compile

# 列出所有配置键
tpmgr config list

# 重置为默认值
tpmgr config reset
```

## 📁 项目结构

使用 `tpmgr init` 初始化项目时创建：

```txt
my-project/
├── tpmgr.toml          # 项目配置文件
├── main.tex            # 主 LaTeX 文档
└── packages/           # 本地包安装目录
```

## ⚙️ 配置

tpmgr 支持全局和项目级配置：

- **全局配置**: 使用 `tpmgr config set --global <key> <value>` 设置。这些设置在创建新项目时作为默认值应用。
- **项目配置**: 存储在 `tpmgr.toml` 文件中。项目设置会覆盖全局设置。
- **配置继承**: 使用 `tpmgr init` 创建的新项目会自动继承全局配置设置作为初始默认值。

`tpmgr.toml` 文件包含项目配置：

```toml
[project]
name = "my-paper"
version = "0.1.0"
package_dir = "packages"

# 编译配置
[project.compile]
auto_clean = true  # 编译后自动清理中间文件

# 自定义清理模式（可选，支持 * 和 ** 通配符）
clean_patterns = [
    "*.aux",
    "*.log", 
    "*.out",
    "*.toc",
    "*.lot",
    "*.lof",
    "*.nav",
    "*.snm",
    "*.vrb",
    "*.bbl",
    "*.blg",
    "*.idx",
    "*.ind",
    "*.ilg",
    "*.glo",
    "*.gls",
    "*.ist",
    "*.fls",
    "*.fdb_latexmk",
    "*.synctex.gz",
    "*.synctex(busy)",
    "*.pdfsync",
    "*.figlist",
    "*.makefile",
    "*.figlist.bak",
    "*.makefile.bak",
    "*.thm",
    "*.pyg",
    "*.auxlock",
    "*.bcf",
    "*.run.xml",
    "src/**/*.aux",  # 递归清理 src 目录中的 aux 文件
    "build/*.tmp"    # 清理 build 目录中的临时文件
]

# 多步骤编译链
[[project.compile.steps]]
tool = "pdflatex"
args = ["-interaction=nonstopmode", "${PROJECT_ROOT}/main.tex"]

[[project.compile.steps]]
tool = "bibtex"
args = ["${PROJECT_ROOT}/main.aux"]

[[project.compile.steps]]
tool = "pdflatex" 
args = ["-interaction=nonstopmode", "${PROJECT_ROOT}/main.tex"]

[dependencies]
amsmath = "2.17"
geometry = "5.9"

[[repositories]]
name = "ctan"
url = "https://ctan.org/"
priority = 1

[[repositories]]
name = "texlive"
url = "https://mirror.ctan.org/systems/texlive/tlnet/"
priority = 2
```

编译链中可能用到的魔术变量说明如下：

- `${PROJECT_ROOT}`: 项目根目录
- `${CURRENT_DIR}`: 当前执行目录
- `${HOME}`: 用户主目录

使用魔法变量确保了项目在分发时具有良好的可迁移性，避免频繁修改构建命令。

## 📋 命令参考

### `tpmgr init [NAME]`

初始化带包管理的新 LaTeX 项目。如果没有提供`NAME`，则将当前目录视为项目根目录，对其进行管理。

### `tpmgr install [PACKAGES]...`

安装一个或多个包。如果未指定包，自动检测当前项目的依赖关系并安装所有缺失的包。默认安装为项目级的包，该行为可以通过 `tpmgr config set install_global = true` 设置为默认全局安装。

- `--global, -g`: 全局安装
- `--path, -p`: 仅为指定的文件添加依赖
- `--compile, -c`: 使用编译模式来检测缺失的包

### `tpmgr remove <PACKAGES>...`

删除一个或多个（项目级的）包。如果未指定包，删除所有项目级的包。

- `--global, -g`: 在全局中删除包

### `tpmgr update [PACKAGES]...`

更新一个或多个包。如果未指定包，更新所有包。

### `tpmgr list`

列出（当前项目）已安装的包。

- `--global, -g`: 列出全局包

### `tpmgr search <QUERY>`

搜索匹配查询的包。

### `tpmgr info <PACKAGE>`

显示包的详细信息。

### `tpmgr analyze [PATH]`

分析 TeX 文件依赖。

- `--path, -p`: TeX 文件或项目目录路径
- `--verbose, -v`: 显示详细依赖信息
- `--compile, -c`: 使用编译模式来检测缺失的包

### `tpmgr compile [PATH]`

按照配置的编译链编译 TeX 文件。

- `--path, -p`: TeX 文件或项目目录路径
- `--clean, -c`: 编译后清理中间文件
- `--verbose, -v`: 显示详细编译输出

### `tpmgr config <ACTION>`

配置管理。

- `show`: 显示当前配置
  - `--global, -g`: 仅显示全局配置
- `set <KEY> <VALUE>`: 设置配置值
  - `--global, -g`: 设置全局配置（应用于新项目）
- `get <KEY>`: 获取配置值
  - `--global, -g`: 仅从全局配置获取
- `list`: 列出所有配置键
  - `--global, -g`: 仅显示全局配置键
- `reset`: 重置配置为默认值
  - `--global, -g`: 仅重置全局配置

### `tpmgr mirror <ACTION>`

镜像管理。

- `list`: 列出可用镜像
- `use <NAME>`: 按名称选择特定镜像
- `use --auto`: 自动选择最快镜像

## 🗺️ 路线图

### 即将推出

- **📦 包管理器发布**: 发布到 Homebrew (macOS)、APT (Ubuntu/Debian)、DNF (Fedora) 等官方包管理器
- **🌐 Web 界面**: 基于 Web 的图形化包管理界面
- **🔗 IDE 集成**: VS Code、TeXstudio 等编辑器的扩展插件
- **📊 依赖可视化**: 图形化显示包依赖关系
- **🚀 性能优化**: 更快的包解析和下载速度
- **🌍 国际化**: 支持更多语言界面

### 长期规划

- **☁️ 云同步**: 项目配置和包列表云端同步
- **🏢 企业版**: 私有包仓库和团队协作功能
- **🤖 AI 助手**: 智能包推荐和文档生成
- **📱 移动端**: 移动设备上的 LaTeX 编辑和预览

## 🏗️ 架构设计

tpmgr 采用模块化设计，注重性能和易用性：

- **⚡ 快速依赖解析**: 高效的包依赖解析算法
- **🔄 并行下载**: 支持多个包同时下载
- **📈 增量更新**: 仅下载变更内容
- **🔒 包完整性验证**: 校验和验证确保包完整性
- **🌐 多仓库支持**: 支持 CTAN、TeXLive 和自定义仓库
- **🎯 环境隔离**: 使用 TEXINPUTS 环境变量，不污染系统环境

## 🎓 编辑器中的手动编译与包检测

如果您更喜欢在编辑器中执行编译，同时仍然使用 tpmgr 的包管理功能，您需要配置 LaTeX 引擎以找到项目中安装的包。

### 设置 TEXINPUTS 环境变量

tpmgr 将包安装在项目的 `packages/` 目录中。要让 LaTeX 引擎找到这些包，您需要设置 `TEXINPUTS` 环境变量：

#### Windows (PowerShell)

```powershell
$env:TEXINPUTS = ".\packages\;$env:TEXINPUTS"
pdflatex main.tex
```

#### Linux/macOS (Bash)

```bash
export TEXINPUTS="./packages/:$TEXINPUTS"
pdflatex main.tex
```

### 自动化设置

您也可以使用 `tpmgr compile` 来自动设置环境并运行自定义编译命令：

1. 在 `tpmgr.toml` 中配置编译命令：

   ```toml
   [[project.compile.steps]]
   tool = "xelatex"  # 或您偏好的引擎
   args = ["-interaction=nonstopmode", "${PROJECT_ROOT}/main.tex"]
   ```

2. 运行编译并自动检测包：

   ```bash
   tpmgr compile
   ```

这种方法确保：

- 自动配置 `TEXINPUTS` 路径
- LaTeX 引擎能找到项目包
- 可以使用任何您偏好的 LaTeX 引擎
- 包管理保持项目级隔离

## 📊 与其他工具对比

| 特性 | tpmgr | tlmgr | 手动管理 |
|---------|-------|-------|--------|
| 速度 | ⚡ 快速 | 🐌 缓慢 | 😴 极慢 |
| 项目级包管理 | ✅ 是 | ❌ 否 | ❌ 否 |
| 自动依赖解析 | ✅ 自动 | ⚠️ 手动 | ❌ 手动 |
| 多仓库支持 | ✅ 是 | ⚠️ 有限 | ❌ 否 |
| 跨平台支持 | ✅ 是 | ⚠️ 有限 | ✅ 是 |
| 编译链支持 | ✅ 高级 | ❌ 否 | ❌ 否 |
| 魔术变量 | ✅ 是 | ❌ 否 | ❌ 否 |
| 环境隔离 | ✅ 是 | ❌ 否 | ❌ 否 |

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。

### 开发环境设置

#### 构建要求

- Rust 1.70+
- Cargo

#### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/username/tpmgr.git
cd tpmgr

# 构建调试版本
cargo build

# 构建发布版本
cargo build --release

# 运行测试
cargo test

# 运行示例测试
cd examples
.\test_examples.ps1  # Windows
./test_examples.sh   # Linux/macOS
```

#### 项目结构

```text
tpmgr/
├── src/                    # 源代码
│   ├── main.rs            # 主程序入口
│   ├── commands.rs        # 命令实现
│   ├── package.rs         # 包管理核心
│   ├── config.rs          # 配置管理
│   ├── tex_parser.rs      # TeX 文件解析
│   ├── texlive.rs         # TeXLive 集成
│   └── mirror.rs          # 镜像管理
├── examples/               # 测试示例
│   ├── basic-project/     # 基础项目测试
│   ├── multi-package-test/# 多包测试
│   ├── complex-compile-chain/ # 复杂编译链测试
│   ├── presentation/      # 演示文档测试
│   ├── test_examples.ps1  # Windows 测试脚本
│   └── test_examples.sh   # Linux/macOS 测试脚本
├── docs/                   # 文档目录
├── Cargo.toml             # Rust 项目配置
├── README.md              # 英文文档
└── README_zh.md           # 中文文档
```

## 📄 许可证

本项目基于 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。
