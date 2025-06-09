use crate::{error::WebpackGraphError, graph::{ModuleGraph, ModuleNode}, Result};
use rustc_hash::FxHashMap;
use swc_core::common::{sync::Lrc, SourceMap, FileName, Span};
use swc_core::ecma::parser::{Parser, StringInput, Syntax, EsSyntax};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

/// Parser for webpack bundles that extracts module dependency graphs
pub struct WebpackBundleParser {
    source_map: Lrc<SourceMap>,
}

impl WebpackBundleParser {
    pub fn new() -> Result<Self> {
        Ok(Self {
            source_map: Default::default(),
        })
    }

    /// Parse a webpack bundle from source code and extract the module graph
    pub fn parse_bundle(&self, source: &str) -> Result<ModuleGraph> {
        // Parse the JavaScript using SWC
        let fm = self.source_map.new_source_file(
            FileName::Custom("webpack-bundle.js".to_string()).into(),
            source.to_string(),
        );

        let mut parser = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            None,
        );

        let program = parser
            .parse_program()
            .map_err(|e| WebpackGraphError::ParseError(format!("Failed to parse JavaScript: {:?}", e)))?;

        // Create visitor to extract webpack information
        let mut visitor = WebpackVisitor::new();
        program.visit_with(&mut visitor);

        if visitor.webpack_modules.is_empty() {
            return Err(WebpackGraphError::InvalidBundleFormat(
                "No __webpack_modules__ found in bundle".to_string(),
            ));
        }

        // Build the module graph
        let mut graph = ModuleGraph::new();

        // Add all modules to the graph and extract their dependencies
        for (module_id, module_source) in &visitor.webpack_modules {
            let dependencies = self.extract_dependencies_from_source(module_source);
            let mut module_node = ModuleNode::new(module_id.clone(), module_source.clone());
            
            for dep_id in dependencies {
                module_node.add_dependency(dep_id);
            }
            
            graph.add_module(module_node);
        }

        // Build dependency relationships
        for (module_id, module_node) in &graph.modules.clone() {
            for dep_id in &module_node.dependencies {
                graph.add_dependency(module_id, dep_id);
            }
        }

        // Add entry points from visitor
        for entry_id in visitor.entry_points {
            if graph.modules.contains_key(&entry_id) {
                graph.add_entry_point(entry_id);
            }
        }

        if graph.entry_points.is_empty() {
            return Err(WebpackGraphError::InvalidBundleFormat(
                "No entry points found - __webpack_require__ calls must exist outside __webpack_modules__".to_string(),
            ));
        }

        Ok(graph)
    }

    /// Extract __webpack_require__ calls from module source code using regex as fallback
    fn extract_dependencies_from_source(&self, source: &str) -> Vec<String> {
        // Simple regex fallback for extracting dependencies from module source
        let re = regex::Regex::new(r"__webpack_require__\((\d+)\)").unwrap();
        re.captures_iter(source)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect()
    }
}

/// AST visitor to extract webpack module information
struct WebpackVisitor {
    webpack_modules: FxHashMap<String, String>,
    entry_points: Vec<String>,
    webpack_modules_span: Option<Span>,
}

impl WebpackVisitor {
    fn new() -> Self {
        Self {
            webpack_modules: FxHashMap::default(),
            entry_points: Vec::new(),
            webpack_modules_span: None,
        }
    }

    /// Extract module content from object property
    fn extract_module_content(&self, prop: &PropOrSpread) -> Option<(String, String)> {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = prop.as_ref() {
                // Extract module ID
                let module_id = match &kv.key {
                    PropName::Num(num) => num.value.to_string().split('.').next()?.to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    PropName::Ident(ident) => ident.sym.to_string(),
                    _ => return None,
                };

                // Extract module source from the function expression
                let module_source = self.extract_function_source(&kv.value)
                    .unwrap_or_else(|| format!("/* Module {} */", module_id));
                
                return Some((module_id, module_source));
            }
        }
        None
    }

    /// Extract source code from a function expression in the module value
    fn extract_function_source(&self, expr: &Expr) -> Option<String> {
        // Instead of trying to convert back to source, let's extract the webpack_require calls directly
        let mut dependencies = Vec::new();
        self.extract_require_calls_from_expr(expr, &mut dependencies);
        
        // Return a representation that includes the dependencies for our regex fallback
        let deps_string = dependencies.iter()
            .map(|dep| format!("__webpack_require__({})", dep))
            .collect::<Vec<_>>()
            .join("; ");
        
        Some(format!("function() {{ {} }}", deps_string))
    }

    /// Recursively extract webpack_require calls from an expression
    fn extract_require_calls_from_expr(&self, expr: &Expr, dependencies: &mut Vec<String>) {
        match expr {
            Expr::Paren(paren) => {
                self.extract_require_calls_from_expr(&paren.expr, dependencies);
            }
            Expr::Fn(func) => {
                if let Some(body) = &func.function.body {
                    for stmt in &body.stmts {
                        self.extract_require_calls_from_stmt(stmt, dependencies);
                    }
                }
            }
            Expr::Call(call) => {
                if let Some(module_id) = self.extract_webpack_require_call(call) {
                    if !dependencies.contains(&module_id) {
                        dependencies.push(module_id);
                    }
                }
                // Also check arguments for nested calls
                for arg in &call.args {
                    self.extract_require_calls_from_expr(&arg.expr, dependencies);
                }
            }
            _ => {}
        }
    }

    /// Extract webpack_require calls from a statement
    fn extract_require_calls_from_stmt(&self, stmt: &Stmt, dependencies: &mut Vec<String>) {
        match stmt {
            Stmt::Expr(expr_stmt) => {
                self.extract_require_calls_from_expr(&expr_stmt.expr, dependencies);
            }
            Stmt::Decl(Decl::Var(var_decl)) => {
                for declarator in &var_decl.decls {
                    if let Some(init) = &declarator.init {
                        self.extract_require_calls_from_expr(init, dependencies);
                    }
                }
            }
            _ => {}
        }
    }

    /// Check if a call expression is a webpack_require call and extract module ID
    fn extract_webpack_require_call(&self, call: &CallExpr) -> Option<String> {
        // Check if callee is __webpack_require__
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                if ident.sym == "__webpack_require__" {
                    // Extract first argument (module ID)
                    if let Some(ExprOrSpread { expr, .. }) = call.args.first() {
                        if let Expr::Lit(Lit::Num(num)) = expr.as_ref() {
                            return Some(num.value.to_string().split('.').next()?.to_string());
                        }
                    }
                }
            }
        }
        None
    }
}

