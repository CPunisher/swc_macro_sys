use swc_common::comments::SingleThreadedComments;
use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};
use swc_core::ecma::visit::VisitMutWith;
use swc_ecma_ast::Program;
use swc_ecma_codegen::{Emitter, Config as CodegenConfig, text_writer};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_macro_condition_transform::condition_transform;

use swc_macro_condition_transform::optimization_pipeline::{run_optimization_pipeline, AdvancedOptimizationPipeline};
use swc_macro_condition_transform::feature_analyzer::should_skip_all_transformations;
use swc_macro_parser::MacroParser;
use wasm_bindgen::prelude::*;
use web_sys::console;

// Console logging macro for WASM
macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&format!($($t)*).into()))
}

/// Main optimization function that processes JavaScript code with conditional macros
/// and webpack-specific tree shaking using the new modular architecture.
/// 
/// # Architecture:
/// - Uses `optimization_pipeline` module for orchestration
/// - Uses `feature_analyzer` module for configuration analysis
/// - Uses `mutation_tracker` module for tracking changes
/// - Uses `webpack_module_graph` and `webpack_tree_shaker` for webpack optimization
/// - WASM crate focuses on parsing, coordination, and code generation
#[wasm_bindgen]
pub fn optimize(source: String, config: &str) -> String {
    console_log!("🚀 === MODULAR OPTIMIZATION PIPELINE START ===");
    console_log!("📄 Input source length: {} chars", source.len());
    console_log!("⚙️  Config: {}", config);

    // Parse the config for fast path check
    let config_value: serde_json::Value = match serde_json::from_str(config) {
        Ok(config) => config,
        Err(e) => {
            console_log!("❌ Failed to parse config: {}", e);
            return source; // Return original on error
        }
    };

    // Fast path check: if all features are enabled, return original code immediately
    if should_skip_all_transformations(&config_value) {
        console_log!("🏃‍♂️ FAST PATH: All features enabled - returning original code");
        return source;
    }

    // Parse the source code into AST
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = match parse_source_to_ast(&source, &cm) {
        Ok((program, comments)) => (program, comments),
        Err(e) => {
            console_log!("❌ Failed to parse source: {}", e);
            return source; // Return original on parse error
        }
    };

    // Step 1: Apply conditional macro transformations if needed
    if has_conditional_macros(&comments) {
        console_log!("🔍 Found conditional macros - applying transformations");
        if let Err(e) = apply_conditional_transformations(&mut program, &config_value, &comments) {
            console_log!("⚠️  Warning during conditional transformations: {}", e);
            // Continue with optimization even if this fails
        }
    }

    // Step 2: Run the main optimization pipeline
    match run_optimization_pipeline(&source, config, &mut program) {
        Ok(result) => {
            console_log!("✅ Optimization pipeline completed!");
            console_log!("📊 Statistics:");
            console_log!("  📦 Original size: {} chars", result.statistics.original_size);
            console_log!("  ⚡ Optimized size: {} chars", result.statistics.optimized_size);
            console_log!("  📉 Size reduction: {} chars ({:.1}%)", 
                        result.statistics.size_reduction_bytes, result.statistics.size_reduction_percent);
            console_log!("  🔧 Mutations applied: {}", result.statistics.mutations_applied);
            console_log!("  🗑️  Modules eliminated: {}", result.statistics.modules_eliminated);
            console_log!("  ⏱️  Execution time: {:.2}ms", result.execution_time_ms);
            
            if !result.recommendations.is_empty() {
                console_log!("💡 Recommendations:");
                for recommendation in &result.recommendations {
                    console_log!("  • {}", recommendation);
                }
            }
            
            result.optimized_code
        },
        Err(e) => {
            console_log!("❌ Optimization pipeline failed: {}", e);
            console_log!("🔄 Falling back to basic code generation");
            
            // Fallback: just generate code from current AST state
            match generate_code_from_ast(&program, &cm, &comments) {
                Ok(code) => code,
                Err(gen_e) => {
                    console_log!("❌ Code generation also failed: {}", gen_e);
                    source // Return original as last resort
                }
            }
        }
    }
}

