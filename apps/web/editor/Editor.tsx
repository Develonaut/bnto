"use client";

/**
 * Editor — compound component namespace for the recipe editor.
 *
 * Usage:
 *
 *   <Editor.Root slug="compress-images">
 *     <Editor.Canvas>
 *       <Editor.LayerPanel />
 *       <Editor.ConfigPanel />
 *       <Editor.Toolbar />
 *     </Editor.Canvas>
 *   </Editor.Root>
 *
 * Each sub-component is also exported individually for Server Component
 * pages that can't use Object.assign dot-notation across the RSC boundary.
 */

import { EditorCanvasRoot } from "./components/EditorCanvas/EditorCanvasRoot";
import { CanvasShell } from "./components/EditorCanvas/CanvasShell";
import { LayerPanel } from "./components/LayerPanel";
import { ConfigPanel } from "./components/ConfigPanel";
import { EditorToolbar } from "./components/EditorToolbar";

export {
  EditorCanvasRoot as EditorRoot,
  CanvasShell as EditorCanvas,
  LayerPanel as EditorLayerPanel,
  ConfigPanel as EditorConfigPanel,
  EditorToolbar,
};
