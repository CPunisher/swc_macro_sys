use std::time::{Duration, Instant};
use swc_common::comments::SingleThreadedComments;
use swc_common::pass::Repeated;
use swc_common::sync::Lrc;
use swc_common::{FileName, Mark, SourceMap};
use swc_core::ecma::codegen;
use swc_core::ecma::visit::VisitMutWith;
use swc_ecma_ast::Program;
use swc_ecma_codegen::text_writer::WriteJs;
use swc_ecma_codegen::{text_writer, Emitter};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_transforms_base::resolver;
use swc_macro_condition_transform::condition_transform;
use swc_macro_parser::MacroParser;
use thiserror::Error;
// Re-enabling webpack_analyzer_v2 step by step
use webpack_analyzer_v2::{ChunkCharacteristics, TreeShaker as AnalyzerTreeShaker, tree_shaker::SplitChunkOptimizer, WebpackAnalyzer, ShareUsageConfig};

// Cross-platform logging helper
#[cfg(target_arch = "wasm32")]
fn log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}
#[cfg(not(target_arch = "wasm32"))]
fn log(_msg: &str) {
    // No-op on native to avoid noisy stdout and wasm_bindgen linkage
}

/// Error types for optimization operations
#[derive(Error, Debug)]
pub enum OptimizationError {
    #[error("Failed to parse JavaScript: {0}")]
    ParseError(String),

    #[error("Failed to emit JavaScript: {0}")]
    EmitError(String),

    #[error("Invalid UTF-8 encoding in generated code")]
    Utf8Error,

    #[error("Webpack analysis failed: {0}")]
    WebpackAnalysisError(String),

    #[error("Numeric conversion failed: {0}")]
    NumericConversionError(String),
}

type OptimizationResult<T> = Result<T, OptimizationError>;

/// Optimize with share-usage.json configuration for configuration-driven split chunk optimization
pub fn optimize_with_share_usage_config(
    source: String,
    config: serde_json::Value,
    share_usage_config_path: &str,
) -> OptimizationResult<String> {
    log("optimize_with_share_usage_config: Starting configuration-driven optimization");
    
    // Load ShareUsageConfig from file
    let share_config = ShareUsageConfig::load_from_file(share_usage_config_path)
        .map_err(|e| OptimizationError::WebpackAnalysisError(format!("Failed to load share-usage config: {}", e)))?;
    
    optimize_with_config(source, config, Some(share_config))
}

/// Internal optimization method that accepts optional ShareUsageConfig
fn optimize_with_config(
    source: String,
    config: serde_json::Value,
    share_config: Option<ShareUsageConfig>,
) -> OptimizationResult<String> {
    log("optimize_with_config: Starting optimization with configuration support");
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = {
        log("optimize_with_config: Creating source file");
        let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source);
        let comments = SingleThreadedComments::default();
        log("optimize_with_config: About to parse with Parser::new");
        let program = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments),
        )
        .parse_program()
        .map_err(|e| OptimizationError::ParseError(format!("Parser error: {:?}", e)))?;
        (program, comments)
    };

    let macros = {
        let parser = MacroParser::new("common");
        parser.parse(&comments)
    };

    let program = {
        let mut transformer = condition_transform(config.clone(), macros);
        program.visit_mut_with(&mut transformer);

        // Apply resolver and optimization
        swc_common::GLOBALS.set(&Default::default(), || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            program.mutate(resolver(unresolved_mark, top_level_mark, false));
            
            perform_dce(&mut program, comments.clone(), unresolved_mark);
            
            // Use configuration-driven tree shaking if available
            if has_macro_processing_config(&config) {
                run_webpack_tree_shake_with_config(&mut program, cm.clone(), &comments, &config, share_config.as_ref());
            }
            
            program.mutate(fixer(Some(&comments)));
            
            program
        })
    };

    let ret = {
        let mut buf = vec![];
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
            as Box<dyn WriteJs>;
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(false),
            comments: Some(&comments),
            cm: cm.clone(),
            wr,
        };
        emitter
            .emit_program(&program)
            .map_err(|e| OptimizationError::EmitError(format!("Emit error: {:?}", e)))?;
        drop(emitter);

        String::from_utf8(buf).map_err(|_| OptimizationError::Utf8Error)?
    };

    Ok(ret)
}

