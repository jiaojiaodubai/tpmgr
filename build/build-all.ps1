#!/usr/bin/env pwsh
<#
.SYNOPSIS
    tpmgr One-Click Build Script - Build all types of Windows installers

.DESCRIPTION
    Automatically builds all tpmgr installer types:
    - Inno Setup Multi-language Installer (English/Simplified Chinese/Traditional Chinese)
    - Portable ZIP Package (No installation required, includes install scripts)

.EXAMPLE
    .\build-all.ps1
    
.EXAMPLE
    .\build-all.ps1 -Clean
    Clean build artifacts only

.PARAMETER Clean
    Execute cleanup operation only, no building

.PARAMETER Help
    Show help information
#>

[CmdletBinding()]
param(
    [switch]$Clean = $false,
    [switch]$Help = $false
)

$ErrorActionPreference = "Stop"

# Set encoding to UTF-8 to prevent garbled characters
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

# Helper: Ensure a file is UTF-8 with BOM (needed by Inno Setup for non-ASCII)
function Ensure-Utf8Bom($Path) {
    try {
        $full = Resolve-Path $Path -ErrorAction Stop
        $bytes = [System.IO.File]::ReadAllBytes($full)
        if ($bytes.Length -ge 3 -and $bytes[0] -eq 0xEF -and $bytes[1] -eq 0xBB -and $bytes[2] -eq 0xBF) {
            Write-ColorOutput "✓ UTF-8 BOM already present: $full" "Green"
            return
        }
        $text = [System.Text.UTF8Encoding]::new($false).GetString($bytes)
        if ($PSVersionTable.PSVersion.Major -ge 7) {
            Set-Content -Path $full -Value $text -Encoding utf8BOM
        } else {
            $bom = [byte[]](0xEF,0xBB,0xBF)
            $data = [System.Text.Encoding]::UTF8.GetBytes($text)
            [System.IO.File]::WriteAllBytes($full, $bom + $data)
        }
        Write-ColorOutput "✓ Converted to UTF-8 BOM: $full" "Green"
    } catch {
        Write-Error "Failed to ensure UTF-8 BOM for $Path`: $($_.Exception.Message)"
    }
}

# Color output functions
function Write-ColorOutput($Message, $Color = "White") {
    Write-Host $Message -ForegroundColor $Color
}

function Write-Step($Message) {
    Write-ColorOutput "🔧 $Message" "Yellow"
}

function Write-Success($Message) {
    Write-ColorOutput "✅ $Message" "Green"
}

function Write-Error($Message) {
    Write-ColorOutput "❌ $Message" "Red"
}

# Check if tool exists
function Test-Tool($Name, $Path) {
    if (Test-Path $Path) {
        Write-ColorOutput "✓ $Name found: $Path" "Green"
        return $true
    } else {
        Write-ColorOutput "✗ $Name not found: $Path" "Red"
        return $false
    }
}

