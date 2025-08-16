//! Extract webpack_require__ dependencies and generate markdown report
//!
//! This utility extracts all __webpack_require__ calls from webpack chunk files
//! and generates a comprehensive markdown report showing dependencies for each module.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use swc_macro_wasm::webpack_parser::{DependencyNode, WebpackChunkParser};
use swc_macro_wasm::optimize_with_prune_result;
use serde_json::Value;
use std::env;

/// Generate a markdown report focusing on pruning analysis for a single chunk
fn generate_pruning_report(chunk_name: &str, total_modules: usize) -> String {
    let mut markdown = String::new();
    
    // Add chunk header
    markdown.push_str(&format!("# {}\n\n", chunk_name));
    markdown.push_str(&format!("**Total Modules in Chunk:** {}\n\n", total_modules));
    
    markdown
}

fn compute_reachable(graph: &HashMap<String, Vec<String>>, start: &str) -> HashSet<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = vec![start.to_string()];
    while let Some(node) = stack.pop() {
        if visited.insert(node.clone()) {
            if let Some(deps) = graph.get(&node) {
                for dep in deps {
                    stack.push(dep.clone());
                }
            }
        }
    }
    visited
}

fn render_tree_ascii(node: &DependencyNode, prefix: &str, last: bool, is_root: bool, out: &mut String) {
    let connector = if is_root {
        "".to_string()
    } else if last {
        "└─ ".to_string()
    } else {
        "├─ ".to_string()
    };

    let cycle_suffix = if node.cycle.unwrap_or(false) { " (cycle)" } else { "" };
    out.push_str(&format!("{}{}{}{}\n", prefix, connector, node.id, cycle_suffix));

    let child_prefix = if is_root {
        String::new()
    } else if last {
        format!("{}   ", prefix)
    } else {
        format!("{}│  ", prefix)
    };

    for (i, child) in node.dependencies.iter().enumerate() {
        let is_last = i == node.dependencies.len() - 1;
        render_tree_ascii(child, &child_prefix, is_last, false, out);
    }
}

fn read_share_usage(path: &Path) -> Option<Value> {
    match fs::read_to_string(path) {
        Ok(data) => serde_json::from_str::<Value>(&data).ok(),
        Err(_) => None,
    }
}

fn extract_chunks_from_share_usage(share_usage: &Value, dist_dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let tree_shake = match share_usage.get("treeShake").and_then(|v| v.as_object()) {
        Some(obj) => obj,
        None => return files,
    };

    for (_lib, cfg) in tree_shake.iter() {
        if let Some(chunk_chars) = cfg.get("chunk_characteristics").and_then(|v| v.as_object()) {
            if let Some(arr) = chunk_chars.get("chunk_files").and_then(|v| v.as_array()) {
                for f in arr {
                    if let Some(name) = f.as_str() {
                        let p = dist_dir.join(name);
                        if p.exists() { files.push(p); }
                    }
                }
            }
        }
    }
    files
}

fn find_entry_ids_for_chunk<'a>(share_usage: &'a Value, chunk_file_name: &str) -> Vec<String> {
    let mut ids = Vec::new();
    if let Some(tree_shake) = share_usage.get("treeShake").and_then(|v| v.as_object()) {
        for (_lib, cfg) in tree_shake.iter() {
            if let Some(chunk_chars) = cfg.get("chunk_characteristics").and_then(|v| v.as_object()) {
                let mut matches_file = false;
                if let Some(arr) = chunk_chars.get("chunk_files").and_then(|v| v.as_array()) {
                    for f in arr {
                        if let Some(name) = f.as_str() {
                            if Path::new(name).file_name().and_then(|n| n.to_str()) == Some(chunk_file_name) {
                                matches_file = true;
                                break;
                            }
                        }
                    }
                }
                if matches_file {
                    if let Some(entry) = chunk_chars.get("entry_module_id").and_then(|v| v.as_str()) {
                        ids.push(entry.to_string());
                    }
                }
            }
        }
    }
    ids
}

