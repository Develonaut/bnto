"use client";

import { useCallback } from "react";
import { core } from "@bnto/core";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@bnto/ui";
import { DeleteRecipeConfirmation } from "./DeleteRecipeConfirmation";

interface DeleteRecipeDialogProps {
  recipeId: string;
  recipeName: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function DeleteRecipeDialog({
  recipeId,
  recipeName,
  open,
  onOpenChange,
}: DeleteRecipeDialogProps) {
  const handleDelete = useCallback(() => {
    core.recipes.remove(recipeId);
    onOpenChange(false);
  }, [recipeId, onOpenChange]);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent size="sm">
        <DialogHeader>
          <DialogTitle>Delete recipe?</DialogTitle>
        </DialogHeader>
        <DeleteRecipeConfirmation recipeName={recipeName} onDelete={handleDelete} />
      </DialogContent>
    </Dialog>
  );
}
