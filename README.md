# tpmgr - Modern LaTeX Package Manager 🚀

*[中文版](README_zh.md) | [Documentation](docs/) | [Examples](examples/)*

A modern LaTeX package management tool written in Rust, designed to provide project-level package management, avoiding the need to globally pre-install too many packages and reducing the hassle of manually configuring dependency packages.

## ✨ Key Features

### 📦 Core Functionality

- **🔍 Auto Package Detection**: Detect missing LaTeX packages via regex patterns and compilation errors
- **🎯 Smart Installation**: Automatically install missing packages with global and project-level support
- **⚙️ Compile Chain Support**: Multi-step compilation processes (LaTeX → BibTeX → LaTeX)
- **🪄 Magic Variables**: Use `${PROJECT_ROOT}`, `${CURRENT_DIR}`, `${HOME}` for portable projects

### 🔧 Advanced Features

- **🚀 Auto-Configuration**: First-run detection and automatic setup of TeXLive path and optimal mirror
- **🌐 Mirror Management**: Built-in CTAN mirrors with automatic fastest mirror selection
- **⚙️ Configuration Management**: Global and project-level configs with inheritance and override
- **🔗 TeXLive Integration**: Perfect integration with TeXLive, supporting tlmgr collaboration
- **📚 Multi-Document Support**: Complex project structures and multi-document compilation
- **🎯 Environment Isolation**: Project-level package management without polluting system environment

## 📥 Installation

### Windows Users

**Method 1: Remote Installation (Recommended)**
```powershell
# One-click install latest version
iwr -useb https://raw.githubusercontent.com/jiaojiaodubai/tpmgr/master/install-remote.ps1 | iex

# Or download and run with options
curl -o install-remote.ps1 https://raw.githubusercontent.com/jiaojiaodubai/tpmgr/master/install-remote.ps1
.\install-remote.ps1 -InstallerType "inno"    # Use Inno Setup installer
.\install-remote.ps1 -InstallerType "portable" # Use portable version
.\install-remote.ps1 -Help                    # Show all options
```

**Method 2: Manual Download**

