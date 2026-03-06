"use client";

import { useShallow } from "@bnto/core";
import { useEditorStore } from "./useEditorStore";

/**
 * useEditorPanels — panel visibility from the editor store.
 *
 * Toolbar buttons, overlay slots, and canvas placeholders all read
 * from the same store instead of prop-drilling visibility state.
 *
 * Uses useShallow so consumers only re-render when a selected
 * value actually changes — not on every store mutation.
 */
function useEditorPanels() {
  return useEditorStore(
    useShallow((s) => ({
      layersOpen: s.layersOpen,
      configOpen: s.configOpen,
      paletteOpen: s.paletteOpen,
      toggleLayers: s.toggleLayers,
      toggleConfig: s.toggleConfig,
      openConfig: s.openConfig,
      openPalette: s.openPalette,
      closePalette: s.closePalette,
    })),
  );
}

export { useEditorPanels };
