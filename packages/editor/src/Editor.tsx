/**
 * Editor — flat re-exports for the recipe editor.
 *
 * Usage (Server Component page):
 *
 *   import { EditorRoot, EditorCanvas, EditorToolbar, EditorLeftToolbar, EditorRightToolbar } from "@bnto/editor";
 *
 *   <EditorRoot slug="compress-images">
 *     <EditorCanvas>
 *       <EditorToolbar />
 *       <EditorLeftToolbar />
 *       <EditorRightToolbar />
 *     </EditorCanvas>
 *   </EditorRoot>
 *
 * All panels are Menu-based — they live inside their respective toolbars
 * and open from their trigger buttons. No separate composition needed.
 */

import { EditorCanvasRoot } from "./components/EditorCanvas/EditorCanvasRoot";
import { CanvasShell } from "./components/EditorCanvas/CanvasShell";
import { EditorToolbar } from "./components/EditorToolbar";
import { EditorLeftToolbar } from "./components/EditorLeftToolbar";
import { EditorRightToolbar } from "./components/EditorRightToolbar";

export {
  EditorCanvasRoot as EditorRoot,
  CanvasShell as EditorCanvas,
  EditorToolbar,
  EditorLeftToolbar,
  EditorRightToolbar,
};
