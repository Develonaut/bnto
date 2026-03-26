/** Store actions: config setters (setConfigs, setConfig, removeConfig). */

import type { EditorStore } from "./types";
import type { NodeConfig, NodeConfigs } from "../adapters/types";

type Setter = (partial: Partial<EditorStore> | ((s: EditorStore) => Partial<EditorStore>)) => void;

function createConfigActions(set: Setter) {
  return {
    setConfigs: (configs: NodeConfigs) => {
      set({ configs });
    },

    setConfig: (nodeId: string, config: NodeConfig) => {
      set((s) => ({ configs: { ...s.configs, [nodeId]: config } }));
    },

    removeConfig: (nodeId: string) => {
      set((s) => {
        const next = { ...s.configs };
        delete next[nodeId];
        return { configs: next };
      });
    },
  };
}

export { createConfigActions };
