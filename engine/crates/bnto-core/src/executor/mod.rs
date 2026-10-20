// Pipeline Executor — top-level orchestrator for pipeline execution.
//
// Walks nodes in order, chaining outputs to inputs. Split across these files:
//   - `mod.rs` — public API, node dispatch, shared types
//   - `chain.rs` — node chaining: each node's output feeds the next
//   - `closing.rs` — after a failure: skip nodes, run `always` closing steps
//   - `auto_iteration.rs` — implicit per-file loops for `iteration: "auto"`
//   - `primitive.rs` — leaf node execution (image compress, file rename, etc.)
//   - `leaf_output.rs` — a leaf node's output files and node record
//   - `node_reporter.rs` — a node's progress reporter, relayed as events
//   - `container.rs` — container node execution (loop, group, parallel)
//   - `loop_config.rs` — loop container params (error handling, persistence)
//   - `node_outputs.rs` — each node's output metadata, for `{{node.<id>.*}}`
//   - `count_keys.rs` — which metadata keys each node adds up across runs
//   - `result.rs` — the run's result: final files and every node's record
//   - `resolve.rs` — `{{fields.*}}` template resolution
//   - `template.rs` — multi-namespace template resolution (env, ctx, node, item)
//
// Pure Rust — no WASM deps. Works with `cargo test` natively.
// The WASM bridge (`bnto-wasm/src/execute.rs`) handles JS type conversions.

mod auto_iteration;
mod chain;
mod closing;
mod container;
mod count_keys;
mod flow;
mod flow_closing;
mod leaf_output;
pub mod loop_config;
pub mod node_outputs;
mod node_reporter;
mod primitive;
pub mod resolve;
mod result;
mod routed;
mod segments;
pub mod template;
mod unfilled_items;

use std::rc::Rc;

use crate::context::ProcessContext;
use crate::errors::BntoError;
use crate::events::{NodeInfo, PipelineEvent, PipelineReporter};
use crate::pipeline::{
    PipelineDefinition, PipelineFile, PipelineNode, PipelineResult, is_container_node,
};
use crate::registry::NodeRegistry;
use chain::run_node_chain;
use closing::ChainFailure;
use node_outputs::{CountKeys, NodeOutputs, NodeRecords};

#[cfg(test)]
use crate::processor::NodeInput;
#[cfg(test)]
use crate::progress::ProgressReporter;

// --- Shared Types ---

type PipelineNodeRef<'a> = &'a PipelineNode;

/// Bundles shared immutable state for executor functions, keeping
/// signatures clean and under clippy's 7-argument limit.
struct PipelineContext<'a, F: Fn() -> u64 + Copy> {
    registry: &'a NodeRegistry,
    reporter: &'a PipelineReporter,
    /// System access boundary for processors (commands, temp files, env vars).
    process_ctx: &'a dyn ProcessContext,
    /// Original input file count — used in progress events so the UI
    /// reports global position even inside loop container iterations.
    pipeline_total_files: usize,
    /// Returns current time in ms (injected for testability in WASM).
    now_ms: F,
    /// The same clock, shareable with the closures that throttle node
    /// progress (they must be `'static`, which `F` can't promise).
    live_clock: Rc<dyn Fn() -> u64>,
    /// When the run started (from `now_ms`), for `{{ctx.started}}`.
    started_ms: u64,
    /// Current loop iteration's per-file metadata (CSV row columns, etc.).
    /// Set by loop containers from PipelineFile.metadata each iteration.
    loop_item: Option<serde_json::Map<String, serde_json::Value>>,
    /// If executing inside a container, the container's node ID.
    /// Used to set `parent_node_id` on child `NodeStarted` events.
    parent_node_id: Option<String>,
    /// Output metadata of the nodes run so far, for {{node.<id>.*}} templates.
    /// Seeded with input file metadata before execution begins.
    node_outputs: NodeOutputs<'a>,
    /// Each node's count keys, which merges of its records add up.
    count_keys: &'a CountKeys,
}

