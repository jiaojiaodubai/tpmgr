# tpmgr Examples Testing Guide

*[中文版](README_zh.md)*

This directory contains example projects and automated testing scripts for testing `tpmgr` functionality.

## 📁 Test Projects

### 1. basic-project

**Description**: Basic LaTeX project for testing fundamental package installation features.  
**Required Packages**: `geometry`, `graphicx`, `lipsum`  
**Features**: Simple document structure, suitable for basic functionality testing.

### 2. multi-package-test

**Description**: Multi-package test project with math, color, and code highlighting features.  
**Required Packages**: `amsmath`, `amsfonts`, `amssymb`, `xcolor`, `listings`, `hyperref`  
**Features**: Tests detection and installation of various package types.

### 3. complex-compile-chain

**Description**: Complex compilation chain project with BibTeX support.  
**Required Packages**: `amsmath`, `natbib`, `hyperref`  
**Features**: Multi-step compilation workflow, testing references and compilation chain features.

### 4. presentation

**Description**: Beamer presentation project.  
**Required Packages**: `tikz`, `subcaption`  
**Features**: Tests Beamer document class and graphics packages.

## 🚀 Testing Scripts

### PowerShell Version (`test_examples.ps1`)

For Windows systems with complete functionality.

#### Basic Usage

```powershell
# Test all projects
.\test_examples.ps1

# Test single project
.\test_examples.ps1 -ProjectName basic-project

# Verbose mode
.\test_examples.ps1 -ProjectName basic-project -Verbose

# Skip cleanup step
.\test_examples.ps1 -ProjectName basic-project -SkipClean

# Compile only (skip package installation tests)
.\test_examples.ps1 -ProjectName basic-project -CompileOnly
```

#### Parameters

- `-ProjectName`: Specify project name to test (optional)
- `-Verbose`: Show detailed output
- `-SkipClean`: Skip project reset step
- `-CompileOnly`: Test compilation functionality only

### Bash Version (`test_examples.sh`)

For Linux/macOS systems with the same functionality as PowerShell version.

#### Basic Usage

```bash
# Grant execution permission
chmod +x test_examples.sh

# Test all projects
./test_examples.sh

# Test single project
./test_examples.sh basic-project

# Verbose mode
./test_examples.sh -v basic-project

# Skip cleanup step
./test_examples.sh -s basic-project

# Compile only
./test_examples.sh -c basic-project

# Show help
./test_examples.sh -h
```

## 🔄 Testing Workflow

The testing scripts automatically execute the following steps:

### 1. Project Reset

- Remove all generated files (PDF, auxiliary files, etc.)
- Keep source files (`.tex`, `.bib`, `.cls`, `.sty`, `.md`)
- Delete configuration file (`tpmgr.toml`)

### 2. Project Initialization

- Run `tpmgr init` to initialize project in current directory
- Check if configuration file is generated correctly

### 3. Regex Package Detection Test

- Run `tpmgr analyze --verbose` to analyze dependencies
- Run `tpmgr install` to install missing packages
- Attempt to compile project to verify package installation

### 4. Compilation Error Package Detection Test

- Reset project state
- Re-initialize project
- Run `tpmgr analyze --compile --verbose` to analyze dependencies through compilation errors
- Run `tpmgr install --compile` to install packages based on compilation errors

### 5. Final Compilation Verification

- Run `tpmgr compile --verbose` to compile project
- Check if PDF output file is generated

## 📋 Test Results Interpretation

### Success Indicators

- ✅ Test step completed successfully
- 🔄 Step in progress  
- ℹ️ Informational output

### Warnings and Errors

- ⚠️ Warning: Step may fail but testing continues
- ❌ Error: Critical issue causing test termination

### Summary Report

The script displays a test results summary for all projects at the end.

## 🛠️ Prerequisites

1. **Build tpmgr**:

   ```bash
   cargo build
   ```

2. **Install LaTeX Distribution**:

   - Windows: TeX Live or MiKTeX
   - Linux: TeX Live (`sudo apt install texlive-full`)
   - macOS: MacTeX

3. **Ensure Tools in PATH**:

   - `pdflatex`
   - `bibtex` (for complex-compile-chain)

## 🐛 Troubleshooting

### Common Issues

**1. "tpmgr binary not found"**

- Solution: Run `cargo build` to build the project

**2. "pdflatex: command not found"**

- Solution: Install LaTeX distribution and ensure it's in PATH

**3. Package installation failures**

- Some packages may not be in standard repositories, this is expected behavior
- Script will continue testing compilation to verify installed packages

**4. Permission errors**

- Linux/macOS: Ensure script has execution permissions (`chmod +x test_examples.sh`)
- Windows: Ensure PowerShell execution policy allows script execution

### Debugging Tips

1. **Use verbose mode**:

   ```bash
   ./test_examples.sh -v basic-project
   ```

2. **Check temporary logs**:

   - `/tmp/tpmgr_*.log` (Linux/macOS)
   - Or view detailed output in script

3. **Manual single project testing**:

   ```bash
   cd examples/basic-project
   ../../target/debug/tpmgr init
   ../../target/debug/tpmgr analyze --verbose
   ../../target/debug/tpmgr install
   ../../target/debug/tpmgr compile
   ```

## 📝 Adding New Test Projects

To add a new test project:

1. Create new folder under `examples/` directory
2. Add `.tex` and other necessary source files  
3. Add project name to `TEST_PROJECTS` array in testing scripts
4. Run tests to verify new project works correctly

## 🔗 Related Documentation

- [tpmgr User Guide](../README.md)
- [Configuration Documentation](../docs/configuration.md)
- [Compilation Chain Configuration](../docs/compilation.md)