pub fn optimize(source: String, config: serde_json::Value) -> OptimizationResult<String> {
    optimize_with_config(source, config, None)
}

fn perform_dce(m: &mut Program, comments: SingleThreadedComments, unresolved_mark: Mark) {
    let mut visitor = crate::dce::dce(
        comments,
        crate::dce::Config {
            module_mark: None,
            top_level: true,
            top_retain: Vec::new(),
            preserve_imports_with_side_effects: true,
        },
        unresolved_mark,
    );

    loop {
        m.visit_mut_with(&mut visitor);

        if !visitor.changed() {
            break;
        }

        visitor.reset();
    }
}

/// Check if the config contains macro processing directives that might create orphaned modules
fn has_macro_processing_config(config: &serde_json::Value) -> bool {
    // Check if there are macro processing directives like features, treeShake, etc.
    // Entry modules are no longer supported; rely strictly on chunk_characteristics
    config.get("treeShake").is_some()
}

// Simple tree shaking removed - using webpack_analyzer_v2 TreeShaker instead

// Synthetic entry point calls removed - TreeShaker handles this properly

// The complex TreeShaker using webpack_analyzer_v2 - re-enabling step by step

/// Extract chunk characteristics strictly from config
/// Falls back to a safe default (CommonJSSync) without inspecting source
fn get_chunk_characteristics(config: &serde_json::Value) -> Option<ChunkCharacteristics> {
    // First check if chunk_characteristics is at the root level (old format)
    if let Some(chars_value) = config.get("chunk_characteristics")
        && let Ok(characteristics) =
            serde_json::from_value::<ChunkCharacteristics>(chars_value.clone())
        {
            return Some(characteristics);
        }

    // Check if chunk_characteristics is nested within treeShake (new format)
    if let Some(tree_shake_config) = config.get("treeShake")
        && let Some(tree_shake_obj) = tree_shake_config.as_object() {
            // Look for chunk_characteristics in any of the modules
            for (_, module_config) in tree_shake_obj {
                if let Some(module_obj) = module_config.as_object()
                    && let Some(chars_value) = module_obj.get("chunk_characteristics")
                        && let Ok(characteristics) =
                            serde_json::from_value::<ChunkCharacteristics>(chars_value.clone())
                        {
                            return Some(characteristics);
                        }
            }
        }

    // No fallback: hard requirement enforced
    None
}

/// Metrics collected during tree shaking optimization
#[derive(Debug, Clone)]
struct TreeShakeMetrics {
    modules_before: usize,
    modules_after: usize,
    modules_removed: usize,
    iterations: u32,
    time_taken: Duration,
    chunks_processed: usize,
}

impl TreeShakeMetrics {
    fn new() -> Self {
        Self {
            modules_before: 0,
            modules_after: 0,
            modules_removed: 0,
            iterations: 0,
            time_taken: Duration::new(0, 0),
            chunks_processed: 0,
        }
    }

    fn log_summary(&self) {
        if self.modules_removed > 0 {
            println!(
                "Tree shaking summary: Removed {} modules across {} iterations in {:?} ({}% reduction)",
                self.modules_removed,
                self.iterations,
                self.time_taken,
                (self.modules_removed * 100) / self.modules_before.max(1)
            );
        } else {
            println!(
                "Tree shaking summary: No modules removed in {} iterations ({:?})",
                self.iterations, self.time_taken
            );
        }
    }
}

