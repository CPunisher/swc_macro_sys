//! Webpack Chunk Parser
//
//! A module for parsing webpack chunks and extracting module information.
//! This parser can analyze webpack bundle files and extract module object keys from chunks.
//! Uses AST parsing instead of regex for more reliable parsing.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use swc_core::ecma::ast::*;
use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_core::ecma::visit::{Visit, VisitWith};
// Removed unused imports: SourceMap, sync::Lrc

/// Errors that can occur during webpack parsing
#[derive(Error, Debug)]
pub enum WebpackParseError {
    #[error("Failed to parse JavaScript: {0}")]
    ParseError(String),
    
    #[error("Invalid webpack chunk format: {0}")]
    InvalidChunkFormat(String),
    
    #[error("Module not found: {0}")]
    ModuleNotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("SWC parsing error: {0}")]
    SwcError(String),
}

/// Result type for webpack parsing operations
pub type Result<T> = std::result::Result<T, WebpackParseError>;

/// Information about a webpack module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// The module path/key
    pub path: String,
    /// The module content (function body)
    pub content: String,
    /// Dependencies of this module
    pub dependencies: Vec<String>,
}

/// Information about a webpack chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    /// Chunk name/identifier
    pub name: String,
    /// Modules contained in this chunk
    pub modules: HashMap<String, ModuleInfo>,
}

/// Webpack chunk parser
pub struct WebpackChunkParser {}

/// AST visitor to extract webpack chunk information
struct WebpackChunkVisitor {
    pub chunk_name: String,
    pub modules: HashMap<String, ModuleInfo>,
    in_webpack_push: bool,
    current_object_depth: usize,
}

impl WebpackChunkVisitor {
    fn new() -> Self {
        Self {
            chunk_name: String::new(),
            modules: HashMap::new(),
            in_webpack_push: false,
            current_object_depth: 0,
        }
    }
    
    fn extract_string_value(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
            _ => None,
        }
    }
    
    fn extract_dependencies_from_function(&self, func: &Function) -> Vec<String> {
        let mut visitor = DependencyVisitor::new();
        if let Some(body) = &func.body {
            body.visit_with(&mut visitor);
        }
        visitor.dependencies
    }
    
    fn extract_dependencies_from_arrow_function(&self, arrow: &ArrowExpr) -> Vec<String> {
        let mut visitor = DependencyVisitor::new();
        arrow.body.visit_with(&mut visitor);
        visitor.dependencies
    }
    
    fn extract_dependencies_from_call_expr(&self, call: &CallExpr) -> Vec<String> {
        let mut visitor = DependencyVisitor::new();
        call.visit_with(&mut visitor);
        visitor.dependencies
    }
}

impl Visit for WebpackChunkVisitor {
    fn visit_call_expr(&mut self, call: &CallExpr) {
        // Look for any push call - we assume this is a webpack chunk since chunk type is provided externally
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Member(member) = expr.as_ref() {
                if let MemberProp::Ident(ident) = &member.prop {
                    if ident.sym.as_ref() == "push" {
                        // Extract chunk info from push arguments
                        self.in_webpack_push = true;
                        self.extract_chunk_info(&call.args);
                    }
                }
            }
        }
        
        call.visit_children_with(self);
    }
    
    fn visit_object_lit(&mut self, obj: &ObjectLit) {
        // Do not attempt to extract module keys from arbitrary object literals.
        // We only extract module entries from the explicit modules object handled in extract_chunk_info.
        obj.visit_children_with(self);
    }
}

