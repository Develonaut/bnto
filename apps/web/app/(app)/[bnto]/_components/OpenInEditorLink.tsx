"use client";

import { useCallback } from "react";
import { useRouter } from "next/navigation";
import { Badge, Button, PenLineIcon } from "@bnto/ui";
import { core } from "@bnto/core";
import { editorUrl } from "@/lib/routes";
import { getRecipeBySlug } from "@bnto/nodes";

/**
 * "Open in Editor" — creates a personal recipe from the predefined
 * definition and navigates to the editor.
 *
 * Synchronous: the definition is already in memory (registry), so
 * the recipe entry lands in localStorage before navigation starts.
 */
export function OpenInEditorLink({ slug }: { slug: string }) {
  const router = useRouter();

  const handleClick = useCallback(() => {
    const recipe = getRecipeBySlug(slug);
    if (!recipe) return;

    const id = core.recipes.createFromDefinition(recipe.definition);
    router.push(editorUrl(id));
  }, [slug, router]);

  return (
    <Button onClick={handleClick} variant="outline" elevation="sm">
      <PenLineIcon className="size-3.5" />
      Open in Editor
      <Badge variant="secondary">Beta</Badge>
    </Button>
  );
}
