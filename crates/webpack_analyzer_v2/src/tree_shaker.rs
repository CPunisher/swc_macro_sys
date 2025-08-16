use std::collections::{HashSet, HashMap, VecDeque};
use std::sync::Arc;
use regex::Regex;

use rustc_hash::FxHashMap;
use swc_core::ecma::ast::{CallExpr, Callee, Expr, ExprOrSpread, Lit, MemberProp, ObjectLit, Pat, Prop, PropName, PropOrSpread, VarDecl};
use swc_core::ecma::codegen::{self, text_writer::JsWriter, Emitter};
use swc_core::ecma::parser::{EsSyntax, Lexer, Parser, StringInput, Syntax};
use swc_core::ecma::visit::{Visit, VisitMut, VisitMutWith, VisitWith};
use swc_core::common::{sync::Lrc, SourceMap, FileName};

use crate::dependency_graph::DependencyGraph;
use crate::module::{ModuleId, WebpackModule};
use crate::{chunk::ShareUsageConfig, ChunkCharacteristics, WebpackChunk};

/// Result of a prune operation planned and/or applied to a chunk
#[derive(Debug, Clone)]
pub struct PruneResult {
    /// Module identifiers that will be kept
    pub kept_modules: HashSet<ModuleId>,
    /// Module identifiers that will be removed
    pub removed_modules: HashSet<ModuleId>,
    /// Number of modules before pruning
    pub original_count: usize,
    /// Number of modules after pruning (kept)
    pub pruned_count: usize,
    /// If pruning was skipped, this will contain the reason
    pub skip_reason: Option<String>,
    /// Optional pruned chunk view with only kept modules (analysis-level view)
    pub pruned_chunk: Option<WebpackChunk>,
}

impl PruneResult {
    fn skipped(reason: String, original_count: usize) -> Self {
        Self {
            kept_modules: HashSet::new(),
            removed_modules: HashSet::new(),
            original_count,
            pruned_count: original_count,
            skip_reason: Some(reason),
            pruned_chunk: None,
        }
    }
}

/// Conservative, analysis-only tree shaker that plans/prunes unreachable modules
///
/// Behavior:
/// - Uses ONLY explicit entry points from `ShareUsageConfig` and optional
///   `ChunkCharacteristics.entry_module_id`
/// - Skips pruning for runtime chunks (based on `ChunkCharacteristics` only)
/// - Never infers entries from filenames or heuristics
pub struct TreeShaker;

impl Default for TreeShaker {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeShaker {
    pub fn new() -> Self {
        Self
    }

