"use client";

import { useCallback } from "react";
import { useRouter } from "next/navigation";
import { Button, PenLineIcon } from "@bnto/ui";
import { core, applyConfigToDefinition, stashFilesForTransfer } from "@bnto/core";
import { editorUrl } from "@/lib/routes";
import { useRecipeStepperStore } from "../../_stores/recipeStepperContext";

/** Icon button that opens the current recipe in the editor. */
export function RecipeStepperEditButton({ slug }: { slug: string }) {
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
      variant="secondary"
      size="icon"
      onClick={handleClick}
      aria-label="Edit in editor"
      data-testid="edit-in-editor-button"
    >
      <PenLineIcon className="size-4" />
    </Button>
  );
}
