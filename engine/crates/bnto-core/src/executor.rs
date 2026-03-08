// =============================================================================
// Pipeline Executor — The Engine's Brain
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the heart of the engine. It takes a pipeline definition (a list of
// nodes), a batch of input files, and a registry of processors, then:
//   1. Walks the nodes in order
//   2. Skips I/O marker nodes (input, output)
//   3. For each processing node, iterates files and calls the right processor
//   4. Chains outputs → inputs between sequential nodes
//   5. Handles container nodes (loop, group, parallel) via recursion
//   6. Emits structured progress events at every stage
//
// WHY IS THIS IN RUST (not JavaScript)?
// Previously, ALL orchestration lived in JS (`executePipeline.ts`). The Rust
// engine only knew how to process ONE file at a time. This meant:
//   - The "intelligence" was in the wrong layer (JS, not the engine)
//   - Every new consumer (CLI, desktop, server) would need to reimplement
//     the orchestration logic
//   - Complex recipes (loops, groups, nested nodes) would need to be built
//     twice (JS for browser, native for everything else)
//
// By moving the executor to Rust, we get ONE implementation that works
// everywhere: browser (WASM), CLI (native binary), desktop (Tauri), server.
//
// HOW IT FITS WITH EXISTING CODE:
// - `NodeProcessor` trait (processor.rs) — the executor calls this per-file
// - `ProgressReporter` (progress.rs) — wraps per-file progress into pipeline events
// - `NodeRegistry` (registry.rs) — looks up the right processor for each node
// - `PipelineReporter` (events.rs) — emits structured events to the consumer
//
// The executor is PURE RUST — no WASM dependencies, no JS types. It works
// with `cargo test` natively. The WASM bridge (`bnto-wasm/src/execute.rs`)
// wraps this with JS type conversions.

use crate::errors::BntoError;
use crate::events::{PipelineEvent, PipelineReporter};
use crate::pipeline::{
    is_container_node, is_io_node, PipelineDefinition, PipelineFile, PipelineFileResult,
    PipelineNode, PipelineResult,
};
use crate::processor::NodeInput;
use crate::progress::ProgressReporter;
use crate::registry::NodeRegistry;

// =============================================================================
// Public API
// =============================================================================

/// Execute a complete pipeline: walk nodes, iterate files, chain outputs.
///
/// This is the main entry point for the engine. Every consumer (browser WASM,
/// CLI, desktop, server) calls this function with its own adapter for time
/// and progress reporting.
///
/// # Arguments
/// - `definition` — the parsed pipeline definition (nodes in order)
/// - `files` — the input files to process
/// - `registry` — maps node types to processor implementations
/// - `reporter` — receives structured progress events
/// - `now_ms` — returns current time in milliseconds (injected for testability)
///
/// # Returns
/// - `Ok(PipelineResult)` — all files processed successfully
/// - `Err(BntoError)` — a node failed, pipeline stopped
///
/// RUST CONCEPT: `impl Fn() -> u64`
/// The `now_ms` parameter is a closure that returns the current time.
/// We inject it instead of calling `std::time::Instant::now()` directly
/// because:
///   - In WASM, there's no `std::time::Instant` — we use `js_sys::Date::now()`
///   - In tests, we can inject a fake clock for deterministic timing
///   - This keeps the executor target-agnostic (no platform-specific imports)
pub fn execute_pipeline(
    definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    now_ms: impl Fn() -> u64 + Copy,
) -> Result<PipelineResult, BntoError> {
    // --- Step 1: Record the start time ---
    let start_ms = now_ms();

    // --- Step 2: Filter out I/O marker nodes to find processing nodes ---
    // I/O nodes ("input", "output") are structural markers in the recipe
    // definition. They tell the editor where files enter and leave the
    // pipeline, but they don't perform any processing.
    let processing_nodes: Vec<&PipelineNode> = definition
        .nodes
        .iter()
        .filter(|n| !is_io_node(&n.node_type))
        .collect();

    let total_nodes = processing_nodes.len();
    let total_files = files.len();

    // --- Step 3: Emit PipelineStarted event ---
    reporter.emit(PipelineEvent::PipelineStarted {
        total_nodes,
        total_files,
    });

    // --- Step 4: Execute processing nodes sequentially ---
    // The current batch of files starts as the input files. After each
    // node processes them, the output becomes the input for the next node.
    let mut current_files = files;
    let mut total_files_processed: usize = 0;

    for (node_index, node) in processing_nodes.iter().enumerate() {
        // Execute this node (handles both primitive and container nodes).
        let result = execute_node(
            node,
            current_files,
            registry,
            reporter,
            node_index,
            total_nodes,
            &now_ms,
        )?;

        // Track total files processed across all nodes.
        total_files_processed += result.files_processed;

        // Chain: this node's output becomes the next node's input.
        current_files = result.output_files;
    }

    // --- Step 5: Calculate total duration and emit PipelineCompleted ---
    let duration_ms = now_ms() - start_ms;

    reporter.emit(PipelineEvent::PipelineCompleted {
        duration_ms,
        total_files_processed,
    });

    // --- Step 6: Convert final files to PipelineFileResults ---
    let result_files: Vec<PipelineFileResult> = current_files
        .into_iter()
        .map(|f| PipelineFileResult {
            name: f.name,
            data: f.data,
            mime_type: f.mime_type,
            metadata: serde_json::Map::new(),
        })
        .collect();

    Ok(PipelineResult {
        files: result_files,
        duration_ms,
    })
}

// =============================================================================
// Internal: Node Execution Result
// =============================================================================

/// The result of executing a single node (or container sub-pipeline).
/// Used internally to chain outputs between nodes.
struct NodeExecutionResult {
    /// The output files from this node (become input for the next node).
    output_files: Vec<PipelineFile>,
    /// How many files this node processed (for progress tracking).
    files_processed: usize,
}

// =============================================================================
// Internal: Execute a Single Node
// =============================================================================

/// Execute a single node — either a primitive processor or a container.
///
/// This is the recursive workhorse. For primitive nodes, it iterates files
/// and calls the processor. For container nodes, it recurses into children.
fn execute_node(
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    node_index: usize,
    total_nodes: usize,
    now_ms: &(impl Fn() -> u64 + Copy),
) -> Result<NodeExecutionResult, BntoError> {
    let node_start = now_ms();

    // --- Emit NodeStarted event ---
    reporter.emit(PipelineEvent::NodeStarted {
        node_id: node.id.clone(),
        node_index,
        total_nodes,
        node_type: node.node_type.clone(),
    });

    // --- Decide: container or primitive? ---
    let result = if is_container_node(&node.node_type) {
        execute_container_node(node, files, registry, reporter, now_ms)
    } else {
        execute_primitive_node(node, files, registry, reporter)
    };

    // --- Handle success or failure ---
    match result {
        Ok(exec_result) => {
            // Emit NodeCompleted on success.
            let duration_ms = now_ms() - node_start;
            reporter.emit(PipelineEvent::NodeCompleted {
                node_id: node.id.clone(),
                duration_ms,
                files_processed: exec_result.files_processed,
            });
            Ok(exec_result)
        }
        Err(error) => {
            // Emit NodeFailed, then PipelineFailed, then propagate.
            reporter.emit(PipelineEvent::NodeFailed {
                node_id: node.id.clone(),
                error: error.to_string(),
            });
            reporter.emit(PipelineEvent::PipelineFailed {
                node_id: node.id.clone(),
                error: error.to_string(),
            });
            Err(error)
        }
    }
}

// =============================================================================
// Internal: Execute a Primitive (Leaf) Node
// =============================================================================

/// Execute a primitive node: look up the processor, iterate files, call it.
fn execute_primitive_node(
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
) -> Result<NodeExecutionResult, BntoError> {
    // --- Step 1: Resolve the processor from the registry ---
    let processor = registry
        .resolve(&node.node_type, &node.params)
        .ok_or_else(|| {
            // Build a descriptive error message including the compound key.
            let operation = node
                .params
                .get("operation")
                .and_then(|v| v.as_str())
                .unwrap_or("default");
            BntoError::InvalidInput(format!(
                "No processor registered for '{}:{}' (node '{}')",
                node.node_type, operation, node.id
            ))
        })?;

    // --- Step 2: Process each file ---
    let total_files = files.len();

    // PERFORMANCE: Pre-allocate with capacity. Most processors produce 1 output
    // per input, so `total_files` is a good estimate. Avoids repeated
    // reallocation and copying as the Vec grows.
    let mut output_files: Vec<PipelineFile> = Vec::with_capacity(total_files);

    // PERFORMANCE: Clone node.id once outside the loop. Each file iteration
    // needs the node_id for progress events, but cloning inside the loop
    // allocates a new String per file. One clone, reused via references.
    let node_id = node.id.clone();

    // PERFORMANCE: Clone params once outside the loop. Every file iteration
    // needs the same params map for the processor input. Cloning inside
    // the loop would deep-copy the entire JSON map per file.
    let params_for_input = node.params.clone();

    for (file_index, file) in files.into_iter().enumerate() {
        // Create a per-file ProgressReporter that converts to FileProgress events.
        // This bridges the existing per-file progress system with the new
        // pipeline event system.
        let file_progress_reporter = {
            let node_id_for_closure = node_id.clone();
            ProgressReporter::new(move |percent, message| {
                // Note: We can't call reporter.emit() here because it would
                // require capturing `reporter` in the closure, which conflicts
                // with the borrow checker. Instead, we use a simpler approach:
                // the node-level FileProgress events are emitted from the
                // outer scope after each file completes.
                //
                // For now, the per-file ProgressReporter is a no-op at the
                // pipeline level. The FileProgress event is emitted below
                // with the correct file_index and total_files.
                let _ = (percent, message, &node_id_for_closure);
            })
        };

        // PERFORMANCE: Capture filename before it's moved into NodeInput.
        // This avoids a clone — we take ownership here and use a reference
        // for progress messages.
        let file_name = file.name;

        // Emit FileProgress at 0% to signal "starting this file".
        reporter.emit(PipelineEvent::FileProgress {
            node_id: node_id.clone(),
            file_index,
            total_files,
            percent: 0,
            message: format!("Processing {}...", &file_name),
        });

        // Build the NodeInput from our PipelineFile.
        let input = NodeInput {
            data: file.data,
            filename: file_name.clone(),
            mime_type: Some(file.mime_type),
            params: params_for_input.clone(),
        };

        // Call the processor.
        let output = processor.process(input, &file_progress_reporter)?;

        // Emit FileProgress at 100% to signal "done with this file".
        reporter.emit(PipelineEvent::FileProgress {
            node_id: node_id.clone(),
            file_index,
            total_files,
            percent: 100,
            message: format!("Completed {}", &file_name),
        });

        // Convert NodeOutput files to PipelineFiles for chaining.
        for output_file in output.files {
            output_files.push(PipelineFile {
                name: output_file.filename,
                data: output_file.data,
                mime_type: output_file.mime_type,
            });
        }
    }

    Ok(NodeExecutionResult {
        files_processed: total_files,
        output_files,
    })
}

// =============================================================================
// Internal: Execute a Container Node
// =============================================================================

/// Execute a container node (loop, group, parallel) by recursing into children.
///
/// Container semantics:
/// - `loop` — run children sub-pipeline once PER file (each iteration gets one file)
/// - `group` — run children sub-pipeline once on the FULL batch
/// - `parallel` — same as group for now (concurrent execution is future work)
fn execute_container_node(
    node: &PipelineNode,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    now_ms: &(impl Fn() -> u64 + Copy),
) -> Result<NodeExecutionResult, BntoError> {
    // Get children, defaulting to empty if none (passthrough).
    let children = match &node.children {
        Some(c) => c,
        None => {
            // Container with no children = passthrough.
            return Ok(NodeExecutionResult {
                files_processed: 0,
                output_files: files,
            });
        }
    };

    // If no children, passthrough.
    if children.is_empty() {
        return Ok(NodeExecutionResult {
            files_processed: 0,
            output_files: files,
        });
    }

    // Build a sub-pipeline definition from the children.
    let sub_definition = crate::pipeline::PipelineDefinition {
        nodes: children.clone(),
    };

    match node.node_type.as_str() {
        "loop" => {
            // --- Loop: run sub-pipeline once PER file ---
            // Each iteration gets a single-file batch. Results are collected.
            let mut all_output_files: Vec<PipelineFile> = Vec::new();
            let mut total_processed: usize = 0;

            for file in files {
                let single_file_batch = vec![file];
                let result = execute_sub_pipeline(
                    &sub_definition,
                    single_file_batch,
                    registry,
                    reporter,
                    now_ms,
                )?;
                total_processed += result.files_processed;
                all_output_files.extend(result.output_files);
            }

            Ok(NodeExecutionResult {
                files_processed: total_processed,
                output_files: all_output_files,
            })
        }

        // "group" and "parallel" both run the sub-pipeline on the full batch.
        // "parallel" is the same as "group" for now — concurrent execution
        // is future work.
        "group" | "parallel" => {
            let result =
                execute_sub_pipeline(&sub_definition, files, registry, reporter, now_ms)?;
            Ok(NodeExecutionResult {
                files_processed: result.files_processed,
                output_files: result.output_files,
            })
        }

        _ => {
            // Unknown container type — treat as passthrough with a warning.
            Ok(NodeExecutionResult {
                files_processed: 0,
                output_files: files,
            })
        }
    }
}

// =============================================================================
// Internal: Execute a Sub-Pipeline (for container children)
// =============================================================================

/// Execute a sub-pipeline (the children of a container node).
///
/// This is essentially `execute_pipeline` but without the top-level
/// PipelineStarted/PipelineCompleted events (those belong to the
/// outer pipeline, not each container's children).
fn execute_sub_pipeline(
    definition: &PipelineDefinition,
    files: Vec<PipelineFile>,
    registry: &NodeRegistry,
    reporter: &PipelineReporter,
    now_ms: &(impl Fn() -> u64 + Copy),
) -> Result<NodeExecutionResult, BntoError> {
    // Filter out I/O nodes from children too.
    let processing_nodes: Vec<&PipelineNode> = definition
        .nodes
        .iter()
        .filter(|n| !is_io_node(&n.node_type))
        .collect();

    let total_nodes = processing_nodes.len();
    let mut current_files = files;
    let mut total_files_processed: usize = 0;

    for (node_index, node) in processing_nodes.iter().enumerate() {
        let result = execute_node(
            node,
            current_files,
            registry,
            reporter,
            node_index,
            total_nodes,
            now_ms,
        )?;
        total_files_processed += result.files_processed;
        current_files = result.output_files;
    }

    Ok(NodeExecutionResult {
        files_processed: total_files_processed,
        output_files: current_files,
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::RecordingReporter;
    use crate::processor::{NodeOutput, OutputFile};

    // =========================================================================
    // Mock Processors for Testing
    // =========================================================================

    /// Echoes input files back unchanged. The simplest possible processor.
    struct EchoProcessor;

    impl crate::processor::NodeProcessor for EchoProcessor {
        fn name(&self) -> &str {
            "echo"
        }

        fn process(
            &self,
            input: NodeInput,
            _progress: &ProgressReporter,
        ) -> Result<NodeOutput, BntoError> {
            Ok(NodeOutput {
                files: vec![OutputFile {
                    data: input.data,
                    filename: input.filename,
                    mime_type: input
                        .mime_type
                        .unwrap_or_else(|| "application/octet-stream".to_string()),
                }],
                metadata: serde_json::Map::new(),
            })
        }
    }

    /// Converts filename to uppercase. Verifies data transformation works.
    struct UpperCaseProcessor;

    impl crate::processor::NodeProcessor for UpperCaseProcessor {
        fn name(&self) -> &str {
            "uppercase"
        }

        fn process(
            &self,
            input: NodeInput,
            _progress: &ProgressReporter,
        ) -> Result<NodeOutput, BntoError> {
            Ok(NodeOutput {
                files: vec![OutputFile {
                    data: input.data,
                    filename: input.filename.to_uppercase(),
                    mime_type: input
                        .mime_type
                        .unwrap_or_else(|| "application/octet-stream".to_string()),
                }],
                metadata: serde_json::Map::new(),
            })
        }
    }

    /// Always fails. For testing error handling.
    struct FailProcessor;

    impl crate::processor::NodeProcessor for FailProcessor {
        fn name(&self) -> &str {
            "fail"
        }

        fn process(
            &self,
            _input: NodeInput,
            _progress: &ProgressReporter,
        ) -> Result<NodeOutput, BntoError> {
            Err(BntoError::ProcessingFailed(
                "intentional test failure".to_string(),
            ))
        }
    }

    /// Reports progress at 25/50/75/100%. For testing progress events.
    struct SlowProcessor;

    impl crate::processor::NodeProcessor for SlowProcessor {
        fn name(&self) -> &str {
            "slow"
        }

        fn process(
            &self,
            input: NodeInput,
            progress: &ProgressReporter,
        ) -> Result<NodeOutput, BntoError> {
            progress.report(25, "Quarter done");
            progress.report(50, "Half done");
            progress.report(75, "Three quarters");
            progress.report(100, "Complete");

            Ok(NodeOutput {
                files: vec![OutputFile {
                    data: input.data,
                    filename: input.filename,
                    mime_type: input
                        .mime_type
                        .unwrap_or_else(|| "application/octet-stream".to_string()),
                }],
                metadata: serde_json::Map::new(),
            })
        }
    }

    /// Returns two files per input. Verifies file count changes through pipeline.
    struct DoubleProcessor;

    impl crate::processor::NodeProcessor for DoubleProcessor {
        fn name(&self) -> &str {
            "double"
        }

        fn process(
            &self,
            input: NodeInput,
            _progress: &ProgressReporter,
        ) -> Result<NodeOutput, BntoError> {
            let mime = input
                .mime_type
                .unwrap_or_else(|| "application/octet-stream".to_string());
            Ok(NodeOutput {
                files: vec![
                    OutputFile {
                        data: input.data.clone(),
                        filename: format!("{}-a", input.filename),
                        mime_type: mime.clone(),
                    },
                    OutputFile {
                        data: input.data,
                        filename: format!("{}-b", input.filename),
                        mime_type: mime,
                    },
                ],
                metadata: serde_json::Map::new(),
            })
        }
    }

    // =========================================================================
    // Test Helpers
    // =========================================================================

    /// Create a simple file for testing.
    fn make_file(name: &str, data: &[u8]) -> PipelineFile {
        PipelineFile {
            name: name.to_string(),
            data: data.to_vec(),
            mime_type: "application/octet-stream".to_string(),
        }
    }

    /// Build a registry with mock processors under test compound keys.
    fn mock_registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        registry.register("test:echo", Box::new(EchoProcessor));
        registry.register("test:uppercase", Box::new(UpperCaseProcessor));
        registry.register("test:fail", Box::new(FailProcessor));
        registry.register("test:slow", Box::new(SlowProcessor));
        registry.register("test:double", Box::new(DoubleProcessor));
        registry
    }

    /// Build a registry that maps REAL recipe operation keys to mock processors.
    /// This lets us test real recipe JSON structures without needing actual
    /// image/CSV/file processors — we verify the orchestration, not the processing.
    fn recipe_registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        // Image operations → EchoProcessor (preserves files, verifies routing).
        registry.register("image:compress", Box::new(EchoProcessor));
        registry.register("image:resize", Box::new(EchoProcessor));
        registry.register("image:convert", Box::new(EchoProcessor));
        // CSV operations → EchoProcessor.
        registry.register("spreadsheet:clean", Box::new(EchoProcessor));
        registry.register("spreadsheet:rename", Box::new(EchoProcessor));
        // File operations → UpperCaseProcessor (verifies transformation happened).
        registry.register("file-system:rename", Box::new(UpperCaseProcessor));
        registry
    }

    /// Parse a JSON string into a PipelineDefinition.
    fn parse_def(json: &str) -> PipelineDefinition {
        serde_json::from_str(json).unwrap()
    }

    /// A fake time source that always returns 1000ms.
    /// Keeps tests deterministic — no real clock needed.
    fn fake_now() -> u64 {
        1000
    }

    // =========================================================================
    // Basic Execution Tests
    // =========================================================================

    #[test]
    fn test_empty_definition_no_files() {
        let def = parse_def(r#"{ "nodes": [] }"#);
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let result = execute_pipeline(&def, vec![], &registry, &reporter, fake_now).unwrap();

        assert!(result.files.is_empty());
    }

    #[test]
    fn test_io_only_pipeline_is_passthrough() {
        // A pipeline with ONLY input + output nodes = passthrough.
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("test.txt", b"hello")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // Files pass through unchanged.
        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].name, "test.txt");
        assert_eq!(result.files[0].data, b"hello");
    }

    #[test]
    fn test_single_node_single_file() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("test.txt", b"hello world")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].data, b"hello world");
    }

    #[test]
    fn test_single_node_multiple_files() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![
            make_file("a.txt", b"aaa"),
            make_file("b.txt", b"bbb"),
            make_file("c.txt", b"ccc"),
        ];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        assert_eq!(result.files.len(), 3);
        assert_eq!(result.files[0].data, b"aaa");
        assert_eq!(result.files[1].data, b"bbb");
        assert_eq!(result.files[2].data, b"ccc");
    }

    #[test]
    fn test_two_sequential_nodes() {
        // uppercase then echo — verify chaining works.
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "upper", "type": "test", "params": { "operation": "uppercase" } },
                { "id": "echo", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("test.txt", b"hello")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // Filename should be uppercased by the first node, then echoed by the second.
        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].name, "TEST.TXT");
    }

    #[test]
    fn test_double_processor_increases_file_count() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "dbl", "type": "test", "params": { "operation": "double" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("test.txt", b"data")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // DoubleProcessor returns 2 files per input.
        assert_eq!(result.files.len(), 2);
        assert_eq!(result.files[0].name, "test.txt-a");
        assert_eq!(result.files[1].name, "test.txt-b");
    }

    #[test]
    fn test_empty_files_array() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let result = execute_pipeline(&def, vec![], &registry, &reporter, fake_now).unwrap();
        assert!(result.files.is_empty());
    }

    #[test]
    fn test_100_files_batch() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files: Vec<PipelineFile> = (0..100)
            .map(|i| make_file(&format!("file-{}.txt", i), b"data"))
            .collect();

        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();
        assert_eq!(result.files.len(), 100);
    }

    // =========================================================================
    // Container Node Tests
    // =========================================================================

    #[test]
    fn test_loop_node_runs_children_per_file() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                {
                    "id": "loop-1", "type": "loop",
                    "children": [
                        { "id": "child", "type": "test", "params": { "operation": "uppercase" } }
                    ]
                },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![
            make_file("a.txt", b"aaa"),
            make_file("b.txt", b"bbb"),
        ];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // Each file processed individually through the loop.
        assert_eq!(result.files.len(), 2);
        assert_eq!(result.files[0].name, "A.TXT");
        assert_eq!(result.files[1].name, "B.TXT");
    }

    #[test]
    fn test_group_node_runs_children_on_full_batch() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                {
                    "id": "group-1", "type": "group",
                    "children": [
                        { "id": "child", "type": "test", "params": { "operation": "uppercase" } }
                    ]
                },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![
            make_file("a.txt", b"aaa"),
            make_file("b.txt", b"bbb"),
        ];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        assert_eq!(result.files.len(), 2);
        assert_eq!(result.files[0].name, "A.TXT");
        assert_eq!(result.files[1].name, "B.TXT");
    }

    #[test]
    fn test_nested_loop_inside_group() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                {
                    "id": "group-1", "type": "group",
                    "children": [
                        {
                            "id": "loop-1", "type": "loop",
                            "children": [
                                { "id": "proc", "type": "test", "params": { "operation": "echo" } }
                            ]
                        }
                    ]
                },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![
            make_file("a.txt", b"aaa"),
            make_file("b.txt", b"bbb"),
        ];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        assert_eq!(result.files.len(), 2);
        assert_eq!(result.files[0].name, "a.txt");
    }

    #[test]
    fn test_container_with_no_children_is_passthrough() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "loop-1", "type": "loop", "children": [] },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("test.txt", b"hello")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].name, "test.txt");
    }

    // =========================================================================
    // Progress Event Tests
    // =========================================================================

    #[test]
    fn test_single_node_emits_correct_event_sequence() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![make_file("test.txt", b"hello")];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();

        // Expected sequence:
        // 1. PipelineStarted
        // 2. NodeStarted
        // 3. FileProgress (0%)
        // 4. FileProgress (100%)
        // 5. NodeCompleted
        // 6. PipelineCompleted
        assert!(events.len() >= 4, "Expected at least 4 events, got {}", events.len());

        // First event is PipelineStarted.
        assert!(matches!(events[0], PipelineEvent::PipelineStarted { .. }));

        // Second event is NodeStarted.
        assert!(matches!(events[1], PipelineEvent::NodeStarted { .. }));

        // Last event is PipelineCompleted.
        assert!(matches!(
            events.last().unwrap(),
            PipelineEvent::PipelineCompleted { .. }
        ));
    }

    #[test]
    fn test_multi_node_events_in_order() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "n1", "type": "test", "params": { "operation": "echo" } },
                { "id": "n2", "type": "test", "params": { "operation": "uppercase" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![make_file("test.txt", b"hello")];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();

        // Collect NodeStarted events.
        let node_started: Vec<&PipelineEvent> = events
            .iter()
            .filter(|e| matches!(e, PipelineEvent::NodeStarted { .. }))
            .collect();

        assert_eq!(node_started.len(), 2, "Should have 2 NodeStarted events");

        // First NodeStarted should be for n1.
        if let PipelineEvent::NodeStarted { node_id, node_index, .. } = &node_started[0] {
            assert_eq!(node_id, "n1");
            assert_eq!(*node_index, 0);
        }

        // Second NodeStarted should be for n2.
        if let PipelineEvent::NodeStarted { node_id, node_index, .. } = &node_started[1] {
            assert_eq!(node_id, "n2");
            assert_eq!(*node_index, 1);
        }
    }

    #[test]
    fn test_file_progress_includes_correct_indices() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![
            make_file("a.txt", b"aaa"),
            make_file("b.txt", b"bbb"),
            make_file("c.txt", b"ccc"),
        ];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();
        let progress_events: Vec<&PipelineEvent> = events
            .iter()
            .filter(|e| matches!(e, PipelineEvent::FileProgress { percent: 0, .. }))
            .collect();

        // Should have 3 FileProgress(0%) events — one per file.
        assert_eq!(progress_events.len(), 3);

        // Verify file indices.
        for (i, event) in progress_events.iter().enumerate() {
            if let PipelineEvent::FileProgress {
                file_index,
                total_files,
                ..
            } = event
            {
                assert_eq!(*file_index, i);
                assert_eq!(*total_files, 3);
            }
        }
    }

    // =========================================================================
    // Error Handling Tests
    // =========================================================================

    #[test]
    fn test_unknown_node_type_returns_error() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "unknown", "params": { "operation": "magic" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("test.txt", b"hello")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("unknown:magic"),
            "Error should include the compound key: {}",
            err
        );
    }

    #[test]
    fn test_processor_failure_emits_node_failed() {
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "test", "params": { "operation": "fail" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![make_file("test.txt", b"hello")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

        assert!(result.is_err());

        let events = recorder.events();

        // Should have NodeFailed and PipelineFailed events.
        let has_node_failed = events.iter().any(|e| matches!(e, PipelineEvent::NodeFailed { .. }));
        let has_pipeline_failed = events
            .iter()
            .any(|e| matches!(e, PipelineEvent::PipelineFailed { .. }));

        assert!(has_node_failed, "Should emit NodeFailed event");
        assert!(has_pipeline_failed, "Should emit PipelineFailed event");
    }

    #[test]
    fn test_error_mid_pipeline_stops_execution() {
        // First node succeeds (echo), second fails, third never runs.
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "n1", "type": "test", "params": { "operation": "echo" } },
                { "id": "n2", "type": "test", "params": { "operation": "fail" } },
                { "id": "n3", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![make_file("test.txt", b"hello")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

        assert!(result.is_err());

        let events = recorder.events();

        // n1 should complete, n2 should fail, n3 should never start.
        let completed: Vec<&PipelineEvent> = events
            .iter()
            .filter(|e| matches!(e, PipelineEvent::NodeCompleted { .. }))
            .collect();

        assert_eq!(completed.len(), 1, "Only n1 should complete");

        if let PipelineEvent::NodeCompleted { node_id, .. } = completed[0] {
            assert_eq!(node_id, "n1");
        }

        // n3 should never have a NodeStarted event.
        let n3_started = events.iter().any(|e| {
            matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "n3")
        });
        assert!(!n3_started, "n3 should never start");
    }

    // =========================================================================
    // Thorough Event Reporting Tests
    // =========================================================================
    //
    // These tests assert the EXACT sequence and content of PipelineEvents
    // emitted during execution. They verify what the UI will see — progress
    // bar accuracy, node highlighting, duration tracking, and error messages.

    #[test]
    fn test_single_file_full_event_sequence() {
        // Verify the exact event sequence for one file through one node.
        // This is the simplest case — the "golden path" the UI depends on.
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![make_file("photo.jpg", b"image-data")];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();

        // Assert the exact sequence: PipelineStarted → NodeStarted →
        // FileProgress(0%) → FileProgress(100%) → NodeCompleted → PipelineCompleted
        assert_eq!(events.len(), 6, "Expected exactly 6 events for 1 file, 1 node");

        // Event 0: PipelineStarted with correct counts.
        if let PipelineEvent::PipelineStarted { total_nodes, total_files } = &events[0] {
            assert_eq!(*total_nodes, 1, "1 processing node (I/O excluded)");
            assert_eq!(*total_files, 1);
        } else {
            panic!("Event 0 should be PipelineStarted, got {:?}", events[0]);
        }

        // Event 1: NodeStarted with correct ID and index.
        if let PipelineEvent::NodeStarted { node_id, node_index, total_nodes, node_type } = &events[1] {
            assert_eq!(node_id, "proc");
            assert_eq!(*node_index, 0);
            assert_eq!(*total_nodes, 1);
            assert_eq!(node_type, "test");
        } else {
            panic!("Event 1 should be NodeStarted, got {:?}", events[1]);
        }

        // Event 2: FileProgress at 0% (processing starts).
        if let PipelineEvent::FileProgress { node_id, file_index, total_files, percent, message } = &events[2] {
            assert_eq!(node_id, "proc");
            assert_eq!(*file_index, 0);
            assert_eq!(*total_files, 1);
            assert_eq!(*percent, 0);
            assert!(message.contains("photo.jpg"), "Message should include filename: {}", message);
        } else {
            panic!("Event 2 should be FileProgress(0%), got {:?}", events[2]);
        }

        // Event 3: FileProgress at 100% (processing done).
        if let PipelineEvent::FileProgress { percent, message, .. } = &events[3] {
            assert_eq!(*percent, 100);
            assert!(message.contains("photo.jpg"), "Message should include filename: {}", message);
        } else {
            panic!("Event 3 should be FileProgress(100%), got {:?}", events[3]);
        }

        // Event 4: NodeCompleted with correct file count and duration.
        if let PipelineEvent::NodeCompleted { node_id, files_processed, duration_ms } = &events[4] {
            assert_eq!(node_id, "proc");
            assert_eq!(*files_processed, 1);
            // Duration should be non-negative (we use fake_now so it's 0).
            assert_eq!(*duration_ms, 0, "fake_now produces 0ms duration");
        } else {
            panic!("Event 4 should be NodeCompleted, got {:?}", events[4]);
        }

        // Event 5: PipelineCompleted with correct totals.
        if let PipelineEvent::PipelineCompleted { total_files_processed, duration_ms } = &events[5] {
            assert_eq!(*total_files_processed, 1);
            assert_eq!(*duration_ms, 0);
        } else {
            panic!("Event 5 should be PipelineCompleted, got {:?}", events[5]);
        }
    }

    #[test]
    fn test_multi_file_progress_tracking() {
        // 3 files through 1 node. Verify each file gets its own
        // FileProgress(0%) and FileProgress(100%) with correct indices.
        // This is what powers "File 2/3 at 50%" in the UI progress bar.
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![
            make_file("a.jpg", b"aaa"),
            make_file("b.jpg", b"bbb"),
            make_file("c.jpg", b"ccc"),
        ];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();

        // Collect all FileProgress events grouped by file_index.
        let progress_events: Vec<&PipelineEvent> = events
            .iter()
            .filter(|e| matches!(e, PipelineEvent::FileProgress { .. }))
            .collect();

        // 3 files × 2 progress events each (0% + 100%) = 6 FileProgress events.
        assert_eq!(progress_events.len(), 6, "3 files × (0% + 100%) = 6 progress events");

        // Verify the file_index and total_files are correct for each event.
        // Events should come in pairs: (file 0, 0%), (file 0, 100%),
        // (file 1, 0%), (file 1, 100%), (file 2, 0%), (file 2, 100%).
        let expected_sequence = [
            (0, 0, "a.jpg"),
            (0, 100, "a.jpg"),
            (1, 0, "b.jpg"),
            (1, 100, "b.jpg"),
            (2, 0, "c.jpg"),
            (2, 100, "c.jpg"),
        ];

        for (i, (expected_idx, expected_pct, expected_name)) in expected_sequence.iter().enumerate() {
            if let PipelineEvent::FileProgress {
                file_index,
                total_files,
                percent,
                message,
                ..
            } = &progress_events[i]
            {
                assert_eq!(
                    *file_index, *expected_idx,
                    "Event {} file_index: expected {}, got {}",
                    i, expected_idx, file_index
                );
                assert_eq!(*total_files, 3, "total_files should always be 3");
                assert_eq!(
                    *percent, *expected_pct as u32,
                    "Event {} percent: expected {}, got {}",
                    i, expected_pct, percent
                );
                assert!(
                    message.contains(expected_name),
                    "Event {} message should contain '{}': {}",
                    i, expected_name, message
                );
            } else {
                panic!("Expected FileProgress at index {}", i);
            }
        }

        // PipelineCompleted should report total files processed = 3.
        if let PipelineEvent::PipelineCompleted { total_files_processed, .. } = events.last().unwrap() {
            assert_eq!(*total_files_processed, 3);
        } else {
            panic!("Last event should be PipelineCompleted");
        }
    }

    #[test]
    fn test_multi_node_multi_file_progress() {
        // 2 files through 2 nodes. Verify that file_index resets per node
        // and total_files reflects the files available at each stage.
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "n1", "type": "test", "params": { "operation": "echo" } },
                { "id": "n2", "type": "test", "params": { "operation": "uppercase" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![
            make_file("x.txt", b"hello"),
            make_file("y.txt", b"world"),
        ];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();

        // Collect FileProgress events per node.
        let n1_progress: Vec<&PipelineEvent> = events
            .iter()
            .filter(|e| matches!(e, PipelineEvent::FileProgress { node_id, .. } if node_id == "n1"))
            .collect();
        let n2_progress: Vec<&PipelineEvent> = events
            .iter()
            .filter(|e| matches!(e, PipelineEvent::FileProgress { node_id, .. } if node_id == "n2"))
            .collect();

        // Each node processes 2 files → 2 × (0% + 100%) = 4 events per node.
        assert_eq!(n1_progress.len(), 4, "n1 should emit 4 progress events (2 files × 2)");
        assert_eq!(n2_progress.len(), 4, "n2 should emit 4 progress events (2 files × 2)");

        // Verify file_index resets to 0 for the second node.
        if let PipelineEvent::FileProgress { file_index, .. } = n2_progress[0] {
            assert_eq!(*file_index, 0, "file_index should reset to 0 for n2's first file");
        }

        // Verify NodeCompleted reports correct file counts for each node.
        let completed: Vec<&PipelineEvent> = events
            .iter()
            .filter(|e| matches!(e, PipelineEvent::NodeCompleted { .. }))
            .collect();
        assert_eq!(completed.len(), 2);

        for node_completed in &completed {
            if let PipelineEvent::NodeCompleted { files_processed, .. } = node_completed {
                assert_eq!(*files_processed, 2, "Each node processed 2 files");
            }
        }

        // PipelineCompleted should report total_files_processed = 4 (2 files × 2 nodes).
        if let PipelineEvent::PipelineCompleted { total_files_processed, .. } = events.last().unwrap() {
            assert_eq!(*total_files_processed, 4, "2 files × 2 nodes = 4 total processed");
        }
    }

    #[test]
    fn test_node_completed_fields_are_correct() {
        // Verify NodeCompleted reports accurate files_processed and duration.
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![
            make_file("1.txt", b"one"),
            make_file("2.txt", b"two"),
            make_file("3.txt", b"three"),
            make_file("4.txt", b"four"),
            make_file("5.txt", b"five"),
        ];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();

        // Find NodeCompleted for "proc".
        let completed = events
            .iter()
            .find(|e| matches!(e, PipelineEvent::NodeCompleted { node_id, .. } if node_id == "proc"))
            .expect("Should have NodeCompleted for 'proc'");

        if let PipelineEvent::NodeCompleted { node_id, files_processed, .. } = completed {
            assert_eq!(node_id, "proc");
            assert_eq!(*files_processed, 5, "Should report 5 files processed");
        }
    }

    #[test]
    fn test_error_events_contain_useful_information() {
        // Verify NodeFailed and PipelineFailed events include the error
        // message and correct node_id so the UI can show meaningful errors.
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "broken", "type": "test", "params": { "operation": "fail" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![make_file("test.txt", b"hello")];
        let _ = execute_pipeline(&def, files, &registry, &reporter, fake_now);

        let events = recorder.events();

        // Find NodeFailed event and verify its fields.
        let node_failed = events
            .iter()
            .find(|e| matches!(e, PipelineEvent::NodeFailed { .. }))
            .expect("Should have NodeFailed event");

        if let PipelineEvent::NodeFailed { node_id, error } = node_failed {
            assert_eq!(node_id, "broken", "NodeFailed should reference the failing node");
            assert!(
                error.contains("intentional test failure"),
                "Error message should be descriptive: {}",
                error
            );
        }

        // Find PipelineFailed event and verify it also references the failing node.
        let pipeline_failed = events
            .iter()
            .find(|e| matches!(e, PipelineEvent::PipelineFailed { .. }))
            .expect("Should have PipelineFailed event");

        if let PipelineEvent::PipelineFailed { node_id, error } = pipeline_failed {
            assert_eq!(node_id, "broken", "PipelineFailed should reference the failing node");
            assert!(!error.is_empty(), "Error message should not be empty");
        }
    }

    #[test]
    fn test_container_node_event_nesting() {
        // Group → Loop → Processor: verify events nest correctly.
        // The UI uses this to know which node is "active" at each level.
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                {
                    "id": "group1", "type": "group",
                    "nodes": [{
                        "id": "loop1", "type": "loop",
                        "parameters": { "mode": "forEach" },
                        "nodes": [{
                            "id": "leaf", "type": "test",
                            "params": { "operation": "echo" }
                        }]
                    }]
                },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![
            make_file("a.txt", b"aaa"),
            make_file("b.txt", b"bbb"),
        ];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();

        // Collect all NodeStarted events in order.
        let started: Vec<(&str, &str)> = events
            .iter()
            .filter_map(|e| match e {
                PipelineEvent::NodeStarted { node_id, node_type, .. } => {
                    Some((node_id.as_str(), node_type.as_str()))
                }
                _ => None,
            })
            .collect();

        // Group starts first, then loop, then leaf (once per file).
        assert!(started.len() >= 4, "Expected at least 4 NodeStarted: group + loop + 2× leaf, got {}", started.len());
        assert_eq!(started[0], ("group1", "group"), "Group should start first");
        assert_eq!(started[1], ("loop1", "loop"), "Loop should start second");
        // Leaf starts once per file inside the loop.
        assert_eq!(started[2].0, "leaf", "Leaf should start for first file");
        assert_eq!(started[3].0, "leaf", "Leaf should start for second file");

        // Verify NodeCompleted nesting: leaf completes before loop, loop before group.
        let completed_ids: Vec<&str> = events
            .iter()
            .filter_map(|e| match e {
                PipelineEvent::NodeCompleted { node_id, .. } => Some(node_id.as_str()),
                _ => None,
            })
            .collect();

        // Leaf completes twice (per file), then loop, then group.
        assert!(completed_ids.len() >= 4, "Expected at least 4 NodeCompleted events");
        assert_eq!(completed_ids[0], "leaf", "First leaf completes");
        assert_eq!(completed_ids[1], "leaf", "Second leaf completes");

        // FileProgress events should reference the leaf node (the processor).
        let leaf_progress: Vec<&PipelineEvent> = events
            .iter()
            .filter(|e| matches!(e, PipelineEvent::FileProgress { node_id, .. } if node_id == "leaf"))
            .collect();
        assert_eq!(leaf_progress.len(), 4, "2 files × (0% + 100%) = 4 leaf progress events");
    }

    #[test]
    fn test_pipeline_started_excludes_io_nodes() {
        // PipelineStarted.total_nodes should NOT count input/output nodes.
        // The UI uses this to calculate "Node 2 of 5" progress indicators.
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "n1", "type": "test", "params": { "operation": "echo" } },
                { "id": "n2", "type": "test", "params": { "operation": "uppercase" } },
                { "id": "n3", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![make_file("test.txt", b"data")];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();

        // PipelineStarted should report 3 nodes (not 5).
        if let PipelineEvent::PipelineStarted { total_nodes, total_files } = &events[0] {
            assert_eq!(*total_nodes, 3, "3 processing nodes, 2 I/O excluded");
            assert_eq!(*total_files, 1);
        } else {
            panic!("First event should be PipelineStarted");
        }

        // Each NodeStarted should have total_nodes = 3.
        for event in &events {
            if let PipelineEvent::NodeStarted { total_nodes, .. } = event {
                assert_eq!(*total_nodes, 3, "NodeStarted.total_nodes should match pipeline total");
            }
        }
    }

    #[test]
    fn test_empty_files_emit_pipeline_events() {
        // Even with 0 files, PipelineStarted and PipelineCompleted should
        // still fire. The UI needs these to transition state correctly.
        let def = parse_def(
            r#"{
            "nodes": [
                { "id": "in", "type": "input" },
                { "id": "proc", "type": "test", "params": { "operation": "echo" } },
                { "id": "out", "type": "output" }
            ]
        }"#,
        );
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files: Vec<PipelineFile> = vec![];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();

        // Should still get PipelineStarted (total_files: 0).
        assert!(matches!(
            &events[0],
            PipelineEvent::PipelineStarted { total_files: 0, .. }
        ));

        // Should still get PipelineCompleted.
        assert!(matches!(
            events.last().unwrap(),
            PipelineEvent::PipelineCompleted { total_files_processed: 0, .. }
        ));
    }

    // =========================================================================
    // Real Recipe Structure Tests
    // =========================================================================
    //
    // These tests use the EXACT JSON shapes produced by the TypeScript
    // recipe definitions (with "nodes", "parameters", "version", "position",
    // "metadata", "inputPorts", "outputPorts", "edges" — all the fields
    // the Rust struct silently ignores plus the aliased field names).
    //
    // Mock processors verify orchestration — we're testing that the executor
    // correctly walks container nodes, chains outputs, and skips I/O nodes.
    // The actual image/CSV/file processing is tested separately in each
    // node crate's own test suite.

    /// Helper: JSON for compress-images recipe structure.
    /// Compositional: Group → Input → Group("Batch Compress") → Loop → [image:compress] → Output
    /// This mirrors how users build recipes — the batch processing logic is a
    /// reusable sub-recipe (group node) that could be shared independently.
    fn compress_images_json() -> &'static str {
        r#"{
            "nodes": [
                {
                    "id": "input", "type": "input", "version": "1.0.0",
                    "name": "Input Files", "position": {"x": 0, "y": 100},
                    "metadata": {},
                    "parameters": { "mode": "file-upload" },
                    "inputPorts": [], "outputPorts": [{"id": "out-1", "name": "files"}]
                },
                {
                    "id": "batch-compress", "type": "group", "version": "1.0.0",
                    "name": "Batch Compress", "position": {"x": 250, "y": 100},
                    "metadata": { "description": "Reusable sub-recipe: loops over files and compresses each one." },
                    "parameters": {},
                    "inputPorts": [{"id": "in-1", "name": "files"}],
                    "outputPorts": [{"id": "out-1", "name": "files"}],
                    "nodes": [
                        {
                            "id": "compress-loop", "type": "loop", "version": "1.0.0",
                            "name": "Compress Each Image", "position": {"x": 0, "y": 0},
                            "metadata": {},
                            "parameters": { "mode": "forEach" },
                            "inputPorts": [{"id": "in-1", "name": "items"}], "outputPorts": [],
                            "nodes": [
                                {
                                    "id": "compress-image", "type": "image", "version": "1.0.0",
                                    "name": "Compress Image", "position": {"x": 0, "y": 0},
                                    "metadata": {},
                                    "parameters": { "operation": "compress", "quality": 80 },
                                    "inputPorts": [], "outputPorts": []
                                }
                            ],
                            "edges": []
                        }
                    ],
                    "edges": []
                },
                {
                    "id": "output", "type": "output", "version": "1.0.0",
                    "name": "Compressed Images", "position": {"x": 500, "y": 100},
                    "metadata": {},
                    "parameters": { "mode": "download", "zip": true },
                    "inputPorts": [{"id": "in-1", "name": "files"}], "outputPorts": []
                }
            ],
            "edges": [
                {"id": "e1", "source": "input", "target": "batch-compress"},
                {"id": "e2", "source": "batch-compress", "target": "output"}
            ]
        }"#
    }

    /// Helper: JSON for clean-csv recipe structure.
    /// Compositional: Group → Input → Group("CSV Cleaner") → [spreadsheet:clean] → Output
    /// The CSV cleaner is a reusable sub-recipe containing the processor directly
    /// (no loop — CSV operations process the whole file at once).
    fn clean_csv_json() -> &'static str {
        r#"{
            "nodes": [
                {
                    "id": "input", "type": "input", "version": "1.0.0",
                    "name": "Input Files", "position": {"x": 0, "y": 100},
                    "metadata": {},
                    "parameters": { "mode": "file-upload" },
                    "inputPorts": [], "outputPorts": [{"id": "out-1", "name": "files"}]
                },
                {
                    "id": "csv-cleaner", "type": "group", "version": "1.0.0",
                    "name": "CSV Cleaner", "position": {"x": 250, "y": 100},
                    "metadata": { "description": "Reusable sub-recipe: trims whitespace, removes empty rows, deduplicates." },
                    "parameters": {},
                    "inputPorts": [{"id": "in-1", "name": "files"}],
                    "outputPorts": [{"id": "out-1", "name": "files"}],
                    "nodes": [
                        {
                            "id": "clean", "type": "spreadsheet", "version": "1.0.0",
                            "name": "Clean CSV", "position": {"x": 0, "y": 0},
                            "metadata": {},
                            "parameters": {
                                "operation": "clean",
                                "trimWhitespace": true,
                                "removeEmptyRows": true,
                                "removeDuplicates": true
                            },
                            "inputPorts": [{"id": "in-1", "name": "files"}],
                            "outputPorts": [{"id": "out-1", "name": "files"}]
                        }
                    ],
                    "edges": []
                },
                {
                    "id": "output", "type": "output", "version": "1.0.0",
                    "name": "Cleaned CSV", "position": {"x": 500, "y": 100},
                    "metadata": {},
                    "parameters": { "mode": "download" },
                    "inputPorts": [{"id": "in-1", "name": "files"}], "outputPorts": []
                }
            ],
            "edges": [
                {"id": "e1", "source": "input", "target": "csv-cleaner"},
                {"id": "e2", "source": "csv-cleaner", "target": "output"}
            ]
        }"#
    }

    /// Helper: JSON for rename-files recipe structure.
    /// Compositional: Group → Input → Group("Batch Rename") → Loop → [file-system:rename] → Output
    /// Same pattern as image recipes — the batch rename logic is a reusable sub-recipe.
    fn rename_files_json() -> &'static str {
        r#"{
            "nodes": [
                {
                    "id": "input", "type": "input", "version": "1.0.0",
                    "name": "Input Files", "position": {"x": 0, "y": 100},
                    "metadata": {},
                    "parameters": { "mode": "file-upload" },
                    "inputPorts": [], "outputPorts": [{"id": "out-1", "name": "files"}]
                },
                {
                    "id": "batch-rename", "type": "group", "version": "1.0.0",
                    "name": "Batch Rename", "position": {"x": 250, "y": 100},
                    "metadata": { "description": "Reusable sub-recipe: loops over files and renames each one." },
                    "parameters": {},
                    "inputPorts": [{"id": "in-1", "name": "files"}],
                    "outputPorts": [{"id": "out-1", "name": "files"}],
                    "nodes": [
                        {
                            "id": "rename-loop", "type": "loop", "version": "1.0.0",
                            "name": "Rename Each File", "position": {"x": 0, "y": 0},
                            "metadata": {},
                            "parameters": { "mode": "forEach" },
                            "inputPorts": [{"id": "in-1", "name": "items"}], "outputPorts": [],
                            "nodes": [
                                {
                                    "id": "rename-file", "type": "file-system", "version": "1.0.0",
                                    "name": "Rename File", "position": {"x": 0, "y": 0},
                                    "metadata": {},
                                    "parameters": { "operation": "rename", "prefix": "renamed-" },
                                    "inputPorts": [], "outputPorts": []
                                }
                            ],
                            "edges": []
                        }
                    ],
                    "edges": []
                },
                {
                    "id": "output", "type": "output", "version": "1.0.0",
                    "name": "Renamed Files", "position": {"x": 500, "y": 100},
                    "metadata": {},
                    "parameters": { "mode": "download", "zip": true },
                    "inputPorts": [{"id": "in-1", "name": "files"}], "outputPorts": []
                }
            ],
            "edges": [
                {"id": "e1", "source": "input", "target": "batch-rename"},
                {"id": "e2", "source": "batch-rename", "target": "output"}
            ]
        }"#
    }

    // --- Image Recipe Execution ---

    #[test]
    fn test_recipe_compress_images_single_file() {
        let def = parse_def(compress_images_json());
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("photo.jpg", b"jpeg-data")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // Loop runs once (1 file), EchoProcessor passes it through.
        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].name, "photo.jpg");
        assert_eq!(result.files[0].data, b"jpeg-data");
    }

    #[test]
    fn test_recipe_compress_images_multiple_files() {
        let def = parse_def(compress_images_json());
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![
            make_file("photo1.jpg", b"data1"),
            make_file("photo2.png", b"data2"),
            make_file("photo3.webp", b"data3"),
            make_file("photo4.jpg", b"data4"),
            make_file("photo5.png", b"data5"),
        ];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // Loop runs 5 times (once per file).
        assert_eq!(result.files.len(), 5);
        assert_eq!(result.files[0].name, "photo1.jpg");
        assert_eq!(result.files[4].name, "photo5.png");
    }

    #[test]
    fn test_recipe_resize_images() {
        // Compositional: Input → Group("Batch Resize") → Loop → [image:resize] → Output
        let json = r#"{
            "nodes": [
                { "id": "input", "type": "input", "parameters": {} },
                {
                    "id": "batch-resize", "type": "group", "parameters": {},
                    "nodes": [
                        {
                            "id": "resize-loop", "type": "loop",
                            "parameters": { "mode": "forEach" },
                            "nodes": [
                                {
                                    "id": "resize-image", "type": "image",
                                    "parameters": { "operation": "resize", "width": 200 }
                                }
                            ]
                        }
                    ]
                },
                { "id": "output", "type": "output", "parameters": {} }
            ]
        }"#;

        let def = parse_def(json);
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![
            make_file("a.jpg", b"img-a"),
            make_file("b.jpg", b"img-b"),
            make_file("c.jpg", b"img-c"),
        ];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        assert_eq!(result.files.len(), 3);
    }

    #[test]
    fn test_recipe_convert_image_format() {
        // Compositional: Input → Group("Batch Convert") → Loop → [image:convert] → Output
        let json = r#"{
            "nodes": [
                { "id": "input", "type": "input", "parameters": {} },
                {
                    "id": "batch-convert", "type": "group", "parameters": {},
                    "nodes": [
                        {
                            "id": "convert-loop", "type": "loop",
                            "parameters": { "mode": "forEach" },
                            "nodes": [
                                {
                                    "id": "convert-image", "type": "image",
                                    "parameters": { "operation": "convert", "format": "webp" }
                                }
                            ]
                        }
                    ]
                },
                { "id": "output", "type": "output", "parameters": {} }
            ]
        }"#;

        let def = parse_def(json);
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("photo.jpg", b"jpeg"), make_file("icon.png", b"png")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        assert_eq!(result.files.len(), 2);
    }

    // --- CSV Recipe Execution ---

    #[test]
    fn test_recipe_clean_csv_single_file() {
        let def = parse_def(clean_csv_json());
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("data.csv", b"name,age\nAlice,30\n")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // Flat pipeline: one processor node, file passes through.
        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].name, "data.csv");
    }

    #[test]
    fn test_recipe_rename_csv_columns() {
        // Compositional: Input → Group("Column Renamer") → [spreadsheet:rename] → Output
        let json = r#"{
            "nodes": [
                { "id": "input", "type": "input", "parameters": {} },
                {
                    "id": "column-renamer", "type": "group", "parameters": {},
                    "nodes": [
                        {
                            "id": "rename-columns", "type": "spreadsheet",
                            "parameters": { "operation": "rename", "columns": {} }
                        }
                    ]
                },
                { "id": "output", "type": "output", "parameters": {} }
            ]
        }"#;

        let def = parse_def(json);
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("data.csv", b"old_name\nvalue\n")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        assert_eq!(result.files.len(), 1);
    }

    // --- File System Recipe Execution ---

    #[test]
    fn test_recipe_rename_files() {
        let def = parse_def(rename_files_json());
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![
            make_file("report.pdf", b"pdf-data"),
            make_file("notes.txt", b"text-data"),
            make_file("photo.jpg", b"img-data"),
            make_file("data.csv", b"csv-data"),
        ];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // Loop runs 4 times, UpperCaseProcessor uppercases filenames.
        assert_eq!(result.files.len(), 4);
        assert_eq!(result.files[0].name, "REPORT.PDF");
        assert_eq!(result.files[1].name, "NOTES.TXT");
        assert_eq!(result.files[2].name, "PHOTO.JPG");
        assert_eq!(result.files[3].name, "DATA.CSV");
    }

    // --- Nested Container Tests (Synthetic Recipes) ---

    #[test]
    fn test_group_containing_group_containing_loop() {
        // Group → Group → Loop → EchoProcessor. 3 levels deep.
        let json = r#"{
            "nodes": [
                {
                    "id": "outer", "type": "group",
                    "parameters": {},
                    "nodes": [
                        {
                            "id": "inner", "type": "group",
                            "parameters": {},
                            "nodes": [
                                {
                                    "id": "the-loop", "type": "loop",
                                    "parameters": { "mode": "forEach" },
                                    "nodes": [
                                        {
                                            "id": "proc", "type": "image",
                                            "parameters": { "operation": "compress" }
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let def = parse_def(json);
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![
            make_file("a.jpg", b"aaa"),
            make_file("b.jpg", b"bbb"),
            make_file("c.jpg", b"ccc"),
        ];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // Files pass through all 3 container levels to the processor.
        assert_eq!(result.files.len(), 3);
        assert_eq!(result.files[0].name, "a.jpg");
    }

    #[test]
    fn test_multiple_processors_inside_loop() {
        // Loop → [echo, then uppercase]. Two sequential processors per iteration.
        let json = r#"{
            "nodes": [
                {
                    "id": "the-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [
                        { "id": "step1", "type": "test", "params": { "operation": "echo" } },
                        { "id": "step2", "type": "test", "params": { "operation": "uppercase" } }
                    ]
                }
            ]
        }"#;

        let def = parse_def(json);
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![
            make_file("a.txt", b"aaa"),
            make_file("b.txt", b"bbb"),
        ];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // Each file goes through echo then uppercase inside the loop.
        assert_eq!(result.files.len(), 2);
        assert_eq!(result.files[0].name, "A.TXT");
        assert_eq!(result.files[1].name, "B.TXT");
    }

    #[test]
    fn test_sequential_loops_in_pipeline() {
        // Loop1(echo) → Loop2(uppercase). Two loops in sequence.
        let json = r#"{
            "nodes": [
                {
                    "id": "loop1", "type": "loop",
                    "params": { "mode": "forEach" },
                    "children": [
                        { "id": "echo", "type": "test", "params": { "operation": "echo" } }
                    ]
                },
                {
                    "id": "loop2", "type": "loop",
                    "params": { "mode": "forEach" },
                    "children": [
                        { "id": "upper", "type": "test", "params": { "operation": "uppercase" } }
                    ]
                }
            ]
        }"#;

        let def = parse_def(json);
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("file.txt", b"data")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // Passes through loop1 (echo, unchanged) then loop2 (uppercase).
        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].name, "FILE.TXT");
    }

    #[test]
    fn test_four_levels_deep_nesting() {
        // Group → Group → Group → Loop → uppercase. Maximum nesting.
        let json = r#"{
            "nodes": [
                {
                    "id": "g1", "type": "group", "parameters": {},
                    "nodes": [
                        {
                            "id": "g2", "type": "group", "parameters": {},
                            "nodes": [
                                {
                                    "id": "g3", "type": "group", "parameters": {},
                                    "nodes": [
                                        {
                                            "id": "the-loop", "type": "loop",
                                            "parameters": { "mode": "forEach" },
                                            "nodes": [
                                                {
                                                    "id": "proc", "type": "test",
                                                    "params": { "operation": "uppercase" }
                                                }
                                            ]
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let def = parse_def(json);
        let registry = mock_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("deep.txt", b"deep")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].name, "DEEP.TXT");
    }

    // --- Edge Cases with Recipe Structures ---

    #[test]
    fn test_recipe_with_only_io_nodes_passthrough() {
        // Recipe with only input + output. No processing nodes.
        let json = r#"{
            "nodes": [
                {
                    "id": "input", "type": "input", "version": "1.0.0",
                    "name": "Input", "position": {"x": 0, "y": 0}, "metadata": {},
                    "parameters": {}, "inputPorts": [], "outputPorts": []
                },
                {
                    "id": "output", "type": "output", "version": "1.0.0",
                    "name": "Output", "position": {"x": 0, "y": 0}, "metadata": {},
                    "parameters": {}, "inputPorts": [], "outputPorts": []
                }
            ]
        }"#;

        let def = parse_def(json);
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("test.txt", b"hello")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        assert_eq!(result.files.len(), 1);
        assert_eq!(result.files[0].data, b"hello");
    }

    #[test]
    fn test_recipe_empty_files_no_error() {
        let def = parse_def(compress_images_json());
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let result = execute_pipeline(&def, vec![], &registry, &reporter, fake_now).unwrap();
        assert!(result.files.is_empty());
    }

    #[test]
    fn test_recipe_container_io_children_skipped() {
        // A loop containing input + output + processor. I/O children are skipped.
        let json = r#"{
            "nodes": [
                {
                    "id": "the-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [
                        { "id": "inner-input", "type": "input", "parameters": {} },
                        {
                            "id": "proc", "type": "image",
                            "parameters": { "operation": "compress" }
                        },
                        { "id": "inner-output", "type": "output", "parameters": {} }
                    ]
                }
            ]
        }"#;

        let def = parse_def(json);
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("photo.jpg", b"data")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        // I/O nodes inside loop are skipped, only the processor runs.
        assert_eq!(result.files.len(), 1);
    }

    // --- Error Cases with Recipe Structures ---

    #[test]
    fn test_recipe_unregistered_operation_inside_loop() {
        // Loop contains a node with an operation that has no processor.
        let json = r#"{
            "nodes": [
                {
                    "id": "the-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [
                        {
                            "id": "bad-node", "type": "spreadsheet",
                            "parameters": { "operation": "pivot" }
                        }
                    ]
                }
            ]
        }"#;

        let def = parse_def(json);
        let registry = recipe_registry();
        let reporter = PipelineReporter::new_noop();

        let files = vec![make_file("data.csv", b"csv-data")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("spreadsheet:pivot"), "Error should name the missing key: {}", err);
    }

    #[test]
    fn test_recipe_failure_inside_nested_container() {
        // Group → Loop → FailProcessor. Error should propagate up.
        let json = r#"{
            "nodes": [
                {
                    "id": "group-1", "type": "group",
                    "parameters": {},
                    "nodes": [
                        {
                            "id": "the-loop", "type": "loop",
                            "parameters": { "mode": "forEach" },
                            "nodes": [
                                {
                                    "id": "fail-proc", "type": "test",
                                    "params": { "operation": "fail" }
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let def = parse_def(json);
        let registry = mock_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![make_file("test.txt", b"data")];
        let result = execute_pipeline(&def, files, &registry, &reporter, fake_now);

        assert!(result.is_err());

        let events = recorder.events();
        let has_pipeline_failed = events
            .iter()
            .any(|e| matches!(e, PipelineEvent::PipelineFailed { .. }));
        assert!(has_pipeline_failed, "Should emit PipelineFailed for nested failure");
    }

    // --- Progress Events with Recipe Structures ---

    #[test]
    fn test_recipe_compress_images_event_sequence() {
        let def = parse_def(compress_images_json());
        let registry = recipe_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![
            make_file("a.jpg", b"aaa"),
            make_file("b.jpg", b"bbb"),
        ];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();

        // Should start with PipelineStarted.
        assert!(matches!(events[0], PipelineEvent::PipelineStarted { .. }));

        // Should end with PipelineCompleted.
        assert!(matches!(events.last().unwrap(), PipelineEvent::PipelineCompleted { .. }));

        // Should have NodeStarted for the batch-compress group (sub-recipe).
        let group_started = events.iter().any(|e| {
            matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "batch-compress")
        });
        assert!(group_started, "Should emit NodeStarted for sub-recipe group node");

        // Should have NodeStarted for the loop inside the sub-recipe.
        let loop_started = events.iter().any(|e| {
            matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "compress-loop")
        });
        assert!(loop_started, "Should emit NodeStarted for loop node inside sub-recipe");

        // Should have NodeStarted for the child processor (runs per file).
        let child_started_count = events
            .iter()
            .filter(|e| {
                matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "compress-image")
            })
            .count();
        assert_eq!(child_started_count, 2, "Child processor should start once per file");

        // Should have NodeCompleted for the child processor (runs per file).
        let child_completed_count = events
            .iter()
            .filter(|e| {
                matches!(e, PipelineEvent::NodeCompleted { node_id, .. } if node_id == "compress-image")
            })
            .count();
        assert_eq!(child_completed_count, 2, "Child processor should complete once per file");
    }

    #[test]
    fn test_recipe_clean_csv_event_sequence() {
        let def = parse_def(clean_csv_json());
        let registry = recipe_registry();
        let recorder = RecordingReporter::new();
        let reporter = recorder.reporter();

        let files = vec![make_file("data.csv", b"csv-content")];
        execute_pipeline(&def, files, &registry, &reporter, fake_now).unwrap();

        let events = recorder.events();

        // PipelineStarted should report the csv-cleaner group as 1 processing
        // node at the top level (I/O nodes excluded). The group's children are
        // counted separately during sub-pipeline execution.
        if let PipelineEvent::PipelineStarted { total_nodes, total_files } = &events[0] {
            assert_eq!(*total_nodes, 1, "1 top-level processing node (csv-cleaner group), I/O excluded");
            assert_eq!(*total_files, 1);
        } else {
            panic!("First event should be PipelineStarted");
        }

        // NodeStarted for the csv-cleaner group + clean processor inside it = 2.
        let node_started_count = events
            .iter()
            .filter(|e| matches!(e, PipelineEvent::NodeStarted { .. }))
            .count();
        assert_eq!(node_started_count, 2, "Group + processor = 2 NodeStarted events");
    }

    // --- Smoke Tests: All 6 Recipes Deserialize ---

    #[test]
    fn test_all_six_recipe_structures_deserialize() {
        // Verify every recipe structure can be parsed without error.
        // All 6 use the compositional pattern: Input → Group(sub-recipe) → Output.
        let recipes = [
            compress_images_json(),
            clean_csv_json(),
            rename_files_json(),
            // Resize: Input → Group → Loop → [image:resize] → Output
            r#"{
                "nodes": [
                    { "id": "in", "type": "input", "parameters": {} },
                    { "id": "batch-resize", "type": "group", "parameters": {}, "nodes": [
                        { "id": "loop", "type": "loop", "parameters": { "mode": "forEach" }, "nodes": [
                            { "id": "proc", "type": "image", "parameters": { "operation": "resize", "width": 200 } }
                        ]}
                    ]},
                    { "id": "out", "type": "output", "parameters": {} }
                ]
            }"#,
            // Convert: Input → Group → Loop → [image:convert] → Output
            r#"{
                "nodes": [
                    { "id": "in", "type": "input", "parameters": {} },
                    { "id": "batch-convert", "type": "group", "parameters": {}, "nodes": [
                        { "id": "loop", "type": "loop", "parameters": { "mode": "forEach" }, "nodes": [
                            { "id": "proc", "type": "image", "parameters": { "operation": "convert", "format": "webp" } }
                        ]}
                    ]},
                    { "id": "out", "type": "output", "parameters": {} }
                ]
            }"#,
            // Rename CSV columns: Input → Group → [spreadsheet:rename] → Output
            r#"{
                "nodes": [
                    { "id": "in", "type": "input", "parameters": {} },
                    { "id": "col-renamer", "type": "group", "parameters": {}, "nodes": [
                        { "id": "proc", "type": "spreadsheet", "parameters": { "operation": "rename", "columns": {} } }
                    ]},
                    { "id": "out", "type": "output", "parameters": {} }
                ]
            }"#,
        ];

        for (i, json) in recipes.iter().enumerate() {
            let result: Result<PipelineDefinition, _> = serde_json::from_str(json);
            assert!(result.is_ok(), "Recipe {} failed to deserialize: {:?}", i, result.err());
        }
    }

    #[test]
    fn test_all_six_recipes_execute_with_mocks() {
        // Run every recipe with the compositional sub-recipe pattern.
        let recipes = [
            compress_images_json(),
            clean_csv_json(),
            rename_files_json(),
            // Resize: Input → Group → Loop → [image:resize] → Output
            r#"{
                "nodes": [
                    { "id": "in", "type": "input", "parameters": {} },
                    { "id": "batch-resize", "type": "group", "parameters": {}, "nodes": [
                        { "id": "loop", "type": "loop", "parameters": {}, "nodes": [
                            { "id": "proc", "type": "image", "parameters": { "operation": "resize" } }
                        ]}
                    ]},
                    { "id": "out", "type": "output", "parameters": {} }
                ]
            }"#,
            // Convert: Input → Group → Loop → [image:convert] → Output
            r#"{
                "nodes": [
                    { "id": "in", "type": "input", "parameters": {} },
                    { "id": "batch-convert", "type": "group", "parameters": {}, "nodes": [
                        { "id": "loop", "type": "loop", "parameters": {}, "nodes": [
                            { "id": "proc", "type": "image", "parameters": { "operation": "convert" } }
                        ]}
                    ]},
                    { "id": "out", "type": "output", "parameters": {} }
                ]
            }"#,
            // Rename CSV columns: Input → Group → [spreadsheet:rename] → Output
            r#"{
                "nodes": [
                    { "id": "in", "type": "input", "parameters": {} },
                    { "id": "col-renamer", "type": "group", "parameters": {}, "nodes": [
                        { "id": "proc", "type": "spreadsheet", "parameters": { "operation": "rename" } }
                    ]},
                    { "id": "out", "type": "output", "parameters": {} }
                ]
            }"#,
        ];

        let registry = recipe_registry();
        let files = vec![make_file("test-file.dat", b"test-data")];

        for (i, json) in recipes.iter().enumerate() {
            let def = parse_def(json);
            let reporter = PipelineReporter::new_noop();
            let result = execute_pipeline(&def, files.clone(), &registry, &reporter, fake_now);
            assert!(result.is_ok(), "Recipe {} failed to execute: {:?}", i, result.err());
            assert!(!result.unwrap().files.is_empty(), "Recipe {} produced no output files", i);
        }
    }
}
