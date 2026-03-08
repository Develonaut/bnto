/**
 * Re-export from generated catalog module.
 *
 * The Rust engine is the single source of truth for all node type metadata.
 * See `generated/catalog.ts` (auto-generated via `task nodes:generate`).
 *
 * This file exists so internal imports (`./nodeTypes`) continue to work.
 */
export { NODE_TYPES, NODE_TYPE_NAMES, NODE_TYPE_INFO } from "./generated/catalog";

export type { NodeTypeName, NodeCategory, NodeTypeInfo } from "./generated/catalog";
