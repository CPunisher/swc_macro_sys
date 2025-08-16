use swc_core::common::{sync::Lrc, SourceMap, FileName, GLOBALS};
use swc_core::ecma::parser::{Parser, StringInput, Syntax, EsSyntax};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};
use swc_core::atoms::Atom;
use rustc_hash::FxHashMap;

use crate::{
    chunk::{ChunkType, ChunkCharacteristics, WebpackChunk},
    module::{ModuleId, WebpackModule},
    Result,
};

/// Shared methods for extracting webpack_require calls from AST nodes
/// This eliminates ~400 lines of duplicated code across visitor implementations
struct SharedExtractionMethods;

impl SharedExtractionMethods {
    /// Extract webpack_require calls from function body
    fn extract_webpack_requires_from_function_body(body: &Option<BlockStmt>) -> String {
        let mut source = String::new();
        
        if let Some(body) = body {
            for stmt in body.stmts.iter() {
                Self::extract_webpack_requires_from_stmt(stmt, &mut source);
            }
        }
        
        source
    }

    /// Extract webpack_require calls from statements
    fn extract_webpack_requires_from_stmt(stmt: &Stmt, source: &mut String) {
        match stmt {
            Stmt::Expr(expr_stmt) => {
                Self::extract_webpack_requires_from_expr(&expr_stmt.expr, source, ExtractionMode::Basic);
            }
            Stmt::Decl(decl) => {
                if let Decl::Var(var_decl) = decl {
                    for decl in &var_decl.decls {
                        if let Some(init) = &decl.init {
                            Self::extract_webpack_requires_from_expr(init, source, ExtractionMode::Basic);
                        }
                    }
                }
            }
            Stmt::Block(block) => {
                for s in &block.stmts {
                    Self::extract_webpack_requires_from_stmt(s, source);
                }
            }
            Stmt::If(if_stmt) => {
                Self::extract_webpack_requires_from_expr(&if_stmt.test, source, ExtractionMode::Basic);
                Self::extract_webpack_requires_from_stmt(&if_stmt.cons, source);
                if let Some(alt) = &if_stmt.alt {
                    Self::extract_webpack_requires_from_stmt(alt, source);
                }
            }
            Stmt::Return(ret) => {
                if let Some(arg) = &ret.arg {
                    Self::extract_webpack_requires_from_expr(arg, source, ExtractionMode::Basic);
                }
            }
            Stmt::While(wh) => {
                Self::extract_webpack_requires_from_expr(&wh.test, source, ExtractionMode::Basic);
                Self::extract_webpack_requires_from_stmt(&wh.body, source);
            }
            Stmt::DoWhile(dw) => {
                Self::extract_webpack_requires_from_stmt(&dw.body, source);
                Self::extract_webpack_requires_from_expr(&dw.test, source, ExtractionMode::Basic);
            }
            Stmt::For(f) => {
                if let Some(init) = &f.init { match init { VarDeclOrExpr::VarDecl(v) => {
                        for d in &v.decls { if let Some(i) = &d.init { Self::extract_webpack_requires_from_expr(i, source, ExtractionMode::Basic); } }
                    }, VarDeclOrExpr::Expr(e) => Self::extract_webpack_requires_from_expr(e, source, ExtractionMode::Basic) } }
                if let Some(test) = &f.test { Self::extract_webpack_requires_from_expr(test, source, ExtractionMode::Basic); }
                if let Some(update) = &f.update { Self::extract_webpack_requires_from_expr(update, source, ExtractionMode::Basic); }
                Self::extract_webpack_requires_from_stmt(&f.body, source);
            }
            Stmt::ForIn(fi) => {
                if let ForHead::Pat(p) = &fi.left { let _ = p; }
                Self::extract_webpack_requires_from_expr(&fi.right, source, ExtractionMode::Basic);
                Self::extract_webpack_requires_from_stmt(&fi.body, source);
            }
            Stmt::ForOf(fo) => {
                if let ForHead::Pat(p) = &fo.left { let _ = p; }
                Self::extract_webpack_requires_from_expr(&fo.right, source, ExtractionMode::Basic);
                Self::extract_webpack_requires_from_stmt(&fo.body, source);
            }
            Stmt::Switch(sw) => {
                Self::extract_webpack_requires_from_expr(&sw.discriminant, source, ExtractionMode::Basic);
                for cc in &sw.cases {
                    if let Some(test) = &cc.test { Self::extract_webpack_requires_from_expr(test, source, ExtractionMode::Basic); }
                    for s in &cc.cons { Self::extract_webpack_requires_from_stmt(s, source); }
                }
            }
            _ => {}
        }
    }

