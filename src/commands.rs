use anyhow::Result;
use crate::config::Config;
use crate::package::PackageManager;
use crate::mirror::MirrorManager;
use crate::texlive::TeXLiveManager;
use crate::tex_parser::TeXParser;
use crate::{MirrorAction, ConfigAction};
use std::path::Path;
use glob;

/// Initialize global configuration if it's the first run
pub async fn ensure_global_config_initialized() -> Result<()> {
    use crate::config::GlobalConfig;
    
    let global_config = GlobalConfig::load()?;
    let mut needs_save = false;
    let mut updated_config = global_config.clone();
    
    // Only show first run message if we actually need to configure something
    let is_first_run = global_config.texlive_path.is_none() || global_config.mirror_url.is_none();
    
    if is_first_run {
        println!("🔍 First run detected - auto-configuring global settings...");
    }
    
    // Check if TeXLive path is not set
    if global_config.texlive_path.is_none() {
        // Try to detect TeXLive installation
        let mut texlive_manager = TeXLiveManager::new();
        match texlive_manager.detect_texlive() {
            Ok(_) => {
                if let Some(info) = texlive_manager.get_texlive_info() {
                    let texlive_path = info.install_path.to_string_lossy().to_string();
                    updated_config.texlive_path = Some(texlive_path.clone());
                    needs_save = true;
                    println!("✅ Detected TeXLive installation: {}", texlive_path);
                } else {
                    println!("⚠️  TeXLive installation detected but path information unavailable");
                }
            }
            Err(e) => {
                println!("⚠️  Could not detect TeXLive installation: {}", e);
                println!("   You can manually set it later with: tpmgr config set --global texlive_path <path>");
            }
        }
    }
    
    // Check if mirror URL is not set
    if global_config.mirror_url.is_none() {
        println!("🌐 Auto-selecting best mirror...");
        
        let mut mirror_manager = MirrorManager::new();
        match mirror_manager.select_best_mirror().await {
            Ok(_) => {
                if let Some(mirror) = mirror_manager.get_selected_mirror() {
                    let mirror_url = format!("{}/systems/texlive/tlnet/", mirror.url);
                    updated_config.mirror_url = Some(mirror_url.clone());
                    needs_save = true;
                    println!("✅ Selected mirror: {} ({})", mirror.name, mirror.country);
                } else {
                    println!("⚠️  Could not select best mirror, using default");
                }
            }
            Err(e) => {
                println!("⚠️  Could not fetch mirrors: {}", e);
                println!("   You can manually set it later with: tpmgr config set --global mirror_url <url>");
            }
        }
    }
    
    if needs_save {
        updated_config.save()?;
        if is_first_run {
            println!("💾 Global configuration saved");
            println!("   View settings with: tpmgr config show --global");
            println!("   Modify settings with: tpmgr config set --global <key> <value>");
            println!();
        }
    }
    
    Ok(())
}

