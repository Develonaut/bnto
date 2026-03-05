"use client";

import { useEditorStore } from "./useEditorStore";

/**
 * useEditorPanels — panel visibility from the editor store.
 *
 * Toolbar buttons and overlay slots both read from the same store
 * instead of prop-drilling visibility state through the component tree.
 */
function useEditorPanels() {
  const layersOpen = useEditorStore((s) => s.layersOpen);
  const configOpen = useEditorStore((s) => s.configOpen);
  const toggleLayers = useEditorStore((s) => s.toggleLayers);
  const toggleConfig = useEditorStore((s) => s.toggleConfig);
  const openConfig = useEditorStore((s) => s.openConfig);

  return { layersOpen, configOpen, toggleLayers, toggleConfig, openConfig };
}

export { useEditorPanels };