impl<F: Fn() -> u64 + Copy> PipelineContext<'_, F> {
    /// The same context over a new, empty layer of node output records.
    fn layer(&self) -> PipelineContext<'_, F> {
        PipelineContext {
            registry: self.registry,
            reporter: self.reporter,
            process_ctx: self.process_ctx,
            pipeline_total_files: self.pipeline_total_files,
            now_ms: self.now_ms,
            live_clock: Rc::clone(&self.live_clock),
            started_ms: self.started_ms,
            loop_item: self.loop_item.clone(),
            parent_node_id: self.parent_node_id.clone(),
            node_outputs: self.node_outputs.layer(),
            count_keys: self.count_keys,
        }
    }
}

/// Where a node chain sits among its sibling nodes, for `NodeStarted` numbering.
///
/// Auto-iteration runs a pipeline as several chains; each one is a slice of
/// the pipeline and must report positions against the whole of it.
#[derive(Clone, Copy)]
struct ChainPosition {
    /// Position of the chain's first node among its siblings.
    first_index: usize,
    /// Number of sibling processing nodes (pipeline-wide, or within a container).
    total_nodes: usize,
}

impl ChainPosition {
    /// A chain that covers every one of its `total_nodes` siblings.
    fn whole(total_nodes: usize) -> Self {
        Self::starting_at(0, total_nodes)
    }

    /// A chain whose first node is at `first_index` among `total_nodes` siblings.
    fn starting_at(first_index: usize, total_nodes: usize) -> Self {
        Self {
            first_index,
            total_nodes,
        }
    }

    /// The rest of the chain, after its node at offset `i`.
    fn after(self, i: usize) -> Self {
        self.offset(i + 1)
    }

    /// The chain's node at offset `i`, onward.
    fn offset(self, i: usize) -> Self {
        Self::starting_at(self.first_index + i, self.total_nodes)
    }
}

/// Result of executing a single node or container sub-pipeline.
struct NodeExecutionResult {
    output_files: Vec<PipelineFile>,
    files_processed: usize,
    /// Non-fatal warnings (e.g. skipped loop iterations with continue-on-error).
    warnings: Vec<String>,
    /// Output metadata the node (or a container's children) recorded, by node id.
    node_outputs: NodeRecords,
}

// --- Public API ---

/// Execute a complete pipeline: walk nodes, iterate files, chain outputs.
///
/// Main entry point for the engine. `now_ms` is injected for testability
/// (no `std::time::Instant` in WASM — uses `js_sys::Date::now()` instead).
pub fn execute_pipeline(
    definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    process_ctx: &dyn ProcessContext,
    now_ms: impl Fn() -> u64 + Copy + 'static,
) -> Result<PipelineResult, BntoError> {
    let flow = flow::root_flow(definition)?;
    let count_keys = count_keys::count_keys_for(&definition.nodes, registry);
    let ctx = build_root_context(
        definition,
        &files,
        registry,
        reporter,
        process_ctx,
        &count_keys,
        now_ms,
    );
    emit_pipeline_started(&ctx, flow.order(), files.len());
    let mode = definition.resolved_iteration();
    match flow::run_level(&ctx, &flow, files, 0, mode) {
        Ok(result) => Ok(finish_pipeline(&ctx, ctx.started_ms, result)),
        Err(failure) => Err(fail_pipeline(&ctx, failure)),
    }
}

/// Build the top-level context, seeding node outputs from the input files.
fn build_root_context<'a, F: Fn() -> u64 + Copy + 'static>(
    definition: &PipelineDefinition,
    files: &[PipelineFile],
    registry: &'a NodeRegistry,
    reporter: &'a PipelineReporter,
    process_ctx: &'a dyn ProcessContext,
    count_keys: &'a CountKeys,
    now_ms: F,
) -> PipelineContext<'a, F> {
    // Read once: every node of the run sees this as `{{ctx.started}}`.
    let started_ms = now_ms();
    let seed = template::build_node_outputs_for_input(&definition.nodes, files);
    PipelineContext {
        registry,
        reporter,
        process_ctx,
        pipeline_total_files: files.len(),
        now_ms,
        live_clock: Rc::new(now_ms),
        started_ms,
        loop_item: None,
        parent_node_id: None,
        node_outputs: NodeOutputs::root(seed),
        count_keys,
    }
}

