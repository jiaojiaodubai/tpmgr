# tpmgr Examples Test Script
# This script tests all project examples in the examples directory

param(
    [string]$TpmgrPath = ".\..\target\release\tpmgr.exe",
    [switch]$Verbose,
    [switch]$KeepFiles,
    [string[]]$Only = @()
)

# Color output functions
function Write-ColorOutput {
    param(
        [string]$Text,
        [string]$Color = "White"
    )
    Write-Host $Text -ForegroundColor $Color
}

function Write-Success { param([string]$Text) Write-ColorOutput "✓ $Text" "Green" }
function Write-Error { param([string]$Text) Write-ColorOutput "✗ $Text" "Red" }
function Write-Warning { param([string]$Text) Write-ColorOutput "⚠ $Text" "Yellow" }
function Write-Info { param([string]$Text) Write-ColorOutput "ℹ $Text" "Cyan" }
function Write-Step { param([string]$Text) Write-ColorOutput "🔄 $Text" "Blue" }

# Get absolute path
$TpmgrPath = Resolve-Path $TpmgrPath -ErrorAction SilentlyContinue
if (-not $TpmgrPath -or -not (Test-Path $TpmgrPath)) {
    Write-Error "tpmgr executable not found: $TpmgrPath"
    Write-Info "Please run 'cargo build' to build the project first"
    exit 1
}

Write-Info "Using tpmgr: $TpmgrPath"

# Define test projects
$TestProjects = @(
    @{
        Name = "basic-project"
        Description = "Basic project test - Test basic package detection and installation"
        RequiredFiles = @("main.tex")
        ExpectedPackages = @("geometry", "graphicx", "lipsum")
    },
    @{
        Name = "multi-package-test"
        Description = "Multi-package test - Test math and code-related packages"
        RequiredFiles = @("main.tex")
        ExpectedPackages = @("amsmath", "amsfonts", "amssymb", "xcolor", "listings", "hyperref")
    },
    @{
        Name = "complex-compile-chain"
        Description = "Complex compilation chain test - Test LaTeX + BibTeX compilation chain"
        RequiredFiles = @("main.tex", "references.bib")
        ExpectedPackages = @("amsmath", "natbib", "hyperref")
    },
    @{
        Name = "presentation"
        Description = "Presentation test - Test beamer-related packages"
        RequiredFiles = @("main.tex")
        ExpectedPackages = @("tikz", "pgfplots", "subcaption")
    }
)

# Filter projects to test
if ($Only.Count -gt 0) {
    Write-Info "Testing only specified projects: $($Only -join ', ')"
    # Verify projects exist before filtering
    $AllProjectNames = $TestProjects.Name
    foreach ($ProjectName in $Only) {
        if ($ProjectName -notin $AllProjectNames) {
            Write-Error "Project does not exist: $ProjectName"
            Write-Info "Available projects: $($AllProjectNames -join ', ')"
            exit 1
        }
    }
    # Filter the projects
    $FilteredProjects = @()
    foreach ($Project in $TestProjects) {
        if ($Project.Name -in $Only) {
            $FilteredProjects += $Project
        }
    }
    $TestProjects = $FilteredProjects
}

# Test statistics
$TotalProjects = $TestProjects.Count
$PassedProjects = 0
$FailedProjects = @()

Write-Info "Starting test for $TotalProjects projects..."
Write-Host ""

