use std::fmt;
use serde_json::Error as JsonError;

/// Comprehensive error type for optimization operations
#[derive(Debug, Clone)]
pub enum OptimizationError {
    /// Parser-related errors
    ParseError {
        message: String,
        context: String,
        source: Option<String>,
    },
    /// AST transformation errors
    TransformError {
        message: String,
        pass: String,
        context: String,
    },
    /// Code generation/emission errors
    EmissionError {
        message: String,
        phase: String,
        context: String,
    },
    /// Tree shaking analysis errors
    AnalysisError {
        message: String,
        chunk_type: String,
        context: String,
    },
    /// Configuration validation errors
    ConfigError {
        message: String,
        field: Option<String>,
        value: Option<String>,
    },
    /// Memory/resource related errors
    ResourceError {
        message: String,
        resource_type: String,
        available: Option<usize>,
        requested: Option<usize>,
    },
    /// Convergence/iteration limit errors
    ConvergenceError {
        message: String,
        iterations: usize,
        max_iterations: usize,
    },
    /// General optimization errors
    OptimizationFailed {
        message: String,
        recovery_suggestion: Option<String>,
    },
}

impl fmt::Display for OptimizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptimizationError::ParseError { message, context, source } => {
                write!(f, "Parse error: {} (context: {})", message, context)?;
                if let Some(src) = source {
                    write!(f, " - source: {}", src)?;
                }
                Ok(())
            }
            OptimizationError::TransformError { message, pass, context } => {
                write!(f, "Transform error in pass '{}': {} (context: {})", pass, message, context)
            }
            OptimizationError::EmissionError { message, phase, context } => {
                write!(f, "Emission error in phase '{}': {} (context: {})", phase, message, context)
            }
            OptimizationError::AnalysisError { message, chunk_type, context } => {
                write!(f, "Analysis error for {} chunk: {} (context: {})", chunk_type, message, context)
            }
            OptimizationError::ConfigError { message, field, value } => {
                write!(f, "Configuration error: {}", message)?;
                if let Some(field_name) = field {
                    write!(f, " (field: {})", field_name)?;
                }
                if let Some(val) = value {
                    write!(f, " (value: {})", val)?;
                }
                Ok(())
            }
            OptimizationError::ResourceError { message, resource_type, available, requested } => {
                write!(f, "Resource error ({}): {}", resource_type, message)?;
                if let (Some(avail), Some(req)) = (available, requested) {
                    write!(f, " (available: {}, requested: {})", avail, req)?;
                }
                Ok(())
            }
            OptimizationError::ConvergenceError { message, iterations, max_iterations } => {
                write!(f, "Convergence error: {} (iterations: {}/{})", message, iterations, max_iterations)
            }
            OptimizationError::OptimizationFailed { message, recovery_suggestion } => {
                write!(f, "Optimization failed: {}", message)?;
                if let Some(suggestion) = recovery_suggestion {
                    write!(f, " - suggestion: {}", suggestion)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for OptimizationError {}

/// Result type for optimization operations
pub type OptimizationResult<T> = Result<T, OptimizationError>;

/// Error recovery strategies
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// Skip the failing operation and continue
    Skip,
    /// Use a fallback implementation
    Fallback,
    /// Retry with different parameters
    Retry { max_attempts: usize },
    /// Abort the entire optimization
    Abort,
}

/// Error context for better debugging
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub file_hint: Option<String>,
    pub line_hint: Option<usize>,
    pub additional_info: Option<String>,
}

impl ErrorContext {
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            file_hint: None,
            line_hint: None,
            additional_info: None,
        }
    }
    
    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file_hint = Some(file.into());
        self
    }
    
    pub fn with_line(mut self, line: usize) -> Self {
        self.line_hint = Some(line);
        self
    }
    
    pub fn with_info(mut self, info: impl Into<String>) -> Self {
        self.additional_info = Some(info.into());
        self
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.operation)?;
        if let Some(file) = &self.file_hint {
            write!(f, " in {}", file)?;
        }
        if let Some(line) = self.line_hint {
            write!(f, " at line {}", line)?;
        }
        if let Some(info) = &self.additional_info {
            write!(f, " - {}", info)?;
        }
        Ok(())
    }
}

