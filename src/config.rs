use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use std::path::PathBuf;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CompileStep {
    pub tool: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompileCommand {
    pub steps: Vec<CompileStep>,
    #[serde(default)]
    pub auto_clean: bool,
    #[serde(default)]
    pub clean_patterns: Vec<String>,
}

impl CompileCommand {
    pub fn new() -> Self {
        Self {
            steps: vec![CompileStep {
                tool: "pdflatex".to_string(),
                args: vec!["-interaction=nonstopmode".to_string(), "${PROJECT_ROOT}/main.tex".to_string()],
            }],
            auto_clean: false,
            clean_patterns: Self::default_clean_patterns(),
        }
    }

    /// 获取默认的清理文件模式
    fn default_clean_patterns() -> Vec<String> {
        vec![
            "*.aux".to_string(),
            "*.log".to_string(),
            "*.out".to_string(),
            "*.toc".to_string(),
            "*.lot".to_string(),
            "*.lof".to_string(),
            "*.nav".to_string(),
            "*.snm".to_string(),
            "*.vrb".to_string(),
            "*.bbl".to_string(),
            "*.blg".to_string(),
            "*.idx".to_string(),
            "*.ind".to_string(),
            "*.ilg".to_string(),
            "*.glo".to_string(),
            "*.gls".to_string(),
            "*.ist".to_string(),
            "*.fls".to_string(),
            "*.fdb_latexmk".to_string(),
            "*.synctex.gz".to_string(),
            "*.synctex(busy)".to_string(),
            "*.pdfsync".to_string(),
            "*.figlist".to_string(),
            "*.makefile".to_string(),
            "*.figlist.bak".to_string(),
            "*.makefile.bak".to_string(),
            "*.thm".to_string(),
            "*.pyg".to_string(),
            "*.auxlock".to_string(),
            "*.bcf".to_string(),
            "*.run.xml".to_string(),
        ]
    }

    /// 从命令字符串创建CompileCommand
    /// 支持单个命令: "pdflatex -interaction=nonstopmode main.tex"
    /// 以及编译链: "pdflatex main.tex | bibtex main | pdflatex main.tex"
    pub fn from_string(command: &str) -> Result<Self> {
        let parts: Vec<&str> = command.split('|').map(|s| s.trim()).collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!("Empty compile command"));
        }

        let mut steps = Vec::new();

        for part in parts.iter() {
            let cmd_parts: Vec<&str> = part.split_whitespace().collect();
            if cmd_parts.is_empty() {
                continue;
            }

            let tool = cmd_parts[0].to_string();
            let args = cmd_parts.iter().skip(1).map(|s| s.to_string()).collect();
            steps.push(CompileStep { tool, args });
        }

        if steps.is_empty() {
            return Err(anyhow::anyhow!("No valid compile steps found"));
        }

        Ok(Self {
            steps,
            auto_clean: false,
            clean_patterns: Self::default_clean_patterns(),
        })
    }

    /// 从编译链配置创建CompileCommand (已弃用，使用from_string代替)
    /// 格式: "tool1 arg1 arg2 | tool2 arg3 arg4"
    #[deprecated(note = "Use from_string instead, which supports both single commands and chains")]
    #[allow(dead_code)]
    pub fn from_chain(chain: &str) -> Result<Self> {
        Self::from_string(chain)
    }

    pub fn to_string(&self) -> String {
        let steps_str: Vec<String> = self.steps.iter().map(|step| {
            let mut cmd = vec![step.tool.clone()];
            cmd.extend(step.args.clone());
            cmd.join(" ")
        }).collect();
        
        steps_str.join(" | ")
    }

    /// 解析魔法变量并构建实际的编译命令列表
    pub fn resolve_variables(&self, project_root: &std::path::Path) -> Result<Vec<Vec<String>>> {
        let mut resolved_commands = Vec::new();
        
        for step in &self.steps {
            let mut resolved_args = vec![step.tool.clone()];
            
            // 解析参数中的魔法变量
            for arg in &step.args {
                let resolved_arg = self.resolve_variables_in_string(arg, project_root)?;
                resolved_args.push(resolved_arg);
            }
            
            resolved_commands.push(resolved_args);
        }
        
        Ok(resolved_commands)
    }

    /// 解析字符串中的魔法变量
    fn resolve_variables_in_string(&self, input: &str, project_root: &std::path::Path) -> Result<String> {
        let mut resolved = input.to_string();
        
        // 替换 ${PROJECT_ROOT}
        if resolved.contains("${PROJECT_ROOT}") {
            let project_root_str = project_root.to_string_lossy();
            resolved = resolved.replace("${PROJECT_ROOT}", &project_root_str);
        }
        
        // 替换 ${CURRENT_DIR}
        if resolved.contains("${CURRENT_DIR}") {
            let current_dir = std::env::current_dir()?;
            let current_dir_str = current_dir.to_string_lossy();
            resolved = resolved.replace("${CURRENT_DIR}", &current_dir_str);
        }
        
        // 替换 ${HOME}
        if resolved.contains("${HOME}") {
            if let Some(home_dir) = dirs::home_dir() {
                let home_str = home_dir.to_string_lossy();
                resolved = resolved.replace("${HOME}", &home_str);
            }
        }
        
        Ok(resolved)
    }

    /// 获取支持的魔法变量列表
    #[allow(dead_code)]
    pub fn supported_variables() -> Vec<&'static str> {
        vec!["${PROJECT_ROOT}", "${CURRENT_DIR}", "${HOME}"]
    }
}