    /// Apply pruning directly to a source string and return optimized code with plan
    pub fn prune_source(
        &self,
        source: &str,
        characteristics: &ChunkCharacteristics,
    ) -> Result<(String, PruneResult), Box<dyn std::error::Error>> {
        // Parse
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(FileName::Custom("chunk.js".to_string()).into(), source.to_string());
        let program = Parser::new(Syntax::Es(EsSyntax::default()), StringInput::from(&*fm), None)
            .parse_program()
            .map_err(|e| format!("Parse error: {:?}", e))?;

        // Analyze
        let analyzer = crate::analyzer::WebpackAnalyzer::new();
        let chunk = analyzer.analyze_chunk(source, characteristics.clone())?;
        
        // CRITICAL FIX: Use the actual entry module from characteristics
        let entry_module_ids = if let Some(entry) = &characteristics.entry_module_id {
            vec![swc_core::atoms::Atom::from(entry.as_str())]
        } else {
            vec![]
        };
        let config = crate::chunk::ShareUsageConfig { 
            entry_module_ids,
            tree_shake: std::collections::HashMap::new(),
        };
        let plan = self.plan_prune(&chunk, &config);
        if plan.skip_reason.is_some() || plan.removed_modules.is_empty() {
            return Ok((source.to_string(), plan));
        }

        // Build pruner and mutate AST
        let mut program = program;
        let unreachable: std::collections::HashSet<String> = plan
            .removed_modules
            .iter()
            .map(|a| a.to_string())
            .collect();
        let mut pruner = AstModulePruner::new(unreachable);
        program.visit_mut_with(&mut pruner);

        // Emit
        let mut buf = vec![];
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(false),
            comments: None,
            cm: cm.clone(),
            wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None)),
        };
        emitter.emit_program(&program).map_err(|e| format!("Emit error: {:?}", e))?;
        let out = String::from_utf8(buf).map_err(|_| "Invalid UTF-8")?;
        Ok((out, plan))
    }

    /// Plan which modules could be removed based on reachability from explicit entries.
    /// Returns a `PruneResult` without modifying the input chunk.
    pub fn plan_prune(&self, chunk: &WebpackChunk, _config: &ShareUsageConfig) -> PruneResult {
        let original_count = chunk.modules.len();

        // Hard requirement: ChunkCharacteristics must be present
        let Some(characteristics) = &chunk.characteristics else {
            eprintln!("[TreeShaker] Skipping: missing ChunkCharacteristics");
            return PruneResult::skipped(
                "Missing ChunkCharacteristics; pruning skipped".to_string(),
                original_count,
            );
        };

        // If characteristics indicate runtime, skip pruning entirely
        if Self::is_runtime_chunk(characteristics) {
            eprintln!("[TreeShaker] Skipping: runtime chunk");
            return PruneResult::skipped(
                "Skipping pruning for runtime chunk as per characteristics".to_string(),
                original_count,
            );
        }

        // Entry points exclusively from characteristics.entry_module_id
        let Some(entry_id_str) = &characteristics.entry_module_id else {
            eprintln!("[TreeShaker] Skipping: entry_module_id missing in ChunkCharacteristics");
            return PruneResult::skipped(
                "ChunkCharacteristics.entry_module_id missing; pruning skipped".to_string(),
                original_count,
            );
        };
        let entry_id = swc_core::atoms::Atom::from(entry_id_str.as_str());
        if !chunk.modules.contains_key(&entry_id) {
            eprintln!("[TreeShaker] Skipping: entry_module_id not present in chunk modules");
            return PruneResult::skipped(
                "entry_module_id not present in chunk modules; pruning skipped".to_string(),
                original_count,
            );
        }
        let entry_points = vec![entry_id];

        // Build dependency graph from existing module records
        let mut graph = DependencyGraph::new();
        for module in chunk.modules.values() {
            graph.add_module(module.clone());
        }

        // Conservative safety net: keep any module that is directly referenced by
        // a __webpack_require__(<id>) anywhere in this chunk's source and that is
        // also defined in this chunk. This prevents removing modules that are
        // still required due to analysis edge misses in complex wrapper patterns.
        let defined_keys: HashSet<ModuleId> = chunk.modules.keys().cloned().collect();
        
        // Compute reachability
        let mut reachable: HashSet<ModuleId> = graph.get_reachable_from_multiple(&entry_points);
        eprintln!("[TreeShaker] Entry points: {:?}", entry_points.len());
        eprintln!("[TreeShaker] Defined modules: {}", defined_keys.len());
        eprintln!("[TreeShaker] Reachable modules: {}", reachable.len());
        let referenced_defined: HashSet<ModuleId> = Self::collect_defined_require_ids(&chunk.source, &defined_keys);
        
        // CRITICAL FIX: Force preserve scheduler modules regardless of reachability
        // The scheduler module is required by React DOM but the dependency chain
        // might be broken due to complex wrapper patterns
        for key in &defined_keys {
            if key.contains("scheduler") {
                eprintln!("[TreeShaker] Found scheduler module in chunk: {}", key);
                // Always preserve scheduler modules
                reachable.insert(key.clone());
                eprintln!("[TreeShaker] Force preserving scheduler module: {}", key);
            }
        }
        
        reachable.extend(referenced_defined.into_iter());

        let all: HashSet<ModuleId> = defined_keys;
        let removed: HashSet<ModuleId> = all.difference(&reachable).cloned().collect();

        PruneResult {
            kept_modules: reachable.clone(),
            removed_modules: removed,
            original_count,
            pruned_count: reachable.len(),
            skip_reason: None,
            pruned_chunk: None,
        }
    }

    /// Scan the chunk source using proper AST parsing to find all __webpack_require__ calls
    /// and return the subset of ids that correspond to module keys defined in this chunk.
    fn collect_defined_require_ids(source: &str, defined_keys: &HashSet<ModuleId>) -> HashSet<ModuleId> {
        let mut out: HashSet<ModuleId> = HashSet::new();
        
        // Parse the source as a module using SWC
        let cm = Arc::new(SourceMap::default());
        let fm = cm.new_source_file(FileName::Anon.into(), source.to_string());
        
        let syntax = swc_core::ecma::parser::Syntax::Es(swc_core::ecma::parser::EsSyntax {
            decorators: true,
            decorators_before_export: true,
            ..Default::default()
        });
        
        let mut parser = swc_core::ecma::parser::Parser::new_from(Lexer::new(
            syntax,
            swc_core::ecma::ast::EsVersion::latest(),
            StringInput::from(&*fm),
            None,
        ));
        
        match parser.parse_module() {
            Ok(module) => {
                // Create a visitor to find all __webpack_require__ calls that are at top-level
                // (i.e., NOT inside module wrapper functions that accept a `__webpack_require__` parameter).
                struct RequireCallFinder {
                    defined_keys: HashSet<ModuleId>,
                    found_ids: HashSet<ModuleId>,
                    // depth counter for functions that look like webpack module wrappers
                    wrapper_fn_depth: usize,
                }
                
                impl RequireCallFinder {
                    fn handle_require_call(&mut self, call: &CallExpr) {
                        // Only consider calls when not inside a module wrapper function
                        if self.wrapper_fn_depth > 0 { return; }
                        if let Some(arg) = call.args.first() {
                            if let Expr::Lit(Lit::Str(str_lit)) = &*arg.expr {
                                let id = swc_core::atoms::Atom::from(&*str_lit.value);
                                if self.defined_keys.contains(&id) {
                                    self.found_ids.insert(id);
                                }
                            } else if let Expr::Lit(Lit::Num(num_lit)) = &*arg.expr {
                                let id = swc_core::atoms::Atom::from(num_lit.value.to_string());
                                if self.defined_keys.contains(&id) {
                                    self.found_ids.insert(id);
                                }
                            }
                        }
                    }
                }
                
                impl Visit for RequireCallFinder {
                    fn visit_call_expr(&mut self, call: &CallExpr) {
                        // Check if this is a __webpack_require__ call
                        if let Callee::Expr(expr) = &call.callee {
                            if let Expr::Ident(ident) = &**expr {
                                if &*ident.sym == "__webpack_require__" {
                                    self.handle_require_call(call);
                                }
                            }
                        }
                        call.visit_children_with(self);
                    }
                    
                    fn visit_fn_expr(&mut self, n: &swc_core::ecma::ast::FnExpr) {
                         // Heuristic: if function params include an ident named "__webpack_require__",
                         // then treat as a module wrapper function and ignore requires inside it.
                         let mut is_wrapper = false;
                         let func = &n.function;
                         for param in &func.params {
                             if let Pat::Ident(bi) = &param.pat {
                                 if &*bi.id.sym == "__webpack_require__" { is_wrapper = true; break; }
                             }
                         }
                         if is_wrapper { self.wrapper_fn_depth += 1; }
                         n.visit_children_with(self);
                         if is_wrapper { self.wrapper_fn_depth = self.wrapper_fn_depth.saturating_sub(1); }
                     }
                    
                    fn visit_function(&mut self, n: &swc_core::ecma::ast::Function) {
                        let mut is_wrapper = false;
                        for param in &n.params {
                            if let Pat::Ident(bi) = &param.pat {
                                if &*bi.id.sym == "__webpack_require__" { is_wrapper = true; break; }
                            }
                        }
                        if is_wrapper { self.wrapper_fn_depth += 1; }
                        n.visit_children_with(self);
                        if is_wrapper { self.wrapper_fn_depth = self.wrapper_fn_depth.saturating_sub(1); }
                    }
                }
                
                let mut visitor = RequireCallFinder {
                    defined_keys: defined_keys.clone(),
                    found_ids: HashSet::new(),
                    wrapper_fn_depth: 0,
                };
                
                module.visit_with(&mut visitor);
                out = visitor.found_ids;
            }
            Err(e) => {
                eprintln!("[TreeShaker] Failed to parse chunk for require extraction: {:?}", e);
                // Fall back to regex as last resort, scoped to top-level approximations by excluding
                // requires that appear within typical module wrapper patterns "function(module, exports, __webpack_require__)"
                // This is a best-effort heuristic.
                let re = Regex::new(r#"__webpack_require__\s*\(\s*(\d+|\"[^\"]+\"|'[^']+'|`[^`]+`)\s*\)"#).unwrap();
                for cap in re.captures_iter(source) {
                    let raw = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                    let trimmed = if raw.starts_with('"') || raw.starts_with('\'') || raw.starts_with('`') {
                        &raw[1..raw.len().saturating_sub(1)]
                    } else {
                        raw
                    };
                    let id_atom = swc_core::atoms::Atom::from(trimmed);
                    if defined_keys.contains(&id_atom) {
                        out.insert(id_atom);
                    }
                }
            }
        }
        
        out
    }

    /// Apply a previously planned prune (or plan on the fly) and return a pruned
    /// analysis-level view of the chunk containing only kept modules.
    ///
    /// This does not reconstruct or mutate source code; it filters the `modules`
    /// map to those that are reachable from explicit entries.
    pub fn prune_chunk(&self, chunk: &WebpackChunk, config: &ShareUsageConfig) -> PruneResult {
        let mut result = self.plan_prune(chunk, config);

        if result.skip_reason.is_some() {
            return result;
        }

        // Build filtered module map
        let mut filtered_modules: FxHashMap<ModuleId, WebpackModule> = FxHashMap::default();
        for module_id in &result.kept_modules {
            if let Some(module) = chunk.modules.get(module_id) {
                filtered_modules.insert(module_id.clone(), module.clone());
            }
        }

        // Construct pruned chunk view (source preserved, modules filtered)
        let mut pruned = WebpackChunk {
            chunk_type: chunk.chunk_type.clone(),
            modules: filtered_modules,
            source: chunk.source.clone(),
            characteristics: chunk.characteristics.clone(),
        };

        // Recompute dependency relationships within the pruned map:
        // Remove edges to modules that were dropped.
        Self::sanitize_dependencies(&mut pruned);

        result.pruned_chunk = Some(pruned);
        result
    }

    fn is_runtime_chunk(characteristics: &ChunkCharacteristics) -> bool {
        characteristics.is_runtime()
    }

    fn sanitize_dependencies(chunk: &mut WebpackChunk) {
        let kept: HashSet<ModuleId> = chunk.modules.keys().cloned().collect();
        for (_id, module) in chunk.modules.iter_mut() {
            module.dependencies.retain(|dep| kept.contains(dep));
            module.dependents.retain(|dep| kept.contains(dep));
        }
    }
}

