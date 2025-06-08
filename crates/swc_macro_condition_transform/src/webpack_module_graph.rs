use rustc_hash::{FxHashMap, FxHashSet};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

// Console logging macro for WASM environment
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

/// Represents a webpack module with basic dependency tracking
#[derive(Debug, Clone)]
pub struct WebpackModule {
    pub id: String,
    pub dependencies: Vec<String>,
    pub exports: FxHashSet<String>,
    pub is_entry: bool,
    pub has_side_effects: bool,
    pub source_code: Option<String>,
}

/// Webpack module graph for linking analysis
#[derive(Debug, Default)]
pub struct WebpackModuleGraph {
    /// All modules indexed by ID
    pub modules: FxHashMap<String, WebpackModule>,
    /// Entry point modules (directly executed)
    pub entry_modules: FxHashSet<String>,
    /// Modules reachable from entry points
    pub reachable_modules: FxHashSet<String>,
    /// Webpack runtime functions
    pub runtime_functions: FxHashSet<String>,
    /// Module execution order
    pub execution_order: Vec<String>,
}

impl WebpackModule {
    pub fn new(id: String) -> Self {
        Self {
            id: id.clone(),
            dependencies: Vec::new(),
            exports: FxHashSet::default(),
            is_entry: false,
            has_side_effects: false,
            source_code: None,
        }
    }

    /// Add a dependency to this module
    pub fn add_dependency(&mut self, target: String) {
        if !self.dependencies.contains(&target) {
            self.dependencies.push(target);
        }
    }

    /// Get all direct dependencies of this module
    pub fn get_dependency_ids(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    /// Check if this module has side effects
    pub fn analyze_side_effects(&mut self, function_body: &BlockStmt) {
        // Analyze the module function body for side effects
        struct SideEffectAnalyzer {
            has_side_effects: bool,
        }

        impl Visit for SideEffectAnalyzer {
            fn visit_call_expr(&mut self, call: &CallExpr) {
                // Check for potential side effect calls
                if let Callee::Expr(callee) = &call.callee {
                    match &**callee {
                        Expr::Member(_member) => {
                            // Method calls can have side effects
                            self.has_side_effects = true;
                        }
                        Expr::Ident(ident) => {
                            // Function calls can have side effects
                            if ident.sym != "__webpack_require__" {
                                self.has_side_effects = true;
                            }
                        }
                        _ => {}
                    }
                }
                call.visit_children_with(self);
            }

            fn visit_assign_expr(&mut self, _assign: &AssignExpr) {
                // Assignments can have side effects
                self.has_side_effects = true;
            }

            fn visit_update_expr(&mut self, _update: &UpdateExpr) {
                // Updates have side effects
                self.has_side_effects = true;
            }
        }

        let mut analyzer = SideEffectAnalyzer { has_side_effects: false };
        function_body.visit_with(&mut analyzer);
        self.has_side_effects = analyzer.has_side_effects;
    }
}

impl WebpackModuleGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Recreate the module graph from a webpack chunk for linking
    pub fn hydrate_module_graph_from_chunk(&mut self, program: &Program) {
        console_log!("🔍 Starting webpack module graph analysis for linking...");
        
        // Step 1: Extract all module definitions
        self.extract_module_definitions(program);
        console_log!("📦 Found {} module definitions", self.modules.len());
        
        // Step 2: Analyze dependencies for each module
        self.analyze_module_dependencies(program);
        console_log!("🔗 Analyzed dependencies for all modules");
        
        // Step 3: Identify entry points
        self.identify_entry_points(program);
        console_log!("🚀 Found {} entry points", self.entry_modules.len());
        
        // Step 4: Analyze webpack runtime
        self.analyze_runtime_functions(program);
        console_log!("⚙️  Found {} runtime functions", self.runtime_functions.len());
        
        // Step 5: Calculate reachability from entry points
        self.calculate_reachable_modules();
        console_log!("✅ Calculated {} reachable modules", self.reachable_modules.len());
        
        // Step 6: Determine execution order
        self.calculate_execution_order();
        console_log!("📋 Determined execution order for {} modules", self.execution_order.len());
        
        self.print_comprehensive_analysis();
    }

