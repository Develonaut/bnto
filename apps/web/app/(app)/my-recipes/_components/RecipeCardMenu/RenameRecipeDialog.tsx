"use client";

import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@bnto/ui";
import { RenameRecipeForm } from "./RenameRecipeForm";
import { useRenameRecipe } from "./useRenameRecipe";

interface RenameRecipeDialogProps {
  recipeId: string;
  recipeName: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function RenameRecipeDialog({
  recipeId,
  recipeName,
  open,
  onOpenChange,
}: RenameRecipeDialogProps) {
  const rename = useRenameRecipe(recipeId, recipeName, open, onOpenChange);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent size="sm">
        <DialogHeader>
          <DialogTitle>Rename recipe</DialogTitle>
          <DialogClose />
        </DialogHeader>
        <DialogDescription className="sr-only">Enter a new name for this recipe.</DialogDescription>
        <RenameRecipeForm
          nameInputId={rename.nameInputId}
          name={rename.name}
          onNameChange={rename.setName}
          hasChanges={rename.hasChanges}
          onSubmit={rename.handleSave}
        />
      </DialogContent>
    </Dialog>
  );
}
