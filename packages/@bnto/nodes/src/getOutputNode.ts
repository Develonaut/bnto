/**
 * Finds the output node in a definition's node tree.
 *
 * Searches the top-level nodes array for a node with type "output".
 * Returns the first match, or undefined if no output node exists.
 */

import type { Definition } from "./definition";

/** Finds the output node in a definition. Returns undefined if not found. */
export function getOutputNode(definition: Definition): Definition | undefined {
  return definition.nodes?.find((node) => node.type === "output");
}
