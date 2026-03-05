/**
 * Finds the input node in a definition's node tree.
 *
 * Searches the top-level nodes array for a node with type "input".
 * Returns the first match, or undefined if no input node exists.
 */

import type { Definition } from "./definition";

/** Finds the input node in a definition. Returns undefined if not found. */
export function getInputNode(definition: Definition): Definition | undefined {
  return definition.nodes?.find((node) => node.type === "input");
}
