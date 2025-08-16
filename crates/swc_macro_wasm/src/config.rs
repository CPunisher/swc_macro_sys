use serde::{Deserialize, Serialize};
use crate::error::{OptimizationError, OptimizationResult};

/// Configuration for optimization passes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Maximum iterations for DCE and tree shaking
    pub max_iterations: usize,
    
    /// Enable debug output for optimization passes
    pub debug_output: bool,
    
    /// Enable performance monitoring and timing
    pub enable_performance_monitoring: bool,
    
    /// Memory optimization settings
    pub memory_config: MemoryConfig,
    
    /// Tree shaking specific configuration
    pub tree_shaking: TreeShakingConfig,
    
    /// Dead code elimination configuration
    pub dce_config: DceConfig,
    
    /// Convergence detection settings
    pub convergence: ConvergenceConfig,
    
    /// Error handling and recovery settings
    pub error_handling: ErrorHandlingConfig,
}

/// Memory optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Enable collection reuse to reduce allocations
    pub reuse_collections: bool,
    
    /// Enable AST emission caching
    pub cache_ast_emission: bool,
    
    /// Maximum cache size for AST emissions (in items)
    pub max_cache_size: usize,
    
    /// Enable lazy regex compilation
    pub lazy_regex_compilation: bool,
}

/// Tree shaking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeShakingConfig {
    /// Enable webpack module tree shaking
    pub enabled: bool,
    
    /// Enable simple orphan removal as fallback
    pub enable_simple_orphan_removal: bool,
    
    /// Chunk type detection strategy
    pub chunk_detection_strategy: ChunkDetectionStrategy,
    
    /// Entry point extraction settings
    pub entry_point_extraction: EntryPointConfig,
}

/// Dead code elimination configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DceConfig {
    /// Enable DCE pass
    pub enabled: bool,
    
    /// Preserve imports with side effects
    pub preserve_side_effect_imports: bool,
    
    /// Top-level DCE settings
    pub top_level: bool,
}

/// Convergence detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceConfig {
    /// Enable AST hashing for convergence detection
    pub enable_ast_hashing: bool,
    
    /// Enable oscillation detection
    pub enable_oscillation_detection: bool,
    
    /// Number of iterations to look back for oscillation detection
    pub oscillation_history_size: usize,
    
    /// Early termination threshold (percentage of no change)
    pub early_termination_threshold: f64,
}

/// Error handling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandlingConfig {
    /// Continue optimization on non-critical errors
    pub continue_on_error: bool,
    
    /// Log detailed error context
    pub detailed_error_logging: bool,
    
    /// Enable error recovery strategies
    pub enable_recovery: bool,
    
    /// Maximum retry attempts for recoverable errors
    pub max_retry_attempts: usize,
}

/// Chunk detection strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkDetectionStrategy {
    /// Auto-detect from source code patterns
    Auto,
    /// Force specific chunk type
    Force(String),
    /// Use configuration-provided characteristics
    UseConfig,
}

/// Entry point extraction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPointConfig {
    /// Enable automatic entry point extraction
    pub auto_extract: bool,
    
    /// Explicit entry points (module IDs)
    pub explicit_entries: Vec<String>,
    
    /// Regex patterns for entry point detection
    pub entry_patterns: Vec<String>,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            max_iterations: 5,
            debug_output: cfg!(debug_assertions),
            enable_performance_monitoring: false,
            memory_config: MemoryConfig::default(),
            tree_shaking: TreeShakingConfig::default(),
            dce_config: DceConfig::default(),
            convergence: ConvergenceConfig::default(),
            error_handling: ErrorHandlingConfig::default(),
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            reuse_collections: true,
            cache_ast_emission: true,
            max_cache_size: 100,
            lazy_regex_compilation: true,
        }
    }
}

impl Default for TreeShakingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            enable_simple_orphan_removal: true,
            chunk_detection_strategy: ChunkDetectionStrategy::Auto,
            entry_point_extraction: EntryPointConfig::default(),
        }
    }
}

impl Default for DceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            preserve_side_effect_imports: true,
            top_level: true,
        }
    }
}

impl Default for ConvergenceConfig {
    fn default() -> Self {
        Self {
            enable_ast_hashing: true,
            enable_oscillation_detection: true,
            oscillation_history_size: 3,
            early_termination_threshold: 0.95, // 95% no change
        }
    }
}

impl Default for ErrorHandlingConfig {
    fn default() -> Self {
        Self {
            continue_on_error: true,
            detailed_error_logging: cfg!(debug_assertions),
            enable_recovery: true,
            max_retry_attempts: 2,
        }
    }
}

