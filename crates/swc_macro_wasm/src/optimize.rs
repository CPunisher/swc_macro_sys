use swc_common::comments::SingleThreadedComments;
use swc_common::pass::Repeated;
use swc_common::sync::Lrc;
use swc_common::{FileName, Mark, SourceMap};
use swc_core::ecma::codegen;
use swc_core::ecma::visit::VisitMutWith;
use swc_ecma_ast::Program;
use swc_ecma_codegen::text_writer::WriteJs;
use swc_ecma_codegen::{Emitter, text_writer};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_transforms_base::resolver;
use swc_macro_condition_transform::condition_transform;
use swc_macro_condition_transform::optimization_pipeline::{run_optimization_pipeline, AdvancedOptimizationPipeline};
use swc_macro_condition_transform::feature_analyzer::should_skip_all_transformations;
use swc_macro_parser::MacroParser;
use web_sys::console;

// Console logging macro for WASM
macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&format!($($t)*).into()))
}

/// Original upstream optimize function (kept for compatibility)
pub fn optimize(source: String, config: serde_json::Value) -> String {
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = {
        let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source);
        let comments = SingleThreadedComments::default();
        let program = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments),
        )
        .parse_program()
        .unwrap();
        (program, comments)
    };

    let macros = {
        let parser = MacroParser::new("common");
        parser.parse(&comments)
    };

    let program = {
        let mut transformer = condition_transform(config, macros);
        program.visit_mut_with(&mut transformer);

        // Apply resolver and optimization
        swc_common::GLOBALS.set(&Default::default(), || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            program.mutate(resolver(unresolved_mark, top_level_mark, false));

            perform_dce(&mut program, unresolved_mark);

            program.mutate(fixer(Some(&comments)));

            program
        })
    };

    let ret = {
        let mut buf = vec![];
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
            as Box<dyn WriteJs>;
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(true),
            comments: Some(&comments),
            cm: cm.clone(),
            wr,
        };
        emitter.emit_program(&program).unwrap();
        drop(emitter);

        unsafe { String::from_utf8_unchecked(buf) }
    };

    ret
}

/// Enhanced optimize function using our modular pipeline
pub fn optimize_with_modular_pipeline(source: String, config: serde_json::Value) -> String {
    console_log!("🚀 === MODULAR OPTIMIZATION PIPELINE START ===");
    console_log!("📄 Input source length: {} chars", source.len());
    console_log!("⚙️  Config: {:?}", config);

    // Fast path check: if all features are enabled, return original code immediately
    if should_skip_all_transformations(&config) {
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
        if let Err(e) = apply_conditional_transformations(&mut program, &config, &comments) {
            console_log!("⚠️  Warning during conditional transformations: {}", e);
            // Continue with optimization even if this fails
        }
    }

    // Step 2: Run the main optimization pipeline
    let config_str = serde_json::to_string(&config).unwrap_or_else(|_| "{}".to_string());
    match run_optimization_pipeline(&source, &config_str, &mut program) {
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
            console_log!("🔄 Falling back to original optimize function");
            
            // Fallback to the original upstream optimize function
            optimize(source, config)
        }
    }
}

/// Get optimization information without applying changes
pub fn get_optimization_info(source: String, config: serde_json::Value) -> String {
    console_log!("📊 Getting optimization info without applying changes");
    
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, _comments) = match parse_source_to_ast(&source, &cm) {
        Ok((program, comments)) => (program, comments),
        Err(e) => {
            return format!("{{\"error\": \"Failed to parse: {}\"}}", e);
        }
    };
    
    let config_str = serde_json::to_string(&config).unwrap_or_else(|_| "{}".to_string());
    match run_optimization_pipeline(&source, &config_str, &mut program) {
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

/// Original DCE function from upstream
fn perform_dce(m: &mut Program, unresolved_mark: Mark) {
    let mut visitor = swc_ecma_transforms_optimization::simplify::dce::dce(
        swc_ecma_transforms_optimization::simplify::dce::Config {
            module_mark: None,
            top_level: true,
            top_retain: Default::default(),
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