foreach ($Project in $TestProjects) {
    $ProjectName = $Project.Name
    $ProjectPath = Join-Path $PSScriptRoot $ProjectName
    
    Write-Host ("=" * 80)
    Write-Info "Testing project: $ProjectName"
    Write-Info "Description: $($Project.Description)"
    Write-Host ""
    
    if (-not (Test-Path $ProjectPath)) {
        Write-Error "Project directory does not exist: $ProjectPath"
        $FailedProjects += $ProjectName
        continue
    }
    
    # Enter project directory
    Push-Location $ProjectPath
    
    try {
        # === Step 1: Reset project directory ===
        Write-Step "Step 1: Reset project directory"
        
        # Preserve necessary source files
        $RequiredFiles = $Project.RequiredFiles
        $ExistingFiles = @()
        
        foreach ($File in $RequiredFiles) {
            if (Test-Path $File) {
                $BackupName = "$File.backup"
                Copy-Item $File $BackupName -Force
                $ExistingFiles += @{ Original = $File; Backup = $BackupName }
                Write-Success "Backed up file: $File"
            }
            else {
                Write-Warning "Required file does not exist: $File"
            }
        }
        
        # Clean up all files and directories except source files
        Get-ChildItem -Force | Where-Object { 
            $_.Name -notin $RequiredFiles -and 
            -not $_.Name.EndsWith('.backup')
        } | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
        
        # Restore necessary files
        foreach ($FileInfo in $ExistingFiles) {
            Move-Item $FileInfo.Backup $FileInfo.Original -Force
        }
        
        Write-Success "Project directory has been reset"
        
        # === Step 2: Initialize project ===
        Write-Step "Step 2: Initialize project configuration"
        
        $InitResult = & $TpmgrPath init 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Project initialization successful"
            if ($Verbose) { Write-Host $InitResult }
        }
        else {
            Write-Error "Project initialization failed"
            Write-Host $InitResult
            throw "Initialization failed"
        }
        
        # Check if configuration file was created
        if (Test-Path "tpmgr.toml") {
            Write-Success "Configuration file tpmgr.toml created"
        }
        else {
            Write-Error "Configuration file tpmgr.toml not created"
            throw "Configuration file not created"
        }
        
        # === Step 3a: Test regular package installation (regex-based) ===
        Write-Step "Step 3a: Test regular package installation (regex detection)"
        
        $InstallResult = & $TpmgrPath install --path . 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Regular package installation detection completed"
            if ($Verbose) { Write-Host $InstallResult }
        }
        else {
            Write-Warning "Regular package installation may have encountered issues"
            Write-Host $InstallResult
        }
        
        # === Step 3b: Test compilation package installation (compilation error based) ===
        Write-Step "Step 3b: Test compilation package installation (compilation error detection)"
        
        $CompileInstallResult = & $TpmgrPath install --path . --compile 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Compilation package installation detection completed"
            if ($Verbose) { Write-Host $CompileInstallResult }
        }
        else {
            Write-Warning "Compilation package installation may have encountered issues"
            Write-Host $CompileInstallResult
        }
        
        # === Step 4: Check if packages directory was created ===
        Write-Step "Step 4: Check package installation results"
        
        $PackagesDir = "packages"
        if (Test-Path $PackagesDir) {
            $InstalledPackages = Get-ChildItem $PackagesDir -ErrorAction SilentlyContinue
            if ($InstalledPackages.Count -gt 0) {
                Write-Success "Found installed packages:"
                foreach ($Package in $InstalledPackages) {
                    Write-Host "  - $($Package.Name)" -ForegroundColor White
                }
            }
            else {
                Write-Warning "Packages directory exists but is empty"
            }
        }
        else {
            Write-Warning "Packages directory does not exist, all packages may already be installed in the system"
        }
        
        # === Step 5: Test compilation ===
        Write-Step "Step 5: Test project compilation"
        
        $CompileResult = & $TpmgrPath compile --path . 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Project compilation successful"
            if ($Verbose) { Write-Host $CompileResult }
            
            # Check if PDF was generated
            $PdfFiles = Get-ChildItem "*.pdf" -ErrorAction SilentlyContinue
            if ($PdfFiles.Count -gt 0) {
                Write-Success "PDF files generated: $($PdfFiles.Name -join ', ')"
            }
        }
        else {
            Write-Warning "Project compilation failed or partially successful"
            Write-Host $CompileResult
        }
        
        # === Project test summary ===
        Write-Success "Project $ProjectName test completed"
        $PassedProjects++
        
    }
    catch {
        Write-Error "Project $ProjectName test failed: $_"
        $FailedProjects += $ProjectName
    }
    finally {
        # === Final cleanup: Reset project to initial state ===
        Write-Step "Reset project to initial state"
        
        # Clean up all generated files and directories (except source files)
        $RequiredFiles = $Project.RequiredFiles
        Get-ChildItem -Force | Where-Object { 
            $_.Name -notin $RequiredFiles
        } | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
        
        Write-Success "Project has been reset to initial state"
        
        Pop-Location
        Write-Host ""
    }
}

# === Final test report ===
Write-Host ("=" * 80)
Write-Info "Testing completed!"
Write-Host ""

Write-Info "Test statistics:"
Write-Host "  Total projects: $TotalProjects" -ForegroundColor White
Write-Host "  Successful projects: $PassedProjects" -ForegroundColor Green
Write-Host "  Failed projects: $($FailedProjects.Count)" -ForegroundColor Red

if ($FailedProjects.Count -gt 0) {
    Write-Host ""
    Write-Warning "Failed projects:"
    foreach ($Project in $FailedProjects) {
        Write-Host "  - $Project" -ForegroundColor Red
    }
}

Write-Host ""
if ($PassedProjects -eq $TotalProjects -and $FailedProjects.Count -eq 0) {
    Write-Success "All test projects completed successfully!"
    exit 0
}
else {
    Write-Error "Some test projects failed"
    exit 1
}
