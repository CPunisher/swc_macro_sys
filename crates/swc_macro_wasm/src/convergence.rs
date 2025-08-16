use std::collections::VecDeque;
use std::hash::{Hash, Hasher, DefaultHasher};
use swc_ecma_ast::Program;
use crate::config::ConvergenceConfig;
use crate::error::OptimizationResult;

/// Convergence detector for iterative optimization passes
#[derive(Debug)]
pub struct ConvergenceDetector {
    config: ConvergenceConfig,
    ast_hashes: VecDeque<u64>,
    iteration_count: usize,
    no_change_count: usize,
    oscillation_detected: bool,
    last_significant_change: usize,
}

impl ConvergenceDetector {
    pub fn new(config: ConvergenceConfig) -> Self {
        let history_size = config.oscillation_history_size.max(2);
        Self {
            config,
            ast_hashes: VecDeque::with_capacity(history_size),
            iteration_count: 0,
            no_change_count: 0,
            oscillation_detected: false,
            last_significant_change: 0,
        }
    }
    
    /// Record the current iteration's AST state
    pub fn record_iteration(&mut self, program: &Program) -> ConvergenceResult {
        self.iteration_count += 1;
        
        let current_hash = if self.config.enable_ast_hashing {
            self.compute_ast_hash(program)
        } else {
            // Use iteration count as a simple hash when AST hashing is disabled
            self.iteration_count as u64
        };
        
        // Check for exact match with previous iteration (no change)
        let has_change = if let Some(&last_hash) = self.ast_hashes.back() {
            current_hash != last_hash
        } else {
            true // First iteration always has change
        };
        
        if has_change {
            self.no_change_count = 0;
            self.last_significant_change = self.iteration_count;
        } else {
            self.no_change_count += 1;
        }
        
        // Add to history and maintain size limit
        self.ast_hashes.push_back(current_hash);
        if self.ast_hashes.len() > self.config.oscillation_history_size {
            self.ast_hashes.pop_front();
        }
        
        // Check for oscillation if enabled and we have enough history
        if self.config.enable_oscillation_detection && self.ast_hashes.len() >= 3 {
            self.oscillation_detected = self.detect_oscillation();
        }
        
        // Determine convergence status
        let convergence_status = self.evaluate_convergence();
        
        ConvergenceResult {
            iteration: self.iteration_count,
            has_change,
            no_change_count: self.no_change_count,
            oscillation_detected: self.oscillation_detected,
            convergence_status,
            stability_score: self.calculate_stability_score(),
        }
    }
    
    /// Check if optimization should continue
    pub fn should_continue(&self, max_iterations: usize) -> bool {
        if self.iteration_count >= max_iterations {
            return false;
        }
        
        match self.evaluate_convergence() {
            ConvergenceStatus::Converged | 
            ConvergenceStatus::Oscillating | 
            ConvergenceStatus::Stable => false,
            ConvergenceStatus::Progressing | 
            ConvergenceStatus::Starting => true,
        }
    }
    
    /// Get convergence statistics
    pub fn stats(&self) -> ConvergenceStats {
        ConvergenceStats {
            iterations: self.iteration_count,
            no_change_iterations: self.no_change_count,
            last_significant_change: self.last_significant_change,
            oscillation_detected: self.oscillation_detected,
            stability_score: self.calculate_stability_score(),
            convergence_status: self.evaluate_convergence(),
        }
    }
    
    /// Reset the detector for a new optimization session
    pub fn reset(&mut self) {
        self.ast_hashes.clear();
        self.iteration_count = 0;
        self.no_change_count = 0;
        self.oscillation_detected = false;
        self.last_significant_change = 0;
    }
    
    /// Compute a hash of the AST structure for convergence detection
    fn compute_ast_hash(&self, program: &Program) -> u64 {
        let mut hasher = DefaultHasher::new();
        
        // Hash the program structure in a way that's sensitive to optimization changes
        match program {
            Program::Module(module) => {
                "module".hash(&mut hasher);
                module.body.len().hash(&mut hasher);
                
                // Hash key structural elements that change during optimization
                for stmt in &module.body {
                    // Hash statement type and key properties
                    std::mem::discriminant(stmt).hash(&mut hasher);
                    
                    // For variable declarations, hash the number of declarators
                    if let swc_ecma_ast::ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(swc_ecma_ast::Decl::Var(var_decl))) = stmt {
                        var_decl.decls.len().hash(&mut hasher);
                    }
                    
                    // For expressions, hash basic structure
                    if let swc_ecma_ast::ModuleItem::Stmt(swc_ecma_ast::Stmt::Expr(expr_stmt)) = stmt {
                        std::mem::discriminant(&*expr_stmt.expr).hash(&mut hasher);
                    }
                }
            }
            Program::Script(script) => {
                "script".hash(&mut hasher);
                script.body.len().hash(&mut hasher);
                
                for stmt in &script.body {
                    std::mem::discriminant(stmt).hash(&mut hasher);
                    
                    if let swc_ecma_ast::Stmt::Decl(swc_ecma_ast::Decl::Var(var_decl)) = stmt {
                        var_decl.decls.len().hash(&mut hasher);
                    }
                    
                    if let swc_ecma_ast::Stmt::Expr(expr_stmt) = stmt {
                        std::mem::discriminant(&*expr_stmt.expr).hash(&mut hasher);
                    }
                }
            }
        }
        
