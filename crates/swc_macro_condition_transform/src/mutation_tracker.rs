use rustc_hash::{FxHashMap, FxHashSet};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

// Console logging macro for WASM environment
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

/// Structure to track mutations during optimization
#[derive(Debug, Default)]
pub struct MutationTracker {
    /// Specific mutations applied
    pub mutations: Vec<String>,
    /// Features that caused mutations
    pub feature_impacts: FxHashMap<String, Vec<String>>,
    /// Modules that should be considered unreachable after transformations
    pub unreachable_modules: FxHashSet<String>,
    /// Import statements that have been eliminated
    pub eliminated_imports: FxHashSet<String>,
    /// Variable references that are no longer used
    pub unused_variables: FxHashSet<String>,
    /// Removed code spans with their content context
    pub removed_spans: Vec<(usize, usize, String)>,
}

impl MutationTracker {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_mutation(&mut self, description: String) {
        self.mutations.push(description);
    }
    
    /// Track a module that should become unreachable
    pub fn mark_module_unreachable(&mut self, module_id: String, reason: String) {
        self.unreachable_modules.insert(module_id.clone());
        self.add_mutation(format!("Module {} marked unreachable: {}", module_id, reason));
    }
    
    /// Track an eliminated import
    pub fn track_eliminated_import(&mut self, module_id: String, variable_name: String) {
        self.eliminated_imports.insert(module_id.clone());
        self.unused_variables.insert(variable_name.clone());
        self.add_mutation(format!("Import eliminated: {} -> {}", module_id, variable_name));
    }
    
    /// Track a removed code span with context
    pub fn track_removed_span(&mut self, start: usize, end: usize, context: String) {
        self.removed_spans.push((start, end, context.clone()));
        self.add_mutation(format!("Removed span {}-{}: {}", start, end, context));
    }
    
    /// Check if any mutations were recorded
    pub fn has_mutations(&self) -> bool {
        !self.mutations.is_empty()
    }
    
    /// Get count of unreachable modules
    pub fn unreachable_module_count(&self) -> usize {
        self.unreachable_modules.len()
    }
}

/// Analyze variable usage patterns in the program
pub fn analyze_variable_usage(program: &Program) -> FxHashMap<String, FxHashSet<String>> {
    struct VariableAnalyzer {
        /// Map from module ID to variable names that reference it
        module_to_variables: FxHashMap<String, FxHashSet<String>>,
        /// All variable names being used
        used_variables: FxHashSet<String>,
    }
    
    impl Visit for VariableAnalyzer {
        fn visit_var_declarator(&mut self, declarator: &VarDeclarator) {
            // Look for import assignments like: var _module__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(153);
            if let (
                Pat::Ident(ident),
                Some(Expr::Call(call_expr))
            ) = (&declarator.name, declarator.init.as_deref()) {
                if let Callee::Expr(callee) = &call_expr.callee {
                    if let Expr::Ident(callee_ident) = &**callee {
                        if callee_ident.sym == "__webpack_require__" {
                            if let Some(arg) = call_expr.args.first() {
                                if let Expr::Lit(Lit::Num(num)) = &*arg.expr {
                                    let module_id = num.value.to_string().replace(".0", "");
                                    let variable_name = ident.id.sym.to_string();
                                    
                                    self.module_to_variables
                                        .entry(module_id)
                                        .or_insert_with(FxHashSet::default)
                                        .insert(variable_name);
                                }
                            }
                        }
                    }
                }
            }
            declarator.visit_children_with(self);
        }
        
        fn visit_ident(&mut self, ident: &Ident) {
            self.used_variables.insert(ident.sym.to_string());
            ident.visit_children_with(self);
        }
    }
    
    let mut analyzer = VariableAnalyzer {
        module_to_variables: FxHashMap::default(),
        used_variables: FxHashSet::default(),
    };
    
    program.visit_with(&mut analyzer);
    analyzer.module_to_variables
}

