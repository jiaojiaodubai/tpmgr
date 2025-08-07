# tpmgr Build Guide

This document describes how to build different types of Windows installers for tpmgr. The project supports one installer type and a portable version.

## Prerequisites

### Basic Environment

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **Git** - Version control
- **PowerShell 5.1+** - For Windows build scripts

### Windows Installer Build Tools

- **Inno Setup 6.x**
  - Download: [Inno Setup Official Website](https://jrsoftware.org/isinfo.php)
  - Installation path: `C:\Program Files (x86)\Inno Setup 6\`

## Quick Build

### Local Development Build

```bash
# Clone repository
git clone https://github.com/jiaojiaodubai/tpmgr.git
cd tpmgr

# Build for current platform
cargo build --release

# Test build result
./target/release/tpmgr --help
```

### Windows Installer Build

```powershell
# Build all installer types
cd build
.\build-all.ps1

# Clean build artifacts only
.\build-all.ps1 -Clean
```

## Remote Installation

One-click installation script for quick setup:

```powershell
# Install latest version (portable)
iwr -useb https://raw.githubusercontent.com/jiaojiaodubai/tpmgr/master/install-remote.ps1 | iex

# Install specific version
.\install-remote.ps1 -Version "0.1.0"

# Show help
.\install-remote.ps1 -Help
```

## Installer Types

### 1. Inno Setup Installer (Recommended) ⭐

**Features:**

- Professional installer UI
- Three-language support: English, Simplified Chinese, Traditional Chinese
- Advanced scripting capabilities
- Widely adopted in Windows ecosystem

**Output file:** `dist\tpmgr-0.1.0-setup.exe`

**Build command:**

```powershell
cd inno
& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" tpmgr.iss
```

### 2. Portable Version

**Features:**

- No installation required
- Single executable file
- Includes install/uninstall batch scripts
- Ideal for portable software users

**Output file:** `dist\tpmgr-0.1.0-portable.zip`

**Build command:**

```powershell
# Manual portable build
mkdir temp-portable
copy target\release\tpmgr.exe temp-portable\
copy LICENSE temp-portable\LICENSE.txt
# Create install.bat and uninstall.bat scripts
Compress-Archive -Path temp-portable\* -DestinationPath dist\tpmgr-0.1.0-portable.zip
```

## File Structure

```text
build/
├── build-all.ps1           # One-click build script
├── BUILD.md                # This documentation
└── inno/
    ├── tpmgr.iss          # Inno Setup script
    ├── ChineseSimplified.isl    # Simplified Chinese language file
    ├── ChineseTraditional.isl   # Traditional Chinese language file
    └── license.txt        # License file for Inno Setup
```

## Build Artifacts

After successful build, artifacts are located in the `dist/` directory:

- `tpmgr-0.1.0-setup.exe` - Inno Setup installer
- `tpmgr-0.1.0-portable.zip` - Portable version

## Development Environment Setup

### Visual Studio Code

Recommended extensions:

- Install `rust-analyzer` extension
- Install `CodeLLDB` for debugging

### JetBrains IntelliJ IDEA

- Install Rust plugin
- Configure Cargo integration

## Build Scripts

### Inno Setup Script (`inno/tpmgr.iss`)

Features:

- Multi-language support
- Component-based installation
- Custom PATH management functions
- Professional UI with language detection

### Build Automation (`build-all.ps1`)

Features:

- Ensure PowerShell execution policy allows scripts
- Automatic tool detection
- Error handling and reporting
- Clean build artifacts
- Progress indicators

## Version Management

When releasing a new version, update the following files:

1. `Cargo.toml` - Rust package version
2. `inno/tpmgr.iss` - Inno Setup script version
3. `build-all.ps1` - Portable ZIP filename

## Testing

Before releasing, ensure:

1. Installation functionality testing
2. PATH environment variable management
3. Uninstallation cleanup
4. Multi-language interface testing
5. Different Windows versions compatibility

## Troubleshooting

### Common Issues

- **Inno Setup not found**: Install Inno Setup 6.x to default location  
- **PowerShell execution policy**: Run `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser`
- **Build artifacts not generated**: Check tool installation paths and permissions

### Debugging

Enable verbose output in build scripts for troubleshooting:

```powershell
$VerbosePreference = "Continue"
.\build-all.ps1
```

---

Last updated: August 11, 2025
