"use client";

import { useCallback, useId, useState } from "react";
import { core } from "@bnto/core";

/** Manages rename dialog state — name input, dirty check, and save handler. */
export function useRenameRecipe(
  recipeId: string,
  recipeName: string,
  open: boolean,
  onOpenChange: (v: boolean) => void,
) {
  const nameInputId = useId();
  const [name, setName] = useState(recipeName);

  const [prevOpen, setPrevOpen] = useState(open);
  if (open && !prevOpen) setName(recipeName);
  if (open !== prevOpen) setPrevOpen(open);

  const recipe = open ? core.recipes.get(recipeId) : undefined;
  const hasChanges = name.trim().length > 0 && name.trim() !== recipeName;

  const handleSave = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      if (!recipe || !hasChanges) return;
      core.recipes.save(recipe.definition, {
        id: recipe.id,
        name: name.trim(),
        slug: recipe.slug,
        cloudId: recipe.cloudId ?? null,
      });
      onOpenChange(false);
    },
    [recipe, name, hasChanges, onOpenChange],
  );

  return { nameInputId, name, setName, hasChanges, handleSave };
}