pub async fn init_command(name: Option<String>) -> Result<()> {
    if let Some(project_name) = name {
        // Create new project in a subdirectory
        println!("Initializing LaTeX project: {}", project_name);
        
        std::fs::create_dir_all(&project_name)?;
        std::env::set_current_dir(&project_name)?;
        
        // Create tpmgr.toml configuration file
        let global_config = crate::config::GlobalConfig::load()?;
        let mut config = Config::new();
        config.project.name = project_name.clone();
        
        // Apply global configuration as defaults
        if let Some(texlive_path) = &global_config.texlive_path {
            config.project.texlive_path = Some(texlive_path.clone());
        }
        if let Some(mirror_url) = &global_config.mirror_url {
            config.project.mirror_url = Some(mirror_url.clone());
        }
        config.project.install_global = Some(global_config.install_global);
        config.project.compile = global_config.compile_command.clone();
        
        config.save("tpmgr.toml")?;
        
        // Create basic LaTeX project structure
        std::fs::create_dir_all("packages")?;
        
        // Create main.tex file in project root
        let main_tex = r#"\documentclass{article}
\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}

\title{LaTeX Project}
\author{Your Name}
\date{\today}

\begin{document}
\maketitle

\section{Introduction}
Welcome to your new LaTeX project managed by tpmgr!

\end{document}
"#;
        std::fs::write("main.tex", main_tex)?;
        
        println!("✓ Project initialized successfully!");
        println!("  - Configuration: tpmgr.toml");
        println!("  - Main document: main.tex");
        println!("  - Package directory: packages/");
    } else {
        // Initialize in current directory
        let current_dir = std::env::current_dir()?;
        let dir_name = current_dir.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("latex-project");
        
        println!("Initializing LaTeX project in current directory: {}", dir_name);
        
        // Create tpmgr.toml configuration file with main.tex as default
        let global_config = crate::config::GlobalConfig::load()?;
        let mut config = Config::new();
        // Set project name to current directory name
        config.project.name = dir_name.to_string();
        
        // Apply global configuration as defaults
        if let Some(texlive_path) = &global_config.texlive_path {
            config.project.texlive_path = Some(texlive_path.clone());
        }
        if let Some(mirror_url) = &global_config.mirror_url {
            config.project.mirror_url = Some(mirror_url.clone());
        }
        config.project.install_global = Some(global_config.install_global);
        config.project.compile = global_config.compile_command.clone();
        
        // Set default compile target to main.tex
        config.project.compile.steps = vec![
            crate::config::CompileStep {
                tool: "pdflatex".to_string(),
                args: vec!["-interaction=nonstopmode".to_string(), "main.tex".to_string()],
            },
        ];
        config.save("tpmgr.toml")?;
        
        // Create packages directory if it doesn't exist
        if !std::path::Path::new("packages").exists() {
            std::fs::create_dir_all("packages")?;
        }
        
        // Create main.tex file if it doesn't exist
        if !std::path::Path::new("main.tex").exists() {
            let main_tex = r#"\documentclass{article}
\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}

\title{LaTeX Project}
\author{Your Name}
\date{\today}

\begin{document}
\maketitle

\section{Introduction}
Welcome to your LaTeX project managed by tpmgr!

\end{document}
"#;
            std::fs::write("main.tex", main_tex)?;
            println!("✓ Created main.tex");
        } else {
            println!("✓ main.tex already exists");
        }
        
        println!("✓ Project initialized successfully!");
        println!("  - Configuration: tpmgr.toml");
        println!("  - Main document: main.tex");
        println!("  - Package directory: packages/");
    }
    
    Ok(())
}

pub async fn install_command(
    packages: &[String], 
    global: bool, 
    path: &str, 
    use_compile: bool
) -> Result<()> {
    if packages.is_empty() {
        println!("No packages specified - scanning for missing dependencies...");
        return auto_install_missing_packages(path, use_compile).await;
    }
    
    let manager = PackageManager::new(global)?;
    let mut any_installed = false;
    
    for package_name in packages {
        println!("Installing {}...", package_name);
        match manager.install(package_name).await {
            Ok(_) => {
                println!("✓ {} installed successfully", package_name);
                any_installed = true;
            },
            Err(e) => println!("✗ Failed to install {}: {}", package_name, e),
        }
    }
    
    // Auto-clean cache after installation
    if any_installed {
        println!("Auto-cleaning package cache...");
        if let Err(e) = manager.clean_cache().await {
            println!("Warning: Failed to clean cache: {}", e);
        } else {
            println!("✓ Package cache cleaned");
        }
    }
    
    Ok(())
}

pub async fn remove_command(packages: &[String], global: bool) -> Result<()> {
    if packages.is_empty() {
        println!("No packages specified - auto-cleaning package cache...");
        let manager = PackageManager::new(global)?;
        manager.clean_cache().await?;
        println!("Cache cleaned successfully.");
        return Ok(());
    }
    
    let manager = PackageManager::new(global)?;
    
    for package_name in packages {
        println!("Removing {}...", package_name);
        match manager.remove(package_name).await {
            Ok(_) => println!("✓ {} removed successfully", package_name),
            Err(e) => println!("✗ Failed to remove {}: {}", package_name, e),
        }
    }
    
    Ok(())
}

