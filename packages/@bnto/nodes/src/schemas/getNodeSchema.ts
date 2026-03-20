/**
 * Lookup function for node type schema definitions.
 */

import type { NodeTypeName } from "../nodeTypes";
import type { NodeSchema } from "./types";
import { NODE_SCHEMAS } from "./registry";

/** Returns the schema definition for a node type, or undefined if not registered. */
export function getNodeSchema(typeName: string): NodeSchema | undefined {
  return NODE_SCHEMAS[typeName as NodeTypeName];
}
