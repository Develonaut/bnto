"use client";

import { useMemo } from "react";
import { useSearchParams } from "next/navigation";
import { getRecipeBySlug } from "@bnto/nodes";

import { EditorRoot, EditorCanvas, EditorToolbar, EditorRightToolbar } from "@bnto/editor";

import { EditorBetaDialog } from "./_components/EditorBetaDialog";
import { EditorEffects } from "./_components/EditorEffects";
import { SavedRecipeLoader } from "./_components/SavedRecipeLoader";

/**
 * /editor — full-viewport recipe editor.
 *
 * Two entry modes:
 *   ?from=<slug>   — load a predefined recipe by slug (from @bnto/nodes)
 *   ?recipe=<id>   — load a saved recipe by Convex ID (from My Recipes)
 *
 * Slug lookup and saved recipe resolution happen here (app layer).
 * The editor accepts a Definition directly — it has no knowledge of
 * slugs, IDs, or the recipe registry.
 */
export default function EditorPage() {
  const searchParams = useSearchParams();
  const from = searchParams.get("from") ?? undefined;
  const recipeId = searchParams.get("recipe") ?? undefined;

  const predefinedDefinition = useMemo(() => {
    if (!from) return undefined;
    return getRecipeBySlug(from)?.definition;
  }, [from]);

  // Saved recipe — fetch from Convex, render editor once loaded
  if (recipeId) {
    return (
      <>
        <EditorBetaDialog />
        <SavedRecipeLoader recipeId={recipeId}>
          {(definition) => (
            <EditorRoot definition={definition}>
              <EditorEffects />
              <EditorCanvas>
                <EditorToolbar />
                <EditorRightToolbar />
              </EditorCanvas>
            </EditorRoot>
          )}
        </SavedRecipeLoader>
      </>
    );
  }

  // Predefined recipe or blank canvas
  return (
    <>
      <EditorBetaDialog />
      <EditorRoot definition={predefinedDefinition}>
        <EditorEffects />
        <EditorCanvas>
          <EditorToolbar />
          <EditorRightToolbar />
        </EditorCanvas>
      </EditorRoot>
    </>
  );
}
