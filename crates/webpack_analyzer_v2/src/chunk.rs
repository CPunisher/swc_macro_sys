use serde::{Deserialize, Serialize};
use rustc_hash::FxHashMap;
use crate::module::{ModuleId, WebpackModule};

/// Configuration for share usage that contains explicit entry point information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareUsageConfig {
    /// Explicit entry module IDs - no inference or filename pattern matching
    pub entry_module_ids: Vec<ModuleId>,
    /// Tree shake configuration with library-specific settings
    #[serde(rename = "treeShake", default)]
    pub tree_shake: std::collections::HashMap<String, LibraryConfig>,
}

/// Configuration for a specific library in the tree shake process
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryConfig {
    /// Export usage flags - true means used, false means unused
    #[serde(flatten)]
    pub exports: std::collections::HashMap<String, bool>,
    /// Chunk characteristics for this library
    pub chunk_characteristics: ChunkCharacteristics,
}

impl ShareUsageConfig {
    /// Load configuration from a JSON file
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: ShareUsageConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    /// Load configuration from JSON string
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: ShareUsageConfig = serde_json::from_str(json)?;
        Ok(config)
    }
    
    /// Get used exports for a specific library
    pub fn get_used_exports(&self, library_name: &str) -> Vec<String> {
        if let Some(lib_config) = self.tree_shake.get(library_name) {
            lib_config.exports.iter()
                .filter_map(|(export, &used)| if used { Some(export.clone()) } else { None })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get unused exports for a specific library
    pub fn get_unused_exports(&self, library_name: &str) -> Vec<String> {
        if let Some(lib_config) = self.tree_shake.get(library_name) {
            lib_config.exports.iter()
                .filter_map(|(export, &used)| if !used { Some(export.clone()) } else { None })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Check if a chunk should be processed based on configuration
    pub fn should_process_chunk(&self, chunk_files: &[String]) -> Option<&LibraryConfig> {
        for (_, lib_config) in &self.tree_shake {
            // Check if any of the chunk files match the configured chunk files
            for chunk_file in chunk_files {
                if lib_config.chunk_characteristics.chunk_files.contains(chunk_file) {
                    // Skip runtime chunks
                    if !lib_config.chunk_characteristics.is_runtime_chunk {
                        return Some(lib_config);
                    }
                }
            }
        }
        None
    }
}

/// Represents chunk characteristics from webpack/rspack build metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChunkCharacteristics {
    pub is_runtime_chunk: bool,
    pub has_runtime: bool,
    pub is_entrypoint: bool,
    pub can_be_initial: bool,
    pub is_only_initial: bool,
    pub chunk_format: String,
    pub chunk_loading_type: Option<String>,
    pub runtime_names: Vec<String>,
    pub entry_name: Option<String>,
    pub has_async_chunks: bool,
    pub chunk_files: Vec<String>,
    pub is_shared_chunk: bool,
    pub shared_modules: Vec<serde_json::Value>,
    pub entry_module_id: Option<String>,
}

/// Represents the different types of webpack chunks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChunkType {
    /// JSONP format: (self["webpackChunk..."] = ...).push([[...], {...}])
    /// Used for web browser chunks with "jsonp" chunk_format
    JSONP,
    /// CommonJS async format: exports.modules = {...}
    /// Used for node chunks with "async-node" chunk_format  
    CommonJSAsync,
    /// CommonJS sync format: exports.modules = {...}
    /// Used for node chunks with "require" chunk_format
    CommonJSSync,
    /// Webpack modules format: var __webpack_modules__ = ({...})
    /// Used for main entry chunks or when chunk_characteristics unavailable
    WebpackModules,
    /// ES modules format for modern bundlers
    ESModules,
    /// Unknown chunk type when detection fails
    Unknown,
}

/// Represents a webpack chunk with its modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebpackChunk {
    /// The type of chunk format
    pub chunk_type: ChunkType,
    /// The modules contained in this chunk
    pub modules: FxHashMap<ModuleId, WebpackModule>,
    /// Original source code for debugging
    pub source: String,
    /// Chunk characteristics from webpack/rspack build metadata (optional)
    pub characteristics: Option<ChunkCharacteristics>,
}

impl WebpackChunk {
    /// Create a new webpack chunk
    pub fn new(chunk_type: ChunkType, source: String) -> Self {
        Self {
            chunk_type,
            modules: FxHashMap::default(),
            source,
            characteristics: None,
        }
    }

    /// Create a new webpack chunk with characteristics
    pub fn new_with_characteristics(chunk_type: ChunkType, source: String, characteristics: ChunkCharacteristics) -> Self {
        Self {
            chunk_type,
            modules: FxHashMap::default(),
            source,
            characteristics: Some(characteristics),
        }
    }

    /// Set chunk characteristics
    pub fn set_characteristics(&mut self, characteristics: ChunkCharacteristics) {
        self.characteristics = Some(characteristics);
    }

    /// Add a module to this chunk
    pub fn add_module(&mut self, module_id: ModuleId, module: WebpackModule) {
        self.modules.insert(module_id, module);
    }