    /// Extract webpack_require calls from expressions with different extraction modes
    fn extract_webpack_requires_from_expr(expr: &Expr, source: &mut String, mode: ExtractionMode) {
        match expr {
            Expr::Call(call) => {
                Self::handle_call_expr(call, source, mode);
            }
            Expr::Member(member) => {
                // Handle member expressions that might contain __webpack_require__
                Self::extract_webpack_requires_from_expr(&member.obj, source, mode);
            }
            Expr::Assign(assign) => {
                Self::extract_webpack_requires_from_expr(&assign.right, source, mode);
            }
            Expr::Seq(seq) => {
                for expr in &seq.exprs {
                    Self::extract_webpack_requires_from_expr(expr, source, mode);
                }
            }
            Expr::Arrow(arrow) => {
                Self::handle_arrow_expr(arrow, source, mode);
            }
            _ => {}
        }
    }

    /// Handle call expressions based on extraction mode
    fn handle_call_expr(call: &CallExpr, source: &mut String, mode: ExtractionMode) {
        if let Callee::Expr(callee_expr) = &call.callee {
            // Normalize callee: unwrap Paren and Seq (use last)
            fn normalize<'a>(mut e: &'a Expr) -> &'a Expr {
                loop {
                    match e {
                        Expr::Paren(p) => { e = &p.expr; }
                        Expr::Seq(s) => { if let Some(last) = s.exprs.last() { e = last; } else { break; } }
                        _ => break,
                    }
                }
                e
            }
            let norm = normalize(callee_expr.as_ref());

