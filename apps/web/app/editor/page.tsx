"use client";

import { EditorProvider, EditorCanvas, EditorToolbar, EditorRightToolbar } from "@bnto/editor";
import { Stack, Text } from "@bnto/ui";

import { EditorBetaDialog } from "./_components/EditorBetaDialog";
import { EditorLoadingSkeleton } from "./_components/EditorLoadingSkeleton";
import { useEditorRecipe } from "./_components/useEditorRecipe";

/**
 * /editor — full-viewport recipe editor.
 *
 * Resolves recipe definition from search params (local store → Convex →
 * predefined catalog), then mounts the provider + canvas. loading.tsx
 * provides the Suspense boundary for useSearchParams().
 */
export default function EditorPage() {
  const { definition, cloudId, isLoading, notFound } = useEditorRecipe();

  if (isLoading) return <EditorLoadingSkeleton />;

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
      <EditorProvider definition={definition} cloudId={cloudId}>
        <EditorCanvas>
          <EditorToolbar />
          <EditorRightToolbar />
        </EditorCanvas>
      </EditorProvider>
    </>
  );
}
