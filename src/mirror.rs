use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mirror {
    pub name: String,
    pub url: String,
    pub country: String,
    pub location: String,
    pub continent: String,
    pub sponsor: String,
    pub http: bool,
    pub https: bool,
    pub rsync: bool,
    pub ftp: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorList {
    pub mirrors: Vec<Mirror>,
    pub last_updated: String,
}

pub struct MirrorManager {
    mirrors: Vec<Mirror>,
    selected_mirror: Option<Mirror>,
    client: reqwest::Client,
}

impl MirrorManager {
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
            selected_mirror: None,
            client: reqwest::Client::new(),
        }
    }

    /// 从CTAN获取镜像列表
    pub async fn fetch_mirrors(&mut self) -> Result<()> {
        println!("Fetching mirror list from CTAN...");
        
        // 如果API不可用，使用内置的镜像列表
        let builtin_mirrors = vec![
            Mirror {
                name: "CTAN Main".to_string(),
                url: "https://mirrors.ctan.org".to_string(),
                country: "Global".to_string(),
                location: "Global".to_string(),
                continent: "Global".to_string(),
                sponsor: "CTAN".to_string(),
                http: true,
                https: true,
                rsync: false,
                ftp: false,
            },
            Mirror {
                name: "USTC Mirror".to_string(),
                url: "https://mirrors.ustc.edu.cn/CTAN".to_string(),
                country: "China".to_string(),
                location: "Hefei".to_string(),
                continent: "Asia".to_string(),
                sponsor: "USTC".to_string(),
                http: true,
                https: true,
                rsync: false,
                ftp: false,
            },
            Mirror {
                name: "Tsinghua Mirror".to_string(),
                url: "https://mirrors.tuna.tsinghua.edu.cn/CTAN".to_string(),
                country: "China".to_string(),
                location: "Beijing".to_string(),
                continent: "Asia".to_string(),
                sponsor: "Tsinghua University".to_string(),
                http: true,
                https: true,
                rsync: false,
                ftp: false,
            },
            Mirror {
                name: "MIT Mirror".to_string(),
                url: "http://mirrors.mit.edu/CTAN".to_string(),
                country: "USA".to_string(),
                location: "Cambridge".to_string(),
                continent: "North America".to_string(),
                sponsor: "MIT".to_string(),
                http: true,
                https: false,
                rsync: false,
                ftp: false,
            },
        ];
        
        self.mirrors = builtin_mirrors;
        println!("Loaded {} mirrors", self.mirrors.len());
        Ok(())
    }

    /// 自动选择最佳镜像（基于地理位置和响应速度）
    pub async fn select_best_mirror(&mut self) -> Result<()> {
        if self.mirrors.is_empty() {
            self.fetch_mirrors().await?;
        }

        println!("Testing mirror response times...");
        let mut best_mirror: Option<Mirror> = None;
        let mut best_time = std::time::Duration::from_secs(10);

        // 测试前10个镜像的响应时间
        for mirror in self.mirrors.iter().take(10) {
            let test_url = format!("{}/systems/texlive/tlnet/", mirror.url);
            let start = std::time::Instant::now();
            
            match self.client.head(&test_url).timeout(std::time::Duration::from_secs(5)).send().await {
                Ok(response) if response.status().is_success() => {
                    let elapsed = start.elapsed();
                    if elapsed < best_time {
                        best_time = elapsed;
                        best_mirror = Some(mirror.clone());
                    }
                    println!("  {} ({}) - {}ms", mirror.name, mirror.country, elapsed.as_millis());
                }
                _ => {
                    println!("  {} ({}) - timeout/error", mirror.name, mirror.country);
                }
            }
        }

        if let Some(mirror) = best_mirror {
            println!("Selected mirror: {} ({})", mirror.name, mirror.country);
            self.selected_mirror = Some(mirror);
        } else {
            // 如果没有找到可用镜像，使用默认的CTAN镜像
            self.selected_mirror = Some(Mirror {
                name: "CTAN".to_string(),
                url: "https://mirror.ctan.org".to_string(),
                country: "Global".to_string(),
                location: "Global".to_string(),
                continent: "Global".to_string(),
                sponsor: "CTAN".to_string(),
                http: true,
                https: true,
                rsync: false,
                ftp: false,
            });
        }

        Ok(())
    }

    /// 手动选择镜像
    pub fn select_mirror_by_name(&mut self, name: &str) -> Result<()> {
        if let Some(mirror) = self.mirrors.iter().find(|m| m.name == name) {
            self.selected_mirror = Some(mirror.clone());
            println!("Selected mirror: {} ({})", mirror.name, mirror.country);
            Ok(())
        } else {
            anyhow::bail!("Mirror '{}' not found", name);
        }
    }

    /// 列出所有可用镜像
    pub fn list_mirrors(&self) {
        if self.mirrors.is_empty() {
            println!("No mirrors loaded. Run 'tpmgr mirror update' first.");
            return;
        }

        println!("Available mirrors:");
        for (i, mirror) in self.mirrors.iter().enumerate() {
            let selected = if let Some(ref selected) = self.selected_mirror {
                if selected.name == mirror.name { " (selected)" } else { "" }
            } else { "" };
            
            println!("  {}. {} ({}){}",
                i + 1,
                mirror.name,
                mirror.country,
                selected
            );
        }
    }

    /// 获取当前选择的镜像
    #[allow(dead_code)]
    pub fn get_selected_mirror(&self) -> Option<&Mirror> {
        self.selected_mirror.as_ref()
    }

    /// 获取包的下载URL
    #[allow(dead_code)]
    pub fn get_package_url(&self, package_name: &str) -> Option<String> {
        if let Some(mirror) = &self.selected_mirror {
            Some(format!("{}/systems/texlive/tlnet/archive/{}.tar.xz", 
                mirror.url, package_name))
        } else {
            None
        }
    }

    /// 获取包索引URL
    #[allow(dead_code)]
    pub fn get_package_index_url(&self) -> Option<String> {
        if let Some(mirror) = &self.selected_mirror {
            Some(format!("{}/systems/texlive/tlnet/tlpkg/texlive.tlpdb", mirror.url))
        } else {
            None
        }
    }
}
