/**
 * Deep-clone nodes + configs into an immutable snapshot for undo/redo.
 *
 * Clones the nodes array and each config's parameters object so mutations
 * to the current state don't retroactively alter saved history.
 */

import type { BentoNode, NodeConfigs } from "../adapters/types";
import type { EditorSnapshot } from "./types";

function captureSnapshot(nodes: BentoNode[], configs: NodeConfigs): EditorSnapshot {
  const clonedConfigs: NodeConfigs = {};
  for (const [id, config] of Object.entries(configs)) {
    clonedConfigs[id] = {
      ...config,
      parameters: { ...config.parameters },
    };
  }
  return { nodes: [...nodes], configs: clonedConfigs };
}

export { captureSnapshot };
