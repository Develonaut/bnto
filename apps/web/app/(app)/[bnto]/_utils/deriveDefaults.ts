/** Collects default parameter values from a definition's processing nodes into a flat object. */

import type { Definition } from "@bnto/core";
import { extractProcessingNodes } from "./extractProcessingNodes";

export function deriveDefaults(definition: Definition): Record<string, unknown> {
  const nodes = extractProcessingNodes(definition);
  const defaults: Record<string, unknown> = {};
  for (const node of nodes) {
    Object.assign(defaults, node.parameters);
  }
  return defaults;
}
