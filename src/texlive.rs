use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeXLiveInfo {
    pub version: String,
    pub install_path: PathBuf,
    pub texmf_dist: PathBuf,
    pub texmf_local: PathBuf,
    pub texmf_home: PathBuf,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub files: Vec<PathBuf>,
    pub install_path: PathBuf,
}

pub struct TeXLiveManager {
    texlive_info: Option<TeXLiveInfo>,
    installed_packages: HashMap<String, InstalledPackage>,
}

impl TeXLiveManager {
    pub fn new() -> Self {
        Self {
            texlive_info: None,
            installed_packages: HashMap::new(),
        }
    }

    /// Automatically detect TeXLive installation
    pub fn detect_texlive(&mut self) -> Result<()> {
        println!("Detecting TeXLive installation...");

        let texmf_root = self.find_texlive_root()?;
        
        // Temporarily set texlive_info for version detection methods
        self.texlive_info = Some(TeXLiveInfo {
            version: "Unknown".to_string(),
            install_path: texmf_root.clone(),
            texmf_dist: texmf_root.join("texmf-dist"),
            texmf_local: texmf_root.join("texmf-local"),
            texmf_home: self.get_texmf_home()?,
        });
        
        let version = self.get_texlive_version()?;

        // Build TeXLive information
        let texlive_info = TeXLiveInfo {
            version: version.clone(),
            install_path: texmf_root.clone(),
            texmf_dist: texmf_root.join("texmf-dist"),
            texmf_local: texmf_root.join("texmf-local"),
            texmf_home: self.get_texmf_home()?,
        };

        println!("Found TeXLive {} at: {}", version, texmf_root.display());
        self.texlive_info = Some(texlive_info);
        Ok(())
    }

    /// Unified entry point for finding TeXLive root directory
    fn find_texlive_root(&self) -> Result<PathBuf> {
        // 1. First check environment variables
        if let Ok(path) = self.find_texlive_from_env_vars() {
            return Ok(path);
        }

        // 2. Try to detect TeXLive path through kpsewhich
        if let Ok(path) = self.find_texlive_from_kpsewhich() {
            return Ok(path);
        }

        // 3. Search common installation paths
        self.find_texlive_in_common_paths()
    }

    /// Find TeXLive from environment variables
    fn find_texlive_from_env_vars(&self) -> Result<PathBuf> {
        // Check various possible environment variables
        let env_vars = [
            "TEXLIVE_ROOT",
            "TEXMFROOT", 
            "TEXLIVE_INSTALL_PREFIX",
            "TEX_ROOT",
            "TEXLIVE_PATH",
        ];

        for var_name in &env_vars {
            if let Ok(path_str) = std::env::var(var_name) {
                let path = PathBuf::from(path_str);
                if self.is_valid_texlive_installation(&path) {
                    println!("Found TeXLive via environment variable {}", var_name);
                    return Ok(path);
                }
            }
        }

        anyhow::bail!("TeXLive not found via environment variables")
    }

