/**
 * Node schema registry — thin re-export from generated entries.
 *
 * The engine catalog is the single source of truth for all node schemas
 * and UI field presentation metadata. No hand-written overrides needed.
 *
 * Separated from index.ts so helper files can import NODE_SCHEMAS
 * without circular dependencies.
 */

import type { NodeTypeName } from "../nodeTypes";
import type { NodeParamFields, NodeSchema } from "./types";
import { ENGINE_NODE_SCHEMAS, ENGINE_NODE_PARAM_FIELDS } from "../generated/engineSchemaEntries";

/** Schema definitions for all registered node types. */
export const NODE_SCHEMAS: Partial<Record<NodeTypeName, NodeSchema>> =
  ENGINE_NODE_SCHEMAS as Partial<Record<NodeTypeName, NodeSchema>>;

/** UI field presentation metadata for all registered node types. */
export const NODE_PARAM_FIELDS: Partial<Record<NodeTypeName, NodeParamFields>> =
  ENGINE_NODE_PARAM_FIELDS as Partial<Record<NodeTypeName, NodeParamFields>>;
