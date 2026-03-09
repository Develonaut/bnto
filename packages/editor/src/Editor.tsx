/**
 * Editor — flat re-exports for the recipe editor.
 *
 * Usage (Server Component page):
 *
 *   import { EditorRoot, EditorCanvas, EditorToolbar, EditorRightToolbar } from "@bnto/editor";
 *
 *   <EditorRoot definition={recipe.definition}>
 *     <EditorCanvas>
 *       <EditorToolbar />
 *       <EditorRightToolbar />
 *     </EditorCanvas>
 *   </EditorRoot>
 *
 * The node palette opens as a dialog from the bottom toolbar's + button.
 * Right-side panels (config, run) open from the right toolbar triggers.
 */

import { EditorCanvasRoot } from "./components/EditorCanvas/EditorCanvasRoot";
import { CanvasShell } from "./components/EditorCanvas/CanvasShell";
import { EditorToolbar } from "./components/EditorToolbar";
import { EditorRightToolbar } from "./components/EditorRightToolbar";

export {
  EditorCanvasRoot as EditorRoot,
  CanvasShell as EditorCanvas,
  EditorToolbar,
  EditorRightToolbar,
};