pub async fn update_command(packages: &[String]) -> Result<()> {
    let manager = PackageManager::new(false)?;
    
    if packages.is_empty() {
        println!("Updating all packages...");
        manager.update_all().await?;
    } else {
        for package_name in packages {
            println!("Updating {}...", package_name);
            match manager.update(package_name).await {
                Ok(_) => println!("✓ {} updated successfully", package_name),
                Err(e) => println!("✗ Failed to update {}: {}", package_name, e),
            }
        }
    }
    
    Ok(())
}

pub async fn list_command(global: bool) -> Result<()> {
    let manager = PackageManager::new(global)?;
    let packages = manager.list_installed().await?;
    
    if packages.is_empty() {
        println!("No packages installed.");
    } else {
        println!("Installed packages:");
        for (name, version) in packages {
            println!("  {} ({})", name, version);
        }
    }
    
    Ok(())
}

pub async fn search_command(query: &str) -> Result<()> {
    let manager = PackageManager::new(false)?;
    let results = manager.search(query).await?;
    
    if results.is_empty() {
        println!("No packages found matching '{}'", query);
    } else {
        println!("Search results for '{}':", query);
        for package in results {
            println!("  {} - {}", package.name, package.description);
        }
    }
    
    Ok(())
}

pub async fn info_command(package_name: &str) -> Result<()> {
    let manager = PackageManager::new(false)?;
    let info = manager.get_package_info(package_name).await?;
    
    println!("Package: {}", info.name);
    println!("Version: {}", info.version);
    println!("Description: {}", info.description);
    println!("Dependencies: {:?}", info.dependencies);
    
    Ok(())
}

pub async fn mirror_command(action: &MirrorAction) -> Result<()> {
    let mut mirror_manager = MirrorManager::new();
    
    match action {
        MirrorAction::List => {
            // Automatically update mirror list
            if let Err(e) = mirror_manager.fetch_mirrors().await {
                println!("Warning: Failed to fetch mirrors: {}", e);
                return Ok(());
            }
            mirror_manager.list_mirrors();
        }
        MirrorAction::Use { name, auto } => {
            // Automatically update mirror list
            if let Err(e) = mirror_manager.fetch_mirrors().await {
                println!("Warning: Failed to fetch mirrors: {}", e);
                return Ok(());
            }
            
            if *auto {
                mirror_manager.select_best_mirror().await?;
                println!("✓ Auto-selected best mirror");
            } else if let Some(mirror_name) = name {
                mirror_manager.select_mirror_by_name(mirror_name)?;
                println!("✓ Mirror selected: {}", mirror_name);
            } else {
                println!("Error: Please specify a mirror name or use --auto");
                return Ok(());
            }
        }
    }
    
    Ok(())
}

