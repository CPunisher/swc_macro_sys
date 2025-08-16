use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};
use swc_macro_wasm::optimize;
use swc_common::{FileName, SourceMap, GLOBALS};
use swc_ecma_ast::*;
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use std::collections::HashSet;

fn repo_path(segments: &[&str]) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crates/swc_macro_wasm → repo root
    for s in ["..", ".."].into_iter() { p.push(s); }
    for seg in segments { p.push(seg); }
    p
}

#[test]
fn test_webpack_esm_react_vendor_not_emptied() {
    // Load ESM React vendor chunk
    let chunk_path = repo_path(&[
        "test-cases",
        "webpack-esm",
        "vendors-node_modules_pnpm_react_18_3_1_node_modules_react_index_js.mjs",
    ]);
    if !chunk_path.exists() {
        // Fixture optional
        return;
    }
    let source = fs::read_to_string(&chunk_path).expect("read ESM react chunk");

    // Load share-usage to get characteristics for 'react'
    let usage_path = repo_path(&["test-cases", "webpack-esm", "share-usage.json"]);
    let usage_json: Value = serde_json::from_str(&fs::read_to_string(&usage_path).expect("read usage"))
        .expect("invalid json");
    let chars = usage_json["treeShake"]["react"]["chunk_characteristics"].clone();
    assert!(chars.is_object(), "missing chunk_characteristics for react");

    // Build config strictly providing chunk_characteristics (no heuristics)
    let config = json!({
        "treeShake": { "react": { "chunk_characteristics": chars } }
    });

    let optimized = optimize::optimize(source.clone(), config).expect("optimize");

    // Parse optimized code and ensure modules object still contains entries
    let (count, _keys) = count_webpack_modules(&optimized);
    assert!(count > 0, "modules object emptied unexpectedly for ESM react vendor");
}

#[test]
fn test_rspack_cjs_lodash_vendor_not_emptied() {
    // Load CJS lodash-es vendor chunk
    let chunk_path = repo_path(&[
        "test-cases",
        "rspack-cjs-annotated-output",
        "vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js",
    ]);
    if !chunk_path.exists() {
        return;
    }
    let source = fs::read_to_string(&chunk_path).expect("read CJS lodash chunk");

    // Load share-usage to get characteristics for 'lodash-es'
    let usage_path = repo_path(&["test-cases", "rspack-cjs-annotated-output", "share-usage.json"]);
    let usage_json: Value = serde_json::from_str(&fs::read_to_string(&usage_path).expect("read usage"))
        .expect("invalid json");
    let chars = usage_json["treeShake"]["lodash-es"]["chunk_characteristics"].clone();
    assert!(chars.is_object(), "missing chunk_characteristics for lodash-es");

    let config = json!({
        "treeShake": { "lodash-es": { "chunk_characteristics": chars } }
    });

    let optimized = optimize::optimize(source.clone(), config).expect("optimize");

    // Parse optimized code and ensure CJS exports.modules still has entries
    let (count, _keys) = count_webpack_modules(&optimized);
    assert!(count > 0, "modules object emptied unexpectedly for CJS lodash vendor");
}

#[test]
fn test_rspack_jsonp_react_vendor_not_emptied() {
    // Load Rspack annotated output React vendor chunk (often JSONP-like)
    let chunk_path = repo_path(&[
        "test-cases",
        "rspack-annotated-output",
        "vendors-node_modules_pnpm_react_19_1_0_node_modules_react_index_js.js",
    ]);
    if !chunk_path.exists() {
        return;
    }
    let source = fs::read_to_string(&chunk_path).expect("read Rspack annotated react chunk");

    // Load share-usage to get characteristics for 'react' or 'react-dom'
    let usage_path = repo_path(&["test-cases", "rspack-annotated-output", "share-usage.json"]);
    let usage_json: Value = serde_json::from_str(&fs::read_to_string(&usage_path).expect("read usage"))
        .expect("invalid json");

    // Prefer react characteristics if present, else react-dom
    let chars = usage_json["treeShake"]["react"]["chunk_characteristics"].clone();
    let chars = if chars.is_object() { chars } else { usage_json["treeShake"]["react-dom"]["chunk_characteristics"].clone() };
    assert!(chars.is_object(), "missing chunk_characteristics for react/react-dom");

    let config = json!({
        "treeShake": { "react": { "chunk_characteristics": chars } }
    });

    let optimized = optimize::optimize(source.clone(), config).expect("optimize");

    // Parse optimized code and ensure JSONP modules object still has entries
    let (count, _keys) = count_webpack_modules(&optimized);
    assert!(count > 0, "modules object emptied unexpectedly for JSONP react vendor");
}

