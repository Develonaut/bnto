/**
 * @bnto/nodes — Engine-agnostic node definitions, types, and metadata.
 *
 * This package is consumed by every execution target:
 * - Rust WASM (browser)
 * - JS library adapters (browser fallback)
 * - Go engine (CLI + cloud)
 * - Web app config UI
 *
 * Zero runtime dependencies. Pure TypeScript types and constants.
 */

// Workflow definition types (mirrors Go engine/pkg/node/)
export type {
  Definition,
  Position,
  Metadata,
  Port,
  Edge,
  FieldsConfig,
} from "./definition";

// Execution types
export type { ExecutionContext, NodeResult } from "./execution";

// Recipe types (mirrors Go engine/pkg/menu/)
export type { Recipe, AcceptSpec, SEOSpec } from "./recipe";

// Node type registry
export {
  NODE_TYPES,
  NODE_TYPE_NAMES,
  NODE_TYPE_INFO,
  isNodeType,
  getNodeTypeInfo,
  getBrowserCapableTypes,
  getContainerTypes,
} from "./nodeTypes";
export type { NodeTypeName, NodeCategory, NodeTypeInfo } from "./nodeTypes";

// Categories
export { CATEGORIES, getCategoryInfo } from "./categories";
export type { CategoryInfo } from "./categories";
