use swc_core::ecma::ast::*;
use std::time::Instant;

use crate::feature_analyzer::{FeatureDetectionResult, extract_feature_config, should_skip_all_transformations};
use crate::mutation_tracker::{MutationTracker, analyze_variable_usage, track_eliminated_dependencies, analyze_conditional_span_dependencies, apply_mutation_insights_to_graph};
use crate::webpack_module_graph::WebpackModuleGraph;
use crate::webpack_tree_shaker::perform_webpack_tree_shaking;

// Console logging macro for WASM environment
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

/// Results from the optimization pipeline
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub optimized_code: String,
    pub statistics: OptimizationStatistics,
    pub recommendations: Vec<String>,
    pub execution_time_ms: f64,
}

/// Statistics about the optimization process
#[derive(Debug, Clone)]
pub struct OptimizationStatistics {
    pub original_size: usize,
    pub optimized_size: usize,
    pub size_reduction_bytes: i32,
    pub size_reduction_percent: f64,
    pub mutations_applied: usize,
    pub modules_eliminated: usize,
    pub imports_eliminated: usize,
    pub fast_path_used: bool,
}

impl OptimizationStatistics {
    pub fn new(original_size: usize) -> Self {
        Self {
            original_size,
            optimized_size: original_size,
            size_reduction_bytes: 0,
            size_reduction_percent: 0.0,
            mutations_applied: 0,
            modules_eliminated: 0,
            imports_eliminated: 0,
            fast_path_used: false,
        }
    }
    
    pub fn finalize(&mut self, optimized_size: usize) {
        self.optimized_size = optimized_size;
        self.size_reduction_bytes = self.original_size as i32 - optimized_size as i32;
        self.size_reduction_percent = if self.original_size > 0 {
            (self.size_reduction_bytes as f64 / self.original_size as f64) * 100.0
        } else {
            0.0
        };
    }
}

/// Main optimization pipeline that coordinates all optimization stages
pub struct OptimizationPipeline {
    feature_config: Option<FeatureDetectionResult>,
    mutation_tracker: MutationTracker,
    module_graph: Option<WebpackModuleGraph>,
    statistics: OptimizationStatistics,
}

impl OptimizationPipeline {
    pub fn new(original_size: usize) -> Self {
        Self {
            feature_config: None,
            mutation_tracker: MutationTracker::new(),
            module_graph: None,
            statistics: OptimizationStatistics::new(original_size),
        }
    }
    
