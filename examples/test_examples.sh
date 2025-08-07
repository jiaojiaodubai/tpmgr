#!/bin/bash
# tpmgr Examples Test Script (Linux/macOS version)
# This script tests all project examples in the examples directory

set -e

# Default parameters
TPMGR_PATH="../target/release/tpmgr"
VERBOSE=false
KEEP_FILES=false
ONLY_PROJECTS=()

# Color output functions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

write_success() { echo -e "${GREEN}✓ $1${NC}"; }
write_error() { echo -e "${RED}✗ $1${NC}"; }
write_warning() { echo -e "${YELLOW}⚠ $1${NC}"; }
write_info() { echo -e "${CYAN}ℹ $1${NC}"; }
write_step() { echo -e "${BLUE}🔄 $1${NC}"; }

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --tpmgr-path)
            TPMGR_PATH="$2"
            shift 2
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --keep-files)
            KEEP_FILES=true
            shift
            ;;
        --only)
            IFS=',' read -ra ONLY_PROJECTS <<< "$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --tpmgr-path PATH    Path to tpmgr binary (default: ../target/release/tpmgr)"
            echo "  --verbose            Show verbose output"
            echo "  --keep-files         Keep intermediate files after test"
            echo "  --only PROJECTS      Test only specified projects (comma separated)"
            echo "  -h, --help           Show this help message"
            echo ""
            echo "Note: This script is designed for Linux/macOS systems."
            echo "For Windows, please use test_examples.ps1 instead."
            exit 0
            ;;
        *)
            # Support legacy positional argument format
            if [[ -z "${ONLY_PROJECTS[*]}" && ! "$1" =~ ^-- ]]; then
                ONLY_PROJECTS=("$1")
                shift
            else
                write_error "Unknown option: $1"
                exit 1
            fi
            ;;
    esac
done

# Check if tpmgr exists
if [[ ! -f "$TPMGR_PATH" ]]; then
    write_error "tpmgr executable not found: $TPMGR_PATH"
    write_info "Please run 'cargo build --release' to build the project first"
    exit 1
fi

TPMGR_PATH=$(realpath "$TPMGR_PATH")
write_info "Using tpmgr: $TPMGR_PATH"

# Define test projects
declare -A TEST_PROJECTS
TEST_PROJECTS["basic-project"]="Basic project test - Test basic package detection and installation"
TEST_PROJECTS["multi-package-test"]="Multi-package test - Test math and code-related packages"
TEST_PROJECTS["complex-compile-chain"]="Complex compilation chain test - Test LaTeX + BibTeX compilation chain"
TEST_PROJECTS["presentation"]="Presentation test - Test beamer-related packages"

declare -A REQUIRED_FILES
REQUIRED_FILES["basic-project"]="main.tex"
REQUIRED_FILES["multi-package-test"]="main.tex"
REQUIRED_FILES["complex-compile-chain"]="main.tex references.bib"
REQUIRED_FILES["presentation"]="main.tex"

