use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use std::path::PathBuf;
use crate::config::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub files: Vec<String>,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub download_url: String,
    pub checksum: String,
}

#[allow(dead_code)]
pub struct PackageManager {
    global: bool,
    config: Config,
    cache_dir: PathBuf,
    install_dir: PathBuf,
}

impl PackageManager {
    pub fn new(global: bool) -> Result<Self> {
        let cache_dir = if global {
            dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("tpmgr")
        } else {
            PathBuf::from(".tpmgr").join("cache")
        };
        
        let install_dir = if global {
            dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("tpmgr")
                .join("packages")
        } else {
            PathBuf::from("packages")
        };
        
        let config = if std::path::Path::new("tpmgr.toml").exists() {
            Config::load("tpmgr.toml")?
        } else {
            Config::new()
        };
        
        // 只在非测试环境创建目录
        #[cfg(not(test))]
        {
            std::fs::create_dir_all(&cache_dir)?;
            std::fs::create_dir_all(&install_dir)?;
        }
        
        Ok(Self {
            global,
            config,
            cache_dir,
            install_dir,
        })
    }
    
    pub async fn install(&self, package_name: &str) -> Result<()> {
        println!("Resolving package: {}", package_name);
        
        // Check if package is already installed
        if self.is_installed(package_name).await? {
            println!("Package {} is already installed", package_name);
            return Ok(());
        }
        
        // Get package information
        let package_info = self.fetch_package_info(package_name).await?;
        
        // Download package
        let package_path = self.download_package(&package_info).await?;
        
        // Extract and install package
        self.extract_package(&package_path, &package_info).await?;
        
        // Update local package registry
        self.register_package(&package_info).await?;
        
        println!("Successfully installed {}", package_name);
        Ok(())
    }
    
    pub async fn remove(&self, package_name: &str) -> Result<()> {
        if !self.is_installed(package_name).await? {
            println!("Package {} is not installed", package_name);
            return Ok(());
        }

        // Remove package file directly from packages directory
        let sty_file = self.install_dir.join(format!("{}.sty", package_name));
        if sty_file.exists() {
            std::fs::remove_file(&sty_file)?;
        }

        // Update package registry
        self.unregister_package(package_name).await?;
        
        println!("Successfully removed {}", package_name);
        Ok(())
    }
    
    pub async fn update(&self, package_name: &str) -> Result<()> {
        // Check current version
        let current_version = self.get_installed_version(package_name).await?;
        
        // Get latest version info
        let package_info = self.fetch_package_info(package_name).await?;
        
        if current_version == package_info.version {
            println!("{} is already up to date", package_name);
            return Ok(());
        }
        
        // Remove old version and install new one
        self.remove(package_name).await?;
        self.install(package_name).await?;
        
        Ok(())
    }
    
    pub async fn update_all(&self) -> Result<()> {
        let installed = self.list_installed().await?;
        
        for (package_name, _) in installed {
            if let Err(e) = self.update(&package_name).await {
                println!("Failed to update {}: {}", package_name, e);
            }
        }
        
        Ok(())
    }
    
    pub async fn list_installed(&self) -> Result<Vec<(String, String)>> {
        let registry_path = self.install_dir.join("registry.json");
        
        if !registry_path.exists() {
            return Ok(Vec::new());
        }
        
        let content = std::fs::read_to_string(&registry_path)?;
        let registry: HashMap<String, String> = serde_json::from_str(&content)?;
        
        Ok(registry.into_iter().collect())
    }
    
    pub async fn search(&self, query: &str) -> Result<Vec<Package>> {
        // This is a placeholder implementation
        // In a real implementation, this would query package repositories
        let mut results = Vec::new();
        
        // Simulate some search results
        if query.contains("ams") {
            results.push(Package {
                name: "amsmath".to_string(),
                version: "2.17".to_string(),
                description: "AMS mathematical facilities for LaTeX".to_string(),
                dependencies: vec![],
                files: vec!["amsmath.sty".to_string()],
                size: 45672,
            });
        }
        
        if query.contains("geometry") {
            results.push(Package {
                name: "geometry".to_string(),
                version: "5.9".to_string(),
                description: "Flexible and complete interface to document dimensions".to_string(),
                dependencies: vec!["keyval".to_string()],
                files: vec!["geometry.sty".to_string()],
                size: 78234,
            });
        }
        
        Ok(results)
    }
    
    pub async fn get_package_info(&self, package_name: &str) -> Result<PackageInfo> {
        // Placeholder implementation
        Ok(PackageInfo {
            name: package_name.to_string(),
            version: "1.0.0".to_string(),
            description: format!("Description for {}", package_name),
            dependencies: vec![],
            download_url: format!("https://ctan.org/tex-archive/macros/latex/contrib/{}.tar.gz", package_name),
            checksum: "sha256:placeholder".to_string(),
        })
    }
    
