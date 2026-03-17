"use client";

import { useCallback, useState } from "react";
import { core } from "@bnto/core";
import {
  Button,
  Dialog,
  DialogBody,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  TrashIcon,
} from "@bnto/ui";

/**
 * DeleteRecipeButton — trash icon that opens a confirmation dialog.
 *
 * Uses `dormant` prop: grounded + muted by default, wakes to
 * variant="destructive" with elevation on ancestor .group hover.
 * The CSS transition in surface.css handles the elevation animation.
 */
export function DeleteRecipeButton({ recipeId, recipeName }: DeleteRecipeButtonProps) {
  const [open, setOpen] = useState(false);
  const { mutate, isPending } = core.recipes.useRemoveRecipe();

  const handleConfirm = useCallback(() => {
    mutate(recipeId, {
      onSuccess: () => setOpen(false),
    });
  }, [mutate, recipeId]);

  const handleOpen = useCallback((e: React.MouseEvent) => {
    e.stopPropagation();
    setOpen(true);
  }, []);

  return (
    <>
      <Button
        icon={<TrashIcon />}
        variant="destructive"
        dormant
        onClick={handleOpen}
        aria-label={`Delete ${recipeName}`}
        data-testid="delete-recipe"
      />
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent size="sm">
          <DialogHeader>
            <DialogTitle>Delete recipe?</DialogTitle>
          </DialogHeader>
          <DialogBody>
            <DialogDescription>
              <strong>{recipeName}</strong> will be permanently deleted. This action cannot be
              undone.
            </DialogDescription>
          </DialogBody>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant="ghost" disabled={isPending}>
                Cancel
              </Button>
            </DialogClose>
            <Button
              variant="destructive"
              onClick={handleConfirm}
              disabled={isPending}
              data-testid="confirm-delete-recipe"
            >
              {isPending ? "Deleting…" : "Delete"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}

type DeleteRecipeButtonProps = {
  recipeId: string;
  recipeName: string;
};
