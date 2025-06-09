use std::fs;
use webpack_graph::{WebpackBundleParser, Result};

fn main() -> Result<()> {
    // Read the webpack bundle file
    let bundle_path = "../../test-cases/webpack-bundles/bundle-all-features.js";
    let bundle_content = fs::read_to_string(bundle_path)
        .map_err(|e| webpack_graph::WebpackGraphError::IoError(e))?;

    // Create parser and parse the bundle
    let parser = WebpackBundleParser::new()?;
    let graph = parser.parse_bundle(&bundle_content)?;

    println!("=== Webpack Bundle Analysis ===");
    println!("Found {} modules", graph.modules.len());
    println!("Entry points: {:?}", graph.entry_points);
    println!();

    // Print all modules and their dependencies
    println!("=== Module Dependencies ===");
    for (module_id, module) in &graph.modules {
        println!("Module {}: {} dependencies, {} dependents", 
                 module_id, 
                 module.dependencies.len(), 
                 module.dependents.len());
        
        if !module.dependencies.is_empty() {
            println!("  Dependencies: {:?}", module.dependencies);
        }
        
        if !module.dependents.is_empty() {
            println!("  Dependents: {:?}", module.dependents);
        }
        println!();
    }

    // Show reachable vs unreachable modules
    let reachable = graph.get_reachable_modules();
    let unreachable = graph.get_unreachable_modules();
    
    println!("=== Reachability Analysis ===");
    println!("Reachable modules ({}): {:?}", reachable.len(), reachable);
    
    if !unreachable.is_empty() {
        println!("Unreachable modules ({}): {:?}", unreachable.len(), unreachable);
        println!("These modules could potentially be tree-shaken!");
    } else {
        println!("No unreachable modules found - all modules are used!");
    }
    println!();

    // Show dependency chains for entry points
    println!("=== Dependency Chains ===");
    for entry_id in &graph.entry_points {
        let chain = graph.get_dependency_chain(entry_id);
        println!("Entry module {} dependency chain: {:?}", entry_id, chain);
    }

    Ok(())
} 