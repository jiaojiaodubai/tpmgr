# tpmgr Developer Documentation

## Architecture Overview

tpmgr is a modern LaTeX package manager written in Rust, designed with modularity and performance in mind.

### Core Modules

#### `main.rs`
- Entry point and CLI argument parsing using `clap`
- Command routing and error handling
- Defines the main command structure

#### `commands.rs`
- Implementation of all CLI commands (`init`, `install`, `remove`, `compile`, etc.)
- Handles command-specific logic and user interaction
- Coordinates between different modules

#### `package.rs`
- Core package management functionality
- Package installation, removal, and metadata handling
- Local package storage and registry management
- TEXINPUTS environment setup

#### `config.rs`
- Configuration file parsing and management (`tpmgr.toml`)
- Global and project-level configuration handling
- Compilation chain configuration with magic variables
- Default patterns for file cleaning

#### `tex_parser.rs`
- LaTeX file parsing and dependency detection
- Two modes: regex-based and compilation-error-based detection
- Package filtering to exclude core LaTeX packages
- Error message parsing for missing package detection

#### `texlive.rs`
- TeXLive installation detection and integration
- System package querying and verification
- Filename database management
- Cross-platform TeXLive path detection

#### `mirror.rs`
- CTAN mirror management and selection
- Automatic fastest mirror detection
- Mirror configuration persistence

#### `resolver.rs`
- Dependency resolution algorithms
- Version constraint handling
- Conflict detection and resolution

#### `error.rs`
- Custom error types and error handling
- Structured error messages for better user experience

## Key Design Patterns

### Environment Isolation
- Uses `TEXINPUTS` environment variable instead of system-wide package installation
- Each project maintains its own `packages/` directory
- Flat directory structure: `packages/packagename.sty`

### Dual Detection Strategy
1. **Regex-based**: Fast scanning of LaTeX files for `\usepackage{}` commands
2. **Compilation-based**: Attempts compilation and parses error messages for missing packages

### Magic Variables
Configuration supports dynamic path resolution:
- `${PROJECT_ROOT}`: Absolute path to project root
- `${CURRENT_DIR}`: Current working directory
- `${HOME}`: User home directory

### Compilation Chain
Multi-step compilation support (LaTeX → BibTeX → LaTeX) with:
- Custom tool and argument specification
- Automatic intermediate file cleanup
- Glob pattern-based file matching

## Testing Strategy

### Unit Tests
- **`tex_parser.rs`**: Tests parsing logic, comment handling, error detection
- **`package.rs`**: Tests package management, content generation, metadata

### Integration Tests
- **Examples directory**: Four real-world LaTeX projects
- **Cross-platform scripts**: PowerShell (Windows) and Bash (Linux/macOS)
- **Comprehensive workflow testing**: Init → Analyze → Install → Compile

## Build and Development

### Prerequisites
- Rust 1.70+
- LaTeX distribution (TeX Live recommended)

### Build Commands
```bash
# Debug build
cargo build

# Release build  
cargo build --release

# Run tests
cargo test

# Integration testing
cd examples
./test_examples.sh    # Linux/macOS
.\test_examples.ps1   # Windows
```

### Code Style
- Standard Rust formatting with `rustfmt`
- Error handling with `anyhow::Result`
- Async/await for network operations
- Structured logging for debugging

## Package Installation Strategy

### Local Package Generation
tpmgr generates minimal stub packages for missing dependencies:
- **Standard packages**: Basic `\ProvidesPackage{}` declaration
- **Graphics packages**: Include common commands like `\includegraphics`
- **Math packages**: Include essential mathematical environments
- **Encoding packages**: Provide option declarations

### System Integration
1. **TeXLive Detection**: Automatic discovery of system TeXLive installation
2. **Package Verification**: Check system packages before local installation
3. **Database Updates**: Maintain filename database for package discovery

## Configuration Management

### Global Configuration
- Stored in system config directory
- Mirrors, TeXLive paths, default settings

### Project Configuration
- `tpmgr.toml` in project root
- Compilation chains, package directories, cleaning patterns
- Inherits from global settings with local overrides

## Performance Considerations

### Caching Strategy
- Package metadata cached locally
- Mirror response times cached for selection
- Filename database updates minimized

### Parallel Operations
- Concurrent package downloads when possible
- Async network operations throughout

### File System Optimization
- Minimal file system operations
- Efficient glob pattern matching
- Atomic file operations for consistency

## Error Handling Philosophy

### User-Friendly Messages
- Clear error descriptions with context
- Actionable suggestions for resolution
- Graceful degradation when possible

### Robustness
- Defensive programming against partial installations
- Recovery from interrupted operations
- Validation of external tool availability

## Future Architecture Considerations

### Extensibility
- Plugin system for custom package sources
- Template system for project initialization
- Hook system for compilation pipeline

### Performance Improvements
- Incremental parsing for large projects
- Smart caching of compilation results
- Background package index updates

### Integration Opportunities
- VS Code extension for seamless editing
- CI/CD pipeline integration
- Docker container optimization