    /// Execute the full optimization pipeline
    pub fn optimize(
        &mut self, 
        source: &str, 
        config_str: &str, 
        program: &mut Program
    ) -> Result<OptimizationResult, String> {
        let start_time = Instant::now();
        console_log!("🚀 Starting optimization pipeline...");
        
        // Step 1: Parse configuration and check for fast path
        if let Ok(config) = serde_json::from_str::<serde_json::Value>(config_str) {
            if should_skip_all_transformations(&config) {
                console_log!("🏃‍♂️ FAST PATH: Returning original code without modifications");
                self.statistics.fast_path_used = true;
                return Ok(OptimizationResult {
                    optimized_code: source.to_string(),
                    statistics: self.statistics.clone(),
                    recommendations: vec!["Fast path used - all features enabled".to_string()],
                    execution_time_ms: start_time.elapsed().as_millis() as f64,
                });
            }
        }
        
        // Step 2: Feature analysis
        console_log!("📊 Step 2: Feature analysis");
        console_log!("📊 Step 2a: About to call extract_feature_config");
        let feature_config_result = extract_feature_config(config_str);
        console_log!("📊 Step 2b: extract_feature_config returned");
        self.feature_config = Some(feature_config_result?);
        console_log!("📊 Step 2c: Feature analysis completed successfully");
        
        // Step 3: Check if this looks like webpack bundle code
        console_log!("🔍 Step 3: Checking for webpack module structure");
        let has_webpack_modules = self.contains_webpack_modules(source);
        
        if !has_webpack_modules {
            console_log!("📄 No webpack module structure detected - using simplified optimization");
            return self.optimize_simple_code(source, start_time);
        }
        
        // Step 4: Variable usage analysis (before transformations)
        console_log!("📊 Step 4: Variable usage analysis (before)");
        let variable_usage_before = analyze_variable_usage(program);
        
        // Step 5: Module graph construction and analysis
        console_log!("🕸️  Step 5: Module graph construction");
        let mut module_graph = WebpackModuleGraph::new();
        module_graph.hydrate_module_graph_from_chunk(program);
        
        // Step 6: Conditional span dependency analysis
        console_log!("🔍 Step 6: Conditional span dependency analysis");
        if let Ok(config) = serde_json::from_str::<serde_json::Value>(config_str) {
            analyze_conditional_span_dependencies(&variable_usage_before, &config, &mut self.mutation_tracker);
        }
        
        // Step 7: Apply mutation insights to module graph
        console_log!("🔧 Step 7: Applying mutation insights");
        apply_mutation_insights_to_graph(&mut module_graph, &self.mutation_tracker);
        
        // Step 8: Webpack tree shaking with module graph
        console_log!("🌳 Step 8: Webpack tree shaking");
        let tree_shaking_stats = perform_webpack_tree_shaking(program);
        
        // Step 9: Variable usage analysis (after transformations)  
        console_log!("📊 Step 9: Variable usage analysis (after)");
        let variable_usage_after = analyze_variable_usage(program);
        
        // Step 10: Track eliminated dependencies
        console_log!("🗑️  Step 10: Tracking eliminated dependencies");
        track_eliminated_dependencies(&variable_usage_before, &variable_usage_after, &mut self.mutation_tracker);
        
        // Step 11: Generate optimized code
        console_log!("📝 Step 11: Generating optimized code");
        let optimized_code = self.generate_optimized_code(program)?;
        
        // Step 12: Update statistics
        console_log!("📈 Step 12: Updating statistics");
        self.update_statistics(&optimized_code, &tree_shaking_stats);
        
        // Step 13: Generate recommendations
        console_log!("💡 Step 13: Generating recommendations");
        let feature_config = self.feature_config.clone().unwrap();
        let recommendations = self.generate_recommendations(&feature_config);
        
        let execution_time = start_time.elapsed().as_millis() as f64;
        console_log!("✅ Optimization pipeline completed in {:.2}ms", execution_time);
        
        Ok(OptimizationResult {
            optimized_code,
            statistics: self.statistics.clone(),
            recommendations,
            execution_time_ms: execution_time,
        })
    }
    
    /// Check if source code contains webpack module structures
    fn contains_webpack_modules(&self, source: &str) -> bool {
        source.contains("__webpack_modules__") ||
        source.contains("__webpack_require__") ||
        source.contains("webpackChunkName") ||
        (source.contains("module.exports") && source.len() > 1000) // Large files with module.exports might be bundled
    }
    
    /// Simplified optimization for non-webpack code
    fn optimize_simple_code(&mut self, source: &str, start_time: Instant) -> Result<OptimizationResult, String> {
        console_log!("🚀 Running simplified optimization for non-webpack code");
        
        // For simple code, just apply basic feature analysis
        let feature_config = self.feature_config.as_ref().unwrap();
        
        // Since there are no modules to eliminate, return the original code
        // In the future, we could add other optimizations like dead code elimination
        let optimized_code = source.to_string();
        
        self.statistics.finalize(optimized_code.len());
        
        let recommendations = vec![
            "No webpack module structure detected".to_string(),
            "Applied feature analysis only".to_string(),
            if feature_config.should_optimize {
                "Code could benefit from conditional compilation markers".to_string()
            } else {
                "All configuration values are truthy - no optimization needed".to_string()
            }
        ];
        
        let execution_time = start_time.elapsed().as_millis() as f64;
        console_log!("✅ Simplified optimization completed in {:.2}ms", execution_time);
        
        Ok(OptimizationResult {
            optimized_code,
            statistics: self.statistics.clone(),
            recommendations,
            execution_time_ms: execution_time,
        })
    }
    
    /// Generate the final optimized code from the AST
    fn generate_optimized_code(&self, program: &Program) -> Result<String, String> {
        use swc_core::ecma::codegen::{Emitter, Config as EmitterConfig};
        use swc_core::common::{SourceMap, FileName, sync::Lrc};
        
        let source_map = Lrc::new(SourceMap::default());
        let _source_file = source_map.new_source_file(FileName::Anon.into(), "".into());
        
        let mut buf = Vec::new();
        {
            let mut emitter = Emitter {
                cfg: EmitterConfig::default(),
                cm: source_map.clone(),
                comments: None,
                wr: Box::new(swc_core::ecma::codegen::text_writer::JsWriter::new(
                    source_map.clone(),
                    "\n",
                    &mut buf,
                    None,
                )),
            };
            
            emitter.emit_program(program)
                .map_err(|e| format!("Failed to emit optimized code: {:?}", e))?;
        }
        
        String::from_utf8(buf)
            .map_err(|e| format!("Failed to convert optimized code to string: {}", e))
    }
    
