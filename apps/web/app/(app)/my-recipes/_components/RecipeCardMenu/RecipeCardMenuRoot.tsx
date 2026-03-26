"use client";

import { useCallback, useState } from "react";
import { RecipeCardMenuTrigger } from "./RecipeCardMenuTrigger";
import { RenameRecipeDialog } from "./RenameRecipeDialog";
import { DeleteRecipeDialog } from "./DeleteRecipeDialog";

interface RecipeCardMenuProps {
  recipeId: string;
  recipeName: string;
}

/**
 * Ellipsis menu for a saved recipe card — Rename + Delete.
 *
 * Uses `dormant` trigger so the button is grounded/muted by default
 * and wakes on ancestor `.group` hover.
 */
export function RecipeCardMenuRoot({ recipeId, recipeName }: RecipeCardMenuProps) {
  const [renameOpen, setRenameOpen] = useState(false);
  const [deleteOpen, setDeleteOpen] = useState(false);

  const handleRenameOpen = useCallback(() => setRenameOpen(true), []);
  const handleDeleteOpen = useCallback(() => setDeleteOpen(true), []);

  return (
    <>
      <RecipeCardMenuTrigger
        recipeName={recipeName}
        onRenameOpen={handleRenameOpen}
        onDeleteOpen={handleDeleteOpen}
      />
      <RenameRecipeDialog
        recipeId={recipeId}
        recipeName={recipeName}
        open={renameOpen}
        onOpenChange={setRenameOpen}
      />
      <DeleteRecipeDialog
        recipeId={recipeId}
        recipeName={recipeName}
        open={deleteOpen}
        onOpenChange={setDeleteOpen}
      />
    </>
  );
}