fn find_package_for_chunk<'a>(share_usage: &'a Value, chunk_file_name: &str) -> Option<(String, Value)> {
    if let Some(tree_shake) = share_usage.get("treeShake").and_then(|v| v.as_object()) {
        for (lib_name, cfg) in tree_shake.iter() {
            if let Some(chunk_chars) = cfg.get("chunk_characteristics").and_then(|v| v.as_object()) {
                if let Some(arr) = chunk_chars.get("chunk_files").and_then(|v| v.as_array()) {
                    for f in arr {
                        if let Some(name) = f.as_str() {
                            if Path::new(name).file_name().and_then(|n| n.to_str()) == Some(chunk_file_name) {
                                return Some((lib_name.clone(), cfg.clone()));
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Optionally accept a path to share-usage.json or a directory containing it
    let args: Vec<String> = env::args().collect();

    // Candidate share-usage.json paths
    let mut share_usage_paths: Vec<PathBuf> = Vec::new();

    if let Some(arg1) = args.get(1) {
        let p = PathBuf::from(arg1);
        if p.is_file() {
            // If a file is provided, use directly if it's share-usage.json
            if p.file_name().and_then(|n| n.to_str()) == Some("share-usage.json") {
                share_usage_paths.push(p);
            }
        } else if p.is_dir() {
            // Check common locations within the provided directory
            let direct = p.join("share-usage.json");
            if direct.exists() { share_usage_paths.push(direct); }

            let host = p.join("host/dist/share-usage.json");
            if host.exists() { share_usage_paths.push(host); }

            let remote = p.join("remote/dist/share-usage.json");
            if remote.exists() { share_usage_paths.push(remote); }

            let dist = p.join("dist/share-usage.json");
            if dist.exists() { share_usage_paths.push(dist); }
        }
    }

    // Default hardcoded example path if no CLI arg provided or not found
    if share_usage_paths.is_empty() {
        let default_path = Path::new("/Users/bytedance/dev/swc_macro_sys/examples/module-federation-react-example/host/dist/share-usage.json");
        if default_path.exists() {
            share_usage_paths.push(default_path.to_path_buf());
        }
    }

    // Determine which JS chunk files to analyze
    let mut js_files: Vec<PathBuf> = Vec::new();
    let mut share_usages: Vec<Value> = Vec::new();

    for path in &share_usage_paths {
        if let Some(usage) = read_share_usage(path) {
            // The dist directory is the parent of the share-usage.json file
            let dist_dir = path.parent().unwrap_or_else(|| Path::new("."));
            let mut files = extract_chunks_from_share_usage(&usage, dist_dir);
            js_files.append(&mut files);
            share_usages.push(usage);
        }
    }

    // Fallback to the previous test jsonp dir if nothing found
    if js_files.is_empty() {
        let jsonp_dir = "/Users/bytedance/dev/swc_macro_sys/tests/jsonp";
        println!("No chunk files found via share-usage.json, falling back to: {}", jsonp_dir);
        let entries = fs::read_dir(jsonp_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "js") {
                js_files.push(path);
            }
        }
        // Also attempt to load share-usage.json from the fallback directory so we can compute entry ids
        let su_path = Path::new(jsonp_dir).join("share-usage.json");
        if su_path.exists() {
            match read_share_usage(&su_path) {
                Some(usage) => {
                    share_usages.push(usage);
                }
                None => {
                    println!(
                        "Warning: Failed to parse share-usage.json at fallback path: {}",
                        su_path.display()
                    );
                }
            }
        } else {
            println!(
                "Warning: No share-usage.json found at fallback path: {}",
                su_path.display()
            );
        }
    }

    // Deduplicate and sort files for consistent output
    js_files.sort();
    js_files.dedup();

    println!("Found {} JavaScript chunk files", js_files.len());

    // Create the parser
    let parser = WebpackChunkParser::new()?;

    // Process each chunk file and collect markdown
    let mut full_markdown = String::new();
    let mut total_chunks_processed = 0;
    let mut total_modules_across_chunks = 0;
    let mut total_deps_across_chunks = 0;
    // Track totals for kept/pruned to show an overall number
    let mut total_pruned_across_chunks: usize = 0;
    let mut total_kept_across_chunks: usize = 0;
    
    for js_file in &js_files {
        let file_name_os = js_file.file_name().unwrap_or_default();
        let file_name = file_name_os.to_string_lossy();
        println!("Processing chunk: {}", file_name);
        
        // Read the file content
        match fs::read_to_string(js_file) {
            Ok(content) => {
                // Parse the chunk file
                match parser.parse_chunk_file(&content) {
                    Ok(chunk_info) => {
                        // Get modules with their dependencies
                        let modules_with_deps = parser.get_modules_with_dependencies(&chunk_info);
                        
                        if !modules_with_deps.is_empty() {
                            println!("  Found {} modules", modules_with_deps.len());
                            
                            // Generate markdown for this chunk focusing on pruning
                            let mut chunk_markdown = generate_pruning_report(&file_name, modules_with_deps.len());

                            // If we have share-usage, compute kept vs pruned using entry id(s) mapped to this chunk
                            if !share_usages.is_empty() {
                                // Load full content for plan analysis
                                let mut chunk_content: Option<String> = None;
                                if let Ok(c) = fs::read_to_string(js_file) {
                                    chunk_content = Some(c);
                                }
                                // Collect entry ids from all provided share-usage files that reference this chunk file
                                let mut entry_ids: Vec<String> = Vec::new();
                                for usage in &share_usages {
                                    let mut ids = find_entry_ids_for_chunk(usage, &file_name);
                                    entry_ids.append(&mut ids);
                                }
                                // Dedup entry ids
                                entry_ids.sort();
                                entry_ids.dedup();

                                if !entry_ids.is_empty() {
                                    let graph = parser.build_dependency_graph(&chunk_info);
                                    let mut kept_union: HashSet<String> = HashSet::new();
                                    let mut trees: Vec<(String, String)> = Vec::new();
                                    let entry_ids_clone = entry_ids.clone();

                                    for entry in entry_ids_clone {
                                        if chunk_info.modules.contains_key(&entry) {
                                            let kept = compute_reachable(&graph, &entry)
                                                .into_iter()
                                                .filter(|id| chunk_info.modules.contains_key(id))
                                                .collect::<HashSet<_>>();
                                            kept_union.extend(kept.iter().cloned());

                                            if let Some(tree) = parser.build_dependency_tree(&chunk_info, &entry) {
                                                let mut ascii = String::new();
                                                render_tree_ascii(&tree, "", true, true, &mut ascii);
                                                trees.push((entry.clone(), ascii));
                                            }
                                        }
                                    }

                                    if !kept_union.is_empty() {
                                        let all_modules: HashSet<String> = chunk_info.modules.keys().cloned().collect();
                                        let pruned: Vec<String> = all_modules.difference(&kept_union).cloned().collect();

                                        // Update overall counters
                                        total_kept_across_chunks += kept_union.len();
                                        total_pruned_across_chunks += pruned.len();

                                        chunk_markdown.push_str("## Tree Shaking Analysis\n\n");
                                        chunk_markdown.push_str(&format!("- **Total modules:** {}\n", all_modules.len()));
                                        chunk_markdown.push_str(&format!("- **Kept modules:** {}\n", kept_union.len()));
                                        chunk_markdown.push_str(&format!("- **Pruned modules:** {}\n\n", pruned.len()));

                                        // Show entry points
                                        chunk_markdown.push_str("### Entry Points\n\n");
                                        for entry in &entry_ids {
                                            chunk_markdown.push_str(&format!("- `{}`\n", entry));
                                        }
                                        chunk_markdown.push_str("\n");

                                        // Show pruned list (sorted) - this is the main focus
                                        if !pruned.is_empty() {
                                            let mut pruned_sorted = pruned.clone();
                                            pruned_sorted.sort();
                                            chunk_markdown.push_str("### Pruned Modules (not reachable from entry points)\n\n");
                                            
                                            // Limit to first 50 if there are too many
                                            let display_limit = 50;
                                            let pruned_to_show = if pruned_sorted.len() > display_limit {
                                                &pruned_sorted[..display_limit]
                                            } else {
                                                &pruned_sorted
                                            };
                                            
                                            for m in pruned_to_show {
                                                chunk_markdown.push_str(&format!("- `{}`\n", m));
                                            }
                                            
                                            if pruned_sorted.len() > display_limit {
                                                chunk_markdown.push_str(&format!("\n... and {} more pruned modules\n", pruned_sorted.len() - display_limit));
                                            }
                                        } else {
                                            chunk_markdown.push_str("### No modules pruned\n\n");
                                            chunk_markdown.push_str("All modules are reachable from the entry points.\n");
                                        }

                                        chunk_markdown.push_str("\n---\n\n");
                                    }
                                }
                                
                                // Add optimization analysis using the new wrapper
                                if let Some(ref chunk_content) = chunk_content {
                                    // Find the actual package name and config for this chunk
                                    let mut tree_shake_config = serde_json::json!({"treeShake": {}});
                                    
                                    // Try to find the package configuration from share-usage.json
                                    let mut package_found = false;
                                    for usage in &share_usages {
                                        if let Some((pkg_name, pkg_cfg)) = find_package_for_chunk(usage, &file_name) {
                                            // Build config with the actual package name as key
                                            let mut module_cfg = pkg_cfg.clone();
                                            if let Some(obj) = module_cfg.as_object_mut() {
                                                // Ensure chunk_characteristics has the entry_module_id
                                                if let Some(chars) = obj.get("chunk_characteristics") {
                                                    obj.insert("chunk_characteristics".into(), chars.clone());
                                                }
                                            }
                                            
                                            tree_shake_config["treeShake"][pkg_name] = module_cfg;
                                            package_found = true;
                                            break;
                                        }
                                    }
                                    
                                    // Fallback if no package found
                                    if !package_found && !entry_ids.is_empty() {
                                        tree_shake_config = serde_json::json!({
                                            "treeShake": {
                                                "default": {
                                                    "chunk_characteristics": {
                                                        "entry_module_id": entry_ids.get(0).map(|s| s.as_str()).unwrap_or("0")
                                                    }
                                                }
                                            }
                                        });
                                    }
                                    
                                    let (_optimized_source, prune_result) = optimize_with_prune_result(
                                        chunk_content.clone(),
                                        tree_shake_config
                                    );
                                    
                                    chunk_markdown.push_str("### Optimization Results\n\n");
                                    
                                    if let Some(skip_reason) = &prune_result.skip_reason {
                                        chunk_markdown.push_str(&format!("**Status:** Skipped - {}\n", skip_reason));
                                        chunk_markdown.push_str(&format!("**Module Count:** {}\n\n", prune_result.original_count));
                                    } else {
                                        chunk_markdown.push_str(&format!("**Original Modules:** {}\n", prune_result.original_count));
                                        chunk_markdown.push_str(&format!("**After Pruning:** {} kept, {} removed\n", prune_result.kept_modules.len(), prune_result.pruned_count));
                                        
                                        if prune_result.original_count > 0 {
                                            let reduction_percent = (prune_result.pruned_count as f64 / prune_result.original_count as f64) * 100.0;
                                            chunk_markdown.push_str(&format!("**Size Reduction:** {:.1}%\n\n", reduction_percent));
                                        }
                                        
                                        // Show removed modules only if there are any
                                        if !prune_result.removed_modules.is_empty() {
                                            chunk_markdown.push_str("#### Pruned Modules from Optimization\n\n");
                                            let mut removed_sorted = prune_result.removed_modules.clone();
                                            removed_sorted.sort();
                                            
                                            // Limit display
                                            let display_limit = 50;
                                            let modules_to_show = if removed_sorted.len() > display_limit {
                                                &removed_sorted[..display_limit]
                                            } else {
                                                &removed_sorted
                                            };
                                            
                                            for module in modules_to_show {
                                                chunk_markdown.push_str(&format!("- `{}`\n", module));
                                            }
                                            
                                            if removed_sorted.len() > display_limit {
                                                chunk_markdown.push_str(&format!("\n... and {} more pruned modules\n", removed_sorted.len() - display_limit));
                                            }
                                            chunk_markdown.push_str("\n");
                                        }
                                    }
                                    
                                    chunk_markdown.push_str("---\n\n");
                                }
                            }
                            
                            full_markdown.push_str(&chunk_markdown);
                            
                            total_chunks_processed += 1;
                            total_modules_across_chunks += modules_with_deps.len();
                            total_deps_across_chunks += modules_with_deps.iter().map(|(_, deps)| deps.len()).sum::<usize>();
                        } else {
                            println!("  No modules found in this chunk");
                        }
                    }
                    Err(e) => {
                        println!("  Error parsing chunk {}: {}", file_name, e);
                    }
                }
            }
            Err(e) => {
                println!("  Error reading file {}: {}", file_name, e);
            }
        }
    }
    
    // Add overall summary at the end focusing on pruning
    full_markdown.push_str("# Overall Pruning Summary\n\n");
    full_markdown.push_str(&format!("- **Chunk Files Analyzed:** {}\n", total_chunks_processed));
    full_markdown.push_str(&format!("- **Total Modules Before Pruning:** {}\n", total_modules_across_chunks));
    
    if total_kept_across_chunks > 0 || total_pruned_across_chunks > 0 {
        full_markdown.push_str(&format!("- **Total Modules Kept:** {}\n", total_kept_across_chunks));
        full_markdown.push_str(&format!("- **Total Modules Pruned:** {}\n", total_pruned_across_chunks));
        
        if total_modules_across_chunks > 0 {
            let pruning_percent = (total_pruned_across_chunks as f64 / total_modules_across_chunks as f64) * 100.0;
            full_markdown.push_str(&format!("- **Overall Pruning Rate:** {:.1}%\n", pruning_percent));
        }
    } else {
        full_markdown.push_str("- **No pruning data available** (share-usage.json not found or no entry points defined)\n");
    }

    // Write the markdown to a file
    let output_path = Path::new("webpack_all_chunks_dependencies_report.md");
    fs::write(output_path, full_markdown)?;
    println!("Report generated at: {}", output_path.display());

    Ok(())
}