    /// Update optimization statistics
    fn update_statistics(&mut self, optimized_code: &str, tree_shaking_stats: &crate::webpack_tree_shaker::WebpackTreeShakingStats) {
        self.statistics.finalize(optimized_code.len());
        self.statistics.mutations_applied = self.mutation_tracker.mutations.len();
        self.statistics.modules_eliminated = tree_shaking_stats.unused_modules;
        self.statistics.imports_eliminated = self.mutation_tracker.eliminated_imports.len();
        
        console_log!("📊 Optimization Statistics:");
        console_log!("  📦 Original size: {} chars", self.statistics.original_size);
        console_log!("  ⚡ Optimized size: {} chars", self.statistics.optimized_size);
        console_log!("  📉 Size reduction: {} chars ({:.1}%)", 
                    self.statistics.size_reduction_bytes, self.statistics.size_reduction_percent);
        console_log!("  🔧 Mutations applied: {}", self.statistics.mutations_applied);
        console_log!("  🗑️  Modules eliminated: {}", self.statistics.modules_eliminated);
        console_log!("  📦 Imports eliminated: {}", self.statistics.imports_eliminated);
    }
    
    /// Generate optimization recommendations
    fn generate_recommendations(&self, feature_config: &FeatureDetectionResult) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Feature-based recommendations
        if feature_config.should_optimize {
            recommendations.push("Code optimization was applied based on feature configuration".to_string());
        }
        
        // Size-based recommendations
        if self.statistics.size_reduction_percent > 20.0 {
            recommendations.push("Significant size reduction achieved - consider this configuration for production".to_string());
        } else if self.statistics.size_reduction_percent < 5.0 {
            recommendations.push("Limited optimization benefit - review feature configuration".to_string());
        }
        
        // Module elimination recommendations
        if self.statistics.modules_eliminated > 0 {
            recommendations.push(format!("Successfully eliminated {} unused modules", self.statistics.modules_eliminated));
        }
        
        // Mutation tracking recommendations
        if self.mutation_tracker.has_mutations() {
            recommendations.push(format!("Applied {} targeted optimizations", self.statistics.mutations_applied));
        }
        
        recommendations
    }
}

/// Convenience function for running the complete optimization pipeline
pub fn run_optimization_pipeline(
    source: &str,
    config_str: &str,
    program: &mut Program,
) -> Result<OptimizationResult, String> {
    let mut pipeline = OptimizationPipeline::new(source.len());
    pipeline.optimize(source, config_str, program)
}

/// Advanced pipeline with custom configuration
pub struct AdvancedOptimizationPipeline {
    pub enable_tree_shaking: bool,
    pub enable_dead_code_elimination: bool,
    pub enable_module_graph_analysis: bool,
    pub enable_variable_tracking: bool,
}

impl Default for AdvancedOptimizationPipeline {
    fn default() -> Self {
        Self {
            enable_tree_shaking: true,
            enable_dead_code_elimination: true,
            enable_module_graph_analysis: true,
            enable_variable_tracking: true,
        }
    }
}

impl AdvancedOptimizationPipeline {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Run optimization with custom settings
    pub fn optimize_with_config(
        &self,
        source: &str,
        config_str: &str,
        program: &mut Program,
    ) -> Result<OptimizationResult, String> {
        console_log!("🔧 Running advanced optimization pipeline with custom config...");
        console_log!("  🌳 Tree shaking: {}", self.enable_tree_shaking);
        console_log!("  ❌ Dead code elimination: {}", self.enable_dead_code_elimination);
        console_log!("  🕸️  Module graph analysis: {}", self.enable_module_graph_analysis);
        console_log!("  📊 Variable tracking: {}", self.enable_variable_tracking);
        
        let mut pipeline = OptimizationPipeline::new(source.len());
        
        // Apply optimizations based on configuration
        // For now, delegate to the standard pipeline
        // In the future, we can add more granular control here
        pipeline.optimize(source, config_str, program)
    }
} 