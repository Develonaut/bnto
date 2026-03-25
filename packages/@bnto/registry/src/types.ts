/** Shared types for registry data structures. */

import type { NodeTypeInfo, NodeTypeName, CategoryInfo, ProcessorDef } from "@bnto/nodes";
import type { Recipe } from "./recipe";

/** Shape of the complete registry dataset. */
export interface RegistryData {
  recipes: readonly Recipe[];
  nodeTypes: Record<NodeTypeName, NodeTypeInfo>;
  categories: readonly CategoryInfo[];
  processors: readonly ProcessorDef[];
}

// ── Node system type re-exports ─────────────────────────────────────────

export type { Definition, Position, Metadata, Port, Edge, FieldsConfig } from "@bnto/nodes";
export type { PipelineSettings, IterationMode } from "@bnto/nodes";
export type { ExecutionContext, NodeResult } from "@bnto/nodes";
export type { NodeTypeName, NodeCategory, NodeTypeInfo } from "@bnto/nodes";
export type { CategoryInfo } from "@bnto/nodes";
export type { ProcessorDef, ProcessorParam, ParamType } from "@bnto/nodes";