impl Default for EntryPointConfig {
    fn default() -> Self {
        Self {
            auto_extract: true,
            explicit_entries: Vec::new(),
            entry_patterns: vec![
                r"__webpack_require__\(([^)]+)\);".to_string(),
            ],
        }
    }
}

impl OptimizationConfig {
    /// Create a new configuration from JSON value
    pub fn from_json(json: &serde_json::Value) -> OptimizationResult<Self> {
        let mut config = Self::default();
        
        // Extract optimization-specific settings if they exist
        if let Some(opt_config) = json.get("optimization")
            && let Ok(parsed) = serde_json::from_value::<OptimizationConfig>(opt_config.clone()) {
                return Ok(parsed);
            }
        
        // Legacy compatibility: extract settings from various locations
        if let Some(max_iter) = json.get("maxIterations").and_then(|v| v.as_u64()) {
            config.max_iterations = max_iter as usize;
        }
        
        if let Some(debug) = json.get("debug").and_then(|v| v.as_bool()) {
            config.debug_output = debug;
        }
        
        // Tree shaking configuration from existing structure
        if let Some(_tree_shake) = json.get("treeShake") {
            // Enable tree shaking when treeShake key is present; entryModules is no longer supported
            config.tree_shaking.enabled = true;
        }
        
        Ok(config)
    }
    
    /// Merge with another configuration, preferring values from `other`
    pub fn merge_with(self, other: OptimizationConfig) -> Self {
        // For now, simply replace with other's values
        // In the future, this could be more sophisticated
        other
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> OptimizationResult<()> {
        if self.max_iterations == 0 {
            return Err(OptimizationError::config_error("max_iterations must be greater than 0"));
        }
        
        if self.memory_config.max_cache_size == 0 && self.memory_config.cache_ast_emission {
            return Err(OptimizationError::config_error("max_cache_size must be greater than 0 when caching is enabled"));
        }
        
        if self.convergence.oscillation_history_size == 0 && self.convergence.enable_oscillation_detection {
            return Err(OptimizationError::config_error("oscillation_history_size must be greater than 0 when oscillation detection is enabled"));
        }
        
        if !(0.0..=1.0).contains(&self.convergence.early_termination_threshold) {
            return Err(OptimizationError::config_error("early_termination_threshold must be between 0.0 and 1.0"));
        }
        
        Ok(())
    }
    
    /// Get a default configuration optimized for performance
    pub fn performance_optimized() -> Self {
        Self {
            max_iterations: 10,
            debug_output: false,
            enable_performance_monitoring: true,
            memory_config: MemoryConfig {
                reuse_collections: true,
                cache_ast_emission: true,
                max_cache_size: 500,
                lazy_regex_compilation: true,
            },
            tree_shaking: TreeShakingConfig {
                enabled: true,
                enable_simple_orphan_removal: true,
                chunk_detection_strategy: ChunkDetectionStrategy::Auto,
                entry_point_extraction: EntryPointConfig {
                    auto_extract: true,
                    explicit_entries: Vec::new(),
                    entry_patterns: vec![
                        r"__webpack_require__\(([^)]+)\);".to_string(),
                    ],
                },
            },
            dce_config: DceConfig::default(),
            convergence: ConvergenceConfig {
                enable_ast_hashing: true,
                enable_oscillation_detection: true,
                oscillation_history_size: 5,
                early_termination_threshold: 0.98,
            },
            error_handling: ErrorHandlingConfig {
                continue_on_error: true,
                detailed_error_logging: false,
                enable_recovery: true,
                max_retry_attempts: 3,
            },
        }
    }
    
    /// Get a default configuration optimized for debugging
    pub fn debug_optimized() -> Self {
        Self {
            max_iterations: 3,
            debug_output: true,
            enable_performance_monitoring: true,
            memory_config: MemoryConfig {
                reuse_collections: false, // Disable for cleaner debugging
                cache_ast_emission: false,
                max_cache_size: 10,
                lazy_regex_compilation: false,
            },
            tree_shaking: TreeShakingConfig::default(),
            dce_config: DceConfig::default(),
            convergence: ConvergenceConfig {
                enable_ast_hashing: false, // Disable for cleaner debugging
                enable_oscillation_detection: false,
                oscillation_history_size: 2,
                early_termination_threshold: 0.8,
            },
            error_handling: ErrorHandlingConfig {
                continue_on_error: false, // Fail fast for debugging
                detailed_error_logging: true,
                enable_recovery: false,
                max_retry_attempts: 1,
            },
        }
    }
}