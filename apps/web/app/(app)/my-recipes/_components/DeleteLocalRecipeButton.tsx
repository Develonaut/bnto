"use client";

import { useCallback, useState } from "react";
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

type Props = {
  recipeId: string;
  recipeName: string;
  onDelete: (recipeId: string) => void;
};

/**
 * Delete button for a device-local recipe draft.
 *
 * Same UI as DeleteRecipeButton (Convex variant) — trash icon with
 * confirmation dialog using the dormant pattern.
 */
export function DeleteLocalRecipeButton({ recipeId, recipeName, onDelete }: Props) {
  const [open, setOpen] = useState(false);

  const handleOpen = useCallback((e: React.MouseEvent) => {
    e.stopPropagation();
    setOpen(true);
  }, []);

  const handleConfirm = useCallback(() => {
    onDelete(recipeId);
    setOpen(false);
  }, [onDelete, recipeId]);

  return (
    <>
      <Button
        icon={<TrashIcon />}
        variant="destructive"
        dormant
        onClick={handleOpen}
        aria-label={`Delete ${recipeName}`}
        data-testid="delete-local-recipe"
      />
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent size="sm">
          <DialogHeader>
            <DialogTitle>Delete recipe?</DialogTitle>
          </DialogHeader>
          <DialogBody>
            <DialogDescription>
              <strong>{recipeName}</strong> will be permanently deleted from this device. This
              action cannot be undone.
            </DialogDescription>
          </DialogBody>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant="ghost">Cancel</Button>
            </DialogClose>
            <Button
              variant="destructive"
              onClick={handleConfirm}
              data-testid="confirm-delete-local-recipe"
            >
              Delete
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
