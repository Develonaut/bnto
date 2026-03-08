/**
 * Panel visibility helpers — pure functions for panel state mutations.
 *
 * Used by the store to enforce same-side mutual exclusivity and
 * auto-open config on node selection.
 */

import type { PanelId, PanelState } from "./types";
import { PANEL_SIDES } from "./types";

/** Open a panel and close same-side siblings. */
function closeSameSideSiblings(panels: PanelState, id: PanelId): PanelState {
  const next = { ...panels, [id]: true };
  const side = PANEL_SIDES[id];
  if (side) {
    for (const [otherId, otherSide] of Object.entries(PANEL_SIDES)) {
      if (otherId !== id && otherSide === side) {
        next[otherId as PanelId] = false;
      }
    }
  }
  return next;
}

/** Auto-open config when selecting a node — close same-side siblings only. */
function autoOpenConfig(panels: PanelState): PanelState {
  if (panels.config) return panels;
  return closeSameSideSiblings(panels, "config");
}

export { autoOpenConfig, closeSameSideSiblings };
