"use client";

import { useCallback } from "react";
import { useRouter } from "next/navigation";
import { Badge, Button, PenLineIcon } from "@bnto/ui";
import { core, applyConfigToDefinition } from "@bnto/core";
import { editorUrl } from "@/lib/routes";

/**
 * "Open in Editor" — creates a personal recipe from the predefined
 * definition and navigates to the editor.
 *
 * Synchronous: the definition is already in memory (registry), so
 * the recipe entry lands in localStorage before navigation starts.
 */
export function OpenInEditorLink({
  slug,
  config,
}: {
  slug: string;
  config?: Record<string, unknown>;
}) {
  const router = useRouter();

  const handleClick = useCallback(() => {
    const recipe = core.registry.getRecipeBySlug(slug);
    if (!recipe) return;

    const definition = config
      ? applyConfigToDefinition(recipe.definition, config)
      : recipe.definition;
    const id = core.recipes.createFromDefinition(definition);
    router.push(editorUrl(id));
  }, [slug, config, router]);

  return (
    <Button onClick={handleClick} variant="outline" elevation="sm">
      <PenLineIcon className="size-3.5" />
      Open in Editor
      <Badge variant="secondary">Beta</Badge>
    </Button>
  );
}