1. Go to [Releases page](https://github.com/jiaojiaodubai/tpmgr/releases)
2. Download one of the following:
   - `tpmgr-x.x.x-setup.exe` - Inno Setup installer (professional, 3 languages)
   - `tpmgr-x.x.x-portable.zip` - Portable version (no installation needed)
3. Run the installer or extract the portable version
4. Restart your terminal to use the `tpmgr` command

**Method 3: Build from Source**
```powershell
# Clone and build
git clone https://github.com/jiaojiaodubai/tpmgr.git
cd tpmgr
cd build
.\build-all.ps1

# Built installers will be in dist/ directory
```

### macOS Users

```bash
# Download from GitHub Releases
curl -L https://github.com/jiaojiaodubai/tpmgr/releases/latest/download/tpmgr-macos.tar.gz | tar xz
cd tpmgr-*-macos
./install.sh
```

### Linux Users

```bash
# Download from GitHub Releases
curl -L https://github.com/jiaojiaodubai/tpmgr/releases/latest/download/tpmgr-linux.tar.gz | tar xz
cd tpmgr-*-linux
./install.sh
```

### Build from Source (All Platforms)

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install from source
cargo install --git https://github.com/jiaojiaodubai/tpmgr.git

# Or clone and build
git clone https://github.com/jiaojiaodubai/tpmgr.git
cd tpmgr
cargo build --release
cargo install --path .
```

### Uninstallation

**Windows (MSI version):**
- Through "Settings > Apps & features" or "Control Panel > Programs and Features"

**Windows (Portable version):**
```powershell
# Run the uninstall script included with portable version
.\uninstall.bat
```

**Other platforms:**
```bash
# If installed via cargo install
cargo uninstall tpmgr

# Manual removal (adjust path as needed)
sudo rm /usr/local/bin/tpmgr
# or
rm ~/.local/bin/tpmgr
```

## 🚀 Quick Start

### First Run Auto-Configuration

When you run tpmgr for the first time, it automatically:

- 🔍 **Detects your TeXLive installation** and saves the path to global configuration
- 🌐 **Tests available mirrors** and selects the fastest one for your location
- 💾 **Saves these settings globally** so all future projects benefit from optimal configuration

### Initialize a new LaTeX project

```bash
tpmgr init my-paper
cd my-paper
```

### Install packages

```bash
# Install specific packages
tpmgr install amsmath geometry hyperref

# Install packages globally
tpmgr install --global tikz pgfplots

# Auto-install missing packages (scans current directory)
tpmgr install

# Auto-install using compilation detection
tpmgr install --compile

# Auto-install for specific file
tpmgr install --path main.tex
```

### List installed packages

```bash
# List local packages
tpmgr list

# List global packages
tpmgr list --global
```

### Search for packages

```bash
tpmgr search "math"
tpmgr search "graphics"
```

### Get package information

```bash
tpmgr info tikz
```

### Update packages

```bash
# Update all packages
tpmgr update

# Update specific packages
tpmgr update amsmath geometry
```

### Remove packages

```bash
# Remove project-level packages
tpmgr remove old-package

# Remove global packages
tpmgr remove --global old-package
```

### Clean cache

```bash
tpmgr clean
```

### Dependency Analysis

```bash
# Analyze current project dependencies
tpmgr analyze

# Analyze specific file
tpmgr analyze --path main.tex

# Use compilation detection for analysis
tpmgr analyze --compile

# Show detailed analysis
tpmgr analyze --verbose
```

### Compilation

```bash
# Execute configured compile chain
tpmgr compile

# Compile specific file or path
tpmgr compile --path main.tex

# Auto-clean intermediate files after compilation
tpmgr compile --clean

# Show verbose compilation output
tpmgr compile --verbose

# Combine options
tpmgr compile --path src/paper.tex --clean --verbose
```

### Mirror Management

```bash
# List available mirrors
tpmgr mirror list

# Auto-select fastest mirror
tpmgr mirror use --auto

# Manually select a mirror
tpmgr mirror use "Mirror Name"
```

### Configuration Management

```bash
# Show current configuration
tpmgr config show

# Show global configuration
tpmgr config show --global

# Set configuration values
tpmgr config set compile "xelatex -interaction=nonstopmode ${PROJECT_ROOT}/main.tex"
tpmgr config set install_global true

# Set global configuration
tpmgr config set --global texlive_path "/usr/local/texlive/2024"

# Get specific configuration value
tpmgr config get compile

# List all configuration keys
tpmgr config list

# Reset to default values
tpmgr config reset
```

## 📁 Project Structure

When initializing a project with `tpmgr init`:

```txt
my-project/
├── tpmgr.toml          # Project configuration file
├── main.tex            # Main LaTeX document
└── packages/           # Local package installation directory
```

## ⚙️ Configuration

tpmgr supports both global and project-level configuration:

- **Global Configuration**: Set using `tpmgr config set --global <key> <value>`. These settings are applied as defaults when creating new projects.
- **Project Configuration**: Stored in the `tpmgr.toml` file. Project settings override global settings.
- **Configuration Inheritance**: New projects created with `tpmgr init` automatically inherit global configuration settings as initial defaults.

The `tpmgr.toml` file contains project configuration:

```toml
[project]
name = "my-paper"
version = "0.1.0"
package_dir = "packages"

# Compilation configuration
[project.compile]
auto_clean = true  # Automatically clean intermediate files after compilation

# Custom clean patterns (optional, supports * and ** wildcards)
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
    "src/**/*.aux",  # Recursively clean aux files in src directory
    "build/*.tmp"    # Clean temporary files in build directory
]

# Multi-step compilation chain
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

## 📋 Commands Reference

### `tpmgr init [NAME]`

Initialize a new LaTeX project with package management. If `NAME` is not provided, treats the current directory as the project root and manages it.

### `tpmgr install [PACKAGES]...`

Install one or more packages. If no packages are specified, automatically detects current project dependencies and installs all missing packages. Defaults to project-level installation, but this behavior can be changed by setting `tpmgr config set install_global = true` for default global installation.

- `--global, -g`: Install globally
- `--path, -p`: Add dependencies only for the specified file
- `--compile, -c`: Use compilation mode to detect missing packages

### `tpmgr remove <PACKAGES>...`

Remove one or more (project-level) packages. If no packages are specified, removes all project-level packages.

- `--global, -g`: Remove packages globally

### `tpmgr update [PACKAGES]...`

Update one or more packages. If no packages are specified, updates all packages.

### `tpmgr list`

List installed packages (current project).

- `--global, -g`: List global packages

### `tpmgr search <QUERY>`

Search for packages matching the query.

### `tpmgr info <PACKAGE>`

Display detailed information about a package.

### `tpmgr analyze [PATH]`

Analyze TeX file dependencies.

- `--path, -p`: TeX file or project directory path
- `--verbose, -v`: Show detailed dependency information
- `--compile, -c`: Use compilation mode to detect missing packages

### `tpmgr compile [PATH]`

Compile TeX files according to the configured compilation chain.

- `--path, -p`: TeX file or project directory path
- `--clean, -c`: Clean intermediate files after compilation
- `--verbose, -v`: Show detailed compilation output

### `tpmgr config <ACTION>`

Configuration management.

- `show`: Display current configuration
  - `--global, -g`: Show only global configuration
- `set <KEY> <VALUE>`: Set configuration value
  - `--global, -g`: Set global configuration (applies to new projects)
- `get <KEY>`: Get configuration value
  - `--global, -g`: Get only from global configuration
- `list`: List all configuration keys
  - `--global, -g`: Show only global configuration keys
