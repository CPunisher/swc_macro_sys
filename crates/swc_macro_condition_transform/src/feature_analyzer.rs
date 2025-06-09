use rustc_hash::{FxHashMap, FxHashSet};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

// Console logging macro for WASM environment
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

/// Configuration detection result
#[derive(Debug, Clone)]
pub struct FeatureDetectionResult {
    pub all_enabled: bool,
    pub enabled_features: FxHashSet<String>,
    pub feature_flags: FxHashMap<String, bool>,
    pub should_optimize: bool,
}

impl FeatureDetectionResult {
    pub fn new() -> Self {
        Self {
            all_enabled: false,
            enabled_features: FxHashSet::default(),
            feature_flags: FxHashMap::default(),
            should_optimize: false,
        }
    }
}

/// Detected conditional spans that can be removed
#[derive(Debug, Clone)]
pub struct ConditionalSpan {
    pub start: usize,
    pub end: usize,
    pub condition: String,
    pub should_remove: bool,
    pub content_preview: String,
}

/// Extract and analyze feature configuration from JSON
/// Treats the entire configuration object as potential feature flags since all values can be used in macros
pub fn extract_feature_config(config_str: &str) -> Result<FeatureDetectionResult, String> {
    console_log!("🔍 Extracting feature configuration...");
    
    let config: serde_json::Value = serde_json::from_str(config_str)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    let mut result = FeatureDetectionResult::new();
    
    // Extract configuration values - use simpler approach like the original
    let all_config_values = extract_config_values_simple(&config);
    
    console_log!("📊 Found {} configuration values that can be used as feature flags", all_config_values.len());
        
    let mut enabled_count = 0;
    let total_count = all_config_values.len();
    
    for (key, value) in &all_config_values {
        // For feature analysis, treat any truthy value as "enabled"
        let is_enabled = is_value_truthy(value);
        
        result.feature_flags.insert(key.clone(), is_enabled);
        
        if is_enabled {
            result.enabled_features.insert(key.clone());
            enabled_count += 1;
            console_log!("✅ Config value enabled: {} = {:?}", key, value);
        } else {
            console_log!("❌ Config value disabled: {} = {:?}", key, value);
        }
    }
    
    if total_count == 0 {
        console_log!("⚠️  No configuration values found");
        return Err("No configuration values found".to_string());
    }
    
    // Determine if all configuration values are enabled/truthy
    result.all_enabled = enabled_count == total_count && total_count > 0;
    result.should_optimize = !result.all_enabled; // Only optimize if not all values are truthy
    
    console_log!("📈 Configuration summary: {}/{} enabled, all_enabled: {}, should_optimize: {}", 
                enabled_count, total_count, result.all_enabled, result.should_optimize);
    
    Ok(result)
}

/// Extract configuration values using a simpler, safer approach
fn extract_config_values_simple(config: &serde_json::Value) -> std::collections::HashMap<String, serde_json::Value> {
    let mut result = std::collections::HashMap::new();
    
    if let Some(obj) = config.as_object() {
        for (key, value) in obj {
            // Add top-level values
            result.insert(key.clone(), value.clone());
            
            // If it's an object, also add its direct children with dot notation
            if let Some(nested_obj) = value.as_object() {
                for (nested_key, nested_value) in nested_obj {
                    let full_key = format!("{}.{}", key, nested_key);
                    result.insert(full_key, nested_value.clone());
                }
            }
        }
    }
    
    result
}

/// Check if a JSON value should be considered "truthy" for feature flag purposes
fn is_value_truthy(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Bool(b) => *b,
        serde_json::Value::String(s) => !s.is_empty(),
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
        serde_json::Value::Array(arr) => !arr.is_empty(),
        serde_json::Value::Object(obj) => !obj.is_empty(),
        serde_json::Value::Null => false,
    }
}