/// Helper functions for creating specific error types
impl OptimizationError {
    pub fn parse_error(message: impl Into<String>, context: impl Into<String>) -> Self {
        Self::ParseError {
            message: message.into(),
            context: context.into(),
            source: None,
        }
    }
    
    pub fn transform_error(message: impl Into<String>, pass: impl Into<String>, context: impl Into<String>) -> Self {
        Self::TransformError {
            message: message.into(),
            pass: pass.into(),
            context: context.into(),
        }
    }
    
    pub fn emission_error(message: impl Into<String>, phase: impl Into<String>, context: impl Into<String>) -> Self {
        Self::EmissionError {
            message: message.into(),
            phase: phase.into(),
            context: context.into(),
        }
    }
    
    pub fn analysis_error(message: impl Into<String>, chunk_type: impl Into<String>, context: impl Into<String>) -> Self {
        Self::AnalysisError {
            message: message.into(),
            chunk_type: chunk_type.into(),
            context: context.into(),
        }
    }
    
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
            field: None,
            value: None,
        }
    }
    
    pub fn convergence_error(message: impl Into<String>, iterations: usize, max_iterations: usize) -> Self {
        Self::ConvergenceError {
            message: message.into(),
            iterations,
            max_iterations,
        }
    }
    
    pub fn optimization_failed(message: impl Into<String>) -> Self {
        Self::OptimizationFailed {
            message: message.into(),
            recovery_suggestion: None,
        }
    }
    
    pub fn with_recovery_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        if let Self::OptimizationFailed { recovery_suggestion, .. } = &mut self {
            *recovery_suggestion = Some(suggestion.into());
        }
        self
    }
}

/// Convert from JSON errors
impl From<JsonError> for OptimizationError {
    fn from(err: JsonError) -> Self {
        Self::ConfigError {
            message: format!("JSON parsing error: {}", err),
            field: None,
            value: None,
        }
    }
}

/// Helper macro for creating errors with context
#[macro_export]
macro_rules! optimization_error {
    (parse, $msg:expr, $ctx:expr) => {
        $crate::error::OptimizationError::parse_error($msg, $ctx)
    };
    (transform, $msg:expr, $pass:expr, $ctx:expr) => {
        $crate::error::OptimizationError::transform_error($msg, $pass, $ctx)
    };
    (emission, $msg:expr, $phase:expr, $ctx:expr) => {
        $crate::error::OptimizationError::emission_error($msg, $phase, $ctx)
    };
    (analysis, $msg:expr, $chunk_type:expr, $ctx:expr) => {
        $crate::error::OptimizationError::analysis_error($msg, $chunk_type, $ctx)
    };
    (config, $msg:expr) => {
        $crate::error::OptimizationError::config_error($msg)
    };
    (convergence, $msg:expr, $iter:expr, $max:expr) => {
        $crate::error::OptimizationError::convergence_error($msg, $iter, $max)
    };
    (failed, $msg:expr) => {
        $crate::error::OptimizationError::optimization_failed($msg)
    };
}

/// Trait for operations that can recover from errors
pub trait Recoverable<T> {
    fn recover_with(self, strategy: RecoveryStrategy) -> OptimizationResult<T>;
}

impl<T> Recoverable<T> for OptimizationResult<T> 
where 
    T: Default 
{
    fn recover_with(self, strategy: RecoveryStrategy) -> OptimizationResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => match strategy {
                RecoveryStrategy::Skip => Ok(T::default()),
                RecoveryStrategy::Fallback => Ok(T::default()),
                RecoveryStrategy::Retry { .. } => {
                    // For now, just return default; actual retry logic would need to be implemented per-case
                    Ok(T::default())
                }
                RecoveryStrategy::Abort => Err(err),
            }
        }
    }
}