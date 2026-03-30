/** Build per-node default config from a Definition's processing nodes. */

import type { Definition } from "@bnto/core";
import { extractProcessingNodes } from "./extractProcessingNodes";

export function buildPerNodeDefaults(
  definition: Definition,
): Record<string, Record<string, unknown>> {
  const defaults: Record<string, Record<string, unknown>> = {};
  for (const node of extractProcessingNodes(definition)) {
    defaults[node.id] = { ...node.parameters };
  }
  return defaults;
}
