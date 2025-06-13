use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Represents a single module in the webpack bundle
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleNode {
    /// The webpack module ID (e.g., "153", "422")
    pub id: String,
    /// Raw JavaScript source code of the module
    pub source: String,
    /// Direct dependencies (modules this module requires)
    pub dependencies: FxHashSet<String>,
    /// Modules that depend on this module (reverse dependencies)
    pub dependents: FxHashSet<String>,
}

impl ModuleNode {
    pub fn new(id: String, source: String) -> Self {
        Self {
            id,
            source,
            dependencies: FxHashSet::default(),
            dependents: FxHashSet::default(),
        }
    }

    /// Add a dependency to this module
    pub fn add_dependency(&mut self, module_id: String) {
        self.dependencies.insert(module_id);
    }

    /// Add a dependent (module that depends on this one)
    pub fn add_dependent(&mut self, module_id: String) {
        self.dependents.insert(module_id);
    }
}

/// Represents the complete module dependency graph from a webpack bundle
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleGraph {
    /// Map of module ID to module node
    pub modules: FxHashMap<String, ModuleNode>,
    /// Entry point module IDs
    pub entry_points: Vec<String>,
}

impl ModuleGraph {
    pub fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
            entry_points: Vec::new(),
        }
    }

    /// Add a module to the graph
    pub fn add_module(&mut self, module: ModuleNode) {
        self.modules.insert(module.id.clone(), module);
    }

    /// Get a module by ID
    pub fn get_module(&self, id: &str) -> Option<&ModuleNode> {
        self.modules.get(id)
    }

    /// Get a mutable reference to a module by ID
    pub fn get_module_mut(&mut self, id: &str) -> Option<&mut ModuleNode> {
        self.modules.get_mut(id)
    }

    /// Add a dependency relationship between two modules
    pub fn add_dependency(&mut self, from_id: &str, to_id: &str) {
        // Add to dependency list of from_module
        if let Some(from_module) = self.get_module_mut(from_id) {
            from_module.add_dependency(to_id.to_string());
        }

        // Add to dependents list of to_module
        if let Some(to_module) = self.get_module_mut(to_id) {
            to_module.add_dependent(from_id.to_string());
        }
    }

    /// Add an entry point
    pub fn add_entry_point(&mut self, module_id: String) {
        if !self.entry_points.contains(&module_id) {
            self.entry_points.push(module_id);
        }
    }

    /// Get all modules that are reachable from entry points
    pub fn get_reachable_modules(&self) -> FxHashSet<String> {
        let mut reachable = FxHashSet::default();
        let mut queue = VecDeque::new();

        // Start with entry points
        for entry_id in &self.entry_points {
            queue.push_back(entry_id.clone());
            reachable.insert(entry_id.clone());
        }

        // BFS to find all reachable modules
        while let Some(current_id) = queue.pop_front() {
            if let Some(module) = self.get_module(&current_id) {
                for dep_id in &module.dependencies {
                    if !reachable.contains(dep_id) {
                        reachable.insert(dep_id.clone());
                        queue.push_back(dep_id.clone());
                    }
                }
            }
        }

        reachable
    }

    /// Get modules that are not reachable from any entry point (dead code)
    pub fn get_unreachable_modules(&self) -> Vec<String> {
        let reachable = self.get_reachable_modules();
        self.modules
            .keys()
            .filter(|id| !reachable.contains(*id))
            .cloned()
            .collect()
    }

    /// Get the dependency chain for a specific module
    pub fn get_dependency_chain(&self, module_id: &str) -> Vec<String> {
        let mut chain = Vec::new();
        let mut visited = FxHashSet::default();
        self.collect_dependencies(module_id, &mut chain, &mut visited);
        chain
    }

    fn collect_dependencies(
        &self,
        module_id: &str,
        chain: &mut Vec<String>,
        visited: &mut FxHashSet<String>,
    ) {
        if visited.contains(module_id) {
            return;
        }
        visited.insert(module_id.to_string());

        if let Some(module) = self.get_module(module_id) {
            chain.push(module_id.to_string());
            for dep_id in &module.dependencies {
                self.collect_dependencies(dep_id, chain, visited);
            }
        }
    }
}

impl Default for ModuleGraph {
    fn default() -> Self {
        Self::new()
    }
} 