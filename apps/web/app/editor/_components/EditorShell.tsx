"use client";

import { useMemo } from "react";
import { useSearchParams } from "next/navigation";
import { getRecipeBySlug } from "@bnto/nodes";

import {
  EditorRoot,
  EditorCanvas,
  EditorToolbar,
  EditorRightToolbar,
  useDraftHydration,
} from "@bnto/editor";

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
 *   1. ?recipe=[id] → fetch saved recipe from Convex
 *   2. ?from=[slug] → load predefined recipe definition
 *   3. localStorage draft → restore last auto-saved draft
 *   4. Blank canvas
 */
export function EditorShell() {
  const searchParams = useSearchParams();
  const from = searchParams.get("from") ?? undefined;
  const recipeId = searchParams.get("recipe") ?? undefined;

  const predefinedDefinition = useMemo(() => {
    if (!from) return undefined;
    return getRecipeBySlug(from)?.definition;
  }, [from]);

  const draftDefinition = useDraftHydration({ skip: !!from || !!recipeId });

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

  // Predefined recipe, draft, or blank canvas
  return (
    <>
      <EditorBetaDialog />
      <EditorRoot definition={predefinedDefinition ?? draftDefinition}>
        <EditorEffects />
        <EditorCanvas>
          <EditorToolbar />
          <EditorRightToolbar />
        </EditorCanvas>
      </EditorRoot>
    </>
  );
}