pub async fn analyze_command(path: &str, verbose: bool, use_compile: bool) -> Result<()> {
    let parser = TeXParser::new()?;
    let path = Path::new(path);
    
    println!("Analyzing TeX dependencies in: {}", path.display());
    
    if use_compile {
        // Read compile command from configuration
        let config = if Path::new("tpmgr.toml").exists() {
            Config::load("tpmgr.toml")?
        } else {
            Config::new()
        };
        
        let compile_cmd = &config.project.compile;
        let project_root = std::env::current_dir()?;
        
        let missing_packages = if path.is_file() {
            parser.detect_missing_packages_by_compilation(path, compile_cmd, &project_root)?
        } else {
            let resolved_commands = compile_cmd.resolve_variables(&project_root)?;
            if !resolved_commands.is_empty() {
                let last_command = resolved_commands.last().unwrap();
                if let Some(potential_target) = last_command.last() {
                    let target_path = Path::new(potential_target);
                    if target_path.exists() {
                        parser.detect_missing_packages_by_compilation(&target_path, compile_cmd, &project_root)?
                    } else {
                        println!("Target file specified in compile command not found: {}", potential_target);
                        let mut result_packages = Vec::new();
                        let mut found_tex = false;
                        
                        let src_dir = path.join("src");
                        if src_dir.exists() {
                            for entry in std::fs::read_dir(&src_dir)? {
                                if let Ok(entry) = entry {
                                    if let Some(ext) = entry.path().extension() {
                                        if ext == "tex" {
                                            result_packages = parser.detect_missing_packages_by_compilation(&entry.path(), compile_cmd, &project_root)?;
                                            found_tex = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        
                        if !found_tex {
                            for entry in std::fs::read_dir(path)? {
                                if let Ok(entry) = entry {
                                    if let Some(ext) = entry.path().extension() {
                                        if ext == "tex" {
                                            result_packages = parser.detect_missing_packages_by_compilation(&entry.path(), compile_cmd, &project_root)?;
                                            found_tex = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        
                        if !found_tex {
                            println!("No .tex files found in directory for compilation");
                        }
                        result_packages
                    }
                } else {
                    println!("Invalid compile command: no target file");
                    Vec::new()
                }
            } else {
                println!("Invalid compile command configuration");
                Vec::new()
            }
        };
        
        if missing_packages.is_empty() {
            println!("No missing packages detected from compilation.");
        } else {
            println!("Missing packages detected from compilation:");
            for package in &missing_packages {
                println!("  - {}", package);
            }
            println!("\nRun 'tpmgr install' to install missing packages");
        }
        
        if config.project.compile.auto_clean {
            println!("🧹 Cleaning intermediate files...");
            let project_root = std::env::current_dir()?;
            clean_intermediate_files(&project_root)?;
        }
        
        return Ok(());
    }
    
    // Use regex parsing
    let dependencies = if path.is_file() {
        parser.parse_file(path)?
    } else {
        parser.parse_project(path)?
    };
    
    if verbose {
        TeXParser::print_dependency_analysis(&dependencies);
    }
    
    let packages = TeXParser::get_unique_packages(&dependencies);
    let filtered_packages = TeXParser::filter_core_packages(&packages);
    
    if !filtered_packages.is_empty() {
        println!("\nRequired packages:");
        for package in &filtered_packages {
            println!("  - {}", package);
        }
        
        let mut texlive = TeXLiveManager::new();
        let texlive_available = texlive.detect_texlive().is_ok();
        if texlive_available {
            texlive.scan_installed_packages()?;
        }
        
        let local_manager = PackageManager::new(false)?;
        
        let mut missing_packages = Vec::new();
        let mut installed_packages = Vec::new();
        
        for package in &filtered_packages {
            let mut is_available = false;
            
            // First check system-level TeXLive installation
            if texlive_available && texlive.is_package_installed(package) {
                is_available = true;
            }
            
            // Then check local project installation
            if !is_available {
                if let Ok(true) = local_manager.is_package_installed(package).await {
                    is_available = true;
                }
            }
            
            if is_available {
                installed_packages.push(package);
            } else {
                missing_packages.push(package);
            }
        }
        
        if !installed_packages.is_empty() {
            println!("\nAlready installed:");
            for package in installed_packages {
                println!("  ✓ {}", package);
            }
        }
        
        if !missing_packages.is_empty() {
            println!("\nMissing packages:");
            for package in missing_packages {
                println!("  ✗ {}", package);
            }
            println!("\nRun 'tpmgr install' to install missing packages");
        } else {
            println!("\n✓ All required packages are already installed!");
        }
    } else {
        println!("No external packages required.");
    }
    
    // Clean intermediate files if using compilation analysis
    if use_compile {
        if let Ok(config) = Config::load("tpmgr.toml") {
            if config.project.compile.auto_clean {
                println!("🧹 Cleaning intermediate files...");
                let project_root = std::env::current_dir()?;
                clean_intermediate_files(&project_root)?;
            }
        }
    }
    
    Ok(())
}

async fn auto_install_missing_packages(path: &str, use_compile: bool) -> Result<()> {
    let parser = TeXParser::new()?;
    let path = Path::new(path);
    
    println!("Auto-installing packages for: {}", path.display());
    
    let mut missing_packages = Vec::new();
    
    // If compile detection is enabled, use compile error detection
    if use_compile {
        // Read compile command from configuration
        let config = if Path::new("tpmgr.toml").exists() {
            Config::load("tpmgr.toml")?
        } else {
            Config::new()
        };
        
        let compile_cmd = &config.project.compile;
        let project_root = std::env::current_dir()?;
        
        if path.is_file() {
            missing_packages = parser.detect_missing_packages_by_compilation(path, compile_cmd, &project_root)?;
        } else {
            // For directories, first try to extract target files from compile commands  
            let resolved_commands = compile_cmd.resolve_variables(&project_root)?;
            if !resolved_commands.is_empty() {
                // Get target file from the last command
                let last_command = resolved_commands.last().unwrap();
                if let Some(potential_target) = last_command.last() {
                    let target_path = Path::new(potential_target);
                    if target_path.exists() {
                        missing_packages = parser.detect_missing_packages_by_compilation(&target_path, compile_cmd, &project_root)?;
                    } else {
                        println!("Target file specified in compile command not found: {}", potential_target);
                        // As a fallback, try to find .tex files
                        let mut found_tex = false;
                        
                        // First check src directory
                        let src_dir = path.join("src");
                        if src_dir.exists() {
                            for entry in std::fs::read_dir(&src_dir)? {
                                if let Ok(entry) = entry {
                                    if let Some(ext) = entry.path().extension() {
                                        if ext == "tex" {
                                            missing_packages = parser.detect_missing_packages_by_compilation(&entry.path(), compile_cmd, &project_root)?;
                                            found_tex = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        
                        // If not found in src directory, check current directory
                        if !found_tex {
                            for entry in std::fs::read_dir(path)? {
                                if let Ok(entry) = entry {
                                    if let Some(ext) = entry.path().extension() {
                                        if ext == "tex" {
                                            missing_packages = parser.detect_missing_packages_by_compilation(&entry.path(), compile_cmd, &project_root)?;
                                            found_tex = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        
                        if !found_tex {
                            println!("No .tex files found in directory for compilation");
                        }
                    }
                } else {
                    println!("Invalid compile command: no target file");
                }
            } else {
                println!("Invalid compile command configuration");
            }
        }
    } else {
        // Use regex parsing
        let dependencies = if path.is_file() {
            parser.parse_file(path)?
        } else {
            parser.parse_project(path)?
        };
        
        let packages = TeXParser::get_unique_packages(&dependencies);
        let filtered_packages = TeXParser::filter_core_packages(&packages);
        
        if filtered_packages.is_empty() {
            println!("No packages need to be installed.");
            return Ok(());
        }
        
        // Check TeXLive installation
        let mut texlive = TeXLiveManager::new();
        texlive.detect_texlive()?;
        texlive.scan_installed_packages()?;
        
        // Find missing packages
        for package in &filtered_packages {
            if !texlive.is_package_installed(package) {
                missing_packages.push(package.clone());
            }
        }
    }
    
    if missing_packages.is_empty() {
        println!("✓ All required packages are already installed!");
        return Ok(());
    }
    
    println!("Found {} missing packages:", missing_packages.len());
    for package in &missing_packages {
        println!("  - {}", package);
    }
    
    // Check configuration to determine installation location
    let _config = if Path::new("tpmgr.toml").exists() {
        Config::load("tpmgr.toml")?
    } else {
        Config::new()
    };
    
    // Here can decide whether to install globally or locally based on configuration
    let global = false; // Default local project installation
    
    // Install missing packages
    let manager = PackageManager::new(global)?;
    let mut any_installed = false;
    
    for package in &missing_packages {
        println!("Installing {}...", package);
        match manager.install(package).await {
            Ok(_) => {
                println!("  ✓ {} installed successfully", package);
                any_installed = true;
            },
            Err(e) => println!("  ✗ Failed to install {}: {}", package, e),
        }
    }
    
    // Auto-clean cache after installation
    if any_installed {
        println!("Auto-cleaning package cache...");
        if let Err(e) = manager.clean_cache().await {
            println!("Warning: Failed to clean cache: {}", e);
        } else {
            println!("✓ Package cache cleaned");
        }
    }
    
    // Update filename database
    if !global {
        println!("Updating filename database...");
        let texlive = TeXLiveManager::new();
        if let Err(e) = texlive.update_filename_database() {
            println!("Warning: Failed to update filename database: {}", e);
        } else {
            println!("Filename database updated successfully");
        }
    }
    
    println!("✓ Auto-installation completed!");
    
    // Clean intermediate files if using compilation and auto_clean is enabled
    if use_compile {
        if let Ok(config) = Config::load("tpmgr.toml") {
            if config.project.compile.auto_clean {
                println!("🧹 Cleaning intermediate files...");
                let project_root = std::env::current_dir()?;
                clean_intermediate_files(&project_root)?;
            }
        }
    }
    
    Ok(())
}

pub async fn config_command(action: &ConfigAction) -> Result<()> {
    use crate::config::GlobalConfig;
    
    match action {
        ConfigAction::Show { global } => {
            // Display global configuration
            let global_config = GlobalConfig::load()?;
            println!("Global Configuration:");
            println!("  texlive_path: {}", 
                global_config.texlive_path.as_ref().unwrap_or(&"<not set>".to_string()));
            println!("  mirror_url: {}", 
                global_config.mirror_url.as_ref().unwrap_or(&"<not set>".to_string()));
            println!("  compile_command: {}", global_config.compile_command);
            println!("  install_global: {}", global_config.install_global);
            
            // If project configuration exists and not global-only, also display project configuration
            if !global && Path::new("tpmgr.toml").exists() {
                let project_config = Config::load("tpmgr.toml")?;
                println!("\nProject Configuration:");
                println!("  name: {}", project_config.project.name);
                println!("  version: {}", project_config.project.version);
                println!("  compile: {}", project_config.project.compile);
                println!("  package_dir: {}", project_config.project.package_dir);
                println!("  texlive_path: {}", 
                    project_config.project.texlive_path.as_ref().unwrap_or(&"<not set>".to_string()));
                println!("  mirror_url: {}", 
                    project_config.project.mirror_url.as_ref().unwrap_or(&"<not set>".to_string()));
                println!("  install_global: {}", 
                    project_config.project.install_global.map(|b| b.to_string()).unwrap_or_else(|| "<not set>".to_string()));
            }
        }
        ConfigAction::Set { key, value, global } => {
            if *global {
                // Force set global config
                let mut global_config = GlobalConfig::load()?;
                global_config.set(key, value)?;
                global_config.save()?;
                println!("✓ Set global {} = {}", key, value);
            } else {
                // If in project directory and key belongs to project config, set project config
                if Path::new("tpmgr.toml").exists() && Config::list_project_keys().contains(&key.as_str()) {
                    let mut config = Config::load("tpmgr.toml")?;
                    config.set_project_config(key, value)?;
                    config.save("tpmgr.toml")?;
                    println!("✓ Set project {} = {}", key, value);
                    
                    // If mirror URL, equivalent to executing mirror use
                    if key == "mirror_url" {
                        println!("  (Mirror URL updated for this project)");
                    }
                } else {
                    // Otherwise set global config
                    let mut global_config = GlobalConfig::load()?;
                    global_config.set(key, value)?;
                    global_config.save()?;
                    println!("✓ Set global {} = {}", key, value);
                }
            }
        }
        ConfigAction::Get { key, global } => {
            if *global {
                // Get from global config only
                let global_config = GlobalConfig::load()?;
                if let Some(value) = global_config.get(key) {
                    println!("{}", value);
                } else {
                    println!("Global configuration key '{}' not found", key);
                }
            } else {
                // Get from project config first, then from global config
                let mut found = false;
                
                if Path::new("tpmgr.toml").exists() {
                    let project_config = Config::load("tpmgr.toml")?;
                    if let Some(value) = project_config.get_project_config(key) {
                        println!("{}", value);
                        found = true;
                    }
                }
                
                if !found {
                    let global_config = GlobalConfig::load()?;
                    if let Some(value) = global_config.get(key) {
                        println!("{}", value);
                    } else {
                        println!("Configuration key '{}' not found", key);
                    }
                }
            }
        }
        ConfigAction::List { global } => {
            if *global {
                // Show global configuration keys only
                println!("Available global configuration keys:");
                for key in GlobalConfig::list_keys() {
                    println!("  - {}", key);
                }
            } else {
                println!("Available global configuration keys:");
                for key in GlobalConfig::list_keys() {
                    println!("  - {}", key);
                }
                
                if Path::new("tpmgr.toml").exists() {
                    println!("\nAvailable project configuration keys:");
                    for key in Config::list_project_keys() {
                        println!("  - {}", key);
                    }
                } else {
                    println!("\nNote: Run 'tpmgr init' to create a project and access project-specific configuration.");
                }
            }
        }
        ConfigAction::Reset { global } => {
            if *global {
                // Reset global configuration only
                let global_config = GlobalConfig::new();
                global_config.save()?;
                println!("✓ Global configuration reset to defaults");
            } else {
                // Reset both global and project configuration
                let global_config = GlobalConfig::new();
                global_config.save()?;
                println!("✓ Global configuration reset to defaults");
                
                if Path::new("tpmgr.toml").exists() {
                    let project_config = Config::new();
                    project_config.save("tpmgr.toml")?;
                    println!("✓ Project configuration reset to defaults");
                }
            }
        }
    }
    Ok(())
}

pub async fn compile_command(path: &str, clean: bool, verbose: bool) -> Result<()> {
    use std::process::Command;
    
    let path = Path::new(path);
    let project_root = if path.is_file() {
        path.parent().unwrap_or(Path::new(".")).to_path_buf()
    } else {
        path.to_path_buf()
    };
    
    // Change to project directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&project_root)?;
    
    // Load configuration
    let config = if Path::new("tpmgr.toml").exists() {
        Config::load("tpmgr.toml")?
    } else {
        println!("⚠️  No tpmgr.toml found in {}. Using default compilation settings.", project_root.display());
        Config::new()
    };
    
    println!("📄 Compiling LaTeX project in: {}", project_root.display());
    
    // Setup TEXINPUTS environment variable for local packages
    let package_manager = PackageManager::new(false)?;
    let packages_dir = project_root.join("packages");
    
    if packages_dir.exists() {
        let package_texinputs = package_manager.get_texinputs_path();
        
        // Add current directory and parent search paths
        #[cfg(windows)]
        let separator = ";";
        #[cfg(unix)]
        let separator = ":";
        
        // Get existing TEXINPUTS if any, preserve system paths
        let existing_texinputs = std::env::var("TEXINPUTS").unwrap_or_default();
        
        // Construct TEXINPUTS: current dir + package dirs + existing paths
        let texinputs = if existing_texinputs.is_empty() {
            format!(".{}{}{}", separator, package_texinputs, separator)
        } else {
            format!(".{}{}{}{}", separator, package_texinputs, separator, existing_texinputs)
        };
        
        if verbose {
            println!("📦 Setting TEXINPUTS: {}", texinputs);
        }
        
        // Set environment variable for all child processes
        std::env::set_var("TEXINPUTS", &texinputs);
    }
    
    // Resolve compilation commands
    let resolved_commands = config.project.compile.resolve_variables(&project_root)?;
    
    if resolved_commands.is_empty() {
        println!("❌ No compilation steps defined. Configure compilation chain in tpmgr.toml");
        return Ok(());
    }
    
    println!("🔗 Compilation chain ({} steps):", resolved_commands.len());
    for (i, cmd) in resolved_commands.iter().enumerate() {
        println!("  {}. {}", i + 1, cmd.join(" "));
    }
    println!();
    
    // Execute compilation steps
    let mut success = true;
    for (i, cmd_args) in resolved_commands.iter().enumerate() {
        if cmd_args.is_empty() {
            continue;
        }
        
        let tool = &cmd_args[0];
        let args = &cmd_args[1..];
        
        println!("⚙️  Step {}/{}: Running {}", i + 1, resolved_commands.len(), tool);
        
        if verbose {
            println!("   Command: {}", cmd_args.join(" "));
        }
        
        let mut command = Command::new(tool);
        command.args(args);
        
        if !verbose {
            command.stdout(std::process::Stdio::null());
            command.stderr(std::process::Stdio::null());
        }
        
        match command.status() {
            Ok(status) => {
                if status.success() {
                    println!("✅ Step {}/{} completed", i + 1, resolved_commands.len());
                } else {
                    println!("❌ Step {}/{} failed with exit code: {:?}", i + 1, resolved_commands.len(), status.code());
                    success = false;
                    break;
                }
            }
            Err(e) => {
                println!("❌ Failed to execute {}: {}", tool, e);
                println!("   Make sure {} is installed and available in PATH", tool);
                success = false;
                break;
            }
        }
    }
    
    if success {
        println!("🎉 Compilation completed successfully!");
        
        // Clean intermediate files if requested via command line or config
        if clean || config.project.compile.auto_clean {
            println!("🧹 Cleaning intermediate files...");
            clean_intermediate_files(&project_root)?;
        }
    } else {
        println!("💥 Compilation failed!");
        
        // Clean intermediate files if explicitly requested via command line
        if clean {
            println!("🧹 Cleaning intermediate files...");
            clean_intermediate_files(&project_root)?;
        }
    }
    
    // Restore original directory
    std::env::set_current_dir(original_dir)?;
    
    Ok(())
}

fn clean_intermediate_files(project_root: &Path) -> Result<()> {
    // Try to load patterns from config, fall back to defaults
    let patterns = if let Ok(config) = Config::load("tpmgr.toml") {
        if config.project.compile.clean_patterns.is_empty() {
            // Use default patterns if none specified
            vec![
                "*.aux".to_string(), "*.log".to_string(), "*.out".to_string(), 
                "*.toc".to_string(), "*.lof".to_string(), "*.lot".to_string(), 
                "*.bbl".to_string(), "*.blg".to_string(), "*.fls".to_string(), 
                "*.fdb_latexmk".to_string(), "*.synctex.gz".to_string(), 
                "*.nav".to_string(), "*.snm".to_string(), "*.vrb".to_string(),
                "*.run.xml".to_string(), "*.bcf".to_string(), "*.idx".to_string(), 
                "*.ind".to_string(), "*.ilg".to_string(), "*.glo".to_string(), 
                "*.gls".to_string(), "*.glg".to_string(), "*.auxlock".to_string(),
            ]
        } else {
            config.project.compile.clean_patterns
        }
    } else {
        // Default patterns when no config file
        vec![
            "*.aux".to_string(), "*.log".to_string(), "*.out".to_string(), 
            "*.toc".to_string(), "*.lof".to_string(), "*.lot".to_string(), 
            "*.bbl".to_string(), "*.blg".to_string(), "*.fls".to_string(), 
            "*.fdb_latexmk".to_string(), "*.synctex.gz".to_string(), 
            "*.nav".to_string(), "*.snm".to_string(), "*.vrb".to_string(),
            "*.run.xml".to_string(), "*.bcf".to_string(), "*.idx".to_string(), 
            "*.ind".to_string(), "*.ilg".to_string(), "*.glo".to_string(), 
            "*.gls".to_string(), "*.glg".to_string(), "*.auxlock".to_string(),
        ]
    };
    
    clean_files_by_patterns(project_root, &patterns)
}

fn clean_files_by_patterns(project_root: &Path, patterns: &[String]) -> Result<()> {
    let mut cleaned_count = 0;
    
    for pattern in patterns {
        // Convert pattern to absolute path relative to project root
        let full_pattern = if pattern.starts_with('/') || pattern.contains(':') {
            // Absolute pattern
            pattern.clone()
        } else {
            // Relative pattern - make it relative to project root
            project_root.join(pattern).to_string_lossy().to_string()
        };
        
        // Use glob to find matching files
        match glob::glob(&full_pattern) {
            Ok(paths) => {
                for path_result in paths {
                    match path_result {
                        Ok(path) => {
                            if path.is_file() {
                                match std::fs::remove_file(&path) {
                                    Ok(_) => {
                                        // Show relative path from project root
                                        let relative_path = path.strip_prefix(project_root)
                                            .unwrap_or(&path);
                                        println!("   Removed: {}", relative_path.display());
                                        cleaned_count += 1;
                                    }
                                    Err(e) => {
                                        println!("   Warning: Failed to remove {}: {}", path.display(), e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("   Warning: Pattern error for {}: {}", full_pattern, e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("   Warning: Invalid glob pattern '{}': {}", full_pattern, e);
            }
        }
    }
    
    if cleaned_count > 0 {
        println!("✅ Cleaned {} intermediate files", cleaned_count);
    } else {
        println!("   No intermediate files to clean");
    }
    
    Ok(())
}













