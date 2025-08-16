pub mod chunk;
pub mod module;
pub mod dependency_graph;
pub mod analyzer;
pub mod tree_shaker;

pub use chunk::{ChunkType, ChunkCharacteristics, WebpackChunk, ShareUsageConfig};
pub use module::{ModuleId, WebpackModule};
pub use dependency_graph::DependencyGraph;
pub use analyzer::WebpackAnalyzer;
pub use tree_shaker::TreeShaker;

/// Result type for webpack analysis operations
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(test)]
mod tests;