/// Count number of module properties in a webpack chunk across supported formats
fn count_webpack_modules(source: &str) -> (usize, HashSet<String>) {
    GLOBALS.set(&Default::default(), || {
        let cm: SourceMap = Default::default();
        let fm = cm.new_source_file(FileName::Custom("chunk.js".to_string()).into(), source.to_string());
        let mut parser = Parser::new(Syntax::Es(EsSyntax::default()), StringInput::from(&*fm), None);
        let program = match parser.parse_program() {
            Ok(p) => p,
            Err(_) => return (0, HashSet::new()),
        };

        let mut total = 0usize;
        let mut keys: HashSet<String> = HashSet::new();

        match program {
            Program::Module(m) => {
                // 1) ES module: export const __webpack_modules__ = { ... }
                for item in &m.body {
                    if let ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { decl: Decl::Var(v), .. })) = item {
                        for d in &v.decls {
                            if let Pat::Ident(bi) = &d.name {
                                if bi.sym == "__webpack_modules__" {
                                    if let Some(init) = &d.init {
                                        if let Expr::Object(obj) = init.as_ref() {
                                            total += obj.props.len();
                                            collect_keys_from_object(obj, &mut keys);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 2) CommonJS exports.modules = { ... }
                // 3) JSONP (...).push([[ids], { ...modules... }, runtime?])
                // Scan all statements for assign/call patterns
                for item in &m.body {
                    match item {
                        ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) => {
                            scan_expr_for_modules(expr.as_ref(), &mut total, &mut keys);
                        }
                        ModuleItem::Stmt(Stmt::Decl(Decl::Var(v))) => {
                            for d in &v.decls {
                                if let Some(init) = &d.init { scan_expr_for_modules(init.as_ref(), &mut total, &mut keys); }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Program::Script(s) => {
                for stmt in &s.body {
                    match stmt {
                        Stmt::Expr(ExprStmt { expr, .. }) => scan_expr_for_modules(expr.as_ref(), &mut total, &mut keys),
                        Stmt::Decl(Decl::Var(v)) => {
                            for d in &v.decls {
                                if let Some(init) = &d.init { scan_expr_for_modules(init.as_ref(), &mut total, &mut keys); }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        (total, keys)
    })
}

fn scan_expr_for_modules(expr: &Expr, total: &mut usize, keys: &mut HashSet<String>) {
    match expr {
        // exports.modules = { ... }
        Expr::Assign(AssignExpr { left, right, .. }) => {
            if let AssignTarget::Simple(SimpleAssignTarget::Member(member)) = left {
                if let MemberProp::Ident(p) = &member.prop {
                    if p.sym == "modules" {
                        if let Expr::Ident(obj) = member.obj.as_ref() {
                            if obj.sym == "exports" {
                                if let Expr::Object(obj) = right.as_ref() {
                                    *total += obj.props.len();
                                    collect_keys_from_object(obj, keys);
                                }
                            }
                        }
                    }
                }
            }
        }
        // JSONP: (...).push([[ids], { modules }, ...])
        Expr::Call(CallExpr { callee, args, .. }) => {
            if let Callee::Expr(c) = callee {
                if let Expr::Member(member) = c.as_ref() {
                    if let MemberProp::Ident(p) = &member.prop {
                        if p.sym == "push" {
                            if let Some(ExprOrSpread { expr: arg0, .. }) = args.get(0) {
                                if let Expr::Array(arr) = arg0.as_ref() {
                                    if arr.elems.len() >= 2 {
                                        if let Some(Some(ExprOrSpread { expr: modules_expr, .. })) = arr.elems.get(1) {
                                            if let Expr::Object(obj) = modules_expr.as_ref() {
                                                *total += obj.props.len();
                                                collect_keys_from_object(obj, keys);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn collect_keys_from_object(obj: &ObjectLit, keys: &mut HashSet<String>) {
    for prop in &obj.props {
        if let PropOrSpread::Prop(p) = prop {
            if let Prop::KeyValue(kv) = p.as_ref() {
                match &kv.key {
                    PropName::Str(s) => { keys.insert(s.value.to_string()); }
                    PropName::Ident(i) => { keys.insert(i.sym.to_string()); }
                    PropName::Num(n) => { keys.insert(n.value.to_string()); }
                    _ => {}
                }
            }
        }
    }
}