/// Perform webpack tree shaking with configuration support
fn run_webpack_tree_shake_with_config(
    program: &mut Program,
    cm: Lrc<SourceMap>,
    comments: &SingleThreadedComments,
    config: &serde_json::Value,
    share_config: Option<&ShareUsageConfig>,
) {
    log("TreeShaker::optimize: Starting configuration-driven tree shaking optimization");
    let timer = WasmTimer::start();
    let mut metrics = TreeShakeMetrics::new();

    let mut total_removed = 0;
    let max_iterations = 5; // Prevent infinite loops

    for iteration in 1..=max_iterations {
        // Step 1: Emit current AST to string for analysis
        log(&format!("TreeShaker: Starting iteration {}", iteration));
        let current_code = {
            let mut buf = vec![];
            let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
                as Box<dyn WriteJs>;
            let mut emitter = Emitter {
                cfg: codegen::Config::default().with_minify(false),
                comments: Some(comments),
                cm: cm.clone(),
                wr,
            };
            if emitter.emit_program(program).is_err() {
                break; // Stop if emit fails
            }
            drop(emitter);

            match String::from_utf8(buf) {
                Ok(code) => code,
                Err(_) => break, // Stop if invalid UTF-8
            }
        };

        // Step 2: Analyze the chunk using webpack_analyzer_v2
        log("TreeShaker: Getting chunk characteristics");
        let Some(characteristics) = get_chunk_characteristics(config) else {
            log("Tree shaking: No chunk_characteristics provided in config - skipping tree shaking");
            return;
        };
        // Skip entry or runtime chunks entirely based on characteristics
        if characteristics.is_entrypoint || characteristics.is_runtime_chunk {
            log("Tree shaking: Skipping entry or runtime chunk");
            return;
        };

        // Step 3: Try configuration-driven split chunk optimization first
        let optimized_source = if let Some(split_optimized) = try_split_chunk_optimization_with_config(&current_code, &characteristics, share_config) {
            log("TreeShaker: Applied configuration-driven split chunk optimization");
            split_optimized
        } else {
            // Fallback to regular tree shaking
            log("TreeShaker: Configuration-driven split chunk optimization failed, falling back to regular tree shaking");
            if share_config.is_some() {
                log("TreeShaker: WARNING - ShareUsageConfig provided but split chunk optimization failed!");
            }
            let analyzer_shaker = AnalyzerTreeShaker::new();
            match analyzer_shaker.prune_source(&current_code, &characteristics.clone()) {
                Ok((optimized_source, plan)) => {
                    if iteration == 1 {
                        metrics.modules_before = plan.original_count;
                        metrics.chunks_processed = 1;
                    }
                    if let Some(reason) = &plan.skip_reason {
                        log(&format!("Tree shaking skipped: {}", reason));
                        return;
                    }

                    if plan.removed_modules.is_empty() || optimized_source == current_code {
                        if iteration == 1 {
                            log(&format!(
                                "Tree shaking: No unreachable modules found. Kept {} modules, removed {}",
                                plan.kept_modules.len(),
                                plan.removed_modules.len()
                            ));
                        } else {
                            log(&format!(
                                "Tree shaking: Converged after {} iterations, removed {} total modules",
                                iteration - 1,
                                total_removed
                            ));
                        }
                        metrics.iterations = iteration;
                        break;
                    }

                    total_removed += plan.removed_modules.len();
                    optimized_source
                }
                Err(err) => {
                    log(&format!("Tree shaking: Analyzer prune failed: {} - skipping", err));
                    return;
                }
            }
        };

        // Step 4: Check if optimization made changes
        if optimized_source == current_code {
            log(&format!(
                "Tree shaking: Converged after {} iterations, removed {} total modules",
                iteration,
                total_removed
            ));
            metrics.iterations = iteration;
            break;
        }

        // Step 5: Re-parse optimized source into AST and replace program
        let fm2 = cm
            .new_source_file(FileName::Custom("optimized.js".to_string()).into(), optimized_source);
        let parsed = Parser::new(Syntax::Es(EsSyntax::default()), StringInput::from(&*fm2), Some(comments))
            .parse_program()
            .map_err(|e| OptimizationError::ParseError(format!("Parser error after prune: {:?}", e)));
        if let Ok(new_prog) = parsed {
            *program = new_prog;
            // Continue to next iteration to see if more become unreachable
        } else {
            log("Tree shaking: Failed to reparse optimized source - stopping");
            break;
        }

        // Continue to next iteration to see if more modules become unreachable
    }

    // Update final metrics
    metrics.modules_removed = total_removed;
    metrics.modules_after = metrics.modules_before.saturating_sub(total_removed);
    metrics.time_taken = timer.elapsed();
    metrics.iterations = if metrics.iterations == 0 { max_iterations } else { metrics.iterations };

    // Log comprehensive summary
    metrics.log_summary();

    if total_removed > 0 {
        println!(
            "Tree shaking: Total removed {} modules across all iterations",
            total_removed
        );
    }
}