- `reset`: Reset configuration to default values
  - `--global, -g`: Reset only global configuration

### `tpmgr mirror <ACTION>`

Mirror management.

- `list`: List available mirrors
- `use <NAME>`: Select specific mirror by name
- `use --auto`: Automatically select fastest mirror

## 🗺️ Roadmap

### Coming Soon

- **📦 Package Manager Distribution**: Release to official package managers like Homebrew (macOS), APT (Ubuntu/Debian), DNF (Fedora)
- **🌐 Web Interface**: Web-based graphical package management interface
- **🔗 IDE Integration**: Extensions for VS Code, TeXstudio, and other editors
- **📊 Dependency Visualization**: Graphical display of package dependencies
- **🚀 Performance Optimization**: Faster package resolution and download speeds
- **🌍 Internationalization**: Support for more interface languages

### Long-term Plans

- **☁️ Cloud Sync**: Cloud synchronization of project configurations and package lists
- **🏢 Enterprise Edition**: Private package repositories and team collaboration features
- **🤖 AI Assistant**: Intelligent package recommendations and documentation generation
- **📱 Mobile Support**: LaTeX editing and preview on mobile devices

## 🏗️ Architecture

tpmgr adopts a modular design focused on performance and usability:

- **⚡ Fast Dependency Resolution**: Efficient package dependency resolution algorithms
- **🔄 Parallel Downloads**: Support for concurrent package downloads
- **📈 Incremental Updates**: Download only changed content
- **🔒 Package Integrity Verification**: Checksum verification ensures package integrity
- **🌐 Multi-Repository Support**: Support for CTAN, TeXLive, and custom repositories
- **🎯 Environment Isolation**: Use TEXINPUTS environment variable without polluting system environment

## 🎓 Manual Compilation with Package Detection

If you prefer to execute compilation in your editor while still using tpmgr's package management features, you need to configure the LaTeX engine to find packages installed in the project.

### Setting TEXINPUTS Environment Variable

tpmgr installs packages in the project's `packages/` directory. To let the LaTeX engine find these packages, you need to set the `TEXINPUTS` environment variable:

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

### Automated Setup

You can also use `tpmgr compile` to automatically set up the environment and run custom compilation commands:

1. Configure compilation commands in `tpmgr.toml`:

   ```toml
   [[project.compile.steps]]
   tool = "xelatex"  # or your preferred engine
   args = ["-interaction=nonstopmode", "${PROJECT_ROOT}/main.tex"]
   ```

2. Run compilation with automatic package detection:

   ```bash
   tpmgr compile
   ```

This approach ensures:

- Automatic configuration of `TEXINPUTS` paths
- LaTeX engine can find project packages
- Can use any LaTeX engine you prefer
- Package management maintains project-level isolation

## 📊 Comparison with Other Tools

| Feature | tpmgr | tlmgr | Manual Management |
|---------|-------|-------|--------|
| Speed | ⚡ Fast | 🐌 Slow | 😴 Very Slow |
| Project-level Package Management | ✅ Yes | ❌ No | ❌ No |
| Automatic Dependency Resolution | ✅ Automatic | ⚠️ Manual | ❌ Manual |
| Multi-Repository Support | ✅ Yes | ⚠️ Limited | ❌ No |
| Cross-Platform Support | ✅ Yes | ⚠️ Limited | ✅ Yes |
| Compilation Chain Support | ✅ Advanced | ❌ No | ❌ No |
| Magic Variables | ✅ Yes | ❌ No | ❌ No |
| Environment Isolation | ✅ Yes | ❌ No | ❌ No |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit Pull Requests.

### Development Setup

#### Build Requirements

- Rust 1.70+
- Cargo

#### Building from Source

```bash
# Clone the repository
git clone https://github.com/username/tpmgr.git
cd tpmgr

# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run example tests
cd examples
.\test_examples.ps1  # Windows
./test_examples.sh   # Linux/macOS
```

#### Project Structure

```text
tpmgr/
├── src/                    # Source code
│   ├── main.rs            # Main program entry
│   ├── commands.rs        # Command implementations
│   ├── package.rs         # Package management core
│   ├── config.rs          # Configuration management
│   ├── tex_parser.rs      # TeX file parsing
│   ├── texlive.rs         # TeXLive integration
│   └── mirror.rs          # Mirror management
├── examples/               # Test examples
│   ├── basic-project/     # Basic project test
│   ├── multi-package-test/# Multi-package test
│   ├── complex-compile-chain/ # Complex compile chain test
│   ├── presentation/      # Presentation document test
│   ├── test_examples.ps1  # Windows test script
│   └── test_examples.sh   # Linux/macOS test script
├── docs/                   # Documentation directory
├── Cargo.toml             # Rust project configuration
├── README.md              # English documentation
└── README_zh.md           # Chinese documentation
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