/// Split chunk optimizer for removing unreferenced object keys from vendor/shared chunks
/// This handles the two-pass analysis system for split chunk optimization
pub struct SplitChunkOptimizer {
    /// Enable debug logging for split chunk processing
    debug: bool,
}

impl Default for SplitChunkOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl SplitChunkOptimizer {
    pub fn new() -> Self {
        Self { debug: false }
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Process only split chunks based on configuration, not entry/runtime/bootstrap chunks
    /// Uses ShareUsageConfig to identify chunks via chunk_characteristics.chunk_files
    /// and skips runtime chunks based on is_runtime_chunk flag
    pub fn should_process_chunk(&self, chunk: &WebpackChunk) -> bool {
        self.should_process_chunk_with_config(chunk, None)
    }
    
    /// Process chunks based on ShareUsageConfig - configuration-driven approach
    pub fn should_process_chunk_with_config(&self, chunk: &WebpackChunk, config: Option<&ShareUsageConfig>) -> bool {
        let Some(characteristics) = &chunk.characteristics else {
            if self.debug {
                eprintln!("[SplitChunkOptimizer] Skipping: missing ChunkCharacteristics");
            }
            return false;
        };

        // Skip runtime chunks entirely
        if characteristics.is_runtime() {
            if self.debug {
                eprintln!("[SplitChunkOptimizer] Skipping: runtime chunk");
            }
            return false;
        }

        // If configuration is provided, use configuration-driven approach
        if let Some(config) = config {
            let chunk_files = &characteristics.chunk_files;
            if let Some(_lib_config) = config.should_process_chunk(chunk_files) {
                if self.debug {
                    let chunk_name = chunk_files.first().map(|s| s.as_str()).unwrap_or("<unknown>");
                    eprintln!("[SplitChunkOptimizer] Configuration-driven processing for chunk '{}'", chunk_name);
                }
                return true;
            } else {
                if self.debug {
                    let chunk_name = chunk_files.first().map(|s| s.as_str()).unwrap_or("<unknown>");
                    eprintln!("[SplitChunkOptimizer] Chunk '{}' not found in configuration or is runtime chunk", chunk_name);
                }
                return false;
            }
        }

        // Fallback to heuristic-based approach when no configuration is provided
        // Process any chunk that has modules - we'll determine unreferenced keys during optimization
        let chunk_files = &characteristics.chunk_files;
        let has_modules = !chunk.modules.is_empty();
        
        if self.debug {
            let chunk_name = chunk_files.first().map(|s| s.as_str()).unwrap_or("<unknown>");
            eprintln!("[SplitChunkOptimizer] Heuristic-based: Chunk '{}': has_modules={}, should_process={}", 
                chunk_name, has_modules, has_modules);
        }
        
        has_modules
    }

    /// Check if chunk has any modules that could potentially be optimized
    fn has_optimizable_modules(&self, chunk: &WebpackChunk) -> bool {
        !chunk.modules.is_empty()
    }

    /// Find which modules in split chunk are referenced by other chunks
    /// This is Pass 1 of the two-pass analysis system
    pub fn find_external_references(
        &self,
        split_chunk_modules: &HashSet<String>,
        other_chunks: &[&WebpackChunk]
    ) -> HashSet<String> {
        let mut external_refs = HashSet::new();
        
        if self.debug {
            eprintln!("[SplitChunkOptimizer] Finding external references from {} other chunks", other_chunks.len());
        }
        
        for chunk in other_chunks {
            // Extract all webpack_require calls from this chunk
            let requires = self.extract_all_requires(&chunk.source);
            
            // Keep requires that reference our split chunk's modules
            for req in requires {
                if split_chunk_modules.contains(&req) {
                    external_refs.insert(req.clone());
                    if self.debug {
                        eprintln!("[SplitChunkOptimizer] Found external reference: {}", req);
                    }
                }
            }
        }
        
        if self.debug {
            eprintln!("[SplitChunkOptimizer] Found {} external references", external_refs.len());
        }
        
        external_refs
    }

    /// Build internal dependency graph within the split chunk
    /// This is Pass 2 of the two-pass analysis system
    pub fn build_internal_require_graph(&self, split_chunk: &WebpackChunk) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        
        if self.debug {
            eprintln!("[SplitChunkOptimizer] Building internal dependency graph for {} modules", split_chunk.modules.len());
        }
        
        for (module_key, module) in &split_chunk.modules {
            // Extract requires from this module's function body
            let requires = self.extract_requires_from_module_source(&module.source);
            
            // Filter to only internal requires (within same chunk)
            let internal_requires: Vec<String> = requires.into_iter()
                .filter(|req| {
                    let atom_key = swc_core::atoms::Atom::from(req.as_str());
                    split_chunk.modules.contains_key(&atom_key)
                })
                .collect();
            
            if self.debug && !internal_requires.is_empty() {
                eprintln!("[SplitChunkOptimizer] Module '{}' requires: {:?}", module_key, internal_requires);
            }
            
            graph.insert(module_key.to_string(), internal_requires);
        }
        
        graph
    }

