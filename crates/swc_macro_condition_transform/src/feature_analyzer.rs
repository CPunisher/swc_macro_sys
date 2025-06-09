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
pub fn extract_feature_config(config_str: &str) -> Result<FeatureDetectionResult, String> {
    console_log!("🔍 Extracting feature configuration...");
    
    let config: serde_json::Value = serde_json::from_str(config_str)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    let mut result = FeatureDetectionResult::new();
    
    // Extract feature flags from config - try multiple approaches
    let mut all_features = std::collections::HashMap::new();
    
    if let Some(config_obj) = config.as_object() {
        // Approach 1: Look for nested feature objects (featureFlags, features, etc.)
        for key in &["featureFlags", "features", "flags"] {
            if let Some(nested_obj) = config_obj.get(*key).and_then(|v| v.as_object()) {
                for (feature_key, feature_value) in nested_obj {
                    if let Some(bool_val) = feature_value.as_bool() {
                        all_features.insert(format!("{}.{}", key, feature_key), bool_val);
                    }
                }
            }
        }
        
        // Approach 2: Look for direct boolean values in the root config
        for (key, value) in config_obj {
            if let Some(bool_val) = value.as_bool() {
                all_features.insert(key.clone(), bool_val);
            }
        }
    }
    
    console_log!("📊 Found {} feature flags", all_features.len());
        
    let mut enabled_count = 0;
    let total_count = all_features.len();
    
    for (key, is_enabled) in &all_features {
        result.feature_flags.insert(key.clone(), *is_enabled);
        
        if *is_enabled {
            result.enabled_features.insert(key.clone());
            enabled_count += 1;
            console_log!("✅ Feature enabled: {}", key);
        } else {
            console_log!("❌ Feature disabled: {}", key);
        }
    }
    
    if total_count == 0 {
        console_log!("⚠️  No feature flags found in config");
        return Err("No feature flags found in configuration".to_string());
    }
    
    // Determine if all features are enabled
    result.all_enabled = enabled_count == total_count && total_count > 0;
    result.should_optimize = !result.all_enabled; // Only optimize if not all features are enabled
    
    console_log!("📈 Feature summary: {}/{} enabled, all_enabled: {}, should_optimize: {}", 
                enabled_count, total_count, result.all_enabled, result.should_optimize);
    
    Ok(result)
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

/// Check if all relevant features in the configuration are enabled
pub fn should_skip_all_transformations(config: &serde_json::Value) -> bool {
    console_log!("🔍 Checking if all transformations should be skipped...");
    
    // Extract feature flags from config - try multiple approaches
    let mut all_features = std::collections::HashMap::new();
    
    if let Some(config_obj) = config.as_object() {
        // Approach 1: Look for nested feature objects (featureFlags, features, etc.)
        for key in &["featureFlags", "features", "flags"] {
            if let Some(nested_obj) = config_obj.get(*key).and_then(|v| v.as_object()) {
                for (feature_key, feature_value) in nested_obj {
                    if let Some(bool_val) = feature_value.as_bool() {
                        all_features.insert(format!("{}.{}", key, feature_key), bool_val);
                    }
                }
            }
        }
        
        // Approach 2: Look for direct boolean values in the root config
        for (key, value) in config_obj {
            if let Some(bool_val) = value.as_bool() {
                all_features.insert(key.clone(), bool_val);
            }
        }
    }
    
    if all_features.is_empty() {
        console_log!("⚠️  No feature flags found, proceeding with transformations");
        return false;
    }
    
    let all_features_enabled = all_features.values().all(|&v| v);
    let has_features = !all_features.is_empty();
    
    let skip = all_features_enabled && has_features;
    
    console_log!("📊 Feature analysis:");
    console_log!("  📦 Total feature flags: {}", all_features.len());
    console_log!("  ✅ All enabled: {}", all_features_enabled);
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