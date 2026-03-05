import {
  EditorRoot,
  EditorCanvas,
  EditorLayerPanel,
  EditorConfigPanel,
  EditorToolbar,
} from "@bnto/editor";

/**
 * /editor — full-viewport recipe editor.
 *
 * Server component page. Reads `?from={slug}` to pre-populate with a
 * predefined recipe, or loads a blank canvas with Input + Output nodes.
 *
 * The editor is free for all users — no auth required. Auth gates
 * belong on save/share (Sprint 5 Wave 4), not editor access.
 */
export default async function EditorPage({
  searchParams,
}: {
  searchParams: Promise<{ from?: string }>;
}) {
  const { from } = await searchParams;

  return (
    <EditorRoot slug={from}>
      <EditorCanvas>
        <EditorLayerPanel />
        <EditorConfigPanel />
        <EditorToolbar />
      </EditorCanvas>
    </EditorRoot>
  );
}