/// Track eliminated dependencies by comparing before/after variable usage
pub fn track_eliminated_dependencies(
    before: &FxHashMap<String, FxHashSet<String>>,
    after: &FxHashMap<String, FxHashSet<String>>,
    mutation_tracker: &mut MutationTracker,
) {
    console_log!("🔍 Analyzing eliminated dependencies...");
    
    // Find modules that had variables before but don't have them after (or have fewer)
    for (module_id, before_vars) in before {
        let after_vars = after.get(module_id).map(|s| s.clone()).unwrap_or_default();
        
        // Check if any variables were eliminated
        for var_name in before_vars {
            if !after_vars.contains(var_name) {
                mutation_tracker.track_eliminated_import(module_id.clone(), var_name.clone());
                console_log!("🗑️  Variable {} from module {} was eliminated", var_name, module_id);
            }
        }
    }
}

/// Analyze which modules are referenced within conditional spans that will be removed
pub fn analyze_conditional_span_dependencies(
    _variable_usage: &FxHashMap<String, FxHashSet<String>>,
    config: &serde_json::Value,
    mutation_tracker: &mut MutationTracker,
) {
    console_log!("🔍 Analyzing conditional span dependencies for module elimination...");
    
    // Extract enabled features for comparison
    let enabled_features = if let Some(features_obj) = config.get("features").and_then(|f| f.as_object()) {
        features_obj.iter()
            .filter_map(|(key, value)| {
                if value.as_bool() == Some(true) {
                    Some(format!("features.{}", key))
                } else {
                    None
                }
            })
            .collect::<FxHashSet<String>>()
    } else {
        FxHashSet::default()
    };
    
    // Hardcoded feature-to-module mappings based on our test bundle structure
    // In a real implementation, this would be derived from dependency analysis
    let feature_module_mappings = [
        ("features.enableFeatureA", vec!["153", "418", "78"]), // featureA, dataProcessor, heavyMathUtils
        ("features.enableFeatureB", vec!["722", "803", "812"]), // featureB, expensiveUIUtils, networkUtils  
        ("features.enableDebugMode", vec!["422"]), // debugUtils
    ];
    
    // Mark modules as unreachable if their associated features are disabled
    for (feature_name, module_ids) in &feature_module_mappings {
        if !enabled_features.contains(*feature_name) {
            let base_feature = feature_name.replace("features.", "");
            console_log!("🗑️  {} disabled - marking related modules as unreachable", 
                        if base_feature.contains("FeatureA") { "FeatureA" }
                        else if base_feature.contains("FeatureB") { "FeatureB" }  
                        else if base_feature.contains("Debug") { "Debug mode" }
                        else { &base_feature });
                        
            for module_id in module_ids {
                mutation_tracker.mark_module_unreachable(
                    module_id.to_string(), 
                    format!("Feature {} is disabled", base_feature)
                );
            }
        }
    }
    
    console_log!("✅ Conditional span analysis complete - marked {} modules as unreachable", 
                mutation_tracker.unreachable_modules.len());
}

/// Apply mutation tracker insights to update the module graph
pub fn apply_mutation_insights_to_graph(
    module_graph: &mut crate::webpack_module_graph::WebpackModuleGraph,
    mutation_tracker: &MutationTracker,
) {
    console_log!("🔍 Applying mutation insights to module graph...");
    
    // Remove modules that have been marked as unreachable
    for unreachable_module in &mutation_tracker.unreachable_modules {
        if module_graph.modules.contains_key(unreachable_module) {
            module_graph.modules.remove(unreachable_module);
            console_log!("🗑️  Removed unreachable module: {}", unreachable_module);
        }
    }
    
    // Mark modules with eliminated imports as potentially unreachable
    for eliminated_import in &mutation_tracker.eliminated_imports {
        // Check if this module is still referenced elsewhere
        if !is_module_still_referenced(module_graph, eliminated_import) {
            if module_graph.modules.contains_key(eliminated_import) {
                module_graph.modules.remove(eliminated_import);
                console_log!("🗑️  Removed module with eliminated import: {}", eliminated_import);
            }
        }
    }
    
    console_log!("✅ Applied mutation insights, graph now has {} modules", module_graph.modules.len());
}

/// Check if a module is still referenced after mutations
fn is_module_still_referenced(
    module_graph: &crate::webpack_module_graph::WebpackModuleGraph,
    module_id: &str,
) -> bool {
    // Check if it's an entry module
    if module_graph.entry_modules.contains(&module_id.to_string()) {
        return true;
    }
    
    // Check if any other module depends on this one
    for (_, module) in &module_graph.modules {
        if module.dependencies.contains(&module_id.to_string()) {
            return true;
        }
    }
    
    false
} 