# Main function
function Main {
    if ($Help) {
        Get-Help $PSCommandPath -Full
        return
    }

    Write-ColorOutput @"
╔══════════════════════════════════════════════════════════════╗
║                    tpmgr One-Click Build Script              ║
║               Build all Windows installer types             ║
╚══════════════════════════════════════════════════════════════╝
"@ "Cyan"

    # 1. Clean environment
    Write-Step "Cleaning build environment..."
    Remove-Item "..\dist\*" -Force -ErrorAction SilentlyContinue
    Remove-Item "..\target\release\*" -Recurse -Force -ErrorAction SilentlyContinue
    # Ensure dist directory exists (NSIS cannot create parent directories)
    if (-not (Test-Path "..\dist")) { New-Item -ItemType Directory -Path "..\dist" | Out-Null }
    Write-Success "Build environment cleaned"

    if ($Clean) {
        Write-Success "Cleanup completed, exiting script"
        return
    }

    # 2. Check tools
    Write-Step "Checking build tools..."
    $innoPath = "C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
    
    $hasInno = Test-Tool "Inno Setup" $innoPath

    if (-not $hasInno) {
        Write-Error "No installer build tools found, please install Inno Setup"
        exit 1
    }

    # 3. Build Rust application
    Write-Step "Building Rust application..."
    try {
        Set-Location ".."
        cargo build --release
        if ($LASTEXITCODE -ne 0) { throw "Cargo build failed" }
        Set-Location "build"
        Write-Success "Rust application build completed"
    } catch {
        Write-Error "Rust build failed: $_"
        Set-Location "build"
        exit 1
    }

    $buildCount = 0

    # 4. Build Inno Setup installer
    if ($hasInno) {
        Write-Step "Building Inno Setup installer..."
        try {
            Set-Location "inno"
            & $innoPath "tpmgr.iss" | Out-String | Write-Host
            if ($LASTEXITCODE -ne 0) { throw "Inno Setup build failed" }
            Set-Location ".."
            Write-Success "Inno Setup installer build completed"
            $buildCount++
        } catch {
            Write-Error "Inno Setup build failed: $_"
            Set-Location ".."
        }
    }

    # 5. Build portable version
    Write-Step "Building portable ZIP package..."
    try {
        # Create temporary directory
        $tempDir = "temp-portable"
        if (Test-Path $tempDir) { Remove-Item $tempDir -Recurse -Force }
        New-Item -ItemType Directory -Path $tempDir | Out-Null
        
        # Copy executable file
        Copy-Item "..\target\release\tpmgr.exe" "$tempDir\tpmgr.exe"
        
        # Copy license and documentation
        Copy-Item "..\LICENSE" "$tempDir\LICENSE.txt"
        
        # Create install script
        @"
@echo off
echo Installing tpmgr to %LOCALAPPDATA%\tpmgr...
if not exist "%LOCALAPPDATA%\tpmgr" mkdir "%LOCALAPPDATA%\tpmgr"
copy tpmgr.exe "%LOCALAPPDATA%\tpmgr\tpmgr.exe"

echo Adding to PATH...
for /f "tokens=2*" %%A in ('reg query "HKCU\Environment" /v PATH 2^>nul') do set "currentpath=%%B"
if "%currentpath%" == "" set "currentpath=%LOCALAPPDATA%\tpmgr"
if "%currentpath:~-1%" neq ";" set "currentpath=%currentpath%;"
echo %currentpath% | find "%LOCALAPPDATA%\tpmgr" >nul
if errorlevel 1 (
    reg add "HKCU\Environment" /v PATH /t REG_EXPAND_SZ /d "%currentpath%%LOCALAPPDATA%\tpmgr" /f
    echo PATH updated. Please restart your terminal.
) else (
    echo Already in PATH.
)
echo Installation complete!
pause
"@ | Out-File -FilePath "$tempDir\install.bat" -Encoding ASCII
        
        # Create uninstall script
        @"
@echo off
echo Removing tpmgr from PATH...
for /f "tokens=2*" %%A in ('reg query "HKCU\Environment" /v PATH 2^>nul') do set "currentpath=%%B"
set "newpath=%currentpath:;%LOCALAPPDATA%\tpmgr=%"
set "newpath=%newpath:%LOCALAPPDATA%\tpmgr;=%"
set "newpath=%newpath:%LOCALAPPDATA%\tpmgr=%"
reg add "HKCU\Environment" /v PATH /t REG_EXPAND_SZ /d "%newpath%" /f

echo Removing files...
if exist "%LOCALAPPDATA%\tpmgr" rmdir /s /q "%LOCALAPPDATA%\tpmgr"
echo Uninstallation complete!
pause
"@ | Out-File -FilePath "$tempDir\uninstall.bat" -Encoding ASCII
        
        # Create README file
        @'
# tpmgr Portable Version

## Installation
Run install.bat to install tpmgr to your system.

## Manual Usage
You can also use tpmgr.exe directly without installation.

## Uninstallation
Run uninstall.bat to remove tpmgr from your system.

## Getting Started
After installation, run: tpmgr --help
'@ | Out-File -FilePath "$tempDir\README.txt" -Encoding UTF8
        
        # Create portable ZIP
        $zipPath = "..\dist\tpmgr-0.1.0-portable.zip"
        Compress-Archive -Path "$tempDir\*" -DestinationPath $zipPath -Force
        
        # Clean up temporary directory
        Remove-Item $tempDir -Recurse -Force
        
        Write-Success "Portable ZIP package build completed"
        $buildCount++
    } catch {
        Write-Error "Portable build failed: $_"
    }

    # 6. Clean up intermediate files
    Write-Step "Cleaning up intermediate files..."
    # No cleanup needed

    # 7. Display build results
    Write-ColorOutput @"

╔══════════════════════════════════════════════════════════════╗
║                        Build Complete!                       ║
╚══════════════════════════════════════════════════════════════╝
"@ "Green"

    if (Test-Path "..\dist") {
        $files = Get-ChildItem "..\dist" -File
        if ($files.Count -gt 0) {
            Write-ColorOutput "`n📦 Build artifacts ($($files.Count) files):" "Cyan"
            $files | Select-Object Name, @{Name="Size(MB)";Expression={[math]::Round($_.Length/1MB, 2)}} | Format-Table -AutoSize
            
            $totalSize = ($files | Measure-Object Length -Sum).Sum
            Write-ColorOutput "💾 Total size: $([math]::Round($totalSize/1MB, 2)) MB" "Cyan"
        } else {
            Write-Error "dist directory is empty, no installers were built successfully"
        }
    }

    Write-Success "Successfully built $buildCount installer types"
    
    Write-ColorOutput @"

🚀 Usage instructions:
  • Inno Setup version - Professional installer, supports three languages
  • Portable version - No installation required, ideal for portable software users

📖 For detailed documentation, see BUILD.md
"@ "White"
}

# Script main entry
try {
    Main
} catch {
    Write-Error "Error occurred during build process: $_"
    exit 1
}