    /// Extract all module definitions from __webpack_modules__
    fn extract_module_definitions(&mut self, program: &Program) {
        struct ModuleExtractor<'a> {
            graph: &'a mut WebpackModuleGraph,
        }

        impl<'a> Visit for ModuleExtractor<'a> {
            fn visit_var_declarator(&mut self, declarator: &VarDeclarator) {
                if let Some(ident) = declarator.name.as_ident() {
                    if ident.sym == "__webpack_modules__" {
                        if let Some(init) = &declarator.init {
                            self.extract_modules_from_expr(&**init);
                        }
                    }
                }
                declarator.visit_children_with(self);
            }

            fn visit_assign_expr(&mut self, assign: &AssignExpr) {
                if let AssignTarget::Simple(SimpleAssignTarget::Ident(ident)) = &assign.left {
                    if ident.sym == "__webpack_modules__" {
                        self.extract_modules_from_expr(&assign.right);
                    }
                }
                assign.visit_children_with(self);
            }

            fn visit_object_lit(&mut self, obj: &ObjectLit) {
                // Check if this could be a webpack modules object
                if self.looks_like_webpack_modules(obj) {
                    self.extract_modules_from_object(obj);
                } else {
                    obj.visit_children_with(self);
                }
            }
        }

        impl<'a> ModuleExtractor<'a> {
            fn extract_modules_from_expr(&mut self, expr: &Expr) {
                match expr {
                    Expr::Object(obj) => self.extract_modules_from_object(obj),
                    Expr::Paren(paren) => self.extract_modules_from_expr(&paren.expr),
                    _ => {}
                }
            }

            fn extract_modules_from_object(&mut self, obj: &ObjectLit) {
                for prop in &obj.props {
                    if let PropOrSpread::Prop(prop) = prop {
                        if let Prop::KeyValue(kv) = &**prop {
                            if let Some(module_id) = self.extract_module_id(&kv.key) {
                                let mut module = WebpackModule::new(module_id.clone());
                                
                                // Analyze the module function
                                if let Expr::Fn(func_expr) = &*kv.value {
                                    self.analyze_module_function(&mut module, &func_expr.function);
                                }
                                
                                console_log!("  📌 Extracted module: {}", module_id);
                                self.graph.modules.insert(module_id, module);
                            }
                        }
                    }
                }
            }

            fn analyze_module_function(&mut self, module: &mut WebpackModule, func: &Function) {
                // Analyze side effects in the function body
                if let Some(body) = &func.body {
                    module.analyze_side_effects(body);
                }
            }

            fn extract_module_id(&self, key: &PropName) -> Option<String> {
                match key {
                    PropName::Str(s) => Some(s.value.to_string()),
                    PropName::Num(n) => Some(n.value.to_string()),
                    PropName::Ident(i) => Some(i.sym.to_string()),
                    _ => None,
                }
            }

            fn looks_like_webpack_modules(&self, obj: &ObjectLit) -> bool {
                if obj.props.is_empty() {
                    return false;
                }

                let mut module_like_props = 0;
                for prop in &obj.props {
                    if let PropOrSpread::Prop(prop) = prop {
                        if let Prop::KeyValue(kv) = &**prop {
                            // Check if key looks like a module ID
                            if self.extract_module_id(&kv.key).is_some() {
                                // Check if value looks like a module function
                                if matches!(&*kv.value, Expr::Fn(_) | Expr::Arrow(_)) {
                                    module_like_props += 1;
                                }
                            }
                        }
                    }
                }

                // If most properties look like modules, treat as webpack modules
                module_like_props > 0 && module_like_props as f32 >= obj.props.len() as f32 * 0.6
            }
        }

