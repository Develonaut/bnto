"use client";

import { useMemo } from "react";
import { useSearchParams } from "next/navigation";
import { getRecipeBySlug } from "@bnto/nodes";

import {
  EditorRoot,
  EditorCanvas,
  EditorToolbar,
  EditorRightToolbar,
} from "@bnto/editor";

import { EditorBetaDialog } from "./_components/EditorBetaDialog";

/**
 * /editor — full-viewport recipe editor.
 *
 * Slug lookup happens here (app layer). The editor accepts a Definition
 * directly — it has no knowledge of slugs or the recipe registry.
 */
export default function EditorPage() {
  const searchParams = useSearchParams();
  const from = searchParams.get("from") ?? undefined;

  const definition = useMemo(() => {
    if (!from) return undefined;
    return getRecipeBySlug(from)?.definition;
  }, [from]);

  return (
    <>
      <EditorBetaDialog />
      <EditorRoot definition={definition}>
        <EditorCanvas>
          <EditorToolbar />
          <EditorRightToolbar />
        </EditorCanvas>
      </EditorRoot>
    </>
  );
}