impl fmt::Display for CompileCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalConfig {
    pub texlive_path: Option<String>,
    pub mirror_url: Option<String>,
    pub compile_command: CompileCommand,
    pub install_global: bool,
}

impl GlobalConfig {
    pub fn new() -> Self {
        Self {
            texlive_path: None,
            mirror_url: None,
            compile_command: CompileCommand::new(),
            install_global: false,
        }
    }

    pub fn get_config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        path.push("tpmgr");
        std::fs::create_dir_all(&path)?;
        path.push("config.toml");
        Ok(path)
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_config_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_config_path()?;
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "texlive_path" => {
                if value.trim().is_empty() {
                    self.texlive_path = None;
                } else {
                    self.texlive_path = Some(value.to_string());
                }
            },
            "mirror_url" => {
                if value.trim().is_empty() {
                    self.mirror_url = None;
                } else {
                    self.mirror_url = Some(value.to_string());
                }
            },
            "compile_command" => self.compile_command = CompileCommand::from_string(value)?,
            "install_global" => self.install_global = value.parse()?,
            _ => return Err(anyhow::anyhow!("Unknown config key: {}", key)),
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "texlive_path" => self.texlive_path.clone(),
            "mirror_url" => self.mirror_url.clone(),
            "compile_command" => Some(self.compile_command.to_string()),
            "install_global" => Some(self.install_global.to_string()),
            _ => None,
        }
    }

    pub fn list_keys() -> Vec<&'static str> {
        vec!["texlive_path", "mirror_url", "compile_command", "install_global"]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub project: ProjectConfig,
    pub dependencies: HashMap<String, String>,
    pub repositories: Vec<Repository>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub compile: CompileCommand,
    pub package_dir: String,
    pub texlive_path: Option<String>,
    pub mirror_url: Option<String>,
    pub install_global: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub priority: u8,
}

impl Config {
    pub fn new() -> Self {
        Self {
            project: ProjectConfig {
                name: "latex-project".to_string(),
                version: "0.1.0".to_string(),
                compile: CompileCommand::new(),
                package_dir: "packages".to_string(),
                texlive_path: None,
                mirror_url: None,
                install_global: None,
            },
            dependencies: HashMap::new(),
            repositories: vec![
                Repository {
                    name: "ctan".to_string(),
                    url: "https://ctan.org/".to_string(),
                    priority: 1,
                },
                Repository {
                    name: "texlive".to_string(),
                    url: "https://mirror.ctan.org/systems/texlive/tlnet/".to_string(),
                    priority: 2,
                },
            ],
        }
    }
    
    pub fn load(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save(&self, path: &str) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn add_dependency(&mut self, name: String, version: String) {
        self.dependencies.insert(name, version);
    }
    
    #[allow(dead_code)]
    pub fn remove_dependency(&mut self, name: &str) -> Option<String> {
        self.dependencies.remove(name)
    }
    
    #[allow(dead_code)]
    pub fn get_package_dir(&self) -> &str {
        &self.project.package_dir
    }

    /// 设置项目配置值
    pub fn set_project_config(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "name" => self.project.name = value.to_string(),
            "version" => self.project.version = value.to_string(),
            "compile" => self.project.compile = CompileCommand::from_string(value)?,
            "package_dir" => self.project.package_dir = value.to_string(),
            "texlive_path" => {
                if value.trim().is_empty() {
                    self.project.texlive_path = None;
                } else {
                    self.project.texlive_path = Some(value.to_string());
                }
            },
            "mirror_url" => {
                if value.trim().is_empty() {
                    self.project.mirror_url = None;
                } else {
                    self.project.mirror_url = Some(value.to_string());
                }
            },
            "install_global" => {
                if value.trim().is_empty() {
                    self.project.install_global = None;
                } else {
                    self.project.install_global = Some(value.parse()?);
                }
            },
            _ => return Err(anyhow::anyhow!("Unknown project config key: {}", key)),
        }
        Ok(())
    }

    /// 获取项目配置值
    pub fn get_project_config(&self, key: &str) -> Option<String> {
        match key {
            "name" => Some(self.project.name.clone()),
            "version" => Some(self.project.version.clone()),
            "compile" => Some(self.project.compile.to_string()),
            "package_dir" => Some(self.project.package_dir.clone()),
            "texlive_path" => self.project.texlive_path.clone(),
            "mirror_url" => self.project.mirror_url.clone(),
            "install_global" => self.project.install_global.map(|b| b.to_string()),
            _ => None,
        }
    }

    /// 列出所有项目配置键
    pub fn list_project_keys() -> Vec<&'static str> {
        vec!["name", "version", "compile", "package_dir", "texlive_path", "mirror_url", "install_global"]
    }
}
