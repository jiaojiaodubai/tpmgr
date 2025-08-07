param(
    [string]$Version = "latest",
    [ValidateSet("auto", "portable", "nsis", "inno")]
    [string]$InstallerType = "auto",
    [string]$DownloadMethod = "auto",
    [switch]$Help
)

if ($Help) {
    Write-Host @'
tpmgr Remote Installation Script

Usage:
  .\install-remote.ps1 [OPTIONS]

Options:
  -Version <version>       Specify version to install (default: latest)
  -InstallerType <type>    Installer type: auto, portable, nsis, inno (default: auto)
  -DownloadMethod <method> Download method: auto, curl, wget, powershell (default: auto)
  -Help                    Show this help message

Examples:
  .\install-remote.ps1
  .\install-remote.ps1 -Version "0.1.0" -InstallerType "portable"
  .\install-remote.ps1 -InstallerType "nsis"

Installer Types:
  - portable: Portable ZIP package (recommended for green software users)
  - nsis: NSIS installer (recommended for general users)
  - inno: Inno Setup installer (professional installer with multi-language support)
  - auto: Automatically choose the best installer for your system
'@
    exit 0
}

function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Test-Command {
    param([string]$Command)
    try {
        if (Get-Command $Command -ErrorAction SilentlyContinue) {
            return $true
        }
    } catch {
        return $false
    }
    return $false
}

function Get-LatestVersion {
    try {
        $apiUrl = "https://api.github.com/repos/jiaojiaodubai/tpmgr/releases/latest"
        $response = Invoke-RestMethod -Uri $apiUrl -ErrorAction Stop
        return $response.tag_name.TrimStart('v')
    } catch {
        Write-Warning "Failed to get latest version from GitHub API"
        return "0.1.0"
    }
}

function Get-DownloadUrl {
    param(
        [string]$Version,
        [string]$InstallerType
    )
    
    $baseUrl = "https://github.com/jiaojiaodubai/tpmgr/releases/download/v$Version"
    
    switch ($InstallerType) {
        "portable" { return "$baseUrl/tpmgr-$Version-portable.zip" }
        "nsis" { return "$baseUrl/tpmgr-$Version-installer.exe" }
        "inno" { return "$baseUrl/tpmgr-$Version-setup.exe" }
        default { return "$baseUrl/tpmgr-$Version-installer.exe" }
    }
}

function Invoke-Download {
    param(
        [string]$Url,
        [string]$OutputPath,
        [string]$Method = "auto"
    )
    
    Write-Info "Downloading from: $Url"
    Write-Info "Saving to: $OutputPath"
    
    if ($Method -eq 'auto') {
        if (Test-Command "curl") {
            $Method = "curl"
        } elseif (Test-Command "wget") {
            $Method = "wget"
        } else {
            $Method = "powershell"
        }
    }
    
    try {
        switch ($Method) {
            "curl" {
                Write-Info "Using curl for download"
                & curl -L -o $OutputPath $Url
                if ($LASTEXITCODE -ne 0) { throw "curl failed" }
            }
            "wget" {
                Write-Info "Using wget for download"
                & wget -O $OutputPath $Url
                if ($LASTEXITCODE -ne 0) { throw "wget failed" }
            }
            default {
                Write-Info "Using PowerShell for download"
                Invoke-WebRequest -Uri $Url -OutFile $OutputPath -ErrorAction Stop
            }
        }
        return $true
    } catch {
        Write-Error "Download failed: $_"
        return $false
    }
}

function Install-Portable {
    param([string]$ZipPath)
    
    Write-Info "Installing portable version"
    $tempDir = Join-Path $env:TEMP "tpmgr-install"
    $installDir = Join-Path $env:LOCALAPPDATA "tpmgr"
    
    try {
        # Extract ZIP
        Expand-Archive -Path $ZipPath -DestinationPath $tempDir -Force
        
        # Create install directory
        if (-not (Test-Path $installDir)) {
            New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        }
        
        # Copy files
        Copy-Item -Path "$tempDir\*" -Destination $installDir -Recurse -Force
        
        # Add to PATH
        $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        if ($userPath -notlike "*$installDir*") {
            $newPath = "$userPath;$installDir"
            [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
            Write-Info "Added to PATH. Please restart your terminal."
        }
        
        Write-Info "Portable installation completed successfully!"
        return $true
    } catch {
        Write-Error "Portable installation failed: $_"
        return $false
    } finally {
        if (Test-Path $tempDir) {
            Remove-Item $tempDir -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
}

function Install-Executable {
    param([string]$ExePath)
    
    Write-Info "Running installer: $ExePath"
    try {
        Start-Process -FilePath $ExePath -Wait
        Write-Info "Installation completed!"
        return $true
    } catch {
        Write-Error "Installation failed: $_"
        return $false
    }
}

function Main {
    Write-Host @'
╔══════════════════════════════════════════════════════════════╗
║                    tpmgr Remote Install Script               ║
║               Modern LaTeX Package Manager                   ║
╚══════════════════════════════════════════════════════════════╝
'@ -ForegroundColor Cyan

    # Get version info
    if ($Version -eq 'latest') {
        $Version = Get-LatestVersion
    }
    Write-Info "Target version: $Version"
    
    # Determine installer type
    if ($InstallerType -eq "auto") {
        Write-Info "Auto-selecting installer type: portable (recommended)"
        $InstallerType = "portable"
    }
    Write-Info "Installer type: $InstallerType"
    
    # Get download URL
    $downloadUrl = Get-DownloadUrl -Version $Version -InstallerType $InstallerType
    
    # Create temp directory
    $tempDir = Join-Path $env:TEMP "tpmgr-download"
    if (-not (Test-Path $tempDir)) {
        New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
    }
    
    # Determine file extension
    $extension = if ($InstallerType -eq "portable") { ".zip" } else { ".exe" }
    $fileName = "tpmgr-$Version-$InstallerType$extension"
    $filePath = Join-Path $tempDir $fileName
    
    # Download file
    if (-not (Invoke-Download -Url $downloadUrl -OutputPath $filePath -Method $DownloadMethod)) {
        Write-Error "Download failed. Installation aborted."
        exit 1
    }
    
    # Install
    $success = if ($InstallerType -eq "portable") {
        Install-Portable -ZipPath $filePath
    } else {
        Install-Executable -ExePath $filePath
    }
    
    # Cleanup
    if (Test-Path $tempDir) {
        Remove-Item $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
    
    if ($success) {
        Write-Host "`n🎉 tpmgr installation completed successfully!" -ForegroundColor Green
        Write-Host "📖 Visit https://github.com/jiaojiaodubai/tpmgr for documentation" -ForegroundColor Cyan
    } else {
        Write-Host "`n❌ Installation failed. Please try again or install manually." -ForegroundColor Red
        exit 1
    }
}

# Run main function
Main