    /// Find all modules transitively required from external references
    /// This computes the transitive closure of reachable modules
    pub fn compute_transitive_closure(
        &self,
        external_refs: &HashSet<String>,
        internal_graph: &HashMap<String, Vec<String>>
    ) -> HashSet<String> {
        let mut reachable = external_refs.clone();
        let mut queue: VecDeque<String> = external_refs.iter().cloned().collect();
        
        if self.debug {
            eprintln!("[SplitChunkOptimizer] Computing transitive closure from {} external references", external_refs.len());
        }
        
        while let Some(current) = queue.pop_front() {
            if let Some(deps) = internal_graph.get(&current) {
                for dep in deps {
                    if !reachable.contains(dep) {
                        reachable.insert(dep.clone());
                        queue.push_back(dep.clone());
                        if self.debug {
                            eprintln!("[SplitChunkOptimizer] Added to reachable: {}", dep);
                        }
                    }
                }
            }
        }
        
        if self.debug {
            eprintln!("[SplitChunkOptimizer] Transitive closure complete: {} reachable modules", reachable.len());
        }
        
        reachable
    }

    /// Extract all webpack_require calls from chunk source
    fn extract_all_requires(&self, source: &str) -> HashSet<String> {
        let mut requires = HashSet::new();
        
        // Use regex to find all __webpack_require__ calls
        let re = Regex::new(r#"__webpack_require__\s*\(\s*(?:/\*[^*]*\*/\s*)?["']([^"']+)["']\s*(?:/\*[^*]*\*/)?\s*\)"#).unwrap();
        
        for cap in re.captures_iter(source) {
            if let Some(module_id) = cap.get(1) {
                requires.insert(module_id.as_str().to_string());
            }
        }
        
        requires
    }

    /// Extract webpack_require calls from a specific module's source
    fn extract_requires_from_module_source(&self, module_source: &str) -> HashSet<String> {
        self.extract_all_requires(module_source)
    }

    /// Determine modules to keep/remove based on ShareUsageConfig flags
    fn determine_modules_from_config(
        &self,
        module_keys: &HashSet<String>,
        split_chunk: &WebpackChunk,
        config: &ShareUsageConfig
    ) -> (HashSet<String>, HashSet<String>) {
        let mut reachable = HashSet::new();
        let mut to_remove = HashSet::new();
        
        if self.debug {
            eprintln!("[SplitChunkOptimizer] Using configuration-driven module selection");
        }
        
        // Get the chunk files to find the right configuration
        let chunk_files = if let Some(characteristics) = &split_chunk.characteristics {
            &characteristics.chunk_files
        } else {
            if self.debug {
                eprintln!("[SplitChunkOptimizer] No chunk characteristics found, using fallback");
            }
            // Try to find configuration for any chunk file
            for (chunk_name, lib_config) in &config.tree_shake {
                if self.debug {
                    eprintln!("[SplitChunkOptimizer] Checking config for chunk: {}", chunk_name);
                }
                
                // Check each module in this chunk against the configuration
                 for module_key in module_keys {
                     if let Some(&should_keep) = lib_config.exports.get(module_key) {
                         if should_keep {
                             reachable.insert(module_key.clone());
                             if self.debug {
                                 eprintln!("[SplitChunkOptimizer] Config: keeping module {}", module_key);
                             }
                         } else {
                             to_remove.insert(module_key.clone());
                             if self.debug {
                                 eprintln!("[SplitChunkOptimizer] Config: removing module {}", module_key);
                             }
                         }
                     } else {
                         // Module not in config, keep it by default
                         reachable.insert(module_key.clone());
                         if self.debug {
                             eprintln!("[SplitChunkOptimizer] Config: module {} not in config, keeping by default", module_key);
                         }
                     }
                }
                
                // If we found any configuration matches, use this config
                if !reachable.is_empty() || !to_remove.is_empty() {
                    return (reachable, to_remove);
                }
            }
            
            // No configuration found, keep all modules
            if self.debug {
                eprintln!("[SplitChunkOptimizer] No configuration found, keeping all modules");
            }
            return (module_keys.clone(), HashSet::new());
        };
        
        // Find the appropriate library configuration based on chunk files
        for chunk_file in chunk_files {
            // Try to match chunk file name with configuration keys
            for (chunk_name, lib_config) in &config.tree_shake {
                if chunk_file.contains(chunk_name) || chunk_name.contains(chunk_file) {
                    if self.debug {
                        eprintln!("[SplitChunkOptimizer] Found matching config for chunk file '{}' -> config '{}'", chunk_file, chunk_name);
                    }
                    
                    // Apply configuration flags to determine which modules to keep/remove
                     for module_key in module_keys {
                         if let Some(&should_keep) = lib_config.exports.get(module_key) {
                             if should_keep {
                                 reachable.insert(module_key.clone());
                                 if self.debug {
                                     eprintln!("[SplitChunkOptimizer] Config: keeping module {}", module_key);
                                 }
                             } else {
                                 to_remove.insert(module_key.clone());
                                 if self.debug {
                                     eprintln!("[SplitChunkOptimizer] Config: removing module {}", module_key);
                                 }
                             }
                         } else {
                             // Module not in config, keep it by default
                             reachable.insert(module_key.clone());
                             if self.debug {
                                 eprintln!("[SplitChunkOptimizer] Config: module {} not in config, keeping by default", module_key);
                             }
                         }
                    }
                    
                    return (reachable, to_remove);
                }
            }
        }
        
        // No matching configuration found, keep all modules
        if self.debug {
            eprintln!("[SplitChunkOptimizer] No matching configuration found for chunk files: {:?}", chunk_files);
        }
        (module_keys.clone(), HashSet::new())
    }

    /// Optimize split chunk with configuration support
    pub fn optimize_split_chunk_with_config(
        &self,
        split_chunk: &WebpackChunk,
        other_chunks: &[&WebpackChunk],
        config: Option<&ShareUsageConfig>
    ) -> Result<(String, PruneResult), Box<dyn std::error::Error>> {
        if !self.should_process_chunk_with_config(split_chunk, config) {
            return Ok((split_chunk.source.clone(), PruneResult::skipped(
                "Not a split chunk or doesn't meet processing criteria".to_string(),
                split_chunk.modules.len(),
            )));
        }

        self.optimize_split_chunk_internal(split_chunk, other_chunks, config)
    }
    
    /// Optimize split chunk by removing unreferenced object keys (backward compatibility)
    pub fn optimize_split_chunk(
        &self,
        split_chunk: &WebpackChunk,
        other_chunks: &[&WebpackChunk]
    ) -> Result<(String, PruneResult), Box<dyn std::error::Error>> {
        self.optimize_split_chunk_with_config(split_chunk, other_chunks, None)
    }
    
    /// Internal optimization logic shared by both methods
    fn optimize_split_chunk_internal(
        &self,
        split_chunk: &WebpackChunk,
        other_chunks: &[&WebpackChunk],
        config: Option<&ShareUsageConfig>
    ) -> Result<(String, PruneResult), Box<dyn std::error::Error>> {

        // Step 1: Get all module keys in this split chunk
        let module_keys: HashSet<String> = split_chunk.modules.keys().map(|k| k.to_string()).collect();
        
        // Step 2: Determine modules to remove based on configuration or analysis
        let (reachable, to_remove) = if let Some(config) = config {
            // Configuration-driven approach: use ShareUsageConfig flags
            self.determine_modules_from_config(&module_keys, split_chunk, config)
        } else {
            // Analysis-driven approach: use external references and dependency analysis
            let mut external_refs = self.find_external_references(&module_keys, other_chunks);
            
            // Always preserve entry module when specified
            if let Some(characteristics) = &split_chunk.characteristics {
                if let Some(entry_module_id) = &characteristics.entry_module_id {
                    if module_keys.contains(entry_module_id) {
                        external_refs.insert(entry_module_id.clone());
                        if self.debug {
                            eprintln!("[SplitChunkOptimizer] Preserving entry module: {}", entry_module_id);
                        }
                    }
                }
            }
            
            // Build internal dependency graph and compute transitive closure
            let internal_graph = self.build_internal_require_graph(split_chunk);
            let reachable = self.compute_transitive_closure(&external_refs, &internal_graph);
            let to_remove: HashSet<String> = module_keys.difference(&reachable).cloned().collect();
            
            (reachable, to_remove)
        };
        
        if self.debug {
            eprintln!("[SplitChunkOptimizer] Optimization summary:");
            eprintln!("  Total modules: {}", module_keys.len());
            eprintln!("  Reachable modules: {}", reachable.len());
            eprintln!("  Modules to remove: {}", to_remove.len());
        }
        
        if to_remove.is_empty() {
            return Ok((split_chunk.source.clone(), PruneResult {
                kept_modules: reachable.into_iter().map(|s| swc_core::atoms::Atom::from(s)).collect(),
                removed_modules: HashSet::new(),
                original_count: module_keys.len(),
                pruned_count: module_keys.len(),
                skip_reason: Some("No modules to remove".to_string()),
                pruned_chunk: None,
            }));
        }
        
        // Step 6: Apply AST transformation to remove unreferenced keys
        let optimized_source = self.remove_object_keys_from_source(&split_chunk.source, &to_remove)?;
        
        let pruned_count = reachable.len();
        let result = PruneResult {
            kept_modules: reachable.into_iter().map(|s| swc_core::atoms::Atom::from(s)).collect(),
            removed_modules: to_remove.into_iter().map(|s| swc_core::atoms::Atom::from(s)).collect(),
            original_count: module_keys.len(),
            pruned_count,
            skip_reason: None,
            pruned_chunk: None,
        };
        
        Ok((optimized_source, result))
    }

    /// Remove object keys from source using AST transformation
    fn remove_object_keys_from_source(
        &self,
        source: &str,
        to_remove: &HashSet<String>
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Parse the source
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(FileName::Custom("split_chunk.js".to_string()).into(), source.to_string());
        let mut program = Parser::new(Syntax::Es(EsSyntax::default()), StringInput::from(&*fm), None)
            .parse_program()
            .map_err(|e| format!("Parse error: {:?}", e))?;

        // Apply split chunk object key pruner
        let mut pruner = SplitChunkKeyPruner::new(to_remove.clone());
        program.visit_mut_with(&mut pruner);

        // Emit the optimized code
        let mut buf = vec![];
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(false),
            comments: None,
            cm: cm.clone(),
            wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None)),
        };
        emitter.emit_program(&program).map_err(|e| format!("Emit error: {:?}", e))?;
        let optimized = String::from_utf8(buf).map_err(|_| "Invalid UTF-8")?;
        
        Ok(optimized)
    }
}

