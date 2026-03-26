"use client";

import { Button, DialogBody, DialogClose, DialogDescription, DialogFooter } from "@bnto/ui";

interface DeleteRecipeConfirmationProps {
  recipeName: string;
  onDelete: () => void;
}

/** Body + footer for the delete recipe dialog — warning message + cancel/delete buttons. */
export function DeleteRecipeConfirmation({ recipeName, onDelete }: DeleteRecipeConfirmationProps) {
  return (
    <>
      <DialogBody>
        <DialogDescription>
          <strong>{recipeName}</strong> will be permanently deleted. This action cannot be undone.
        </DialogDescription>
      </DialogBody>
      <DialogFooter>
        <DialogClose asChild>
          <Button variant="ghost">Cancel</Button>
        </DialogClose>
        <Button variant="destructive" onClick={onDelete} data-testid="confirm-delete-recipe">
          Delete
        </Button>
      </DialogFooter>
    </>
  );
}
