/**
 * Editor — flat re-exports for the recipe editor.
 *
 * Usage:
 *
 *   import { EditorProvider, EditorCanvas, EditorToolbar, EditorRightToolbar } from "@bnto/editor";
 *
 *   <EditorProvider definition={recipe.definition}>
 *     <EditorCanvas>
 *       <EditorToolbar />
 *       <EditorRightToolbar />
 *     </EditorCanvas>
 *   </EditorProvider>
 *
 * The node palette opens as a dialog from the bottom toolbar's + button.
 * Right-side panels (config, run) open from the right toolbar triggers.
 */

import { CanvasShell } from "./components/EditorCanvas/CanvasShell";
import { EditorToolbar } from "./components/EditorToolbar";
import { EditorRightToolbar } from "./components/EditorRightToolbar";

export { CanvasShell as EditorCanvas, EditorToolbar, EditorRightToolbar };
