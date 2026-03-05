import {
  EditorRoot,
  EditorCanvas,
  EditorLayerPanel,
  EditorConfigPanel,
  EditorToolbar,
} from "@bnto/editor";

/**
 * /create — full-viewport recipe editor.
 *
 * Server component page — each Editor piece is a "use client" component.
 */
export default function CreatePage() {
  return (
    <EditorRoot>
      <EditorCanvas>
        <EditorLayerPanel />
        <EditorConfigPanel />
        <EditorToolbar />
      </EditorCanvas>
    </EditorRoot>
  );
}
