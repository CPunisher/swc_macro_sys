use rustc_hash::FxHashMap;
use std::collections::{HashSet, VecDeque};
use crate::module::{ModuleId, WebpackModule};

/// Represents a dependency graph for webpack modules
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// All modules in the graph
    pub modules: FxHashMap<ModuleId, WebpackModule>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    /// Create a new dependency graph
    pub fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
        }
    }

    /// Add a module to the graph
    pub fn add_module(&mut self, module: WebpackModule) {
        self.modules.insert(module.id.clone(), module);
    }

    /// Add a dependency relationship between two modules
    pub fn add_dependency(&mut self, from_module: &ModuleId, to_module: &ModuleId) {
        // Add dependency to the from_module
        if let Some(module) = self.modules.get_mut(from_module) {
            module.add_dependency(to_module.clone());
        }

        // Add dependent to the to_module
        if let Some(module) = self.modules.get_mut(to_module) {
            module.add_dependent(from_module.clone());
        }
    }

    /// Get all modules that are reachable from a given module
    pub fn get_reachable_modules(&self, start_module: &ModuleId) -> HashSet<ModuleId> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(start_module.clone());
        visited.insert(start_module.clone());

        while let Some(current) = queue.pop_front() {
            if let Some(module) = self.modules.get(&current) {
                for dependency in &module.dependencies {
                    // Only traverse into dependencies that are present in the graph
                    if self.modules.contains_key(dependency) && !visited.contains(dependency) {
                        visited.insert(dependency.clone());
                        queue.push_back(dependency.clone());
                    }
                }
            }
        }

        visited
    }

    /// Get all modules that are reachable from multiple start modules
    pub fn get_reachable_from_multiple(&self, start_modules: &[ModuleId]) -> HashSet<ModuleId> {
        let mut reachable = HashSet::new();
        
        for start_module in start_modules {
            let module_reachable = self.get_reachable_modules(start_module);
            reachable.extend(module_reachable);
        }

        reachable
    }

    /// Find orphaned modules (modules that are not reachable from any entry point)
    pub fn find_orphaned_modules(&self, entry_points: &[ModuleId]) -> Vec<ModuleId> {
        let reachable = self.get_reachable_from_multiple(entry_points);
        
        self.modules
            .keys()
            .filter(|module_id| !reachable.contains(*module_id))
            .cloned()
            .collect()
    }



    /// Get module count
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Get total dependency count
    pub fn total_dependencies(&self) -> usize {
        self.modules.values().map(|m| m.dependencies.len()).sum()
    }

    /// Print debug information about the graph
    pub fn print_debug_info(&self) {
        println!("Dependency Graph Debug Info:");
        println!("Total modules: {}", self.module_count());
        println!("Total dependencies: {}", self.total_dependencies());
        
        for (module_id, module) in &self.modules {
            println!("  Module '{}': deps={:?}, dependents={:?}", 
                     module_id, 
                     module.dependencies.len(), 
                     module.dependents.len());
        }
    }

}

