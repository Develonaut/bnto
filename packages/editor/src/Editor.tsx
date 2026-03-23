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
 */

import { CanvasShell } from "./components/archive/rf/EditorCanvas/CanvasShell";
import { EditorToolbar } from "./components/EditorToolbar";
import { EditorRightToolbar } from "./components/EditorRightToolbar";

export { CanvasShell as EditorCanvas, EditorToolbar, EditorRightToolbar };
