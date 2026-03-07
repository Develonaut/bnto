/**
 * runPipeline — pure function that converts editor state into a PipelineDefinition
 * and validates it before execution.
 *
 * This is the bridge between the editor's visual state and the runtime-agnostic
 * pipeline executor. It does NOT run the pipeline itself — it prepares and
 * validates, returning a ready-to-execute definition or validation errors.
 */

import { validateDefinition } from "@bnto/nodes";
import { flattenDefinition } from "@bnto/core";
import type { PipelineDefinition } from "@bnto/core";
import type { BentoNode, NodeConfigs } from "../adapters/types";
import type { RecipeMetadata, ExecutionState } from "../store/types";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface RunPipelineInput {
  nodes: BentoNode[];
  configs: NodeConfigs;
  recipeMetadata: RecipeMetadata;
}

interface RunPipelineResult {
  definition: PipelineDefinition;
  /** Initial execution state with all processing nodes set to "pending". */
  initialExecutionState: ExecutionState;
}

interface RunPipelineError {
  errors: string[];
}

// ---------------------------------------------------------------------------
// Pure function
// ---------------------------------------------------------------------------

/**
 * Prepare a pipeline for execution from editor state.
 *
 * 1. Converts RF nodes + configs → Definition (via adapter)
 * 2. Validates the definition (Zod schemas per node type)
 * 3. Uses flattenDefinition() to build PipelineDefinition
 * 4. Returns initial execution state (all processing nodes → "pending")
 *
 * Returns either a ready-to-run result or validation errors.
 */
function preparePipeline(input: RunPipelineInput): RunPipelineResult | RunPipelineError {
  const { nodes, configs, recipeMetadata } = input;

  // Step 1: Build Definition from current editor state
  const definition = rfNodesToDefinition(nodes, recipeMetadata, configs);

  // Step 2: Validate (includes per-node Zod parameter validation)
  const validationErrors = validateDefinition(definition);
  if (validationErrors.length > 0) {
    return { errors: validationErrors.map((e) => e.message) };
  }

  // Step 3: Flatten Definition → PipelineDefinition (preserves container nesting)
  const pipeline = flattenDefinition(definition);

  // Step 4: Build initial execution state — processing nodes "pending", I/O "idle"
  return {
    definition: pipeline,
    initialExecutionState: buildExecutionState(pipeline, "pending"),
  };
}

/** Type guard to check if the result is an error. */
function isPipelineError(result: RunPipelineResult | RunPipelineError): result is RunPipelineError {
  return "errors" in result;
}

/** Build execution state from pipeline nodes with a given status for processing nodes. */
function buildExecutionState(
  definition: PipelineDefinition,
  processingStatus: "pending" | "active" | "completed",
): ExecutionState {
  const state: ExecutionState = {};
  for (const node of definition.nodes) {
    state[node.id] = node.type === "input" || node.type === "output" ? "idle" : processingStatus;
  }
  return state;
}

export { preparePipeline, isPipelineError, buildExecutionState };
export type { RunPipelineInput, RunPipelineResult, RunPipelineError };
