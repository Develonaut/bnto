/**
 * Editor — flat re-exports for the recipe editor.
 *
 * Usage:
 *
 *   import { EditorProvider, EditorCanvas, EditorToolbar, EditorRightToolbar, EditorLeftPanel } from "@bnto/editor";
 *
 *   <EditorProvider definition={recipe.definition}>
 *     <EditorCanvas>
 *       <EditorLeftPanel />
 *       <EditorToolbar />
 *       <EditorRightToolbar />
 *     </EditorCanvas>
 *   </EditorProvider>
 *
 * Left panel: recipe file menu, node list, palette.
 * Bottom toolbar: run, undo/redo, config toggle, help.
 * Right panels (config, run) open from the toolbar triggers.
 */

import { CanvasShell } from "./components/EditorCanvas/CanvasShell";
import { EditorLeftPanel } from "./components/EditorLeftPanel";
import { EditorToolbar } from "./components/EditorToolbar";
import { EditorRightToolbar } from "./components/EditorRightToolbar";

export { CanvasShell as EditorCanvas, EditorLeftPanel, EditorToolbar, EditorRightToolbar };