    /// Find TeXLive through kpsewhich
    fn find_texlive_from_kpsewhich(&self) -> Result<PathBuf> {
        let output = Command::new("kpsewhich")
            .args(["--var-value", "TEXMFROOT"])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let path_str = output_str.trim();
                let path = PathBuf::from(path_str);
                
                if self.is_valid_texlive_installation(&path) {
                    return Ok(path);
                }
            }
            _ => {}
        }

        anyhow::bail!("TeXLive not found via kpsewhich")
    }

    /// Find TeXLive in common paths
    fn find_texlive_in_common_paths(&self) -> Result<PathBuf> {
        if cfg!(windows) {
            // Windows: First try registry
            if let Ok(path) = self.find_texlive_from_registry() {
                return Ok(path);
            }
            
            // Then scan common root directories
            self.scan_texlive_directories(&[
                "C:\\texlive",
                "C:\\Program Files\\texlive",
                "C:\\Program Files (x86)\\texlive",
            ])
        } else if cfg!(target_os = "macos") {
            // macOS: Check common paths and Homebrew installation
            self.scan_texlive_directories(&[
                "/usr/local/texlive",
                "/opt/homebrew/texlive",
                "/Library/TeX/texlive",
                "/usr/local/Cellar/texlive",
            ])
        } else {
            // Linux and other Unix systems
            self.scan_texlive_directories(&[
                "/usr/local/texlive",
                "/opt/texlive",
                "/usr/share/texlive",
                &format!("{}/texlive", std::env::var("HOME").unwrap_or_default()),
            ])
        }
    }

    /// Windows注册表查找TeXLive
    #[cfg(windows)]
    fn find_texlive_from_registry(&self) -> Result<PathBuf> {
        use std::process::Command;
        
        // 查询注册表中的TeXLive安装信息
        let output = Command::new("reg")
            .args([
                "query",
                "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
                "/s",
                "/f",
                "TeXLive",
                "/t",
                "REG_SZ",
            ])
            .output()?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // 解析注册表输出，查找InstallLocation
            for line in output_str.lines() {
                if line.contains("InstallLocation") {
                    if let Some(path_start) = line.find("REG_SZ") {
                        let path_str = line[path_start + 6..].trim();
                        let path = PathBuf::from(path_str);
                        if self.is_valid_texlive_installation(&path) {
                            return Ok(path);
                        }
                    }
                }
            }
        }

        anyhow::bail!("TeXLive not found in Windows registry")
    }

    #[cfg(not(windows))]
    fn find_texlive_from_registry(&self) -> Result<PathBuf> {
        anyhow::bail!("Registry lookup not available on this platform")
    }

    /// 扫描目录查找TeXLive安装
    fn scan_texlive_directories(&self, base_paths: &[&str]) -> Result<PathBuf> {
        for base_path in base_paths {
            let base = PathBuf::from(base_path);
            if !base.exists() {
                continue;
            }

            // 扫描年份目录
            if let Ok(entries) = std::fs::read_dir(&base) {
                let mut found_versions = Vec::new();
                
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(dir_name) = path.file_name() {
                            let name = dir_name.to_string_lossy();
                            // Check if matches year pattern (2020-2030)
                            if Self::is_texlive_year_dir(&name) && self.is_valid_texlive_installation(&path) {
                                found_versions.push((name.to_string(), path));
                            }
                        }
                    }
                }

                // 返回最新版本
                if !found_versions.is_empty() {
                    found_versions.sort_by(|a, b| b.0.cmp(&a.0)); // 降序排列
                    return Ok(found_versions[0].1.clone());
                }
            }

            // 如果基础路径本身就是TeXLive安装目录
            if self.is_valid_texlive_installation(&base) {
                return Ok(base);
            }
        }

        anyhow::bail!("TeXLive installation not found in common directories")
    }

    /// Check if directory name matches TeXLive year pattern
    fn is_texlive_year_dir(name: &str) -> bool {
        if let Ok(year) = name.parse::<u32>() {
            year >= 2015 && year <= 2030 // 合理的TeXLive版本年份范围
        } else {
            false
        }
    }

    /// Validate if path is a valid TeXLive installation
    fn is_valid_texlive_installation(&self, path: &PathBuf) -> bool {
        // Check if key directories and files exist
        let _required_dirs = ["texmf-dist", "bin"];
        let _optional_dirs = ["tlpkg", "texmf-local"];
        
        // Must have at least texmf-dist directory
        if !path.join("texmf-dist").exists() {
            return false;
        }

        // Check bin directory (may have different architecture subdirectories)
        let bin_path = path.join("bin");
        if bin_path.exists() {
            if let Ok(entries) = std::fs::read_dir(&bin_path) {
                // Check if there are any subdirectories containing tex executable files
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        let tex_exe = if cfg!(windows) {
                            entry.path().join("tex.exe")
                        } else {
                            entry.path().join("tex")
                        };
                        if tex_exe.exists() {
                            return true;
                        }
                    }
                }
            }
        }

        // 如果没有找到tex可执行文件，但有texmf-dist，可能是不完整的安装
        // 仍然认为是有效的，但会在后续使用中遇到问题
        true
    }

    /// 获取TeXLive版本
    fn get_texlive_version(&self) -> Result<String> {
        // 首先尝试通过tex命令获取版本
        if let Ok(version) = self.get_version_from_tex_command() {
            return Ok(version);
        }

        // 如果tex命令不可用，尝试从路径推断版本
        if let Ok(version) = self.get_version_from_path() {
            return Ok(version);
        }

        // 最后尝试从tlpdb文件获取版本信息
        if let Ok(version) = self.get_version_from_tlpdb() {
            return Ok(version);
        }

        Ok("Unknown".to_string())
    }

    /// 通过tex命令获取版本
    fn get_version_from_tex_command(&self) -> Result<String> {
        let output = Command::new("tex")
            .arg("--version")
            .output()?;

        if output.status.success() {
            let version_output = String::from_utf8_lossy(&output.stdout);
            // 解析版本信息，通常格式为 "TeX 3.141592653 (TeX Live 2023)"
            if let Some(start) = version_output.find("TeX Live ") {
                let version_part = &version_output[start + 9..];
                if let Some(end) = version_part.find(')') {
                    return Ok(version_part[..end].to_string());
                }
            }
        }

        anyhow::bail!("Could not get version from tex command")
    }

    /// 从安装路径推断版本
    fn get_version_from_path(&self) -> Result<String> {
        if let Some(info) = &self.texlive_info {
            let _path_str = info.install_path.to_string_lossy();
            
            // 尝试从路径中提取年份
            for component in info.install_path.components() {
                if let Some(component_str) = component.as_os_str().to_str() {
                    if Self::is_texlive_year_dir(component_str) {
                        return Ok(component_str.to_string());
                    }
                }
            }
        }

        anyhow::bail!("Could not infer version from path")
    }

    /// 从TLPDB文件获取版本信息
    fn get_version_from_tlpdb(&self) -> Result<String> {
        if let Some(info) = &self.texlive_info {
            let tlpdb_path = info.install_path.join("tlpkg/texlive.tlpdb");
            
            if tlpdb_path.exists() {
                let content = std::fs::read_to_string(&tlpdb_path)?;
                
                // 查找00texlive.installation包，它包含版本信息
                for line in content.lines() {
                    if line.starts_with("name 00texlive.installation") {
                        // 继续读取后续行查找版本信息
                        continue;
                    }
                    if line.starts_with("depend release") {
                        if let Some(version_start) = line.find("release/") {
                            let version_part = &line[version_start + 8..];
                            if let Some(version_end) = version_part.find(' ') {
                                return Ok(version_part[..version_end].to_string());
                            } else {
                                return Ok(version_part.to_string());
                            }
                        }
                    }
                }
            }
        }

        anyhow::bail!("Could not get version from TLPDB")
    }

    /// 获取用户的TEXMF目录
    fn get_texmf_home(&self) -> Result<PathBuf> {
        let output = Command::new("kpsewhich")
            .args(["--var-value", "TEXMFHOME"])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let path_str = output_str.trim();
                Ok(PathBuf::from(path_str))
            }
            _ => {
                // 默认路径
                let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
                Ok(home.join("texmf"))
            }
        }
    }

    /// 扫描已安装的包
    pub fn scan_installed_packages(&mut self) -> Result<()> {
        if self.texlive_info.is_none() {
            self.detect_texlive()?;
        }

        let texlive_info = self.texlive_info.as_ref().unwrap();
        println!("Scanning installed packages...");

        // 读取TeXLive包数据库
        let tlpdb_path = texlive_info.install_path.join("tlpkg/texlive.tlpdb");
        
        if tlpdb_path.exists() {
            self.parse_tlpdb(&tlpdb_path)?;
        } else {
            println!("Warning: TeXLive package database not found at {}", tlpdb_path.display());
            // 作为备选方案，扫描文件系统
            self.scan_filesystem_packages()?;
        }

        println!("Found {} installed packages", self.installed_packages.len());
        Ok(())
    }

    /// 解析TeXLive包数据库
    fn parse_tlpdb(&mut self, tlpdb_path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(tlpdb_path)?;
        let mut current_package: Option<String> = None;
        let mut current_description = String::new();
        let mut current_files = Vec::new();

        for line in content.lines() {
            if line.starts_with("name ") {
                // 保存前一个包
                if let Some(name) = current_package.take() {
                    self.add_package_from_tlpdb(name, current_description.clone(), current_files.clone());
                }

                // 开始新包
                current_package = Some(line[5..].to_string());
                current_description.clear();
                current_files.clear();
            } else if line.starts_with("shortdesc ") {
                current_description = line[10..].to_string();
            } else if line.starts_with(" ") && line.contains('/') {
                // 文件路径
                let file_path = line.trim();
                if let Some(texlive_info) = &self.texlive_info {
                    current_files.push(texlive_info.texmf_dist.join(file_path));
                }
            }
        }

        // 保存最后一个包
        if let Some(name) = current_package {
            self.add_package_from_tlpdb(name, current_description, current_files);
        }

        Ok(())
    }

    fn add_package_from_tlpdb(&mut self, name: String, description: String, files: Vec<PathBuf>) {
        let package = InstalledPackage {
            name: name.clone(),
            version: "unknown".to_string(), // TLPDB通常不包含版本信息
            description,
            files,
            install_path: self.texlive_info.as_ref().unwrap().texmf_dist.clone(),
        };
        self.installed_packages.insert(name, package);
    }

    /// 备选方案：扫描文件系统中的包
    fn scan_filesystem_packages(&mut self) -> Result<()> {
        // 这里可以实现文件系统扫描逻辑
        // 暂时留空，因为TLPDB解析是更可靠的方法
        Ok(())
    }

    /// Check if a package is installed
    pub fn is_package_installed(&self, package_name: &str) -> bool {
        self.installed_packages.contains_key(package_name)
    }

    /// 获取已安装包的信息
    #[allow(dead_code)]
    pub fn get_installed_package(&self, package_name: &str) -> Option<&InstalledPackage> {
        self.installed_packages.get(package_name)
    }

    /// 列出所有已安装的包
    #[allow(dead_code)]
    pub fn list_installed_packages(&self) -> Vec<&InstalledPackage> {
        self.installed_packages.values().collect()
    }

    /// 获取TeXLive信息
    #[allow(dead_code)]
    pub fn get_texlive_info(&self) -> Option<&TeXLiveInfo> {
        self.texlive_info.as_ref()
    }

    /// 获取包的安装路径（项目本地或全局）
    #[allow(dead_code)]
    pub fn get_package_install_path(&self, global: bool) -> Result<PathBuf> {
        if global {
            if let Some(info) = &self.texlive_info {
                Ok(info.texmf_local.clone())
            } else {
                anyhow::bail!("TeXLive not detected")
            }
        } else {
            // 项目本地路径
            Ok(PathBuf::from("packages"))
        }
    }

    /// Update TEXMF filename database
    pub fn update_filename_database(&self) -> Result<()> {
        println!("Updating filename database...");
        
        let output = Command::new("mktexlsr").output();
        
        match output {
            Ok(output) if output.status.success() => {
                println!("Filename database updated successfully");
                Ok(())
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("Failed to update filename database: {}", stderr);
            }
            Err(e) => {
                anyhow::bail!("Failed to run mktexlsr: {}", e);
            }
        }
    }
}