            match norm {
                Expr::Ident(ident) => {
                    if ident.sym == "__webpack_require__" {
                        // Found a webpack_require call
                        if let Some(ExprOrSpread { expr, .. }) = call.args.first() {
                            match expr.as_ref() {
                                Expr::Lit(Lit::Str(s)) => {
                                    source.push_str(&format!("__webpack_require__(\"{}\");\n", s.value));
                                }
                                Expr::Lit(Lit::Num(n)) => {
                                    // Support numeric module IDs (e.g., WebpackModules format)
                                    source.push_str(&format!("__webpack_require__({});\n", n.value));
                                }
                                Expr::Tpl(tpl) if tpl.exprs.is_empty() => {
                                    let cooked = tpl.quasis.iter().map(|q| q.raw.as_ref()).collect::<Vec<_>>().join("");
                                    if !cooked.is_empty() { source.push_str(&format!("__webpack_require__(\"{}\");\n", cooked)); }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Expr::Fn(fn_expr) => {
                    // IIFE: (function(){ ... __webpack_require__(...) ... })()
                    let body = &fn_expr.function.body;
                    let nested = Self::extract_webpack_requires_from_function_body(body);
                    if !nested.is_empty() { source.push_str(&nested); }
                }
                Expr::Arrow(arrow) => {
                    // IIFE arrow: (()=>{ ... __webpack_require__(...) ... })()
                    match &*arrow.body {
                        BlockStmtOrExpr::BlockStmt(block) => {
                            for stmt in &block.stmts {
                                Self::extract_webpack_requires_from_stmt(stmt, source);
                            }
                        }
                        BlockStmtOrExpr::Expr(expr) => {
                            Self::extract_webpack_requires_from_expr(expr, source, mode);
                        }
                    }
                }
                _ => {}
            }
            
            // Handle extended features based on mode
            match mode {
                ExtractionMode::WithWebpackD => {
                    Self::handle_webpack_d_calls(norm, call, source);
                }
                ExtractionMode::Basic => {
                    // Only basic __webpack_require__ calls
                }
            }
        }
    }


    /// Handle __webpack_require__.d() calls (JSONP-specific)
    fn handle_webpack_d_calls(callee_expr: &Expr, call: &CallExpr, source: &mut String) {
        if let Expr::Member(member) = callee_expr {
            if let Expr::Ident(ident) = member.obj.as_ref() {
                if ident.sym == "__webpack_require__" {
                    if let MemberProp::Ident(prop) = &member.prop {
                        if prop.sym == "d" {
                            // This is a __webpack_require__.d() call
                            source.push_str("__webpack_require__.d(exports, {\n");
                            
                            // Extract dependencies from the second argument (object literal)
                            if call.args.len() >= 2 {
                                let ExprOrSpread { expr, .. } = &call.args[1];
                                if let Expr::Object(obj) = expr.as_ref() {
                                    for prop in &obj.props {
                                        if let PropOrSpread::Prop(prop) = prop {
                                            if let Prop::KeyValue(kv) = prop.as_ref() {
                                                // Extract the property name
                                                let prop_name = match &kv.key {
                                                    PropName::Str(s) => s.value.to_string(),
                                                    PropName::Ident(ident) => ident.sym.to_string(),
                                                    _ => "unknown".to_string(),
                                                };
                                                
                                                if let Expr::Arrow(arrow) = kv.value.as_ref() {
                                                    // Extract the module ID from the arrow function
                                                    let module_id = Self::extract_module_id_from_arrow_function(arrow);
                                                    if let Some(module_id) = module_id {
                                                        source.push_str(&format!("  {}: () => __webpack_require__(\"{}\").default,\n", prop_name, module_id));
                                                    } else {
                                                        source.push_str(&format!("  {}: () => __webpack_require__(\"unknown\").default,\n", prop_name));
                                                    }
                                                    // Also extract individual calls for tracking
                                                    Self::extract_webpack_requires_from_expr(&kv.value, source, ExtractionMode::WithWebpackD);
                                                } else {
                                                    // For non-arrow function values, process normally
                                                    Self::extract_webpack_requires_from_expr(&kv.value, source, ExtractionMode::WithWebpackD);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            source.push_str("});\n");
                        }
                    }
                }
            }
        }
    }

    /// Handle arrow expressions
    fn handle_arrow_expr(arrow: &ArrowExpr, source: &mut String, mode: ExtractionMode) {
        match arrow.body.as_ref() {
            BlockStmtOrExpr::BlockStmt(block) => {
                for stmt in &block.stmts {
                    Self::extract_webpack_requires_from_stmt(stmt, source);
                }
            }
            BlockStmtOrExpr::Expr(expr) => {
                Self::extract_webpack_requires_from_expr(expr, source, mode);
            }
        }
    }

    /// Extract module ID from arrow function expressions like () => __webpack_require__("module.js").default
    fn extract_module_id_from_arrow_function(arrow: &ArrowExpr) -> Option<Atom> {
        match arrow.body.as_ref() {
            BlockStmtOrExpr::Expr(expr) => {
                Self::extract_module_id_from_expr(expr)
            }
            BlockStmtOrExpr::BlockStmt(_) => {
                None
            }
        }
    }

    /// Extract module ID from expressions that may contain __webpack_require__ calls
    fn extract_module_id_from_expr(expr: &Expr) -> Option<Atom> {
        match expr {
            Expr::Member(member) => {
                // Handle __webpack_require__("module.js").default
                if let Expr::Call(call) = member.obj.as_ref() {
                    if let Callee::Expr(callee_expr) = &call.callee {
                        if let Expr::Ident(ident) = callee_expr.as_ref() {
                            if ident.sym == "__webpack_require__" {
                                if let Some(ExprOrSpread { expr, .. }) = call.args.first() {
                                    if let Expr::Lit(Lit::Str(s)) = expr.as_ref() {
                                        return Some(s.value.clone());
                                    }
                                }
                            }
                        }
                    }
                }
                None
            }
            Expr::Call(call) => {
                // Handle direct __webpack_require__("module.js") calls
                if let Callee::Expr(callee_expr) = &call.callee {
                    if let Expr::Ident(ident) = callee_expr.as_ref() {
                        if ident.sym == "__webpack_require__" {
                            if let Some(ExprOrSpread { expr, .. }) = call.args.first() {
                                if let Expr::Lit(Lit::Str(s)) = expr.as_ref() {
                                    return Some(s.value.clone());
                                }
                            }
                        }
                    }
                }
                None
            }
            _ => None
        }
    }
}

/// Extraction modes for different visitor types
#[derive(Clone, Copy)]
enum ExtractionMode {
    Basic,        // CommonJS basic extraction
    WithWebpackD, // JSONP with __webpack_require__.d() handling
}

/// Main analyzer for webpack chunks
pub struct WebpackAnalyzer {
    source_map: Lrc<SourceMap>,
}

impl Default for WebpackAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl WebpackAnalyzer {
    /// Create a new webpack analyzer
    pub fn new() -> Self {
        Self {
            source_map: Default::default(),
        }
    }

    /// Analyze a webpack chunk with chunk characteristics (required)
    pub fn analyze_chunk(&self, source: &str, characteristics: ChunkCharacteristics) -> Result<WebpackChunk> {
        // Step 1: Determine chunk type from characteristics
        let chunk_type = characteristics.determine_chunk_type();
        
        // Step 2: Parse the source code (allow Unknown to enable CommonJS/JSONP fallback only)
        let program = self.parse_source(source)?;

        // Step 3: Create chunk with characteristics
        let mut chunk = WebpackChunk::new_with_characteristics(
            chunk_type.clone(),
            source.to_string(),
            characteristics,
        );

        // Step 4: Extract modules based on chunk type
        self.extract_modules(&program, &mut chunk)?;

        // Step 5: Build dependency graph
        self.build_dependency_graph(&mut chunk)?;

        Ok(chunk)
    }

    /// Rebuild the dependency graph for an existing chunk
    /// This is useful when the source code has changed and dependencies need to be re-analyzed
    pub fn rebuild_dependency_graph(&self, chunk: &mut WebpackChunk) -> Result<()> {
        // Clear existing dependencies
        for (_module_id, module) in chunk.modules.iter_mut() {
            module.dependencies.clear();
            module.dependents.clear();
        }

        // Rebuild dependency graph
        self.build_dependency_graph(chunk)?;

        Ok(())
    }


    /// Parse source code into AST
    fn parse_source(&self, source: &str) -> Result<Program> {
        // Wrap parsing in GLOBALS for WASM compatibility
        GLOBALS.set(&Default::default(), || {
            let fm = self.source_map.new_source_file(FileName::Custom("chunk.js".to_string()).into(), source.to_string());
            let mut parser = Parser::new(
                Syntax::Es(EsSyntax::default()),
                StringInput::from(&*fm),
                None,
            );
            
            parser.parse_program()
                .map_err(|e| format!("Failed to parse source: {:?}", e).into())
        })
    }

    /// Extract modules from the AST based on chunk type
    fn extract_modules(&self, program: &Program, chunk: &mut WebpackChunk) -> Result<()> {
        match chunk.chunk_type {
            ChunkType::CommonJSAsync | ChunkType::CommonJSSync => {
                self.extract_commonjs_modules(program, chunk)
            },
            ChunkType::JSONP => self.extract_jsonp_modules(program, chunk),
            // Explicitly disable webpack runtime and ESModules styles
            ChunkType::WebpackModules | ChunkType::ESModules => {
                Err("Unsupported chunk format: only JSONP or CommonJS styles are supported".into())
            },
            ChunkType::Unknown => {
                // Try CommonJS and JSONP extraction methods only
                if self.extract_commonjs_modules(program, chunk).is_ok() && chunk.module_count() > 0 {
                    Ok(())
                } else if self.extract_jsonp_modules(program, chunk).is_ok() && chunk.module_count() > 0 {
                    Ok(())
                } else {
                    Err("Unsupported or unrecognized chunk format: only JSONP or CommonJS styles are supported".into())
                }
            },
        }
    }

    /// Extract modules from CommonJS format (exports.modules = {...})
    fn extract_commonjs_modules(&self, program: &Program, chunk: &mut WebpackChunk) -> Result<()> {
        let mut visitor = CommonJSVisitor::new();
        program.visit_with(&mut visitor);

        for (module_id, module_source) in visitor.modules {
            let module = WebpackModule::new(module_id.clone(), module_source);
            chunk.add_module(module_id, module);
        }

        Ok(())
    }

    /// Extract modules from JSONP format ((self["webpackChunk..."] = ...).push([[...], {...}]))
    fn extract_jsonp_modules(&self, program: &Program, chunk: &mut WebpackChunk) -> Result<()> {
        let mut visitor = JSONPVisitor::new();
        program.visit_with(&mut visitor);

        for (module_id, module_source) in visitor.modules {
            let module = WebpackModule::new(module_id.clone(), module_source);
            chunk.add_module(module_id, module);
        }

        Ok(())
    }

    /// Extract modules from WebpackModules format (var __webpack_modules__ = {...})
    fn extract_webpack_modules(&self, program: &Program, chunk: &mut WebpackChunk) -> Result<()> {
        let mut visitor = WebpackModulesVisitor::new();
        program.visit_with(&mut visitor);

        for (module_id, module_source) in visitor.modules {
            let module = WebpackModule::new(module_id.clone(), module_source);
            chunk.add_module(module_id, module);
        }

        Ok(())
    }

    /// Build dependency graph by analyzing webpack_require calls
    fn build_dependency_graph(&self, chunk: &mut WebpackChunk) -> Result<()> {
        // Extract dependencies from the module sources we already have
        for (_module_id, module) in chunk.modules.iter_mut() {
            let dependencies = self.extract_webpack_require_calls(&module.source)?;
            for dep in dependencies {
                module.add_dependency(dep);
            }
        }

        // Build reverse dependencies
        let module_ids: Vec<_> = chunk.modules.keys().cloned().collect();
        for from_module in &module_ids {
            if let Some(from_mod) = chunk.modules.get(from_module) {
                let deps = from_mod.get_dependencies();
                for to_module in deps {
                    if let Some(to_mod) = chunk.modules.get_mut(&to_module) {
                        to_mod.add_dependent(from_module.clone());
                    }
                }
            }
        }

        Ok(())
    }

    /// Extract webpack_require calls from module source
    fn extract_webpack_require_calls(&self, source: &str) -> Result<Vec<ModuleId>> {
        let mut dependencies = Vec::new();
        
        // If the source is empty, return empty dependencies
        if source.trim().is_empty() {
            return Ok(dependencies);
        }
        
        
        // Parse the module source to find webpack_require calls
        let fm = self.source_map.new_source_file(FileName::Custom("module.js".to_string()).into(), source.to_string());
        let mut parser = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            None,
        );
        
        match parser.parse_program() {
            Ok(program) => {
                let mut visitor = RequireVisitor::new();
                program.visit_with(&mut visitor);
                dependencies = visitor.dependencies;
            }
            Err(_e) => {
                // Parsing failed; skip dependency extraction to avoid false positives
                // Conservative behavior: return no dependencies rather than using best-effort heuristics
                dependencies = Vec::new();
            }
        }

        Ok(dependencies)
    }
}

/// Visitor for CommonJS format chunks
struct CommonJSVisitor {
    modules: FxHashMap<ModuleId, String>,
}

impl CommonJSVisitor {
    fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
        }
    }
}

impl Visit for CommonJSVisitor {
    fn visit_assign_expr(&mut self, node: &AssignExpr) {
        // Look for exports.modules = {...}
        if let AssignTarget::Simple(SimpleAssignTarget::Member(member)) = &node.left {
            if let MemberProp::Ident(prop) = &member.prop {
                if prop.sym == "modules" {
                    if let Expr::Ident(obj_ident) = member.obj.as_ref() {
                        if obj_ident.sym == "exports" {
                            // Found exports.modules = {...}
                            if let Expr::Object(obj) = node.right.as_ref() {
                                self.extract_modules_from_object(obj);
                            }
                        }
                    }
                }
            }
        }
        
        node.visit_children_with(self);
    }
}

impl CommonJSVisitor {
    fn extract_modules_from_object(&mut self, obj: &ObjectLit) {
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(kv) = prop.as_ref() {
                    // Extract module ID from key
                    let module_id = match &kv.key {
                        PropName::Str(s) => s.value.clone(),
                        PropName::Ident(ident) => ident.sym.clone(),
                        PropName::Num(num) => Atom::from(num.value.to_string()),
                        _ => continue,
                    };

                    // Extract module source from the function expression
                    let module_source = self.extract_function_source(&kv.value);
                    
                    
                    self.modules.insert(module_id, module_source);
                }
            }
        }
    }

    fn extract_function_source(&self, expr: &Expr) -> String {
        // Handle different patterns found in webpack chunks
        match expr {
            // Pattern 1: Direct function expressions
            Expr::Fn(func) => {
                
                SharedExtractionMethods::extract_webpack_requires_from_function_body(&func.function.body)
            }
            // Pattern 2: Function expressions wrapped in call expressions (real-world case)
            Expr::Call(call) => {
                if let Callee::Expr(callee_expr) = &call.callee {
                    if let Expr::Paren(paren) = callee_expr.as_ref() {
                        if let Expr::Fn(func) = paren.expr.as_ref() {
                            // This is a (function expression) being called
                            return SharedExtractionMethods::extract_webpack_requires_from_function_body(&func.function.body);
                        }
                    }
                }
                "/* Non-function call */".to_string()
            }
            // Pattern 3: Paren expressions wrapping function expressions
            Expr::Paren(paren) => {
                self.extract_function_source(&paren.expr)
            }
            _ => "/* Non-function module */".to_string(),
        }
    }

    fn extract_webpack_requires_from_function_body(&self, body: &Option<BlockStmt>) -> String {
        SharedExtractionMethods::extract_webpack_requires_from_function_body(body)
    }

    fn extract_webpack_requires_from_stmt(&self, stmt: &Stmt, source: &mut String) {
        SharedExtractionMethods::extract_webpack_requires_from_stmt(stmt, source);
    }

    fn extract_webpack_requires_from_expr(&self, expr: &Expr, source: &mut String) {
        SharedExtractionMethods::extract_webpack_requires_from_expr(expr, source, ExtractionMode::Basic);
    }
}

/// Visitor for JSONP format chunks
struct JSONPVisitor {
    modules: FxHashMap<ModuleId, String>,
}

impl JSONPVisitor {
    fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
        }
    }
}

impl Visit for JSONPVisitor {
    fn visit_call_expr(&mut self, node: &CallExpr) {
        // Look for .push() calls on webpackChunk arrays
        if let Callee::Expr(expr) = &node.callee {
            if let Expr::Member(member) = expr.as_ref() {
                if let MemberProp::Ident(prop) = &member.prop {
                    if prop.sym == "push" {
                        // Found a .push() call, check if it's on a webpackChunk array
                        if !node.args.is_empty() {
                            let ExprOrSpread { expr, .. } = &node.args[0];
                            if let Expr::Array(arr) = expr.as_ref() {
                                // Look for the modules object (usually second element)
                                if arr.elems.len() >= 2 {
                                    if let Some(Some(ExprOrSpread { expr, .. })) = arr.elems.get(1) {
                                        if let Expr::Object(obj) = expr.as_ref() {
                                            self.extract_modules_from_object(obj);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        node.visit_children_with(self);
    }
}

impl JSONPVisitor {
    fn extract_modules_from_object(&mut self, obj: &ObjectLit) {
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(kv) = prop.as_ref() {
                    // Extract module ID from key
                    let module_id = match &kv.key {
                        PropName::Str(s) => s.value.clone(),
                        PropName::Ident(ident) => ident.sym.clone(),
                        PropName::Num(num) => Atom::from(num.value.to_string()),
                        _ => continue,
                    };

                    // Extract complete module source from the function expression
                    let module_source = self.extract_complete_function_source(&kv.value);
                    
                    self.modules.insert(module_id, module_source);
                }
            }
        }
    }

    fn extract_complete_function_source(&self, expr: &Expr) -> String {
        // For now, fall back to the existing extraction method
        // TODO: Implement proper AST-to-source conversion when API is available
        self.extract_function_source(expr)
    }

    fn extract_function_source(&self, expr: &Expr) -> String {
        // Handle different patterns found in webpack chunks
        match expr {
            // Pattern 1: Direct function expressions
            Expr::Fn(func) => {
                
                SharedExtractionMethods::extract_webpack_requires_from_function_body(&func.function.body)
            }
            // Pattern 2: Function expressions wrapped in call expressions (real-world case)
            Expr::Call(call) => {
                if let Callee::Expr(callee_expr) = &call.callee {
                    if let Expr::Paren(paren) = callee_expr.as_ref() {
                        if let Expr::Fn(func) = paren.expr.as_ref() {
                            // This is a (function expression) being called
                            return SharedExtractionMethods::extract_webpack_requires_from_function_body(&func.function.body);
                        }
                    }
                }
                "/* Non-function call */".to_string()
            }
            // Pattern 3: Paren expressions wrapping function expressions
            Expr::Paren(paren) => {
                self.extract_function_source(&paren.expr)
            }
            _ => "/* Non-function module */".to_string(),
        }
    }

    fn extract_webpack_requires_from_function_body(&self, body: &Option<BlockStmt>) -> String {
        SharedExtractionMethods::extract_webpack_requires_from_function_body(body)
    }

    fn extract_webpack_requires_from_stmt(&self, stmt: &Stmt, source: &mut String) {
        SharedExtractionMethods::extract_webpack_requires_from_stmt(stmt, source);
    }

    fn extract_webpack_requires_from_expr(&self, expr: &Expr, source: &mut String) {
        SharedExtractionMethods::extract_webpack_requires_from_expr(expr, source, ExtractionMode::WithWebpackD);
    }

    /// Extract module ID from arrow function expressions like () => __webpack_require__("module.js").default
    fn extract_module_id_from_arrow_function(&self, arrow: &ArrowExpr) -> Option<Atom> {
        SharedExtractionMethods::extract_module_id_from_arrow_function(arrow)
    }

    /// Extract module ID from expressions that may contain __webpack_require__ calls
    fn extract_module_id_from_expr(&self, expr: &Expr) -> Option<Atom> {
        SharedExtractionMethods::extract_module_id_from_expr(expr)
    }
}

/// Visitor for extracting modules from WebpackModules format
struct WebpackModulesVisitor {
    modules: FxHashMap<ModuleId, String>,
}

impl WebpackModulesVisitor {
    fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
        }
    }
}

impl Visit for WebpackModulesVisitor {
    fn visit_var_decl(&mut self, node: &VarDecl) {
        // Look for: var __webpack_modules__ = ({...})
        for declarator in &node.decls {
            if let Some(ident) = declarator.name.as_ident() {
                if ident.sym == "__webpack_modules__" {
                    if let Some(init) = &declarator.init {
                        if let Expr::Paren(paren) = init.as_ref() {
                            if let Expr::Object(obj) = paren.expr.as_ref() {
                                self.extract_modules_from_object(obj);
                            }
                        } else if let Expr::Object(obj) = init.as_ref() {
                            self.extract_modules_from_object(obj);
                        }
                    }
                }
            }
        }
        
        node.visit_children_with(self);
    }
}

impl WebpackModulesVisitor {
    fn extract_modules_from_object(&mut self, obj: &ObjectLit) {
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(kv) = prop.as_ref() {
                    // Extract module ID from key
                    let module_id = match &kv.key {
                        PropName::Str(s) => s.value.clone(),
                        PropName::Ident(ident) => ident.sym.clone(),
                        PropName::Num(num) => Atom::from(num.value.to_string()),
                        _ => continue,
                    };

                    // Extract module source from the function
                    let module_source = self.extract_function_source(&kv.value);
                    
                    self.modules.insert(module_id, module_source);
                }
            }
        }
    }

    fn extract_function_source(&self, expr: &Expr) -> String {
        match expr {
            Expr::Paren(paren) => {
                self.extract_function_source(&paren.expr)
            }
            Expr::Fn(func) => {
                SharedExtractionMethods::extract_webpack_requires_from_function_body(&func.function.body)
            }
            _ => "/* non-function module */".to_string(),
        }
    }
    
    fn extract_webpack_requires_from_function_body(&self, body: &Option<BlockStmt>) -> String {
        SharedExtractionMethods::extract_webpack_requires_from_function_body(body)
    }

    fn extract_webpack_requires_from_stmt(&self, stmt: &Stmt, source: &mut String) {
        SharedExtractionMethods::extract_webpack_requires_from_stmt(stmt, source);
    }

    fn extract_webpack_requires_from_expr(&self, expr: &Expr, source: &mut String) {
        SharedExtractionMethods::extract_webpack_requires_from_expr(expr, source, ExtractionMode::Basic);
    }
}

/// Visitor for extracting webpack_require calls
struct RequireVisitor {
    dependencies: Vec<ModuleId>,
}

impl RequireVisitor {
    fn new() -> Self {
        Self {
            dependencies: Vec::new(),
        }
    }
}

impl Visit for RequireVisitor {
    fn visit_call_expr(&mut self, node: &CallExpr) {
        // Look for __webpack_require__("module_id") calls
        if let Callee::Expr(expr) = &node.callee {
            // Resolve the callee to an identifier symbol, handling parentheses and sequence expressions
            fn resolve_callee_ident<'a>(mut e: &'a Expr) -> Option<&'a Ident> {
                // Unwrap parens and sequence to the last expression
                loop {
                    match e {
                        Expr::Paren(p) => { e = &p.expr; }
                        Expr::Seq(s) => {
                            if let Some(last) = s.exprs.last() { e = last; } else { break; }
                        }
                        _ => break,
                    }
                }
                match e {
                    Expr::Ident(id) => Some(id),
                    _ => None,
                }
            }

            if let Some(id) = resolve_callee_ident(expr.as_ref()) {
                if id.sym == "__webpack_require__" {
                    // Found webpack_require call
                    if let Some(ExprOrSpread { expr, .. }) = node.args.first() {
                        match expr.as_ref() {
                            Expr::Lit(Lit::Str(s)) => {
                                self.dependencies.push(s.value.clone());
                            }
                            Expr::Lit(Lit::Num(n)) => {
                                self.dependencies.push(Atom::from(n.value.to_string()));
                            }
                            // Support template literals with no expressions: `./module.js`
                            Expr::Tpl(tpl) if tpl.exprs.is_empty() => {
                                let cooked = tpl.quasis.iter().map(|q| q.raw.as_ref()).collect::<Vec<_>>().join("");
                                if !cooked.is_empty() { self.dependencies.push(Atom::from(cooked)); }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        
        node.visit_children_with(self);
    }
}