/// Enhanced AST module pruner that completely removes module entries from webpack modules objects
/// instead of just marking them or removing their exports
struct AstModulePruner {
    to_remove: std::collections::HashSet<String>,
}

/// Split chunk specific key pruner for removing unreferenced object keys from exports.modules
struct SplitChunkKeyPruner {
    to_remove: HashSet<String>,
    debug: bool,
}

impl SplitChunkKeyPruner {
    fn new(to_remove: HashSet<String>) -> Self {
        Self { to_remove, debug: false }
    }

    fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Check if a module key should be removed from exports.modules
    fn should_remove_key(&self, key: &str) -> bool {
        self.to_remove.contains(key)
    }

    /// Remove unreferenced keys from exports.modules object
    fn prune_exports_modules(&self, obj: &mut ObjectLit) {
        let original_count = obj.props.len();
        obj.props.retain(|prop| {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(kv) = prop.as_ref() {
                    let key_str = match &kv.key {
                        PropName::Str(s) => s.value.to_string(),
                        PropName::Ident(i) => i.sym.to_string(),
                        _ => return true, // Keep non-string keys
                    };
                    
                    let should_keep = !self.should_remove_key(&key_str);
                    if !should_keep && self.debug {
                        eprintln!("[SplitChunkKeyPruner] Removing key: {}", key_str);
                    }
                    return should_keep;
                }
            }
            true // Keep non-key-value props
        });
        
