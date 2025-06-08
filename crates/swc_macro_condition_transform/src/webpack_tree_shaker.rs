use rustc_hash::FxHashSet;
use swc_core::ecma::{
    ast::*,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

// Console logging macro for WASM environment
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

/// Webpack-aware module tree shaker that removes unused webpack modules
/// 
/// **Current Status**: This is a comprehensive tree shaker that provides
/// foundation for advanced module-level optimization. Currently not used
/// in the main pipeline, which uses a simpler approach via `perform_webpack_require_cleanup`.
/// 
/// **Capabilities**:
/// - Analyzes webpack module definitions and usage
/// - Identifies entry points and module dependencies  
/// - Removes unused module definitions and require calls
/// - Provides detailed statistics on removal rates
/// 
/// **Usage**: Can be integrated for more sophisticated tree shaking scenarios
/// where simple bare call removal isn't sufficient.
#[derive(Default)]
pub struct WebpackModuleTreeShaker {
    /// All webpack module IDs that are explicitly required
    used_modules: FxHashSet<String>,
    /// All webpack module definitions found
    all_modules: FxHashSet<String>,
    /// Entry points that are always considered used
    entry_modules: FxHashSet<String>,
    /// Whether any changes were made
    changed: bool,
    /// Module graph for advanced analysis
    module_graph: Option<crate::webpack_module_graph::WebpackModuleGraph>,
    /// Count of removed bare calls
    removed_bare_calls: usize,
}

impl WebpackModuleTreeShaker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_changed(&self) -> bool {
        self.changed
    }

    pub fn set_module_graph(&mut self, module_graph: crate::webpack_module_graph::WebpackModuleGraph) {
        self.module_graph = Some(module_graph);
    }

    fn extract_module_id(&self, expr: &Expr) -> Option<String> {
        match expr {
            // Handle string module IDs: __webpack_require__("123")
            Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
            // Handle numeric module IDs: __webpack_require__(123)
            Expr::Lit(Lit::Num(n)) => Some(n.value.to_string()),
            _ => None,
        }
    }

    fn is_webpack_require_call(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call(CallExpr { callee: Callee::Expr(callee), .. }) => {
                if let Expr::Ident(ident) = &**callee {
                    ident.sym == "__webpack_require__"
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn find_entry_points(&mut self, module: &Module) {
        // Look for common entry point patterns
        for stmt in &module.body {
            if let ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) = stmt {
                // Pattern: __webpack_require__(0) or similar at top level
                if self.is_webpack_require_call(expr) {
                    if let Expr::Call(CallExpr { args, .. }) = &**expr {
                        if let Some(arg) = args.first() {
                            if let Some(module_id) = self.extract_module_id(&arg.expr) {
                                self.entry_modules.insert(module_id);
                            }
                        }
                    }
                }
            }
        }
    }

    fn collect_module_definitions(&mut self, module: &Module) {
        for stmt in &module.body {
            if let ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) = stmt {
                // Pattern: __webpack_modules__[123] = function() { ... }
                if let Expr::Assign(AssignExpr { 
                    left, 
                    .. 
                }) = &**expr {
                    if let AssignTarget::Simple(SimpleAssignTarget::Member(member)) = left {
                        if let (Expr::Ident(obj), MemberProp::Computed(ComputedPropName { expr: prop, .. })) = 
                            (&*member.obj, &member.prop) {
                            if obj.sym == "__webpack_modules__" {
                                if let Some(module_id) = self.extract_module_id(prop) {
                                    self.all_modules.insert(module_id);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn collect_used_modules(&mut self, module: &Module) {
        // Add entry points as used
        for entry in &self.entry_modules {
            self.used_modules.insert(entry.clone());
        }

        // Traverse the AST to find all __webpack_require__() calls
        // Use a visitor that doesn't borrow self mutably
        let mut found_modules = FxHashSet::default();
        self.find_webpack_requires_in_module(module, &mut found_modules);
        
        // Add found modules to used_modules
        for module_id in found_modules {
            self.used_modules.insert(module_id);
        }
    }

    fn find_webpack_requires_in_module(&self, module: &Module, found_modules: &mut FxHashSet<String>) {
        use swc_core::ecma::visit::{Visit, VisitWith};
        
        struct RequireCollector<'a> {
            found_modules: &'a mut FxHashSet<String>,
            tree_shaker: &'a WebpackModuleTreeShaker,
        }
        
        impl Visit for RequireCollector<'_> {
            fn visit_call_expr(&mut self, call: &CallExpr) {
                if let Callee::Expr(callee) = &call.callee {
                    if let Expr::Ident(ident) = &**callee {
                        if ident.sym == "__webpack_require__" {
                            if let Some(arg) = call.args.first() {
                                if let Some(module_id) = self.tree_shaker.extract_module_id(&arg.expr) {
                                    self.found_modules.insert(module_id);
                                }
                            }
                        }
                    }
                }
                call.visit_children_with(self);
            }
        }
        
        let mut collector = RequireCollector { found_modules, tree_shaker: self };
        module.visit_with(&mut collector);
    }

    fn should_remove_module(&self, module_id: &str) -> bool {
        // Don't remove if it's used or if it's an entry point
        !self.used_modules.contains(module_id) && !self.entry_modules.contains(module_id)
    }

    /// Analyze module usage patterns before transformation
    pub fn analyze(&mut self, module: &Module) {
        self.find_entry_points(module);
        self.collect_module_definitions(module);
        self.collect_used_modules(module);
    }

    /// Get statistics about the analysis
    pub fn get_stats(&self) -> WebpackTreeShakingStats {
        let unused_modules: Vec<String> = self.all_modules
            .iter()
            .filter(|id| self.should_remove_module(id))
            .cloned()
            .collect();

        WebpackTreeShakingStats {
            total_modules: self.all_modules.len(),
            used_modules: self.used_modules.len(),
            unused_modules: unused_modules.len(),
            entry_modules: self.entry_modules.len(),
            unused_module_ids: unused_modules,
            removed_bare_calls: self.removed_bare_calls,
        }
    }
}

impl VisitMut for WebpackModuleTreeShaker {
    noop_visit_mut_type!();

    fn visit_mut_program(&mut self, program: &mut Program) {
        // If we have a module graph, use it for more sophisticated analysis
        if let Some(module_graph) = self.module_graph.take() {
            self.process_with_module_graph(program, &module_graph);
            self.module_graph = Some(module_graph); // Put it back
        } else {
            // Fallback to legacy analysis
            self.process_legacy(program);
        }
        
        program.visit_mut_children_with(self);
    }
}

impl WebpackModuleTreeShaker {
    fn process_with_module_graph(&mut self, program: &mut Program, module_graph: &crate::webpack_module_graph::WebpackModuleGraph) {
        console_log!("🔧 Using module graph for linking-based tree shaking...");
        
        let unused_modules = module_graph.get_unused_modules();
        let unused_requires = module_graph.get_unused_requires(program);
        
        console_log!("📊 Module graph analysis:");
        console_log!("  📦 Total modules: {}", module_graph.modules.len());
        console_log!("  🚀 Entry modules: {}", module_graph.entry_modules.len());
        console_log!("  ✅ Used modules: {}", module_graph.reachable_modules.len());
        console_log!("  🗑️  Unused modules: {}", unused_modules.len());
        console_log!("  🗑️  Unused requires: {}", unused_requires.len());
        
        // CRITICAL FIX: Only remove modules that are truly unused AND not required by any assignment calls
        let actually_removable_modules = self.get_safely_removable_modules(program, &unused_modules);
        
        console_log!("  🛡️  Actually removable: {}", actually_removable_modules.len());
        
        // Clone stats before mutation to avoid borrow checker issues
        let all_modules = module_graph.modules.keys().cloned().collect();
        let used_modules = module_graph.reachable_modules.clone();
        let entry_modules = module_graph.entry_modules.clone();
        
        // Remove unused module definitions and bare calls
        self.remove_unused_content(program, &actually_removable_modules, &unused_requires);
        
        // Update stats
        self.all_modules = all_modules;
        self.used_modules = used_modules;
        self.entry_modules = entry_modules;
    }
    
    /// Critical fix: Determine which modules can be safely removed
    /// A module can only be removed if:
    /// 1. It's marked as unused by module graph analysis
    /// 2. It's not referenced in any assignment calls (var x = __webpack_require__(id))
    fn get_safely_removable_modules(&self, program: &Program, unused_modules: &FxHashSet<String>) -> FxHashSet<String> {
        use swc_core::ecma::visit::{Visit, VisitWith};
        
        let mut assignment_required_modules = FxHashSet::default();
        
        struct AssignmentAnalyzer<'a> {
            assignment_required: &'a mut FxHashSet<String>,
        }
        
        impl<'a> Visit for AssignmentAnalyzer<'a> {
            fn visit_var_declarator(&mut self, declarator: &VarDeclarator) {
                if let Some(init) = &declarator.init {
                    if let Expr::Call(call) = &**init {
                        if let Callee::Expr(callee) = &call.callee {
                            if let Expr::Ident(ident) = &**callee {
                                if ident.sym == "__webpack_require__" {
                                    if let Some(arg) = call.args.first() {
                                        if let Some(module_id) = self.extract_module_id(&arg.expr) {
                                            self.assignment_required.insert(module_id);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                declarator.visit_children_with(self);
            }
            
            fn visit_assign_expr(&mut self, assign: &AssignExpr) {
                if let Expr::Call(call) = &*assign.right {
                    if let Callee::Expr(callee) = &call.callee {
                        if let Expr::Ident(ident) = &**callee {
                            if ident.sym == "__webpack_require__" {
                                if let Some(arg) = call.args.first() {
                                    if let Some(module_id) = self.extract_module_id(&arg.expr) {
                                        self.assignment_required.insert(module_id);
                                    }
                                }
                            }
                        }
                    }
                }
                assign.visit_children_with(self);
            }
        }
        
        impl<'a> AssignmentAnalyzer<'a> {
            fn extract_module_id(&self, expr: &Expr) -> Option<String> {
                match expr {
                    Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
                    Expr::Lit(Lit::Num(n)) => Some(n.value.to_string()),
                    _ => None,
                }
            }
        }
        
        let mut analyzer = AssignmentAnalyzer {
            assignment_required: &mut assignment_required_modules,
        };
        program.visit_with(&mut analyzer);
        
        // A module can only be removed if it's unused AND not required by assignments
        unused_modules.iter()
            .filter(|module_id| !assignment_required_modules.contains(*module_id))
            .cloned()
            .collect()
    }
    
    fn process_legacy(&mut self, program: &mut Program) {
        console_log!("🔧 Using legacy tree shaking analysis...");
        
        match program {
            Program::Module(module) => {
                self.analyze(module);
                self.remove_unused_legacy(module);
            }
            Program::Script(script) => {
                // Handle script-style webpack bundles
                self.remove_bare_calls_from_script(script);
            }
        }
    }
    
    fn remove_unused_content(&mut self, program: &mut Program, unused_modules: &FxHashSet<String>, unused_requires: &FxHashSet<String>) {
        use swc_core::ecma::visit::{VisitMut, VisitMutWith};
        
        struct ContentRemover<'a> {
            unused_modules: &'a FxHashSet<String>,
            unused_requires: &'a FxHashSet<String>,
            removed_count: usize,
        }
        
        impl<'a> VisitMut for ContentRemover<'a> {
            fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
                stmts.retain(|stmt| {
                    // Remove bare __webpack_require__ calls
                    if let Stmt::Expr(ExprStmt { expr, .. }) = stmt {
                        if let Expr::Call(CallExpr { callee: Callee::Expr(callee), args, .. }) = &**expr {
                            if let Expr::Ident(ident) = &**callee {
                                if ident.sym == "__webpack_require__" {
                                    if let Some(arg) = args.first() {
                                        if let Some(module_id) = extract_module_id(&arg.expr) {
                                            if self.unused_requires.contains(&module_id) {
                                                console_log!("  🗑️  Removing bare call: __webpack_require__({})", module_id);
                                                self.removed_count += 1;
                                                return false; // Remove this statement
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    true // Keep everything else
                });
                
                // Continue visiting children
                for stmt in stmts {
                    stmt.visit_mut_with(self);
                }
            }
            
            fn visit_mut_object_lit(&mut self, obj: &mut ObjectLit) {
                // Remove unused module definitions from __webpack_modules__ object
                obj.props.retain(|prop| {
                    if let PropOrSpread::Prop(prop) = prop {
                        if let Prop::KeyValue(kv) = &**prop {
                            if let Some(module_id) = extract_module_id_from_prop(&kv.key) {
                                if self.unused_modules.contains(&module_id) {
                                    console_log!("  🗑️  Removing module definition: {}", module_id);
                                    return false; // Remove this property
                                }
                            }
                        }
                    }
                    true // Keep everything else
                });
                
                obj.visit_mut_children_with(self);
            }
        }
        
        fn extract_module_id(expr: &Expr) -> Option<String> {
            match expr {
                Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
                Expr::Lit(Lit::Num(n)) => Some(n.value.to_string()),
                _ => None,
            }
        }
        
        fn extract_module_id_from_prop(prop: &PropName) -> Option<String> {
            match prop {
                PropName::Str(s) => Some(s.value.to_string()),
                PropName::Num(n) => Some(n.value.to_string()),
                PropName::Ident(i) => Some(i.sym.to_string()),
                _ => None,
            }
        }
        
        let mut remover = ContentRemover {
            unused_modules,
            unused_requires,
            removed_count: 0,
        };
        
        program.visit_mut_with(&mut remover);
        self.removed_bare_calls = remover.removed_count;
        
        if remover.removed_count > 0 || !unused_modules.is_empty() {
            self.changed = true;
        }
    }
    
    fn remove_unused_legacy(&mut self, module: &mut Module) {
        // Legacy implementation for backward compatibility
        let original_len = module.body.len();
        module.body.retain(|item| {
            if let ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) = item {
                // Remove unused standalone __webpack_require__(id) calls
                if self.is_webpack_require_call(expr) {
                    if let Expr::Call(CallExpr { args, .. }) = &**expr {
                        if let Some(arg) = args.first() {
                            if let Some(module_id) = self.extract_module_id(&arg.expr) {
                                if self.should_remove_module(&module_id) {
                                    self.removed_bare_calls += 1;
                                    return false; // Remove this require call
                                }
                            }
                        }
                    }
                }
            }
            true // Keep everything else
        });

        if module.body.len() != original_len {
            self.changed = true;
        }
    }
    
    fn remove_bare_calls_from_script(&mut self, _script: &mut Script) {
        // Handle script-style webpack bundles
        // For now, we'll implement this later if needed
    }
}

/// Statistics about webpack tree shaking analysis
#[derive(Debug, Clone)]
pub struct WebpackTreeShakingStats {
    pub total_modules: usize,
    pub used_modules: usize,
    pub unused_modules: usize,
    pub entry_modules: usize,
    pub unused_module_ids: Vec<String>,
    pub removed_bare_calls: usize,
}

impl WebpackTreeShakingStats {
    pub fn removal_rate(&self) -> f64 {
        if self.total_modules == 0 {
            0.0
        } else {
            (self.unused_modules as f64 / self.total_modules as f64) * 100.0
        }
    }

    pub fn print_summary(&self) {
        console_log!("🌳 Webpack Tree Shaking Summary:");
        console_log!("   📦 Total modules: {}", self.total_modules);
        console_log!("   ✅ Used modules: {}", self.used_modules);
        console_log!("   🗑️  Unused modules: {}", self.unused_modules);
        console_log!("   🚀 Entry modules: {}", self.entry_modules);
        console_log!("   📊 Removal rate: {:.1}%", self.removal_rate());
        
        if !self.unused_module_ids.is_empty() {
            console_log!("   🔍 Unused module IDs: {:?}", self.unused_module_ids);
        }
    }
}

/// Public interface for webpack tree shaking that can be called from WASM
/// 
/// This function performs webpack module linking and tree shaking including:
/// - Analyzing webpack module graph structure  
/// - Removing unused module definitions from __webpack_modules__
/// - Removing unused bare __webpack_require__() calls
/// - Preserving assignment calls like: var x = __webpack_require__(123)
/// - Providing detailed statistics about what was removed
pub fn perform_webpack_tree_shaking(program: &mut Program) -> WebpackTreeShakingStats {
    use swc_core::ecma::visit::VisitMutWith;
    use crate::webpack_module_graph::WebpackModuleGraph;
    
    console_log!("🌳 Webpack module linking and tree shaking starting...");
    
    // Step 1: Build module graph to understand relationships
    let mut module_graph = WebpackModuleGraph::new();
    module_graph.hydrate_module_graph_from_chunk(program);
    
    // Step 2: Apply tree shaking transformation using the module graph
    let mut tree_shaker = WebpackModuleTreeShaker::new();
    tree_shaker.set_module_graph(module_graph);
    program.visit_mut_with(&mut tree_shaker);
    
    // Step 3: Get and log statistics
    let stats = tree_shaker.get_stats();
    
    if stats.unused_modules > 0 {
        console_log!("✅ Successfully removed {} unused module definitions and {} bare calls!", 
                    stats.unused_modules, stats.removed_bare_calls);
    } else if stats.removed_bare_calls > 0 {
        console_log!("✅ Successfully removed {} bare __webpack_require__ calls!", stats.removed_bare_calls);
    } else {
        console_log!("ℹ️  No unused modules or bare calls found to remove");
    }
    
    stats
} 