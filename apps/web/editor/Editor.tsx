"use client";

/**
 * Editor — flat re-exports for the recipe editor.
 *
 * Usage (Server Component page):
 *
 *   import { EditorRoot, EditorCanvas, EditorLayerPanel, EditorConfigPanel, EditorToolbar } from "@/editor/Editor";
 *
 *   <EditorRoot slug="compress-images">
 *     <EditorCanvas>
 *       <EditorLayerPanel />
 *       <EditorConfigPanel />
 *       <EditorToolbar />
 *     </EditorCanvas>
 *   </EditorRoot>
 *
 * Flat exports (not dot-notation) because Object.assign compound components
 * don't work across the Server/Client boundary — sub-properties are undefined.
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