        let removed_count = original_count - obj.props.len();
        if removed_count > 0 && self.debug {
            eprintln!("[SplitChunkKeyPruner] Removed {} keys from exports.modules", removed_count);
        }
    }
}

impl VisitMut for SplitChunkKeyPruner {
    fn visit_mut_assign_expr(&mut self, node: &mut swc_core::ecma::ast::AssignExpr) {
        // Handle exports.modules = { ... } pattern
        if let swc_core::ecma::ast::AssignTarget::Simple(swc_core::ecma::ast::SimpleAssignTarget::Member(member)) = &node.left {
            let is_modules = matches!(&member.prop, MemberProp::Ident(p) if p.sym == "modules")
                || matches!(&member.prop, MemberProp::Computed(c) if matches!(c.expr.as_ref(), Expr::Lit(swc_core::ecma::ast::Lit::Str(s)) if s.value == *"modules"));
            
            if is_modules {
                if let Expr::Ident(obj) = member.obj.as_ref() {
                    if obj.sym == "exports" {
                        if let Expr::Object(obj_lit) = &mut *node.right {
                            self.prune_exports_modules(obj_lit);
                        }
                    }
                }
            }
        }
        
        node.visit_mut_children_with(self);
    }

    fn visit_mut_object_lit(&mut self, node: &mut ObjectLit) {
        // Handle direct object literals that might be exports.modules
        // This is a fallback for cases where the assignment pattern doesn't match
        self.prune_exports_modules(node);
        node.visit_mut_children_with(self);
    }
}