impl Visit for WebpackVisitor {
    /// Visit all declarations to find webpack_modules (var, let, const)
    fn visit_decl(&mut self, node: &Decl) {
        match node {
            Decl::Var(var_decl) => {
                self.process_var_declaration(var_decl);
            }
            _ => {}
        }
        
        // Continue visiting children
        node.visit_children_with(self);
    }

    /// Visit variable declarations to find webpack_modules
    fn visit_var_decl(&mut self, node: &VarDecl) {
        self.process_var_declaration(node);
        // Continue visiting children
        node.visit_children_with(self);
    }

    /// Visit assignment expressions to catch webpack_modules = {...} patterns
    fn visit_assign_expr(&mut self, node: &AssignExpr) {
        if let AssignTarget::Simple(SimpleAssignTarget::Ident(ident)) = &node.left {
            if ident.sym == "__webpack_modules__" {
                self.webpack_modules_span = Some(node.span);
                
                // Extract modules from the object literal - handle both direct and parenthesized objects
                let obj = match node.right.as_ref() {
                    Expr::Object(obj) => Some(obj),
                    Expr::Paren(paren) => {
                        if let Expr::Object(obj) = paren.expr.as_ref() {
                            Some(obj)
                        } else {
                            None
                        }
                    },
                    _ => None,
                };

                if let Some(obj) = obj {
                    for prop in &obj.props {
                        if let Some((module_id, module_source)) = self.extract_module_content(prop) {
                            self.webpack_modules.insert(module_id, module_source);
                        }
                    }
                }
            }
        }
        
        // Continue visiting children
        node.visit_children_with(self);
    }

    /// Visit call expressions to find webpack_require calls outside modules
    fn visit_call_expr(&mut self, node: &CallExpr) {
        // Check if we're inside the webpack_modules span
        let inside_webpack_modules = if let Some(modules_span) = self.webpack_modules_span {
            modules_span.contains(node.span)
        } else {
            false
        };

        // Only collect entry points if we're not inside webpack_modules definition
        if !inside_webpack_modules {
            if let Some(module_id) = self.extract_webpack_require_call(node) {
                if !self.entry_points.contains(&module_id) {
                    self.entry_points.push(module_id);
                }
            }
        }
        
        // Continue visiting children
        node.visit_children_with(self);
    }
}

impl WebpackVisitor {
    /// Process variable declarations (works for var, let, const)
    fn process_var_declaration(&mut self, node: &VarDecl) {
        for declarator in &node.decls {
            if let Pat::Ident(ident) = &declarator.name {
                // Check for various webpack modules variable names
                if ident.sym == "__webpack_modules__" {
                    self.webpack_modules_span = Some(node.span);
                    
                    // Extract modules from the object literal
                    if let Some(init) = &declarator.init {
                        // Handle both direct objects and parenthesized objects
                        let obj = match init.as_ref() {
                            Expr::Object(obj) => Some(obj),
                            Expr::Paren(paren) => {
                                if let Expr::Object(obj) = paren.expr.as_ref() {
                                    Some(obj)
                                } else {
                                    None
                                }
                            },
                            _ => None,
                        };

                        if let Some(obj) = obj {
                            for prop in &obj.props {
                                if let Some((module_id, module_source)) = self.extract_module_content(prop) {
                                    self.webpack_modules.insert(module_id, module_source);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Default for WebpackBundleParser {
    fn default() -> Self {
        Self::new().expect("Failed to create default WebpackBundleParser")
    }
} 