        hasher.finish()
    }
    
    /// Detect if the optimization is oscillating between states
    fn detect_oscillation(&self) -> bool {
        if self.ast_hashes.len() < 3 {
            return false;
        }
        
        let hashes: Vec<u64> = self.ast_hashes.iter().copied().collect();
        
        // Check for simple 2-state oscillation (A -> B -> A -> B...)
        if hashes.len() >= 4 {
            let pattern_length = 2;
            let mut is_oscillating = true;
            
            for i in pattern_length..hashes.len() {
                if hashes[i] != hashes[i - pattern_length] {
                    is_oscillating = false;
                    break;
                }
            }
            
            if is_oscillating {
                return true;
            }
        }
        
        // Check for 3-state oscillation (A -> B -> C -> A -> B -> C...)
        if hashes.len() >= 6 {
            let pattern_length = 3;
            let mut is_oscillating = true;
            
            for i in pattern_length..hashes.len() {
                if hashes[i] != hashes[i - pattern_length] {
                    is_oscillating = false;
                    break;
                }
            }
            
            if is_oscillating {
                return true;
            }
        }
        
        false
    }
    
    /// Evaluate the current convergence status
    fn evaluate_convergence(&self) -> ConvergenceStatus {
        if self.oscillation_detected {
            return ConvergenceStatus::Oscillating;
        }
        
        if self.iteration_count <= 1 {
            return ConvergenceStatus::Starting;
        }
        
        // Check for stability based on no-change threshold
        let stability_threshold = (self.config.early_termination_threshold * 10.0) as usize;
        if self.no_change_count >= stability_threshold.max(2) {
            return ConvergenceStatus::Converged;
        }
        
        // Check for extended stability
        let stability_score = self.calculate_stability_score();
        if stability_score >= self.config.early_termination_threshold {
            return ConvergenceStatus::Stable;
        }
        
        ConvergenceStatus::Progressing
    }
    
    /// Calculate a stability score (0.0 to 1.0) based on recent history
    fn calculate_stability_score(&self) -> f64 {
        if self.iteration_count <= 1 {
            return 0.0;
        }
        
        let recent_window = self.config.oscillation_history_size.min(self.iteration_count);
        let no_change_ratio = self.no_change_count as f64 / recent_window as f64;
        
        no_change_ratio.min(1.0)
    }
}

/// Result of a convergence check
#[derive(Debug, Clone)]
pub struct ConvergenceResult {
    pub iteration: usize,
    pub has_change: bool,
    pub no_change_count: usize,
    pub oscillation_detected: bool,
    pub convergence_status: ConvergenceStatus,
    pub stability_score: f64,
}

/// Status of the convergence detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvergenceStatus {
    /// Just starting, not enough data
    Starting,
    /// Making progress, should continue
    Progressing,
    /// Stable but might still improve
    Stable,
    /// Fully converged, no more changes
    Converged,
    /// Oscillating between states, should stop
    Oscillating,
}

impl std::fmt::Display for ConvergenceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvergenceStatus::Starting => write!(f, "starting"),
            ConvergenceStatus::Progressing => write!(f, "progressing"),
            ConvergenceStatus::Stable => write!(f, "stable"),
            ConvergenceStatus::Converged => write!(f, "converged"),
            ConvergenceStatus::Oscillating => write!(f, "oscillating"),
        }
    }
}

/// Convergence statistics for reporting
#[derive(Debug, Clone)]
pub struct ConvergenceStats {
    pub iterations: usize,
    pub no_change_iterations: usize,
    pub last_significant_change: usize,
    pub oscillation_detected: bool,
    pub stability_score: f64,
    pub convergence_status: ConvergenceStatus,
}

impl std::fmt::Display for ConvergenceStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Convergence: {} after {} iterations (stability: {:.2}, no-change: {}, last-change: {}{})", 
               self.convergence_status,
               self.iterations,
               self.stability_score,
               self.no_change_iterations,
               self.last_significant_change,
               if self.oscillation_detected { ", oscillation detected" } else { "" })
    }
}

/// Convenient wrapper for iterative optimization with convergence detection
pub struct IterativeOptimizer<F> {
    detector: ConvergenceDetector,
    optimization_fn: F,
    max_iterations: usize,
}

impl<F> IterativeOptimizer<F>
where
    F: Fn(&mut Program) -> OptimizationResult<bool>, // Returns true if changes were made
{
    pub fn new(config: ConvergenceConfig, max_iterations: usize, optimization_fn: F) -> Self {
        Self {
            detector: ConvergenceDetector::new(config),
            optimization_fn,
            max_iterations,
        }
    }
    
    /// Run the optimization with convergence detection
    pub fn optimize(&mut self, program: &mut Program) -> OptimizationResult<ConvergenceStats> {
        self.detector.reset();
        
        loop {
            // Record current state
            let convergence_result = self.detector.record_iteration(program);
            
            // Check if we should continue
            if !self.detector.should_continue(self.max_iterations) {
                break;
            }
            
            // Run the optimization function
            match (self.optimization_fn)(program) {
                Ok(_) => {
                    // Continue to next iteration
                }
                Err(err) => {
                    // Handle error based on configuration
                    return Err(err);
                }
            }
        }
        
        Ok(self.detector.stats())
    }
}