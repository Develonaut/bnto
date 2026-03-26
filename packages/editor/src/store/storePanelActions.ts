/** Store actions: panel visibility (open, close, toggle). */

import type { EditorStore, PanelId } from "./types";
import { closeSameSideSiblings } from "./panelHelpers";

type Setter = (partial: Partial<EditorStore> | ((s: EditorStore) => Partial<EditorStore>)) => void;

function createPanelActions(set: Setter) {
  return {
    openPanel: (id: PanelId) => {
      set((s) => ({ panels: closeSameSideSiblings(s.panels, id) }));
    },

    closePanel: (id: PanelId) => {
      set((s) => ({ panels: { ...s.panels, [id]: false } }));
    },

    togglePanel: (id: PanelId) => {
      set((s) => {
        if (s.panels[id]) return { panels: { ...s.panels, [id]: false } };
        return { panels: closeSameSideSiblings(s.panels, id) };
      });
    },
  };
}

export { createPanelActions };