impl WebpackChunkVisitor {

    
    fn extract_chunk_info(&mut self, args: &[ExprOrSpread]) {
        // Extract chunk information from push arguments
        // Expected format: push([chunk_names, {modules}]) - single array with two elements
        
        if let Some(first_arg) = args.first() {
            if let Expr::Array(arr) = &*first_arg.expr {
                // First element should be chunk names array
                if let Some(first_elem) = arr.elems.first() {
                    if let Some(elem) = first_elem {
                        if let Expr::Array(chunk_names) = &*elem.expr {
                            // Extract chunk name from first array
                            if let Some(first_name) = chunk_names.elems.first() {
                                if let Some(name_elem) = first_name {
                                    if let Some(name) = self.extract_string_value(name_elem.expr.as_ref()) {
                                        self.chunk_name = name;
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Second element should be modules object
                if arr.elems.len() >= 2 {
                    if let Some(second_elem) = arr.elems.get(1) {
                        if let Some(elem) = second_elem {
                            if let Expr::Object(obj) = &*elem.expr {
                                // Process all module properties
                                for prop in &obj.props {
                                    if let PropOrSpread::Prop(prop) = prop {
                                        self.extract_module_from_prop(prop);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn extract_module_from_prop(&mut self, prop: &Prop) {
        if let Prop::KeyValue(kv) = prop {
            // Accept string or numeric module IDs only
            let module_path_opt: Option<String> = match &kv.key {
                PropName::Str(key_str) => Some(key_str.value.to_string()),
                PropName::Num(num) => Some(num.value.to_string()),
                _ => None,
            };

            if let Some(module_path) = module_path_opt {
                // Only treat entries whose value is a function (module factory) as valid modules
                // Also handle a parenthesized function expression
                let mut dependencies: Vec<String> = Vec::new();
                let is_module_fn = match kv.value.as_ref() {
                    Expr::Fn(fn_expr) => {
                        dependencies = self.extract_dependencies_from_function(&fn_expr.function);
                        true
                    }
                    Expr::Arrow(arrow) => {
                        dependencies = self.extract_dependencies_from_arrow_function(arrow);
                        true
                    }
                    Expr::Paren(paren) => match paren.expr.as_ref() {
                        Expr::Fn(fn_expr) => {
                            dependencies = self.extract_dependencies_from_function(&fn_expr.function);
                            true
                        }
                        Expr::Arrow(arrow) => {
                            dependencies = self.extract_dependencies_from_arrow_function(arrow);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                };

                if is_module_fn {
                    let module_info = ModuleInfo {
                        path: module_path.clone(),
                        // We avoid storing huge function bodies; this field can be expanded if needed
                        content: "module".to_string(),
                        dependencies,
                    };
                    self.modules.insert(module_path, module_info);
                }
            }
        }
    }
}

/// Visitor to extract dependencies from function bodies
struct DependencyVisitor {
    dependencies: Vec<String>,
}

impl DependencyVisitor {
    fn new() -> Self {
        Self {
            dependencies: Vec::new(),
        }
    }
}

impl Visit for DependencyVisitor {
    fn visit_call_expr(&mut self, call: &CallExpr) {
        // Look for __webpack_require__ calls
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                if ident.sym.as_ref() == "__webpack_require__" {
                    // Extract the dependency path from the first argument
                    if let Some(first_arg) = call.args.first() {
                        match first_arg.expr.as_ref() {
                            // Handle string literals
                            Expr::Lit(Lit::Str(s)) => {
                                self.dependencies.push(s.value.to_string());
                            },
                            // Handle numeric literals
                            Expr::Lit(Lit::Num(n)) => {
                                self.dependencies.push(n.value.to_string());
                            },
                            // Handle other potential cases
                            _ => {}
                        }
                    }
                }
            }
        }
        
        call.visit_children_with(self);
    }
}

impl WebpackChunkParser {
    /// Create a new webpack chunk parser
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
    
    /// Parse a webpack chunk file and extract all module keys
    pub fn parse_chunk_file(&self, content: &str) -> Result<ChunkInfo> {
        // Parse the JavaScript using SWC
        let input = StringInput::new(content, swc_core::common::BytePos(0), swc_core::common::BytePos(content.len() as u32));
        let lexer = Lexer::new(
            Syntax::Es(Default::default()),
            Default::default(),
            input,
            None,
        );
        
        let mut parser = Parser::new_from(lexer);
        let module = parser.parse_module()
            .map_err(|e| WebpackParseError::SwcError(format!("Failed to parse JavaScript: {:?}", e)))?;
        
        // Create a visitor to extract webpack chunk information
        let mut visitor = WebpackChunkVisitor::new();
        module.visit_with(&mut visitor);
        
        if visitor.chunk_name.is_empty() {
            return Err(WebpackParseError::InvalidChunkFormat(
                "Could not find webpack chunk structure".to_string()
            ));
        }
        
        Ok(ChunkInfo {
            name: visitor.chunk_name,
            modules: visitor.modules,
        })
    }

    /// Parse from an existing AST Module without reparsing source text.
    /// Useful when an upstream pipeline (e.g. optimizer) already has a Module AST available.
    pub fn parse_from_module(&self, module: &swc_core::ecma::ast::Module) -> Result<ChunkInfo> {
        let mut visitor = WebpackChunkVisitor::new();
        module.visit_with(&mut visitor);

        if visitor.chunk_name.is_empty() {
            return Err(WebpackParseError::InvalidChunkFormat(
                "Could not find webpack chunk structure".to_string(),
            ));
        }

        Ok(ChunkInfo {
            name: visitor.chunk_name,
            modules: visitor.modules,
        })
    }

    /// Parse directly from a Program. Only Module programs are supported; Script will return an error.
    pub fn parse_from_program(&self, program: &swc_core::ecma::ast::Program) -> Result<ChunkInfo> {
        match program {
            swc_core::ecma::ast::Program::Module(m) => self.parse_from_module(m),
            swc_core::ecma::ast::Program::Script(_) => Err(WebpackParseError::InvalidChunkFormat(
                "Unsupported Program::Script for webpack chunk parsing".to_string(),
            )),
        }
    }
    
    /// Get all module keys from a chunk
    pub fn get_module_keys(&self, chunk: &ChunkInfo) -> Vec<String> {
        chunk.modules.keys().cloned().collect()
    }
    
    /// Get module by key
    pub fn get_module<'a>(&self, chunk: &'a ChunkInfo, key: &str) -> Option<&'a ModuleInfo> {
        chunk.modules.get(key)
    }
    
    /// Get all modules with their dependencies
    pub fn get_modules_with_dependencies(&self, chunk_info: &ChunkInfo) -> Vec<(String, Vec<String>)> {
        chunk_info.modules.iter()
            .map(|(key, module)| (key.clone(), module.dependencies.clone()))
            .collect()
    }
    
    /// Parse multiple chunk files
    pub fn parse_multiple_chunks(&self, files: &[(String, String)]) -> Result<Vec<ChunkInfo>> {
        let mut chunks = Vec::new();
        
        for (filename, content) in files {
            match self.parse_chunk_file(content) {
                Ok(mut chunk) => {
                    // Use filename as chunk name if not found in content
                    if chunk.name.is_empty() {
                        chunk.name = filename.clone();
                    }
                    chunks.push(chunk);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", filename, e);
                }
            }
        }
        
        Ok(chunks)
    }

    /// Build an adjacency-list dependency graph from a parsed chunk
    pub fn build_dependency_graph(&self, chunk: &ChunkInfo) -> HashMap<String, Vec<String>> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for (id, m) in &chunk.modules {
            // Insert the module with its direct dependencies
            graph.insert(id.clone(), m.dependencies.clone());
            // Ensure that all dependencies exist as keys in the graph even if they're not present in the chunk
            for dep in &m.dependencies {
                graph.entry(dep.clone()).or_insert_with(Vec::new);
            }
        }
        graph
    }

    /// Build a full dependency tree starting from a module id.
    /// Cycles are represented by nodes with `cycle: true` and empty `dependencies`.
    pub fn build_dependency_tree(&self, chunk: &ChunkInfo, start_id: &str) -> Option<DependencyNode> {
        if !chunk.modules.contains_key(start_id) {
            // If the start module isn't present, return None per API expectation
            return None;
        }
        let graph = self.build_dependency_graph(chunk);
        let mut visiting: HashSet<String> = HashSet::new();
        Some(self.build_node_rec(start_id, &graph, chunk, &mut visiting))
    }

    fn build_node_rec(
        &self,
        id: &str,
        graph: &HashMap<String, Vec<String>>,
        chunk: &ChunkInfo,
        visiting: &mut HashSet<String>,
    ) -> DependencyNode {
        if !visiting.insert(id.to_string()) {
            // cycle detected
            return DependencyNode { id: id.to_string(), cycle: Some(true), dependencies: vec![] };
        }

        let deps = graph.get(id).cloned().unwrap_or_default();
        let mut children = Vec::new();
        for dep in deps {
            // Build child even if not present in this chunk (will be leaf)
            let child_node = if chunk.modules.contains_key(&dep) {
                self.build_node_rec(&dep, graph, chunk, visiting)
            } else {
                DependencyNode { id: dep.clone(), cycle: None, dependencies: vec![] }
            };
            children.push(child_node);
        }

        visiting.remove(id);
        DependencyNode { id: id.to_string(), cycle: None, dependencies: children }
    }
}

impl Default for WebpackChunkParser {
    fn default() -> Self {
        Self::new().expect("Failed to create webpack parser")
    }
}

/// Serializable node for dependency trees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle: Option<bool>,
    pub dependencies: Vec<DependencyNode>,
}
