"use client";

import { useCallback } from "react";
import { useRouter } from "next/navigation";
import { Badge, Button, PenLineIcon } from "@bnto/ui";
import { core, applyConfigToDefinition, stashFilesForTransfer } from "@bnto/core";
import { editorUrl } from "@/lib/routes";
import { useRecipeStepperStore } from "../_stores/recipeStepperContext";

/** Standalone "Open in Editor" button — creates a personal recipe and navigates. */
export function OpenInEditorLink({ slug }: { slug: string }) {
  const router = useRouter();
  const config = useRecipeStepperStore((s) => s.config);
  const files = useRecipeStepperStore((s) => s.files);

  const handleClick = useCallback(() => {
    const recipe = core.registry.getRecipeBySlug(slug);
    if (!recipe) return;

    const resolved =
      Object.keys(config).length > 0
        ? applyConfigToDefinition(recipe.definition, config)
        : recipe.definition;
    const id = core.recipes.createFromDefinition(resolved);
    stashFilesForTransfer(id, files);
    router.push(editorUrl(id));
  }, [slug, config, files, router]);

  return (
    <Button
      onClick={handleClick}
      variant="outline"
      elevation="sm"
      data-testid="edit-in-editor-button"
    >
      <PenLineIcon className="size-3.5" />
      Open in Editor
      <Badge variant="secondary">Beta</Badge>
    </Button>
  );
}