/// Advanced optimization function with configurable options
#[wasm_bindgen]
pub fn optimize_advanced(source: String, config: &str, options: &str) -> String {
    console_log!("🔧 === ADVANCED OPTIMIZATION WITH CUSTOM OPTIONS ===");
    
    // Parse optimization options
    let optimization_config: serde_json::Value = match serde_json::from_str(options) {
        Ok(config) => config,
        Err(_) => {
            console_log!("⚠️  Invalid options JSON, using defaults");
            serde_json::Value::Object(serde_json::Map::new())
        }
    };
    
    // Create advanced pipeline with custom settings
    let pipeline = AdvancedOptimizationPipeline {
        enable_tree_shaking: optimization_config.get("tree_shaking").and_then(|v| v.as_bool()).unwrap_or(true),
        enable_dead_code_elimination: optimization_config.get("dead_code_elimination").and_then(|v| v.as_bool()).unwrap_or(true),
        enable_module_graph_analysis: optimization_config.get("module_graph_analysis").and_then(|v| v.as_bool()).unwrap_or(true),
        enable_variable_tracking: optimization_config.get("variable_tracking").and_then(|v| v.as_bool()).unwrap_or(true),
    };
    
    // Parse the source code
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, _comments) = match parse_source_to_ast(&source, &cm) {
        Ok((program, comments)) => (program, comments),
        Err(e) => {
            console_log!("❌ Failed to parse source: {}", e);
            return source;
        }
    };
    
    // Run advanced optimization
    match pipeline.optimize_with_config(&source, config, &mut program) {
        Ok(result) => {
            console_log!("✅ Advanced optimization completed!");
            console_log!("📊 Advanced optimization results: {:.1}% reduction", result.statistics.size_reduction_percent);
            result.optimized_code
        },
        Err(e) => {
            console_log!("❌ Advanced optimization failed: {}", e);
            source
        }
    }
}

/// Parse source code into AST
fn parse_source_to_ast(
    source: &str, 
    cm: &Lrc<SourceMap>
) -> Result<(Program, SingleThreadedComments), String> {
    let fm = cm.new_source_file(FileName::Custom("input.js".to_string()).into(), source.into());
    let comments = SingleThreadedComments::default();
    
    let program = Parser::new(
        Syntax::Es(EsSyntax::default()),
        StringInput::from(&*fm),
        Some(&comments),
    )
    .parse_program()
    .map_err(|e| format!("Parse error: {:?}", e))?;
    
    Ok((program, comments))
}

/// Check if the source contains conditional macros
fn has_conditional_macros(comments: &SingleThreadedComments) -> bool {
    let parser = MacroParser::new("common");
    let macros = parser.parse(comments);
    !macros.is_empty()
}

/// Apply conditional macro transformations
fn apply_conditional_transformations(
    program: &mut Program,
    config: &serde_json::Value,
    comments: &SingleThreadedComments,
) -> Result<(), String> {
    let parser = MacroParser::new("common");
    let macros = parser.parse(comments);
    
    if macros.is_empty() {
        return Ok(());
    }
    
    console_log!("🔍 Found {} conditional macros", macros.len());
    
    // Apply conditional transformations
    let mut transformer = condition_transform(config.clone(), macros);
    program.visit_mut_with(&mut transformer);
    
    console_log!("✅ Applied conditional macro transformations");
    Ok(())
}

/// Generate code from AST
fn generate_code_from_ast(
    program: &Program,
    cm: &Lrc<SourceMap>,
    comments: &SingleThreadedComments,
) -> Result<String, String> {
    let mut buf = vec![];
    {
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
            as Box<dyn text_writer::WriteJs>;
        
        let mut emitter = Emitter {
            cfg: CodegenConfig::default().with_minify(false),
            comments: Some(comments),
            cm: cm.clone(),
            wr,
        };
        
        emitter.emit_program(program)
            .map_err(|e| format!("Code generation error: {:?}", e))?;
    } // emitter is dropped here, releasing the borrow on buf
    
    String::from_utf8(buf)
        .map_err(|e| format!("UTF-8 conversion error: {}", e))
}

/// Helper function to get optimization statistics (for testing/debugging)
#[wasm_bindgen]
pub fn get_optimization_info(source: String, config: &str) -> String {
    console_log!("📊 Getting optimization info without applying changes");
    
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, _comments) = match parse_source_to_ast(&source, &cm) {
        Ok((program, comments)) => (program, comments),
        Err(e) => {
            return format!("{{\"error\": \"Failed to parse: {}\"}}", e);
        }
    };
    
    match run_optimization_pipeline(&source, config, &mut program) {
        Ok(result) => {
            serde_json::json!({
                "original_size": result.statistics.original_size,
                "optimized_size": result.statistics.optimized_size,
                "size_reduction_bytes": result.statistics.size_reduction_bytes,
                "size_reduction_percent": result.statistics.size_reduction_percent,
                "mutations_applied": result.statistics.mutations_applied,
                "modules_eliminated": result.statistics.modules_eliminated,
                "imports_eliminated": result.statistics.imports_eliminated,
                "fast_path_used": result.statistics.fast_path_used,
                "execution_time_ms": result.execution_time_ms,
                "recommendations": result.recommendations
            }).to_string()
        },
        Err(e) => {
            format!("{{\"error\": \"{}\"}}", e)
        }
    }
}

/// Export version and build info
#[wasm_bindgen]
pub fn get_version_info() -> String {
    serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "architecture": "modular",
        "features": [
            "webpack_tree_shaking",
            "module_graph_analysis", 
            "mutation_tracking",
            "feature_analysis",
            "optimization_pipeline",
            "conditional_macros"
        ]
    }).to_string()
}


