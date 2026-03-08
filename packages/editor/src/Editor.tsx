/**
 * Editor — flat re-exports for the recipe editor.
 *
 * Usage (Server Component page):
 *
 *   import { EditorRoot, EditorCanvas, EditorLayerPanel, EditorConfigPanel, EditorRunPanel, EditorToolbar } from "@bnto/editor";
 *
 *   <EditorRoot slug="compress-images">
 *     <EditorCanvas>
 *       <EditorLayerPanel />
 *       <EditorConfigPanel />
 *       <EditorRunPanel />
 *       <EditorToolbar />
 *     </EditorCanvas>
 *   </EditorRoot>
 *
 * Flat named exports — compound components use prefixed names (EditorCanvas,
 * EditorToolbar) for RSC compatibility.
 */

import { EditorCanvasRoot } from "./components/EditorCanvas/EditorCanvasRoot";
import { CanvasShell } from "./components/EditorCanvas/CanvasShell";
import { LayerPanel } from "./components/LayerPanel";
import { ConfigPanel } from "./components/ConfigPanel";
import { RunPanel } from "./components/RunPanel";
import { EditorToolbar } from "./components/EditorToolbar";

export {
  EditorCanvasRoot as EditorRoot,
  CanvasShell as EditorCanvas,
  LayerPanel as EditorLayerPanel,
  ConfigPanel as EditorConfigPanel,
  RunPanel as EditorRunPanel,
  EditorToolbar,
};
