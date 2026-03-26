/**
 * Editor — flat re-exports for the recipe editor.
 *
 * Usage:
 *
 *   import { EditorProvider, EditorCanvas, EditorToolbar, EditorLeftToolbar, EditorRightToolbar } from "@bnto/editor";
 *
 *   <EditorProvider definition={recipe.definition}>
 *     <EditorCanvas>
 *       <EditorToolbar />
 *       <EditorLeftToolbar />
 *       <EditorRightToolbar />
 *     </EditorCanvas>
 *   </EditorProvider>
 *
 * The node palette opens as a dialog from the bottom toolbar's + button.
 * Left-side panel (settings) opens from the left toolbar trigger.
 * Right-side panels (config, run) open from the right toolbar triggers.
 */

import { CanvasShell } from "./components/EditorCanvas/CanvasShell";
import { EditorToolbar } from "./components/EditorToolbar";
import { EditorLeftToolbar } from "./components/EditorLeftToolbar";
import { EditorRightToolbar } from "./components/EditorRightToolbar";

export { CanvasShell as EditorCanvas, EditorToolbar, EditorLeftToolbar, EditorRightToolbar };