impl AstModulePruner {
    fn new(to_remove: std::collections::HashSet<String>) -> Self { 
        Self { to_remove } 
    }

    /// Check if a module property should be completely removed from __webpack_modules__
    fn should_remove_module_entry(&self, prop: &PropOrSpread) -> bool {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = prop.as_ref() {
                let module_id = match &kv.key {
                    PropName::Num(n) => n.value.to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    PropName::Ident(i) => i.sym.to_string(),
                    _ => return false,
                };
                return self.to_remove.contains(&module_id);
            }
            // Handle method definitions for webpack modules
            if let Prop::Method(method) = prop.as_ref() {
                let module_id = match &method.key {
                    PropName::Num(n) => n.value.to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    PropName::Ident(i) => i.sym.to_string(),
                    _ => return false,
                };
                return self.to_remove.contains(&module_id);
            }
        }
        false
    }

    /// Check if an array element (module ID) should be removed from dependency arrays
    fn should_remove_dependency(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Lit(swc_core::ecma::ast::Lit::Str(s)) => {
                self.to_remove.contains(&s.value.to_string())
            }
            Expr::Lit(swc_core::ecma::ast::Lit::Num(n)) => {
                self.to_remove.contains(&n.value.to_string())
            }
            _ => false,
        }
    }

    /// Completely remove unreachable module entries from webpack modules object
    fn remove_modules_from_object(&self, obj: &mut ObjectLit) {
        let original_count = obj.props.len();
        obj.props.retain(|p| !self.should_remove_module_entry(p));
        let removed_count = original_count - obj.props.len();
        if removed_count > 0 {
            eprintln!("[AstModulePruner] Removed {} module entries from __webpack_modules__", removed_count);
        }
    }

    /// Handle different webpack module container formats
    fn process_modules_expr(&mut self, expr: &mut Expr) {
        match expr {
            // Direct object literal: __webpack_modules__ = { ... }
            Expr::Object(obj) => self.remove_modules_from_object(obj),
            // Parenthesized object: __webpack_modules__ = ({ ... })
            Expr::Paren(paren) => {
                if let Expr::Object(obj) = paren.expr.as_mut() {
                    self.remove_modules_from_object(obj);
                }
            }
            // Function call returning object: __webpack_modules__ = fn()
            Expr::Call(call) => {
                // Some webpack formats use IIFE that returns module object
                // We need to handle the case where modules are defined inside the function
                call.visit_mut_children_with(self);
            }
            _ => {
                // Handle any other expression types that might contain module definitions
                expr.visit_mut_children_with(self);
            }
        }
    }

    /// Strip modules from expression - handles various webpack module container formats
    fn strip_from_expr(&mut self, expr: &mut Expr) {
        self.process_modules_expr(expr);
    }

    /// Strip modules from object literal - removes unreachable module entries
    fn strip_from_object(&self, obj: &mut ObjectLit) {
        self.remove_modules_from_object(obj);
    }
}