# Filter projects to test
if [[ ${#ONLY_PROJECTS[@]} -gt 0 ]]; then
    write_info "Testing only specified projects: ${ONLY_PROJECTS[*]}"
    # Verify projects exist
    for project in "${ONLY_PROJECTS[@]}"; do
        if [[ ! -v TEST_PROJECTS["$project"] ]]; then
            write_error "Project does not exist: $project"
            exit 1
        fi
    done
    PROJECTS_TO_TEST=("${ONLY_PROJECTS[@]}")
else
    PROJECTS_TO_TEST=($(printf "%s\n" "${!TEST_PROJECTS[@]}" | sort))
fi

TOTAL_PROJECTS=${#PROJECTS_TO_TEST[@]}
PASSED_PROJECTS=0
FAILED_PROJECTS=()

write_info "Starting test for $TOTAL_PROJECTS projects..."
echo ""

for PROJECT_NAME in "${PROJECTS_TO_TEST[@]}"; do
    PROJECT_PATH="$PROJECT_NAME"
    
    echo "================================================================================"
    write_info "Testing project: $PROJECT_NAME"
    write_info "Description: ${TEST_PROJECTS[$PROJECT_NAME]}"
    echo ""
    
    if [[ ! -d "$PROJECT_PATH" ]]; then
        write_error "Project directory does not exist: $PROJECT_PATH"
        FAILED_PROJECTS+=("$PROJECT_NAME")
        continue
    fi
    
    # Enter project directory
    pushd "$PROJECT_PATH" > /dev/null
    
    # Test failure flag
    TEST_FAILED=false
    
    # === Step 1: Reset project directory ===
    write_step "Step 1: Reset project directory"
    
    # Preserve necessary source files
    REQUIRED_FILES_ARRAY=(${REQUIRED_FILES[$PROJECT_NAME]})
    EXISTING_FILES=()
    
    for FILE in "${REQUIRED_FILES_ARRAY[@]}"; do
        if [[ -f "$FILE" ]]; then
            BACKUP_NAME="$FILE.backup"
            cp "$FILE" "$BACKUP_NAME"
            EXISTING_FILES+=("$FILE")
            write_success "Backed up file: $FILE"
        else
            write_warning "Required file does not exist: $FILE"
        fi
    done
    
    # Clean up all files and directories except source files
    shopt -s nullglob
    for item in *; do
        if [[ -f "$item" ]]; then
            # Check if it's a required file or backup
            KEEP_FILE=false
            for REQUIRED in "${REQUIRED_FILES_ARRAY[@]}"; do
                if [[ "$item" == "$REQUIRED" || "$item" == "$REQUIRED.backup" ]]; then
                    KEEP_FILE=true
                    break
                fi
            done
            if [[ "$KEEP_FILE" == "false" ]]; then
                rm -f "$item"
            fi
        elif [[ -d "$item" ]]; then
            rm -rf "$item"
        fi
    done
    shopt -u nullglob
    
    # Restore necessary files
    for FILE in "${EXISTING_FILES[@]}"; do
        mv "$FILE.backup" "$FILE"
    done
    
    write_success "Project directory has been reset"
    
    # === Step 2: Initialize project ===
    write_step "Step 2: Initialize project configuration"
    
    if INIT_RESULT=$("$TPMGR_PATH" init 2>&1); then
        write_success "Project initialization successful"
        if [[ "$VERBOSE" == "true" ]]; then
            echo "$INIT_RESULT"
        fi
    else
        write_error "Project initialization failed"
        echo "$INIT_RESULT"
        TEST_FAILED=true
    fi
    
    # Check if configuration file was created
    if [[ -f "tpmgr.toml" ]]; then
        write_success "Configuration file tpmgr.toml created"
    else
        write_error "Configuration file tpmgr.toml not created"
        TEST_FAILED=true
    fi
    
    if [[ "$TEST_FAILED" == "true" ]]; then
        FAILED_PROJECTS+=("$PROJECT_NAME")
        popd > /dev/null
        continue
    fi
    
    # === Step 3a: Test regular package installation (regex-based) ===
    write_step "Step 3a: Test regular package installation (regex detection)"
    
    if INSTALL_RESULT=$("$TPMGR_PATH" install --path . 2>&1); then
        write_success "Regular package installation detection completed"
        if [[ "$VERBOSE" == "true" ]]; then
            echo "$INSTALL_RESULT"
        fi
    else
        write_warning "Regular package installation may have encountered issues"
        echo "$INSTALL_RESULT"
    fi
    
    # === Step 3b: Test compilation package installation (compilation loop based) ===
    write_step "Step 3b: Test compilation package installation (compilation error detection)"
    
    if COMPILE_INSTALL_RESULT=$("$TPMGR_PATH" install --path . --compile 2>&1); then
        write_success "Compilation package installation detection completed"
        if [[ "$VERBOSE" == "true" ]]; then
            echo "$COMPILE_INSTALL_RESULT"
        fi
    else
        write_warning "Compilation package installation may have encountered issues"
        echo "$COMPILE_INSTALL_RESULT"
    fi
    
    # === Step 4: Check package installation results ===
    write_step "Step 4: Check package installation results"
    
    if [[ -d "packages" ]]; then
        INSTALLED_PACKAGES=($(ls -1 packages/ 2>/dev/null | grep -v "\.json$" || true))
        if [[ ${#INSTALLED_PACKAGES[@]} -gt 0 ]]; then
            write_success "Found installed packages:"
            for PACKAGE in "${INSTALLED_PACKAGES[@]}"; do
                echo "  - $PACKAGE"
            done
        else
            write_warning "Packages directory exists but is empty"
        fi
    else
        write_warning "Packages directory does not exist, all packages may already be installed in the system"
    fi
    
    # === Step 5: Test compilation ===
    write_step "Step 5: Test project compilation"
    
    if COMPILE_RESULT=$("$TPMGR_PATH" compile --path . 2>&1); then
        write_success "Project compilation successful"
        if [[ "$VERBOSE" == "true" ]]; then
            echo "$COMPILE_RESULT"
        fi
        
        # Check if PDF was generated
        PDF_FILES=($(ls *.pdf 2>/dev/null || true))
        if [[ ${#PDF_FILES[@]} -gt 0 ]]; then
            write_success "PDF files generated: ${PDF_FILES[*]}"
        fi
    else
        write_warning "Project compilation failed or partially successful"
        echo "$COMPILE_RESULT"
    fi
    
    # === Project test summary ===
    if [[ "$TEST_FAILED" == "true" ]]; then
        write_error "Project $PROJECT_NAME test failed"
        FAILED_PROJECTS+=("$PROJECT_NAME")
    else
        write_success "Project $PROJECT_NAME test completed"
        ((PASSED_PROJECTS++))
    fi
    
    # === Final cleanup: Reset project to initial state ===
    write_step "Reset project to initial state"
    
    # Clean up all generated files and directories (except source files)
    REQUIRED_FILES_ARRAY=(${REQUIRED_FILES[$PROJECT_NAME]})
    shopt -s nullglob
    for item in *; do
        if [[ -f "$item" || -d "$item" ]]; then
            KEEP_ITEM=false
            for REQUIRED in "${REQUIRED_FILES_ARRAY[@]}"; do
                if [[ "$item" == "$REQUIRED" ]]; then
                    KEEP_ITEM=true
                    break
                fi
            done
            if [[ "$KEEP_ITEM" == "false" ]]; then
                rm -rf "$item"
            fi
        fi
    done
    shopt -u nullglob
    
    write_success "Project has been reset to initial state"
    
    popd > /dev/null
    echo ""
done

# === Final test report ===
echo "================================================================================"
write_info "Testing completed!"
echo ""

write_info "Test statistics:"
echo "  Total projects: $TOTAL_PROJECTS"
echo -e "  Successful projects: ${GREEN}$PASSED_PROJECTS${NC}"
echo -e "  Failed projects: ${RED}${#FAILED_PROJECTS[@]}${NC}"

if [[ ${#FAILED_PROJECTS[@]} -gt 0 ]]; then
    echo ""
    write_warning "Failed projects:"
    for PROJECT in "${FAILED_PROJECTS[@]}"; do
        echo -e "  - ${RED}$PROJECT${NC}"
    done
fi

echo ""
if [[ $PASSED_PROJECTS -eq $TOTAL_PROJECTS ]]; then
    write_success "All test projects completed successfully!"
    exit 0
else
    write_error "Some test projects failed"
    exit 1
fi