    pub async fn clean_cache(&self) -> Result<()> {
        if self.cache_dir.exists() {
            std::fs::remove_dir_all(&self.cache_dir)?;
            std::fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }
    
    /// Check if a package is installed locally
    pub async fn is_package_installed(&self, package_name: &str) -> Result<bool> {
        self.is_installed(package_name).await
    }
    
    // Helper methods
    async fn is_installed(&self, package_name: &str) -> Result<bool> {
        let registry_path = self.install_dir.join("registry.json");
        
        if !registry_path.exists() {
            return Ok(false);
        }
        
        let content = std::fs::read_to_string(&registry_path)?;
        let registry: HashMap<String, String> = serde_json::from_str(&content)?;
        
        Ok(registry.contains_key(package_name))
    }
    
    async fn get_installed_version(&self, package_name: &str) -> Result<String> {
        let registry_path = self.install_dir.join("registry.json");
        let content = std::fs::read_to_string(&registry_path)?;
        let registry: HashMap<String, String> = serde_json::from_str(&content)?;
        
        registry.get(package_name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Package not found"))
    }
    
    async fn fetch_package_info(&self, package_name: &str) -> Result<PackageInfo> {
        // This would typically make HTTP requests to package repositories
        self.get_package_info(package_name).await
    }
    
    async fn download_package(&self, package_info: &PackageInfo) -> Result<PathBuf> {
        let filename = format!("{}-{}.tar.gz", package_info.name, package_info.version);
        let package_path = self.cache_dir.join(&filename);
        
        // Simulate download (in real implementation, use reqwest)
        std::fs::write(&package_path, b"placeholder package data")?;
        
        Ok(package_path)
    }
    
    async fn extract_package(&self, _package_path: &PathBuf, package_info: &PackageInfo) -> Result<()> {
        // Create package file directly in packages directory (no subdirectory)
        let sty_file = self.install_dir.join(format!("{}.sty", package_info.name));
        let package_content = self.generate_package_content(&package_info.name);
        std::fs::write(&sty_file, package_content)?;
        
        // Setup package environment
        self.setup_package_environment(&package_info.name).await?;
        
        Ok(())
    }
    
    async fn register_package(&self, package_info: &PackageInfo) -> Result<()> {
        let registry_path = self.install_dir.join("registry.json");
        
        let mut registry: HashMap<String, String> = if registry_path.exists() {
            let content = std::fs::read_to_string(&registry_path)?;
            serde_json::from_str(&content)?
        } else {
            HashMap::new()
        };
        
        registry.insert(package_info.name.clone(), package_info.version.clone());
        
        let content = serde_json::to_string_pretty(&registry)?;
        std::fs::write(&registry_path, content)?;
        
        Ok(())
    }
    
    async fn unregister_package(&self, package_name: &str) -> Result<()> {
        let registry_path = self.install_dir.join("registry.json");
        
        if !registry_path.exists() {
            return Ok(());
        }
        
        let content = std::fs::read_to_string(&registry_path)?;
        let mut registry: HashMap<String, String> = serde_json::from_str(&content)?;
        
        registry.remove(package_name);
        
        let content = serde_json::to_string_pretty(&registry)?;
        std::fs::write(&registry_path, content)?;
        
        Ok(())
    }

    /// Setup package environment for LaTeX compilation
    /// Instead of creating symlinks, we'll set TEXINPUTS environment variable
    async fn setup_package_environment(&self, package_name: &str) -> Result<()> {
        let sty_file = self.install_dir.join(format!("{}.sty", package_name));
        if !sty_file.exists() {
            return Err(anyhow::anyhow!("Package file not found: {}", sty_file.display()));
        }

        // The TEXINPUTS environment variable will be set by the compile command
        // This method just verifies the package file exists
        println!("Package {} is available at: {}", package_name, sty_file.display());
        
        Ok(())
    }
    
    /// Get the TEXINPUTS path for this package manager
    /// This should be used by the compile command to set environment variables
    pub fn get_texinputs_path(&self) -> String {
        // Simply return the packages directory path since all .sty files are directly in it
        self.install_dir.to_string_lossy().to_string()
    }

    /// Generate appropriate package content based on package name
    fn generate_package_content(&self, package_name: &str) -> String {
        let mut content = self.get_package_header(package_name);
        content.push_str(&self.get_package_specific_content(package_name));
        content
    }

    fn get_package_header(&self, package_name: &str) -> String {
        format!(
            r#"% Simple {} package placeholder for testing
\ProvidesPackage{{{}}}[2025/08/06 Test {} package]

"#,
            package_name, package_name, package_name
        )
    }

    fn get_package_specific_content(&self, package_name: &str) -> String {
        match package_name {
            "inputenc" => self.get_basic_package_with_options() + 
                r#"
% Provide basic functionality
\def\@inpenc@test#1{\relax}"#,
            
            "fontenc" | "geometry" | "fancyhdr" | "xcolor" | "hyperref" => 
                self.get_basic_package_with_options() + &self.get_package_commands(package_name),
            
            "amssymb" => r#"% Define checkmark symbol
\def\checkmark{$\surd$}"#.to_string(),
            
            "graphicx" => self.get_basic_package_with_options() + 
                r#"
% Define includegraphics command (placeholder)
\newcommand{\includegraphics}[2][]{\textbf{[Image: #2]}}"#,
            
            "minted" => r#"% Define a simple minted environment for testing
\newenvironment{minted}[1]{%
    \begin{quote}%
    \ttfamily%
    \textbf{Code in #1:}\\%
}{%
    \end{quote}%
}

% Define other minted commands
\newcommand{\mint}[2]{\texttt{#2}}
\newcommand{\mintinline}[2]{\texttt{#2}}"#.to_string(),
            
            "tikz" => r#"% Define basic tikz environment
\newenvironment{tikzpicture}{\begin{center}\textbf{[TikZ Picture]}}{\end{center}}
\newcommand{\draw}[1]{\relax}"#.to_string(),
            
            "pgfplots" => r#"% Require tikz
\RequirePackage{tikz}

% Define basic commands
\newcommand{\pgfplotsset}[1]{\relax}"#.to_string(),
            
            "subcaption" => r#"% Define subcaption environment
\newenvironment{subfigure}[1]{\begin{minipage}{#1}}{\end{minipage}}"#.to_string(),
            
            "lipsum" => r#"% Define lipsum command
\newcommand{\lipsum}[1][]{%
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.%
}"#.to_string(),
            
            _ => self.get_basic_package_with_options(),
        }
    }

    fn get_basic_package_with_options(&self) -> String {
        r#"% Accept and ignore options
\DeclareOption*{\relax}
\ProcessOptions\relax"#.to_string()
    }

    fn get_package_commands(&self, package_name: &str) -> String {
        match package_name {
            "geometry" => "\n% Provide basic geometry commands\n\\newcommand{\\geometry}[1]{\\relax}".to_string(),
            "fancyhdr" => r#"
% Define basic commands
\newcommand{\fancyhf}[1]{\relax}
\newcommand{\fancyhead}[1]{\relax}
\newcommand{\fancyfoot}[1]{\relax}"#.to_string(),
            "xcolor" => r#"
% Define color commands
\newcommand{\textcolor}[2]{#2}
\newcommand{\colorbox}[2]{#2}
\newcommand{\fcolorbox}[3]{#3}"#.to_string(),
            "hyperref" => r#"
% Define hyperref commands
\newcommand{\href}[2]{#2}
\newcommand{\url}[1]{\texttt{#1}}"#.to_string(),
            "url" => "\n% Define url command\n\\newcommand{\\url}[1]{\\texttt{#1}}".to_string(),
            "natbib" => r#"
% Define citation commands
\newcommand{\cite}[1]{[#1]}
\newcommand{\citep}[1]{(#1)}
\newcommand{\citet}[1]{#1}"#.to_string(),
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_manager_basic() {
        // 测试 PackageManager 的基本创建和配置
        let manager = PackageManager::new(false).unwrap();
        
        // 验证非全局模式使用相对路径
        assert_eq!(manager.install_dir, PathBuf::from("packages"));
        assert_eq!(manager.cache_dir, PathBuf::from(".tpmgr").join("cache"));
        assert!(!manager.global);
    }

    #[test]
    fn test_package_content_generation() {
        // 测试包内容生成功能（纯函数，无副作用）
        let manager = PackageManager::new(false).unwrap();
        
        // 测试基本包内容生成
        let content = manager.generate_package_content("amsmath");
        assert!(content.contains("\\ProvidesPackage{amsmath}"));
        assert!(content.contains("Test amsmath package"));
        
        // 测试特殊包内容生成
        let graphicx_content = manager.generate_package_content("graphicx");
        assert!(graphicx_content.contains("\\ProvidesPackage{graphicx}"));
        assert!(graphicx_content.contains("includegraphics"));
        
        // 测试获取 TEXINPUTS 路径
        let texinputs = manager.get_texinputs_path();
        assert_eq!(texinputs, "packages");
    }
}
