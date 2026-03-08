/**
 * Derive available operations for a node type from the engine catalog.
 *
 * The engine's PROCESSORS array is the single source of truth for which
 * operations are implemented. This helper extracts the operation names
 * for a given node type so schema files don't hardcode them.
 */

import { PROCESSORS } from "../generated/catalog";

/** Returns the list of engine-implemented operations for a node type. */
export function getEngineOperations(nodeType: string): string[] {
  return PROCESSORS.filter((p) => p.nodeType === nodeType).map((p) => p.operation);
}
