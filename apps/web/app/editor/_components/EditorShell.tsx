"use client";

import { EditorRoot, EditorCanvas, EditorToolbar, EditorRightToolbar } from "@bnto/editor";
import { Skeleton, Stack, Text } from "@bnto/ui";

import { EditorBetaDialog } from "./EditorBetaDialog";
import { EditorEffects } from "./EditorEffects";
import { useEditorRecipe } from "./useEditorRecipe";

/**
 * Client shell for the editor — owns search-param resolution and canvas.
 *
 * `useEditorRecipe()` resolves the definition from local store, Convex,
 * or the predefined catalog. This component renders a single editor tree
 * regardless of the definition source.
 */
export function EditorShell() {
  const { definition, cloudId, isLoading, notFound } = useEditorRecipe();

  if (isLoading) {
    return (
      <Stack className="items-center justify-center h-full gap-4">
        <Skeleton className="h-8 w-48" />
        <Text color="muted" size="sm">
          Loading recipe…
        </Text>
      </Stack>
    );
  }

  if (notFound) {
    return (
      <Stack className="items-center justify-center h-full gap-2">
        <Text color="muted">Recipe not found</Text>
      </Stack>
    );
  }

  return (
    <>
      <EditorBetaDialog />
      <EditorRoot definition={definition} cloudId={cloudId}>
        <EditorEffects />
        <EditorCanvas>
          <EditorToolbar />
          <EditorRightToolbar />
        </EditorCanvas>
      </EditorRoot>
    </>
  );
}
