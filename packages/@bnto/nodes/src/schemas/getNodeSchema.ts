/**
 * Lookup function for node type schema definitions.
 */

import type { NodeTypeName } from "../nodeTypes";
import type { NodeSchemaDefinition } from "./types";
import { NODE_SCHEMA_DEFS } from "./registry";

/** Returns the schema definition for a node type, or undefined if not registered. */
export function getNodeSchema(typeName: string): NodeSchemaDefinition | undefined {
  return NODE_SCHEMA_DEFS[typeName as NodeTypeName];
}