        let mut extractor = ModuleExtractor { graph: self };
        program.visit_with(&mut extractor);
    }

    /// Analyze dependencies within each module
    fn analyze_module_dependencies(&mut self, program: &Program) {
        struct DependencyAnalyzer<'a> {
            graph: &'a mut WebpackModuleGraph,
        }

        impl<'a> Visit for DependencyAnalyzer<'a> {
            fn visit_object_lit(&mut self, obj: &ObjectLit) {
                if self.looks_like_webpack_modules(obj) {
                    for prop in &obj.props {
                        if let PropOrSpread::Prop(prop) = prop {
                            if let Prop::KeyValue(kv) = &**prop {
                                if let Some(module_id) = self.extract_module_id(&kv.key) {
                                    self.analyze_module_body(&module_id, &kv.value);
                                }
                            }
                        }
                    }
                } else {
                    obj.visit_children_with(self);
                }
            }
        }

        impl<'a> DependencyAnalyzer<'a> {
            fn analyze_module_body(&mut self, module_id: &str, expr: &Expr) {
                struct ModuleBodyAnalyzer<'a> {
                    module_id: String,
                    graph: &'a mut WebpackModuleGraph,
                }

                impl<'a> Visit for ModuleBodyAnalyzer<'a> {
                    fn visit_call_expr(&mut self, call: &CallExpr) {
                        if let Callee::Expr(callee) = &call.callee {
                            if let Expr::Ident(ident) = &**callee {
                                if ident.sym == "__webpack_require__" {
                                    if let Some(arg) = call.args.first() {
                                        if let Some(target_id) = self.extract_module_id(&arg.expr) {
                                            // Add dependency
                                            if let Some(module) = self.graph.modules.get_mut(&self.module_id) {
                                                module.add_dependency(target_id);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        call.visit_children_with(self);
                    }

                    fn visit_member_expr(&mut self, member: &MemberExpr) {
                        // Look for exports assignments: exports.something = ...
                        if let Expr::Ident(obj) = &*member.obj {
                            if obj.sym == "exports" || obj.sym == "__webpack_exports__" {
                                if let MemberProp::Ident(prop) = &member.prop {
                                    if let Some(module) = self.graph.modules.get_mut(&self.module_id) {
                                        module.exports.insert(prop.sym.to_string());
                                    }
                                }
                            }
                        }
                        member.visit_children_with(self);
                    }
                }

                impl<'a> ModuleBodyAnalyzer<'a> {
                    fn extract_module_id(&self, expr: &Expr) -> Option<String> {
                        match expr {
                            Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
                            Expr::Lit(Lit::Num(n)) => Some(n.value.to_string()),
                            _ => None,
                        }
                    }
                }

                let mut analyzer = ModuleBodyAnalyzer {
                    module_id: module_id.to_string(),
                    graph: self.graph,
                };
                expr.visit_with(&mut analyzer);
            }

            fn extract_module_id(&self, key: &PropName) -> Option<String> {
                match key {
                    PropName::Str(s) => Some(s.value.to_string()),
                    PropName::Num(n) => Some(n.value.to_string()),
                    PropName::Ident(i) => Some(i.sym.to_string()),
                    _ => None,
                }
            }

            fn looks_like_webpack_modules(&self, obj: &ObjectLit) -> bool {
                if obj.props.is_empty() {
                    return false;
                }

                let mut module_like_props = 0;
                for prop in &obj.props {
                    if let PropOrSpread::Prop(prop) = prop {
                        if let Prop::KeyValue(kv) = &**prop {
                            if self.extract_module_id(&kv.key).is_some() {
                                if matches!(&*kv.value, Expr::Fn(_) | Expr::Arrow(_)) {
                                    module_like_props += 1;
                                }
                            }
                        }
                    }
                }

                module_like_props > 0 && module_like_props as f32 >= obj.props.len() as f32 * 0.6
            }
        }

        let mut analyzer = DependencyAnalyzer { graph: self };
        program.visit_with(&mut analyzer);
    }

    /// Identify entry points (modules executed from the main context)
    fn identify_entry_points(&mut self, program: &Program) {
        struct EntryPointAnalyzer<'a> {
            graph: &'a mut WebpackModuleGraph,
            in_function_context: bool,
            in_webpack_modules: bool,
        }

        impl<'a> Visit for EntryPointAnalyzer<'a> {
            fn visit_call_expr(&mut self, call: &CallExpr) {
                // Look for __webpack_require__ calls that are NOT inside module functions
                if !self.in_webpack_modules {
                    if let Callee::Expr(callee) = &call.callee {
                        if let Expr::Ident(ident) = &**callee {
                            if ident.sym == "__webpack_require__" {
                                if let Some(arg) = call.args.first() {
                                    if let Some(module_id) = self.extract_module_id(&arg.expr) {
                                        console_log!("🚀 Entry point detected: {}", module_id);
                                        self.graph.entry_modules.insert(module_id.clone());
                                        if let Some(module) = self.graph.modules.get_mut(&module_id) {
                                            module.is_entry = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                call.visit_children_with(self);
            }

            fn visit_object_lit(&mut self, obj: &ObjectLit) {
                // Check if this is a __webpack_modules__ object
                let was_in_webpack_modules = self.in_webpack_modules;
                if self.looks_like_webpack_modules(obj) {
                    self.in_webpack_modules = true;
                }
                
                obj.visit_children_with(self);
                
                self.in_webpack_modules = was_in_webpack_modules;
            }

            fn visit_function(&mut self, func: &Function) {
                let was_in_function = self.in_function_context;
                self.in_function_context = true;
                func.visit_children_with(self);
                self.in_function_context = was_in_function;
            }

            fn visit_arrow_expr(&mut self, arrow: &ArrowExpr) {
                let was_in_function = self.in_function_context;
                self.in_function_context = true;
                arrow.visit_children_with(self);
                self.in_function_context = was_in_function;
            }

            fn visit_var_declarator(&mut self, declarator: &VarDeclarator) {
                // Skip __webpack_modules__ variable declarations
                if let Some(ident) = declarator.name.as_ident() {
                    if ident.sym == "__webpack_modules__" {
                        declarator.visit_children_with(self);
                        return;
                    }
                }
                
                // Look for entry point assignments: var x = __webpack_require__(id)
                if !self.in_webpack_modules {
                    if let Some(init) = &declarator.init {
                        if let Expr::Call(call) = &**init {
                            if let Callee::Expr(callee) = &call.callee {
                                if let Expr::Ident(ident) = &**callee {
                                    if ident.sym == "__webpack_require__" {
                                        if let Some(arg) = call.args.first() {
                                            if let Some(module_id) = self.extract_module_id(&arg.expr) {
                                                console_log!("🚀 Entry point detected (assignment): {}", module_id);
                                                self.graph.entry_modules.insert(module_id.clone());
                                                if let Some(module) = self.graph.modules.get_mut(&module_id) {
                                                    module.is_entry = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                declarator.visit_children_with(self);
            }
        }

        impl<'a> EntryPointAnalyzer<'a> {
            fn extract_module_id(&self, expr: &Expr) -> Option<String> {
                match expr {
                    Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
                    Expr::Lit(Lit::Num(n)) => Some(n.value.to_string()),
                    _ => None,
                }
            }

            fn looks_like_webpack_modules(&self, obj: &ObjectLit) -> bool {
                if obj.props.is_empty() {
                    return false;
                }

                let mut module_like_props = 0;
                for prop in &obj.props {
                    if let PropOrSpread::Prop(prop) = prop {
                        if let Prop::KeyValue(kv) = &**prop {
                            // Check if key looks like a module ID
                            if self.extract_module_id_from_prop(&kv.key).is_some() {
                                // Check if value looks like a module function
                                if matches!(&*kv.value, Expr::Fn(_) | Expr::Arrow(_)) {
                                    module_like_props += 1;
                                }
                            }
                        }
                    }
                }

                // If most properties look like modules, treat as webpack modules
                module_like_props > 0 && module_like_props as f32 >= obj.props.len() as f32 * 0.6
            }

            fn extract_module_id_from_prop(&self, key: &PropName) -> Option<String> {
                match key {
                    PropName::Str(s) => Some(s.value.to_string()),
                    PropName::Num(n) => Some(n.value.to_string()),
                    PropName::Ident(i) => Some(i.sym.to_string()),
                    _ => None,
                }
            }
        }

        let mut analyzer = EntryPointAnalyzer { 
            graph: self, 
            in_function_context: false,
            in_webpack_modules: false,
        };
        program.visit_with(&mut analyzer);
    }

    /// Analyze webpack runtime functions
    fn analyze_runtime_functions(&mut self, program: &Program) {
        struct RuntimeAnalyzer<'a> {
            graph: &'a mut WebpackModuleGraph,
        }

        impl<'a> Visit for RuntimeAnalyzer<'a> {
            fn visit_member_expr(&mut self, member: &MemberExpr) {
                if let Expr::Ident(obj) = &*member.obj {
                    if obj.sym == "__webpack_require__" {
                        if let MemberProp::Ident(prop) = &member.prop {
                            self.graph.runtime_functions.insert(prop.sym.to_string());
                        }
                    }
                }
                member.visit_children_with(self);
            }

            fn visit_assign_expr(&mut self, assign: &AssignExpr) {
                if let AssignTarget::Simple(SimpleAssignTarget::Member(member)) = &assign.left {
                    if let Expr::Ident(obj) = &*member.obj {
                        if obj.sym == "__webpack_require__" {
                            if let MemberProp::Ident(prop) = &member.prop {
                                self.graph.runtime_functions.insert(prop.sym.to_string());
                            }
                        }
                    }
                }
                assign.visit_children_with(self);
            }
        }

        let mut analyzer = RuntimeAnalyzer { graph: self };
        program.visit_with(&mut analyzer);
    }

    /// Calculate which modules are reachable from entry points
    fn calculate_reachable_modules(&mut self) {
        let mut visited = FxHashSet::default();
        let mut stack = Vec::new();

        // Start with all entry points
        for entry_id in &self.entry_modules {
            if !visited.contains(entry_id) {
                stack.push(entry_id.clone());
            }
        }

        // Depth-first search to find all reachable modules
        while let Some(current_id) = stack.pop() {
            if visited.contains(&current_id) {
                continue;
            }

            visited.insert(current_id.clone());
            self.reachable_modules.insert(current_id.clone());

            // Add dependencies to stack
            if let Some(module) = self.modules.get(&current_id) {
                for dep in &module.dependencies {
                    if !visited.contains(dep) && self.modules.contains_key(dep) {
                        stack.push(dep.clone());
                    }
                }
            }
        }
    }

    /// Calculate execution order using topological sort
    fn calculate_execution_order(&mut self) {
        let mut in_degree = FxHashMap::default();
        let mut adj_list = FxHashMap::default();

        // Initialize in-degree and adjacency list
        for module_id in self.reachable_modules.iter() {
            in_degree.insert(module_id.clone(), 0);
            adj_list.insert(module_id.clone(), Vec::new());
        }

        // Build dependency graph
        for module_id in &self.reachable_modules {
            if let Some(module) = self.modules.get(module_id) {
                for dep in &module.dependencies {
                    if self.reachable_modules.contains(dep) {
                        adj_list.entry(dep.clone()).or_insert_with(Vec::new).push(module_id.clone());
                        *in_degree.entry(module_id.clone()).or_insert(0) += 1;
                    }
                }
            }
        }

        // Topological sort
        let mut queue = Vec::new();
        for (module_id, degree) in &in_degree {
            if *degree == 0 {
                queue.push(module_id.clone());
            }
        }

        let mut order = Vec::new();
        while let Some(module_id) = queue.pop() {
            order.push(module_id.clone());

            if let Some(dependents) = adj_list.get(&module_id) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(dependent.clone());
                        }
                    }
                }
            }
        }

        self.execution_order = order;
    }

    /// Get modules that are not reachable (can be removed)
    pub fn get_unused_modules(&self) -> FxHashSet<String> {
        self.modules.keys()
            .filter(|id| !self.reachable_modules.contains(*id))
            .cloned()
            .collect()
    }

    /// Get bare require calls that can be removed
    pub fn get_unused_requires(&self, program: &Program) -> FxHashSet<String> {
        let mut unused_requires = FxHashSet::default();
        
        struct UnusedCallAnalyzer<'a> {
            graph: &'a WebpackModuleGraph,
            unused_requires: &'a mut FxHashSet<String>,
            depth: usize,
        }

        impl<'a> Visit for UnusedCallAnalyzer<'a> {
            fn visit_stmt(&mut self, stmt: &Stmt) {
                if self.depth <= 2 {  // Only top-level statements
                    if let Stmt::Expr(expr_stmt) = stmt {
                        if let Expr::Call(call) = &*expr_stmt.expr {
                            if let Callee::Expr(callee) = &call.callee {
                                if let Expr::Ident(ident) = &**callee {
                                    if ident.sym == "__webpack_require__" {
                                        if let Some(arg) = call.args.first() {
                                            if let Some(module_id) = self.extract_module_id(&arg.expr) {
                                                if !self.graph.reachable_modules.contains(&module_id) {
                                                    self.unused_requires.insert(module_id);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                let old_depth = self.depth;
                self.depth += 1;
                stmt.visit_children_with(self);
                self.depth = old_depth;
            }

            fn visit_function(&mut self, func: &Function) {
                let old_depth = self.depth;
                self.depth += 1;
                func.visit_children_with(self);
                self.depth = old_depth;
            }
        }

        impl<'a> UnusedCallAnalyzer<'a> {
            fn extract_module_id(&self, expr: &Expr) -> Option<String> {
                match expr {
                    Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
                    Expr::Lit(Lit::Num(n)) => Some(n.value.to_string()),
                    _ => None,
                }
            }
        }

        let mut analyzer = UnusedCallAnalyzer { 
            graph: self, 
            unused_requires: &mut unused_requires,
            depth: 0,
        };
        program.visit_with(&mut analyzer);
        
        unused_requires
    }

    /// Print comprehensive analysis results
    fn print_comprehensive_analysis(&self) {
        console_log!("📊 === WEBPACK MODULE GRAPH ANALYSIS ===");
        console_log!("📦 Total modules: {}", self.modules.len());
        console_log!("🚀 Entry modules: {} {:?}", self.entry_modules.len(), self.entry_modules);
        console_log!("✅ Reachable modules: {}", self.reachable_modules.len());
        console_log!("🗑️  Unreachable modules: {}", self.modules.len() - self.reachable_modules.len());
        console_log!("⚙️  Runtime functions: {} {:?}", self.runtime_functions.len(), self.runtime_functions);
        
        if !self.execution_order.is_empty() {
            console_log!("📋 Execution order: {:?}", self.execution_order);
        }

        // Show dependency counts
        let mut total_deps = 0;
        for module in self.modules.values() {
            total_deps += module.dependencies.len();
        }
        console_log!("🔗 Total dependencies: {}", total_deps);
        
        let unused = self.get_unused_modules();
        if !unused.is_empty() {
            console_log!("🗑️  Unused module IDs: {:?}", unused);
        }
    }
} 