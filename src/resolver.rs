use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version_constraint: String,
    pub optional: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ResolvedPackage {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<Dependency>,
}

#[allow(dead_code)]
pub struct DependencyResolver {
    packages: HashMap<String, Vec<ResolvedPackage>>,
}

#[allow(dead_code)]
impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }
    
    pub fn add_package(&mut self, package: ResolvedPackage) {
        self.packages
            .entry(package.name.clone())
            .or_insert_with(Vec::new)
            .push(package);
    }
    
    pub fn resolve(&self, root_packages: &[String]) -> Result<Vec<ResolvedPackage>> {
        let mut resolved = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Add root packages to queue
        for package_name in root_packages {
            queue.push_back(package_name.clone());
        }
        
        while let Some(package_name) = queue.pop_front() {
            if visited.contains(&package_name) {
                continue;
            }
            
            visited.insert(package_name.clone());
            
            // Find the best version for this package
            if let Some(package) = self.find_best_version(&package_name)? {
                // Add dependencies to queue
                for dep in &package.dependencies {
                    if !dep.optional && !visited.contains(&dep.name) {
                        queue.push_back(dep.name.clone());
                    }
                }
                
                resolved.push(package);
            }
        }
        
        // Sort by dependency order
        self.sort_by_dependencies(&mut resolved)?;
        
        Ok(resolved)
    }
    
    fn find_best_version(&self, package_name: &str) -> Result<Option<ResolvedPackage>> {
        if let Some(versions) = self.packages.get(package_name) {
            // For now, just return the latest version
            // In a real implementation, this would consider version constraints
            Ok(versions.last().cloned())
        } else {
            // Package not found in local cache, would need to fetch from repository
            Ok(None)
        }
    }
    
    fn sort_by_dependencies(&self, packages: &mut Vec<ResolvedPackage>) -> Result<()> {
        // Topological sort to ensure dependencies are installed before dependents
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        
        // Build dependency graph
        for package in packages.iter() {
            in_degree.insert(package.name.clone(), 0);
            graph.insert(package.name.clone(), Vec::new());
        }
        
        for package in packages.iter() {
            for dep in &package.dependencies {
                if let Some(deps) = graph.get_mut(&dep.name) {
                    deps.push(package.name.clone());
                }
                *in_degree.entry(package.name.clone()).or_insert(0) += 1;
            }
        }
        
        // Perform topological sort
        let mut queue = VecDeque::new();
        let mut sorted = Vec::new();
        
        for (name, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(name.clone());
            }
        }
        
        while let Some(package_name) = queue.pop_front() {
            sorted.push(package_name.clone());
            
            if let Some(dependents) = graph.get(&package_name) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }
        
        // Reorder packages based on sorted order
        let order_map: HashMap<String, usize> = sorted
            .iter()
            .enumerate()
            .map(|(i, name)| (name.clone(), i))
            .collect();
        
        packages.sort_by(|a, b| {
            let order_a = order_map.get(&a.name).unwrap_or(&usize::MAX);
            let order_b = order_map.get(&b.name).unwrap_or(&usize::MAX);
            order_a.cmp(order_b)
        });
        
        Ok(())
    }
    
    pub fn check_conflicts(&self, packages: &[ResolvedPackage]) -> Vec<String> {
        let mut conflicts = Vec::new();
        let mut package_versions: HashMap<String, String> = HashMap::new();
        
        for package in packages {
            if let Some(existing_version) = package_versions.get(&package.name) {
                if existing_version != &package.version {
                    conflicts.push(format!(
                        "Version conflict for {}: {} vs {}",
                        package.name, existing_version, package.version
                    ));
                }
            } else {
                package_versions.insert(package.name.clone(), package.version.clone());
            }
        }
        
        conflicts
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}