/// Perform webpack tree shaking with enhanced split chunk optimization (backward compatibility)
fn run_webpack_tree_shake(
    program: &mut Program,
    cm: Lrc<SourceMap>,
    comments: &SingleThreadedComments,
    config: &serde_json::Value,
) {
    run_webpack_tree_shake_with_config(program, cm, comments, config, None);
}

/// Cross-platform timer: uses std::time::Instant on native, js_sys::Date on wasm32
#[cfg(target_arch = "wasm32")]
struct WasmTimer {
    start_ms: f64,
}

#[cfg(target_arch = "wasm32")]
impl WasmTimer {
    fn start() -> Self {
        Self { start_ms: js_sys::Date::now() }
    }

    fn elapsed(&self) -> Duration {
        let now = js_sys::Date::now();
        let ms = if now >= self.start_ms { now - self.start_ms } else { 0.0 };
        // Convert milliseconds to seconds as f64, then to Duration
        Duration::from_secs_f64(ms / 1000.0)
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct WasmTimer {
    start: Instant,
}

#[cfg(not(target_arch = "wasm32"))]
impl WasmTimer {
    fn start() -> Self {
        Self { start: Instant::now() }
    }

    fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

/// Try configuration-driven split chunk optimization for vendor/shared chunks
/// Returns Some(optimized_source) if split chunk optimization was applied, None otherwise
fn try_split_chunk_optimization_with_config(
    source: &str, 
    characteristics: &ChunkCharacteristics,
    share_config: Option<&ShareUsageConfig>
) -> Option<String> {
    // Create a WebpackChunk for analysis
    let analyzer = WebpackAnalyzer::new();
    let chunk = match analyzer.analyze_chunk(source, characteristics.clone()) {
        Ok(chunk) => chunk,
        Err(err) => {
            log(&format!("Split chunk analysis failed: {}", err));
            return None;
        }
    };

    // Initialize split chunk optimizer
    let split_optimizer = SplitChunkOptimizer::new().with_debug(true);
    
    // Use configuration-driven chunk processing if available
    let should_process = if let Some(config) = share_config {
        split_optimizer.should_process_chunk_with_config(&chunk, Some(config))
    } else {
        split_optimizer.should_process_chunk(&chunk)
    };
    
    if !should_process {
        log("Not a split chunk - skipping split chunk optimization");
        return None;
    }

    // Apply configuration-driven optimization if available
     if let Some(config) = share_config {
         let other_chunks: Vec<&webpack_analyzer_v2::WebpackChunk> = vec![];
         match split_optimizer.optimize_split_chunk_with_config(&chunk, &other_chunks, Some(config)) {
             Ok((optimized_source, result)) => {
                 log(&format!(
                     "Configuration-driven split chunk optimization: {} -> {} modules",
                     chunk.modules.len(),
                     result.pruned_count
                 ));
                 return Some(optimized_source);
             }
             Err(err) => {
                 log(&format!("Configuration-driven split chunk optimization failed: {}", err));
                 // Fall through to regular optimization
             }
         }
     }
 
     // Fallback to regular split chunk optimization
     let other_chunks: Vec<&webpack_analyzer_v2::WebpackChunk> = vec![];
     match split_optimizer.optimize_split_chunk(&chunk, &other_chunks) {
         Ok((optimized_source, result)) => {
             log(&format!(
                 "Split chunk optimization: {} -> {} modules",
                 chunk.modules.len(),
                 result.pruned_count
             ));
             Some(optimized_source)
         }
         Err(err) => {
             log(&format!("Split chunk optimization failed: {}", err));
             None
         }
     }
}

/// Try split chunk optimization for vendor/shared chunks (backward compatibility)
/// Returns Some(optimized_source) if split chunk optimization was applied, None otherwise
fn try_split_chunk_optimization(source: &str, characteristics: &ChunkCharacteristics) -> Option<String> {
    try_split_chunk_optimization_with_config(source, characteristics, None)
}

// Legacy entry point extraction removed - explicit entry points required