/**
 * Deep-clone nodes + configs + definition into an immutable snapshot for undo/redo.
 *
 * Clones the nodes array, each config's parameters object, and the definition
 * tree so mutations to the current state don't retroactively alter saved history.
 */

import type { Definition } from "@bnto/core";
import type { BentoNode, NodeConfigs } from "../adapters/types";
import type { EditorSnapshot } from "./types";

function captureSnapshot(
  nodes: BentoNode[],
  configs: NodeConfigs,
  definition: Definition | null = null,
  expandedContainerIds: Set<string> = new Set(),
): EditorSnapshot {
  const clonedConfigs: NodeConfigs = {};
  for (const [id, config] of Object.entries(configs)) {
    clonedConfigs[id] = {
      ...config,
      parameters: { ...config.parameters },
    };
  }
  return {
    nodes: [...nodes],
    configs: clonedConfigs,
    definition: definition ? structuredClone(definition) : null,
    expandedContainerIds: new Set(expandedContainerIds),
  };
}

export { captureSnapshot };