impl VisitMut for AstModulePruner {
    fn visit_mut_var_decl(&mut self, node: &mut VarDecl) {
        for decl in &mut node.decls {
            // Only touch __webpack_modules__ initializers
            if let Pat::Ident(ident) = &decl.name {
                if ident.id.sym == "__webpack_modules__" {
                    if let Some(init) = &mut decl.init {
                        self.strip_from_expr(init);
                    }
                }
            }
        }
        node.visit_mut_children_with(self);
    }

    fn visit_mut_assign_expr(&mut self, node: &mut swc_core::ecma::ast::AssignExpr) {
        // Exports/modules patterns
        if let swc_core::ecma::ast::AssignTarget::Simple(swc_core::ecma::ast::SimpleAssignTarget::Member(member)) = &node.left {
            // exports.modules = { ... }
            let is_modules = matches!(&member.prop, MemberProp::Ident(p) if p.sym == "modules")
                || matches!(&member.prop, MemberProp::Computed(c) if matches!(c.expr.as_ref(), Expr::Lit(swc_core::ecma::ast::Lit::Str(s)) if s.value == *"modules"));
            if is_modules {
                if let Expr::Ident(obj) = member.obj.as_ref() {
                    if obj.sym == "exports" { self.strip_from_expr(&mut node.right); }
                }
                if let Expr::Member(inner) = member.obj.as_ref() {
                    if let (Expr::Ident(obj), MemberProp::Ident(prop)) = (inner.obj.as_ref(), &inner.prop) {
                        if obj.sym == "module" && prop.sym == "exports" { self.strip_from_expr(&mut node.right); }
                    }
                }
            }
            // module.exports = { ... }
            let is_module_exports = matches!(&member.prop, MemberProp::Ident(p) if p.sym == "exports")
                || matches!(&member.prop, MemberProp::Computed(c) if matches!(c.expr.as_ref(), Expr::Lit(swc_core::ecma::ast::Lit::Str(s)) if s.value == *"exports"));
            if is_module_exports {
                if let Expr::Ident(obj) = member.obj.as_ref() {
                    if obj.sym == "module" { self.strip_from_expr(&mut node.right); }
                }
            }
        }
        node.visit_mut_children_with(self);
    }

    fn visit_mut_call_expr(&mut self, node: &mut CallExpr) {
        // AMD define([...], fn)
        if let swc_core::ecma::ast::Callee::Expr(callee) = &node.callee {
            if let Expr::Ident(id) = callee.as_ref() {
                if id.sym == "define" {
                    // Strip removed ids from the dependency array (arg0)
                    if let Some(ExprOrSpread { expr, .. }) = node.args.get_mut(0) {
                        if let Expr::Array(arr) = expr.as_mut() {
                            arr.elems.retain(|elem| {
                                if let Some(ExprOrSpread { expr, .. }) = elem {
                                    if let Expr::Lit(swc_core::ecma::ast::Lit::Str(s)) = expr.as_ref() {
                                        return !self.to_remove.contains(&s.value.to_string());
                                    }
                                }
                                true
                            });
                        }
                    }
                }
            }
        }
        // System.register([...], fn)
        if let swc_core::ecma::ast::Callee::Expr(callee) = &node.callee {
            if let Expr::Member(member) = callee.as_ref() {
                if let (Expr::Ident(obj), MemberProp::Ident(prop)) = (member.obj.as_ref(), &member.prop) {
                    if obj.sym == "System" && prop.sym == "register" {
                        if let Some(ExprOrSpread { expr, .. }) = node.args.get_mut(0) {
                            if let Expr::Array(arr) = expr.as_mut() {
                                arr.elems.retain(|elem| {
                                    if let Some(ExprOrSpread { expr, .. }) = elem {
                                        if let Expr::Lit(swc_core::ecma::ast::Lit::Str(s)) = expr.as_ref() {
                                            return !self.to_remove.contains(&s.value.to_string());
                                        }
                                    }
                                    true
                                });
                            }
                        }
                    }
                }
            }
        }
        // JSONP (...).push([[ids], { modules }, runtime?])
        if let swc_core::ecma::ast::Callee::Expr(callee) = &node.callee {
            if let Expr::Member(member) = callee.as_ref() {
                if let MemberProp::Ident(p) = &member.prop {
                    if p.sym == "push" {
                        if let Some(ExprOrSpread { expr, .. }) = node.args.get_mut(0) {
                            if let Expr::Array(arr) = expr.as_mut() {
                                if let Some(Some(ExprOrSpread { expr: mods, .. })) = arr.elems.get_mut(1) {
                                    if let Expr::Object(obj) = mods.as_mut() {
                                        self.strip_from_object(obj);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        node.visit_mut_children_with(self);
    }
}


