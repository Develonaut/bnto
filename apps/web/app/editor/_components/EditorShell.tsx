"use client";

import { useMemo } from "react";
import { useSearchParams } from "next/navigation";
import { getRecipeBySlug } from "@bnto/nodes";
import { core } from "@bnto/core";
import { EditorRoot, EditorCanvas, EditorToolbar, EditorRightToolbar } from "@bnto/editor";

import { EditorBetaDialog } from "./EditorBetaDialog";
import { EditorEffects } from "./EditorEffects";
import { SavedRecipeLoader } from "./SavedRecipeLoader";

/**
 * Client shell for the editor — owns searchParams, editor state, and canvas.
 *
 * Extracted from page.tsx so the page can be a server component.
 * The layout (navbar, full-viewport chrome) renders on the server;
 * this component handles the interactive editor tree.
 *
 * Hydration order:
 *   1. ?recipe=[id] → try recipes store, then Convex cloud recipe
 *   2. ?from=[slug] → load predefined recipe definition
 *   3. Blank canvas (no auto-restore — My Recipes lists all drafts)
 */
export function EditorShell() {
  const searchParams = useSearchParams();
  const from = searchParams.get("from") ?? undefined;
  const recipeId = searchParams.get("recipe") ?? undefined;

  const predefinedDefinition = useMemo(() => {
    if (!from) return undefined;
    return getRecipeBySlug(from)?.definition;
  }, [from]);

  // Synchronous store read — the Zustand persist middleware hydrates from
  // localStorage synchronously on first access, so this is safe to call
  // during render without an effect.
  const localRecipe = useMemo(
    () => (recipeId ? (core.recipes.get(recipeId) ?? null) : null),
    [recipeId],
  );

  // Recipe ID specified — local store takes priority, then Convex
  if (recipeId) {
    if (localRecipe) {
      return (
        <>
          <EditorBetaDialog />
          <EditorRoot definition={localRecipe.definition}>
            <EditorEffects />
            <EditorCanvas>
              <EditorToolbar />
              <EditorRightToolbar />
            </EditorCanvas>
          </EditorRoot>
        </>
      );
    }

    return (
      <>
        <EditorBetaDialog />
        <SavedRecipeLoader recipeId={recipeId}>
          {(definition) => (
            <EditorRoot definition={definition} cloudId={recipeId}>
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