    /// Get all module IDs in this chunk
    pub fn get_module_ids(&self) -> Vec<ModuleId> {
        self.modules.keys().cloned().collect()
    }

    /// Get a module by ID
    pub fn get_module(&self, module_id: &ModuleId) -> Option<&WebpackModule> {
        self.modules.get(module_id)
    }

    /// Get the number of modules in this chunk
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Extract explicit entry points from ShareUsageConfig - NO filename inference or heuristics
    /// 
    /// This method ONLY uses explicitly defined entry points from configuration.
    /// It performs NO filename pattern matching, NO heuristics, and NO inference.
    /// Returns empty Vec if no explicit entry points are found in the configuration.
    /// 
    /// # Arguments
    /// * `config` - ShareUsageConfig containing explicit entry module IDs
    /// 
    /// # Returns
    /// Vec of ModuleId that are explicitly configured as entry points and exist in this chunk
    pub fn extract_explicit_entry_points(&self, config: &ShareUsageConfig) -> Vec<ModuleId> {
        let mut entry_points = Vec::new();
        
        // Only use explicitly configured entry module IDs
        for entry_module_id in &config.entry_module_ids {
            // Check if this explicitly configured entry point exists in the chunk
            if self.modules.contains_key(entry_module_id) {
                entry_points.push(entry_module_id.clone());
            }
        }
        
        // Also check chunk characteristics for entry_module_id if available
        if let Some(characteristics) = &self.characteristics {
            if let Some(entry_id) = &characteristics.entry_module_id {
                let entry_module_id = swc_core::atoms::Atom::from(entry_id.as_str());
                if self.modules.contains_key(&entry_module_id) && !entry_points.contains(&entry_module_id) {
                    entry_points.push(entry_module_id);
                }
            }
        }
        
        entry_points
    }

    /// Extract explicit entry points with error handling for missing entry points
    /// 
    /// Similar to extract_explicit_entry_points but returns an error if configured
    /// entry points are missing from the chunk.
    /// 
    /// # Arguments
    /// * `config` - ShareUsageConfig containing explicit entry module IDs
    /// 
    /// # Returns
    /// Result containing Vec of ModuleId or error if configured entry points are missing
    pub fn extract_explicit_entry_points_strict(&self, config: &ShareUsageConfig) -> crate::Result<Vec<ModuleId>> {
        let mut entry_points = Vec::new();
        let mut missing_entries = Vec::new();
        
        // Check all explicitly configured entry module IDs
        for entry_module_id in &config.entry_module_ids {
            if self.modules.contains_key(entry_module_id) {
                entry_points.push(entry_module_id.clone());
            } else {
                missing_entries.push(entry_module_id.clone());
            }
        }
        
        // Check chunk characteristics for entry_module_id
        if let Some(characteristics) = &self.characteristics {
            if let Some(entry_id) = &characteristics.entry_module_id {
                let entry_module_id = swc_core::atoms::Atom::from(entry_id.as_str());
                if self.modules.contains_key(&entry_module_id) {
                    if !entry_points.contains(&entry_module_id) {
                        entry_points.push(entry_module_id);
                    }
                } else {
                    missing_entries.push(entry_module_id);
                }
            }
        }
        
        // Return error if any configured entry points are missing
        if !missing_entries.is_empty() {
            let missing_ids: Vec<String> = missing_entries.iter().map(|id| id.to_string()).collect();
            return Err(format!("Missing entry points in chunk: {}", missing_ids.join(", ")).into());
        }
        
        // Return error if no entry points found at all
        if entry_points.is_empty() {
            return Err("No explicit entry points found in configuration".into());
        }
        
        Ok(entry_points)
    }
}

impl ChunkCharacteristics {
    /// Determine the chunk type based on chunk characteristics
    pub fn determine_chunk_type(&self) -> ChunkType {
        match self.chunk_format.as_str() {
            "jsonp" => ChunkType::JSONP,
            "async-node" => ChunkType::CommonJSAsync,
            "require" => ChunkType::CommonJSSync,
            "webpack" => ChunkType::WebpackModules,
            "module" => ChunkType::ESModules,
            _ => {
                // Fall back to heuristics for unknown formats
                if self.is_entrypoint || self.has_runtime {
                    ChunkType::WebpackModules
                } else {
                    ChunkType::Unknown
                }
            }
        }
    }


    /// Check if this is a runtime chunk
    pub fn is_runtime(&self) -> bool {
        self.is_runtime_chunk || self.has_runtime
    }

    /// Check if this chunk represents a vendor bundle.
    ///
    /// A vendor chunk is one that can be loaded on the initial page load,
    /// but is not itself an entrypoint or runtime chunk. This aligns with
    /// how build tools typically emit third-party libraries.
    pub fn is_vendor_chunk(&self) -> bool {
        self.can_be_initial && !self.is_entrypoint && !self.is_runtime()
    }

    /// Get the chunk file names
    pub fn get_chunk_files(&self) -> &Vec<String> {
        &self.chunk_files
    }
}