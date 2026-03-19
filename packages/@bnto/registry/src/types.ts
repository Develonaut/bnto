/** Shared types for registry data structures. */

import type { Recipe, NodeTypeInfo, NodeTypeName, CategoryInfo, ProcessorDef } from "@bnto/nodes";

/** Shape of the complete registry dataset. */
export interface RegistryData {
  recipes: readonly Recipe[];
  nodeTypes: Record<NodeTypeName, NodeTypeInfo>;
  categories: readonly CategoryInfo[];
  processors: readonly ProcessorDef[];
}
