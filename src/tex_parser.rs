use std::fs;
use std::path::Path;
use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct TeXDependency {
    pub package_name: String,
    pub dependency_type: DependencyType,
    pub line_number: usize,
    pub context: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DependencyType {
    UsePackage,      // \usepackage{...}
    RequirePackage,  // \RequirePackage{...}
    DocumentClass,   // \documentclass{...}
    LoadClass,       // \LoadClass{...}
    Input,           // \input{...}
    Include,         // \include{...}
    Bibliography,    // \bibliography{...}
    BibliographyStyle, // \bibliographystyle{...}
}

pub struct TeXParser {
    usepackage_regex: Regex,
    requirepackage_regex: Regex,
    documentclass_regex: Regex,
    loadclass_regex: Regex,
    input_regex: Regex,
    include_regex: Regex,
    bibliography_regex: Regex,
    bibliographystyle_regex: Regex,
}

impl TeXParser {
    pub fn new() -> Result<Self> {
        Ok(Self {
            // Match \usepackage[options]{package1,package2}
            usepackage_regex: Regex::new(r"\\usepackage(?:\[[^\]]*\])?\{([^}]+)\}")?,
            // Match \RequirePackage[options]{package}
            requirepackage_regex: Regex::new(r"\\RequirePackage(?:\[[^\]]*\])?\{([^}]+)\}")?,
            // Match \documentclass[options]{class}
            documentclass_regex: Regex::new(r"\\documentclass(?:\[[^\]]*\])?\{([^}]+)\}")?,
            // Match \LoadClass[options]{class}
            loadclass_regex: Regex::new(r"\\LoadClass(?:\[[^\]]*\])?\{([^}]+)\}")?,
            // Match \input{file}
            input_regex: Regex::new(r"\\input\{([^}]+)\}")?,
            // Match \include{file}
            include_regex: Regex::new(r"\\include\{([^}]+)\}")?,
            // Match \bibliography{files}
            bibliography_regex: Regex::new(r"\\bibliography\{([^}]+)\}")?,
            // Match \bibliographystyle{style}
            bibliographystyle_regex: Regex::new(r"\\bibliographystyle\{([^}]+)\}")?,
        })
    }

    /// Parse dependencies of a single TeX file
    pub fn parse_file(&self, file_path: &Path) -> Result<Vec<TeXDependency>> {
        let content = fs::read_to_string(file_path)?;
        self.parse_content(&content)
    }

    /// Parse dependencies of TeX content
    pub fn parse_content(&self, content: &str) -> Result<Vec<TeXDependency>> {
        let mut dependencies = Vec::new();

        for (line_number, line) in content.lines().enumerate() {
            let line_number = line_number + 1;
            
            // Handle comments: process only the part before comments
            let effective_line = if let Some(comment_pos) = line.find('%') {
                if comment_pos > 0 && line.chars().nth(comment_pos - 1) == Some('\\') {
                    line
                } else {
                    let before_comment = &line[..comment_pos];
                    if before_comment.trim().is_empty() {
                        continue;
                    }
                    before_comment
                }
            } else {
                line
            };

            // Check various dependency types
            self.extract_dependencies(effective_line, line_number, &mut dependencies);
        }

        Ok(dependencies)
    }

    /// Extract dependencies from a line
    fn extract_dependencies(&self, line: &str, line_number: usize, dependencies: &mut Vec<TeXDependency>) {
        // \usepackage{...}
        for caps in self.usepackage_regex.captures_iter(line) {
            let packages = &caps[1];
            for package in self.split_package_list(packages) {
                dependencies.push(TeXDependency {
                    package_name: package,
                    dependency_type: DependencyType::UsePackage,
                    line_number,
                    context: line.trim().to_string(),
                });
            }
        }

        // \RequirePackage{...}
        for caps in self.requirepackage_regex.captures_iter(line) {
            let packages = &caps[1];
            for package in self.split_package_list(packages) {
                dependencies.push(TeXDependency {
                    package_name: package,
                    dependency_type: DependencyType::RequirePackage,
                    line_number,
                    context: line.trim().to_string(),
                });
            }
        }

        // \documentclass{...}
        for caps in self.documentclass_regex.captures_iter(line) {
            let class = caps[1].trim().to_string();
            dependencies.push(TeXDependency {
                package_name: class,
                dependency_type: DependencyType::DocumentClass,
                line_number,
                context: line.trim().to_string(),
            });
        }

        // \LoadClass{...}
        for caps in self.loadclass_regex.captures_iter(line) {
            let class = caps[1].trim().to_string();
            dependencies.push(TeXDependency {
                package_name: class,
                dependency_type: DependencyType::LoadClass,
                line_number,
                context: line.trim().to_string(),
            });
        }

        // \input{...}
        for caps in self.input_regex.captures_iter(line) {
            let file = caps[1].trim().to_string();
            dependencies.push(TeXDependency {
                package_name: file,
                dependency_type: DependencyType::Input,
                line_number,
                context: line.trim().to_string(),
            });
        }

        // \include{...}
        for caps in self.include_regex.captures_iter(line) {
            let file = caps[1].trim().to_string();
            dependencies.push(TeXDependency {
                package_name: file,
                dependency_type: DependencyType::Include,
                line_number,
                context: line.trim().to_string(),
            });
        }

        // \bibliography{...}
        for caps in self.bibliography_regex.captures_iter(line) {
            let files = &caps[1];
            for file in self.split_package_list(files) {
                dependencies.push(TeXDependency {
                    package_name: file,
                    dependency_type: DependencyType::Bibliography,
                    line_number,
                    context: line.trim().to_string(),
                });
            }
        }

        // \bibliographystyle{...}
        for caps in self.bibliographystyle_regex.captures_iter(line) {
            let style = caps[1].trim().to_string();
            dependencies.push(TeXDependency {
                package_name: style,
                dependency_type: DependencyType::BibliographyStyle,
                line_number,
                context: line.trim().to_string(),
            });
        }
    }

    /// Split package list (handle comma-separated package names)
    fn split_package_list(&self, packages: &str) -> Vec<String> {
        packages
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Recursively parse all TeX files in the project
    pub fn parse_project(&self, project_path: &Path) -> Result<Vec<TeXDependency>> {
        let mut all_dependencies = Vec::new();
        let mut visited_files = HashSet::new();

        self.parse_directory_recursive(project_path, &mut all_dependencies, &mut visited_files)?;
        
        Ok(all_dependencies)
    }

    /// Recursively parse directory
    fn parse_directory_recursive(
        &self,
        dir_path: &Path,
        dependencies: &mut Vec<TeXDependency>,
        visited: &mut HashSet<std::path::PathBuf>,
    ) -> Result<()> {
        if !dir_path.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Skip certain directories
                if let Some(dir_name) = path.file_name() {
                    let dir_name = dir_name.to_string_lossy();
                    if dir_name == "packages" || dir_name == ".git" || dir_name.starts_with('.') {
                        continue;
                    }
                }
                self.parse_directory_recursive(&path, dependencies, visited)?;
            } else if path.is_file() {
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if (ext == "tex" || ext == "latex" || ext == "sty" || ext == "cls") 
                        && !visited.contains(&path) {
                        visited.insert(path.clone());
                        match self.parse_file(&path) {
                            Ok(mut file_deps) => dependencies.append(&mut file_deps),
                            Err(e) => println!("Warning: Failed to parse {}: {}", path.display(), e),
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get unique list of package dependencies
    pub fn get_unique_packages(dependencies: &[TeXDependency]) -> Vec<String> {
        let mut packages = HashSet::new();
        
        for dep in dependencies {
            // Only include actual package dependencies, skip file includes
            match dep.dependency_type {
                DependencyType::UsePackage | 
                DependencyType::RequirePackage |
                DependencyType::DocumentClass |
                DependencyType::LoadClass => {
                    packages.insert(dep.package_name.clone());
                }
                _ => {} // Skip file dependencies
            }
        }

        let mut result: Vec<String> = packages.into_iter().collect();
        result.sort();
        result
    }

    /// Filter out LaTeX core packages (do not need separate installation)
    pub fn filter_core_packages(packages: &[String]) -> Vec<String> {
        let core_packages = [
            "latex", "latex2e", "article", "book", "report", "letter",
            "minimal", "size10", "size11", "size12", "a4paper", "letterpaper",
            "twoside", "oneside", "draft", "final", "leqno", "fleqn",
            "openbib", "titlepage", "notitlepage",
        ];

        packages
            .iter()
            .filter(|pkg| !core_packages.contains(&pkg.as_str()))
            .cloned()
            .collect()
    }

    /// Display dependency analysis results
    pub fn print_dependency_analysis(dependencies: &[TeXDependency]) {
        if dependencies.is_empty() {
            println!("No dependencies found.");
            return;
        }

        println!("Found {} dependencies:", dependencies.len());
        
        let mut by_type: std::collections::HashMap<&str, Vec<&TeXDependency>> = std::collections::HashMap::new();
        
        for dep in dependencies {
            let type_name = match dep.dependency_type {
                DependencyType::UsePackage => "Packages",
                DependencyType::RequirePackage => "Required Packages",
                DependencyType::DocumentClass => "Document Classes",
                DependencyType::LoadClass => "Loaded Classes",
                DependencyType::Input => "Input Files",
                DependencyType::Include => "Included Files",
                DependencyType::Bibliography => "Bibliography Files",
                DependencyType::BibliographyStyle => "Bibliography Styles",
            };
            
            by_type.entry(type_name).or_insert_with(Vec::new).push(dep);
        }

        for (type_name, deps) in by_type {
            println!("\n{}:", type_name);
            for dep in deps {
                println!("  {} (line {}): {}", dep.package_name, dep.line_number, dep.context);
            }
        }
    }

    /// Detect missing packages through compilation errors (single detection)
    pub fn detect_missing_packages_by_compilation_once(
        &self,
        compile_cmd: &crate::config::CompileCommand,
        project_root: &Path,
    ) -> Result<Vec<String>> {
        // Parse compile command chain and magic variables
        let resolved_commands = compile_cmd.resolve_variables(project_root)?;
        
        if resolved_commands.is_empty() {
            return Err(anyhow::anyhow!("Empty resolved compile command chain"));
        }

        let mut missing_packages = Vec::new();
        
        // Execute each command in the compilation chain
        for (step_idx, resolved_args) in resolved_commands.iter().enumerate() {
            if resolved_args.is_empty() {
                continue;
            }

            let base_cmd = &resolved_args[0];
            let args = &resolved_args[1..];

            let output = Command::new(base_cmd)
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .current_dir(project_root)
                .output()?;

            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Merge output for analysis
            let combined_output = format!("{}\n{}", stdout, stderr);
            
            // If compilation succeeds, continue to next step
            if output.status.success() {
                continue;
            }
            
            // Compilation failed, analyze error type
            let step_missing_packages = self.parse_compilation_errors(&combined_output);
            
            if !step_missing_packages.is_empty() {
                // Found package missing errors
                for pkg in step_missing_packages {
                    if !missing_packages.contains(&pkg) {
                        missing_packages.push(pkg);
                    }
                }
                // Stop current iteration after finding package missing errors
                break;
            } else {
                // Compilation failed but no package missing detected, check if it's other recognizable error
                if !self.is_package_related_error(&combined_output) {
                    // Not package-related error, return error directly to user
                    return Err(anyhow::anyhow!(
                        "Compilation failed with non-package-related error in step {}:\n{}", 
                        step_idx + 1,
                        combined_output
                    ));
                }
                // Package-related error but no specific package identified, continue trying
                break;
            }
        }
        
        Ok(missing_packages)
    }

    /// Detect missing packages through compilation errors
    pub fn detect_missing_packages_by_compilation(
        &self,
        _tex_file: &Path,
        compile_cmd: &crate::config::CompileCommand,
        project_root: &Path,
    ) -> Result<Vec<String>> {
        println!("Attempting compilation to detect missing packages...");
        
        let mut all_missing_packages = Vec::new();
        let max_iterations = 10; // Prevent infinite loops
        
        for iteration in 1..=max_iterations {
            println!("🔄 Package detection iteration {}/{}", iteration, max_iterations);
            
            // Single detection
            match self.detect_missing_packages_by_compilation_once(compile_cmd, project_root) {
                Ok(missing_packages) => {
                    if missing_packages.is_empty() {
                        // No new missing packages found
                        if iteration == 1 {
                            println!("✅ Compilation successful - no missing packages detected");
                        } else {
                            println!("✅ No more missing packages detected after {} iterations", iteration - 1);
                        }
                        break;
                    } else {
                        // 发现了缺失包
                        let mut found_new_package = false;
                        for pkg in missing_packages {
                            if !all_missing_packages.contains(&pkg) {
                                all_missing_packages.push(pkg.clone());
                                found_new_package = true;
                                println!("📦 Detected missing package: {}", pkg);
                            }
                        }
                        
                        if !found_new_package {
                            println!("⚠️  No new packages detected, stopping iteration");
                            break;
                        }
                        
                        // 这里应该触发包安装，但由于这个函数只负责检测，
                        // 实际安装会在调用方处理
                        println!("🔄 Will retry after package installation");
                    }
                }
                Err(e) => {
                    // 遇到非包相关错误，直接返回
                    return Err(e);
                }
            }
        }
        
        if all_missing_packages.is_empty() && max_iterations > 1 {
            println!("⚠️  Reached maximum iterations ({}), stopping package detection", max_iterations);
        }
        
        Ok(all_missing_packages)
    }

    /// 解析编译错误输出，提取缺失的包名
    fn parse_compilation_errors(&self, error_output: &str) -> Vec<String> {
        let mut missing_packages = HashSet::new();
        
        // 常见的TeX错误模式
        let error_patterns = [
            // LaTeX Error: File `package.sty' not found
            r"File `([^']+)\.sty' not found",
            // LaTeX Error: File `class.cls' not found  
            r"File `([^']+)\.cls' not found",
            // ! LaTeX Error: Unknown option `option' for package `package'
            r"Unknown option `[^']*' for package `([^']*)'",
            // Package package Error:
            r"Package ([^\s]+) Error:",
            // ! Undefined control sequence ... \usepackage{package}
            r"\\usepackage\{([^}]+)\}",
            // Emergency stop ... package.sty not found
            r"([^\s/\\]+)\.sty not found",
            // Can't find file `package.sty'
            r"Can't find file `([^']+)\.sty'",
            // I can't find file `package.sty'
            r"I can't find file `([^']+)\.sty'",
        ];

        for pattern in &error_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for captures in regex.captures_iter(error_output) {
                    if let Some(package) = captures.get(1) {
                        let package_name = package.as_str().trim();
                        if !package_name.is_empty() {
                            missing_packages.insert(package_name.to_string());
                        }
                    }
                }
            }
        }

        // 特殊处理一些常见情况
        let lines: Vec<&str> = error_output.lines().collect();
        for line in &lines {
            // 处理 "! Undefined control sequence" 后跟包名的情况
            if line.contains("Undefined control sequence") {
                // 查找可能的包名提示
                if let Some(package_hint) = self.extract_package_from_undefined_command(line) {
                    missing_packages.insert(package_hint);
                }
            }
        }

        let mut result: Vec<String> = missing_packages.into_iter().collect();
        result.sort();
        result
    }

    /// 从未定义命令错误中提取可能的包名
    fn extract_package_from_undefined_command(&self, error_line: &str) -> Option<String> {
        // 一些常见的命令到包的映射
        let command_to_package = [
            (r"\\includegraphics", "graphicx"),
            (r"\\url", "url"),
            (r"\\href", "hyperref"),
            (r"\\textcolor", "xcolor"),
            (r"\\colorbox", "xcolor"),
            (r"\\fcolorbox", "xcolor"),
            (r"\\begin\{figure\}", "graphicx"),
            (r"\\begin\{table\}", "array"),
            (r"\\toprule", "booktabs"),
            (r"\\midrule", "booktabs"),
            (r"\\bottomrule", "booktabs"),
            (r"\\multicolumn", "array"),
            (r"\\multirow", "multirow"), 
            (r"\\footnotesize", "geometry")
        ];

        for (pattern, package) in &command_to_package {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(error_line) {
                    return Some(package.to_string());
                }
            }
        }

        None
    }

    /// 判断编译错误是否与包相关
    fn is_package_related_error(&self, error_output: &str) -> bool {
        let package_related_patterns = [
            // 明确的包相关错误
            r"\.sty.*not found",
            r"\.cls.*not found", 
            r"Package.*Error",
            r"Package.*Warning",
            r"Unknown option.*for package",
            r"Undefined control sequence.*\\usepackage",
            r"File.*\.sty.*not found",
            r"File.*\.cls.*not found",
            r"Can't find file.*\.sty",
            r"I can't find file.*\.sty",
            
            // 可能与包相关的错误
            r"Undefined control sequence.*\\[a-zA-Z]",
            r"Environment.*undefined",
            r"Unknown environment",
            r"Command.*not defined",
        ];

        // 非包相关的错误模式
        let non_package_patterns = [
            r"Syntax error",
            r"Missing.*begin\{document\}",
            r"Extra.*\}",
            r"Missing.*\$",
            r"Misplaced.*&",
            r"Missing control sequence inserted",
            r"Paragraph ended before.*was complete",
            r"Use of.*doesn't match its definition",
            r"Illegal.*character",
            r"Missing number",
            r"Dimension too large",
        ];

        // 先检查是否是明确的非包相关错误
        for pattern in &non_package_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(error_output) {
                    return false;
                }
            }
        }

        // 再检查是否是包相关错误
        for pattern in &package_related_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(error_output) {
                    return true;
                }
            }
        }

        // 如果都没有匹配，默认认为可能与包相关
        // 这是保守的策略，避免遗漏包依赖问题
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_dependencies() {
        let parser = TeXParser::new().unwrap();
        let content = r"\usepackage{amsmath}\documentclass{article}";
        let deps = parser.parse_content(content).unwrap();
        
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].package_name, "amsmath");
        assert_eq!(deps[1].package_name, "article");
    }

    #[test]
    fn test_parse_compilation_errors() {
        let parser = TeXParser::new().unwrap();
        let error = "! LaTeX Error: File `minted.sty' not found.";
        let missing = parser.parse_compilation_errors(error);
        
        assert_eq!(missing, vec!["minted"]);
    }

    #[test]
    fn test_filter_core_packages() {
        let packages = vec!["amsmath".to_string(), "article".to_string()];
        let filtered = TeXParser::filter_core_packages(&packages);
        
        assert_eq!(filtered, vec!["amsmath"]);
    }
}