/// Analyze conditional spans in the source code that can be eliminated
pub fn analyze_conditional_spans(
    source: &str, 
    feature_config: &FeatureDetectionResult
) -> Vec<ConditionalSpan> {
    console_log!("🔍 Analyzing conditional spans for optimization...");
    
    let mut spans = Vec::new();
    
    // Look for common conditional patterns
    let conditional_patterns = [
        // Webpack conditional imports
        (r#"if\s*\(\s*__WEBPACK_IMPORT_\w+__\.features\.\w+\s*\)"#, "webpack feature check"),
        // Direct feature checks  
        (r#"if\s*\(\s*config\.features\.\w+\s*\)"#, "config feature check"),
        // Ternary conditional assignments
        (r#"\w+\s*=\s*__WEBPACK_IMPORT_\w+__\.features\.\w+\s*\?\s*[^:]+\s*:\s*[^;]+"#, "ternary feature assignment"),
    ];
    
    for (pattern, description) in &conditional_patterns {
        if let Ok(regex) = regex::Regex::new(pattern) {
            for mat in regex.find_iter(source) {
                let content = &source[mat.start()..mat.end()];
                let should_remove = should_remove_conditional_span(content, feature_config);
                
                spans.push(ConditionalSpan {
                    start: mat.start(),
                    end: mat.end(),
                    condition: description.to_string(),
                    should_remove,
                    content_preview: content.chars().take(100).collect::<String>() + "...",
                });
                
                console_log!("🔍 Found conditional span: {} (remove: {})", description, should_remove);
            }
        }
    }
    
    console_log!("✅ Found {} conditional spans, {} can be removed", 
                spans.len(), spans.iter().filter(|s| s.should_remove).count());
    
    spans
}

/// Determine if a conditional span should be removed based on feature configuration
fn should_remove_conditional_span(content: &str, feature_config: &FeatureDetectionResult) -> bool {
    // Simple heuristic: if the conditional mentions a disabled feature, it can likely be removed
    for (feature_name, is_enabled) in &feature_config.feature_flags {
        if content.contains(feature_name) && !is_enabled {
            return true;
        }
    }
    false
}

/// Check if all configuration values are enabled/truthy and transformations should be skipped
pub fn should_skip_all_transformations(config: &serde_json::Value) -> bool {
    console_log!("🔍 Checking if all transformations should be skipped...");
    
    // Extract configuration values using the simpler approach
    let all_config_values = extract_config_values_simple(config);
    
    if all_config_values.is_empty() {
        console_log!("⚠️  No configuration values found, proceeding with transformations");
        return false;
    }
    
    // Check if all configuration values are truthy
    let all_values_enabled = all_config_values.values().all(|value| is_value_truthy(value));
    
    let skip = all_values_enabled && !all_config_values.is_empty();
    
    console_log!("📊 Configuration analysis:");
    console_log!("  📦 Total config values: {}", all_config_values.len());
    console_log!("  ✅ All truthy: {}", all_values_enabled);
    console_log!("  🏃 Skip transformations: {}", skip);
    
    if skip {
        console_log!("🏃‍♂️ FAST PATH: All features enabled, skipping transformations");
    }
    
    skip
}

/// Advanced feature dependency analysis using AST
pub fn analyze_feature_dependencies(program: &Program) -> FxHashMap<String, FxHashSet<String>> {
    console_log!("🔍 Analyzing feature dependencies in AST...");
    
    struct FeatureDependencyAnalyzer {
        /// Map from feature name to modules/functions that depend on it
        feature_dependencies: FxHashMap<String, FxHashSet<String>>,
        /// Current module context
        current_module: Option<String>,
    }
    
    impl Visit for FeatureDependencyAnalyzer {
        fn visit_if_stmt(&mut self, if_stmt: &IfStmt) {
            // Analyze if conditions for feature checks
            self.analyze_condition_for_features(&if_stmt.test);
            if_stmt.visit_children_with(self);
        }
        
        fn visit_cond_expr(&mut self, cond: &CondExpr) {
            // Analyze ternary expressions for feature checks
            self.analyze_condition_for_features(&cond.test);
            cond.visit_children_with(self);
        }
        
        fn visit_member_expr(&mut self, member: &MemberExpr) {
            // Look for feature access patterns like: config.features.enableFeatureA
            if let (Expr::Member(inner), MemberProp::Ident(prop)) = (&*member.obj, &member.prop) {
                if let (Expr::Ident(obj), MemberProp::Ident(features_prop)) = (&*inner.obj, &inner.prop) {
                    if obj.sym == "features" || (obj.sym.contains("config") && features_prop.sym == "features") {
                        let feature_name = prop.sym.to_string();
                        let context = self.current_module.clone().unwrap_or_else(|| "global".to_string());
                        
                        self.feature_dependencies
                            .entry(feature_name)
                            .or_insert_with(FxHashSet::default)
                            .insert(context);
                    }
                }
            }
            member.visit_children_with(self);
        }
    }
    
    impl FeatureDependencyAnalyzer {
        fn analyze_condition_for_features(&mut self, condition: &Expr) {
            // Extract feature names from conditions
            struct ConditionAnalyzer<'a> {
                parent: &'a mut FeatureDependencyAnalyzer,
            }
            
            impl<'a> Visit for ConditionAnalyzer<'a> {
                fn visit_member_expr(&mut self, member: &MemberExpr) {
                    // Delegate to parent's member expression handler
                    self.parent.visit_member_expr(member);
                }
            }
            
            let mut analyzer = ConditionAnalyzer { parent: self };
            condition.visit_with(&mut analyzer);
        }
    }
    
    let mut analyzer = FeatureDependencyAnalyzer {
        feature_dependencies: FxHashMap::default(),
        current_module: None,
    };
    
    program.visit_with(&mut analyzer);
    
    console_log!("✅ Feature dependency analysis complete, found {} features", 
                analyzer.feature_dependencies.len());
    
    analyzer.feature_dependencies
}

/// Generate optimization recommendations based on feature analysis
pub fn generate_optimization_recommendations(
    feature_config: &FeatureDetectionResult,
    feature_dependencies: &FxHashMap<String, FxHashSet<String>>,
) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    console_log!("🔍 Generating optimization recommendations...");
    
    // Recommend optimizations for disabled features
    for (feature_name, is_enabled) in &feature_config.feature_flags {
        if !is_enabled {
            if let Some(dependencies) = feature_dependencies.get(feature_name) {
                recommendations.push(format!(
                    "Feature '{}' is disabled and affects {} modules - consider removing related code",
                    feature_name, dependencies.len()
                ));
            }
        }
    }
    
    // Recommend bundle splitting if many features are available
    if feature_config.feature_flags.len() > 3 {
        recommendations.push(
            "Multiple features detected - consider code splitting for better optimization".to_string()
        );
    }
    
    // Recommend tree shaking if not all features are enabled  
    if !feature_config.all_enabled {
        recommendations.push(
            "Not all features are enabled - tree shaking will be effective".to_string()
        );
    }
    
    console_log!("✅ Generated {} optimization recommendations", recommendations.len());
    
    recommendations
} 