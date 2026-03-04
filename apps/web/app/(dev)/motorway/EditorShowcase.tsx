"use client";

import dynamic from "next/dynamic";

/**
 * Editor showcase — loads the full RecipeEditor in interactive mode.
 *
 * Lazy-loaded because it pulls in ReactFlow (browser-only).
 * Starts with the "compress-images" predefined recipe to show
 * a populated canvas immediately.
 */
const RecipeEditor = dynamic(
  () =>
    import("@/components/editor/RecipeEditor").then((m) => m.RecipeEditor),
  { ssr: false },
);

export function EditorShowcase() {
  return <RecipeEditor slug="compress-images" />;
}
