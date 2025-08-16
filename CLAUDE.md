# CLAUDE Expectations

This document defines how we expect our AI coding assistant to collaborate in this repository and what the outputs should look like when working on webpack dependency analysis and optimization features.

## Goals
- Generate a comprehensive webpack dependencies report per chunk with nested dependency trees (ASCII), not just flat lists.
- Clearly show what was kept vs. pruned by optimization, using the pruning analysis from `crates/swc_macro_wasm/src/optimize.rs` and parsing in `crates/swc_macro_wasm/src/webpack_parser.rs`.
- Respect existing code patterns, minimize changes, and keep the project buildable at all times.

## Required Reporting (webpack_all_chunks_dependencies_report.md)
For each processed chunk, the report must include:
- "Optimization Analysis" section with:
  - status (pruned/skipped + reason)
  - original module count
  - kept count and removed (pruned) count
  - size-reduction percentage when applicable
- "Tree Shaking Summary" section with:
  - total modules considered
  - pruned modules count
- "Reachable Tree from Entry" (ASCII) built from the chunk entry module ID:
  - Entry module must be derived from `share-usage.json` (tests/jsonp/share-usage.json or provided path)
  - Nested tree structure showing transitive dependencies, indicate cycles
- "Pruned Modules (not reachable)" list (can be truncated with a note if very large)
- Overall summary across chunks at the end with totals: chunks, modules, dependencies, kept, pruned.

## Data Sources and Logic
- Always attempt to load `share-usage.json` (real path or tests/jsonp fallback) to determine entry IDs and chunk mapping.
- Use `optimize_with_prune_result` to obtain accurate kept/removed module sets, and surface these in the report.
- Build graphs and trees using `WebpackChunkParser` to extract modules and dependencies; avoid regex-based parsing.

## Coding Guidelines
- Rust style: follow existing project conventions, prefer small, focused changes, and avoid adding files unless strictly necessary.
- Error handling: never panic; for SWC parsing/emitting errors, return original source and include a clear skip reason in the report.
- Performance: avoid repeated reparsing; reuse AST when possible (e.g., parse_from_program); compute reachability with efficient graph traversal.
- Safety: respect the borrow checker; avoid moving values if they are needed later; prefer references or clones where necessary.

## Tooling and Commands
- Use `cargo` for Rust builds and running the extractor: `cargo run --bin extract_webpack_dependencies`.
- When Node tasks are required, use pnpm (not npm).
- If adding any UI/visual output in the future, provide a preview URL when changes are made.

## Communication and Process
- For multi-step tasks, maintain a brief todo list and mark items in progress/completed.
- Provide concise status updates, reference files changed, and verify the report sections are present after changes.
- Do not expose or commit secrets. Keep dependencies pinned within existing constraints.