fn emit_pipeline_started<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    processing_nodes: &[&PipelineNode],
    total_files: usize,
) {
    let nodes = processing_nodes
        .iter()
        .map(|n| NodeInfo {
            id: n.id.clone(),
            node_type: n.node_type.clone(),
        })
        .collect();
    ctx.reporter.emit(PipelineEvent::PipelineStarted {
        total_nodes: processing_nodes.len(),
        total_files,
        nodes,
    });
}

/// Emit PipelineCompleted and convert the final files into the result.
fn finish_pipeline<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    start_ms: u64,
    result: NodeExecutionResult,
) -> PipelineResult {
    let duration_ms = (ctx.now_ms)() - start_ms;
    ctx.reporter.emit(PipelineEvent::PipelineCompleted {
        duration_ms,
        total_files_processed: result.files_processed,
    });
    result::build_pipeline_result(result, duration_ms)
}

/// Emit PipelineFailed once, after every closing step ran, naming the
/// top-level node whose failure is the verdict. Returns the verdict.
fn fail_pipeline<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    failure: ChainFailure,
) -> BntoError {
    let node_id = failure.node_id.clone();
    let error = failure.into_verdict();
    ctx.reporter.emit(PipelineEvent::PipelineFailed {
        node_id,
        error: error.to_string(),
    });
    error
}

// --- Internal: Node Dispatch ---

/// Emit NodeFailed and return the error. The run's PipelineFailed comes
/// later, from `fail_pipeline`, once the closing steps have run.
fn emit_node_failure<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node_id: &str,
    error: BntoError,
) -> BntoError {
    ctx.reporter.emit(PipelineEvent::NodeFailed {
        node_id: node_id.to_string(),
        error: error.to_string(),
        parent_node_id: ctx.parent_node_id.clone(),
    });
    error
}

/// Execute a single node — dispatches to primitive or container handler.
///
/// Emits NodeStarted, delegates to the appropriate handler, then emits
/// NodeCompleted or NodeFailed. Container children call back into this
/// for recursive execution.
fn execute_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    node_index: usize,
    total_nodes: usize,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    let node_start = (ctx.now_ms)();
    emit_node_started(ctx, node, node_index, total_nodes);

    let result = dispatch_node(ctx, node, files, file_offset);

    match result {
        Ok(exec_result) => {
            emit_node_completed(ctx, &node.id, node_start, exec_result.files_processed);
            Ok(exec_result)
        }
        Err(error) => Err(emit_node_failure(ctx, &node.id, error)),
    }
}

fn emit_node_started<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: &PipelineNode,
    node_index: usize,
    total_nodes: usize,
) {
    ctx.reporter.emit(PipelineEvent::NodeStarted {
        node_id: node.id.clone(),
        node_index,
        total_nodes,
        node_type: node.node_type.clone(),
        parent_node_id: ctx.parent_node_id.clone(),
    });
}

fn emit_node_completed<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node_id: &str,
    node_start: u64,
    files_processed: usize,
) {
    let duration_ms = (ctx.now_ms)() - node_start;
    ctx.reporter.emit(PipelineEvent::NodeCompleted {
        node_id: node_id.to_string(),
        duration_ms,
        files_processed,
        parent_node_id: ctx.parent_node_id.clone(),
    });
}

fn dispatch_node<F: Fn() -> u64 + Copy>(
    ctx: &PipelineContext<F>,
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    file_offset: usize,
) -> Result<NodeExecutionResult, BntoError> {
    if is_container_node(&node.node_type) {
        container::execute_container_node(ctx, node, files, file_offset)
    } else {
        primitive::execute_primitive_node(ctx, node, files, file_offset)
    }
}

#[cfg(test)]
mod tests;
