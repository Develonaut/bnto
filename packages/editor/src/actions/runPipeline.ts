/**
 * runPipeline — pure function that converts editor state into a PipelineDefinition
 * and validates it before execution.
 *
 * This is the bridge between the editor's visual state and the runtime-agnostic
 * pipeline executor. It does NOT run the pipeline itself — it prepares and
 * validates, returning a ready-to-execute definition or validation errors.
 */

import { validateDefinition } from "@bnto/nodes";
import type { PipelineDefinition, PipelineNode } from "@bnto/core";
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
 * 3. Builds a flat PipelineDefinition for executePipeline()
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

  // Step 3: Flatten Definition nodes into PipelineNode[]
  const pipelineNodes: PipelineNode[] = (definition.nodes ?? []).map((child) => ({
    id: child.id,
    type: child.type,
    params: child.parameters ?? {},
  }));

  // Step 4: Build initial execution state — processing nodes "pending", I/O "idle"
  const initialExecutionState: ExecutionState = {};
  for (const node of pipelineNodes) {
    initialExecutionState[node.id] =
      node.type === "input" || node.type === "output" ? "idle" : "pending";
  }

  return {
    definition: { nodes: pipelineNodes },
    initialExecutionState,
  };
}

/** Type guard to check if the result is an error. */
function isPipelineError(result: RunPipelineResult | RunPipelineError): result is RunPipelineError {
  return "errors" in result;
}

export { preparePipeline, isPipelineError };
export type { RunPipelineInput, RunPipelineResult, RunPipelineError };
