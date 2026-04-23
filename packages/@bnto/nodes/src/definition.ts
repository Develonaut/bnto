/**
 * Core recipe definition types — the JSON-serializable structure of `.bnto.json` files.
 *
 * **Source of truth:** `engine/crates/bnto-core/src/{definition,pipeline,metadata}.rs`.
 * These types are emitted by `ts-rs` via `task nodes:export-types` and mirrored here
 * through re-exports so every execution target (Rust WASM, desktop, web editor) reads
 * the same shape from a single authoritative definition.
 *
 * Do not edit the generated files in `./generated/definitionTypes/` by hand — run the
 * export task after changing the Rust structs.
 */

export type { Definition } from "./generated/definitionTypes/Definition";
export type { Position } from "./generated/definitionTypes/Position";
export type { Metadata } from "./generated/definitionTypes/Metadata";
export type { Port } from "./generated/definitionTypes/Port";
export type { Edge } from "./generated/definitionTypes/Edge";
export type { PipelineSettings } from "./generated/definitionTypes/PipelineSettings";
export type { IterationMode } from "./generated/definitionTypes/IterationMode";
export type { InputCardinality } from "./generated/definitionTypes/InputCardinality";
export type { Dependency } from "./generated/definitionTypes/Dependency";
