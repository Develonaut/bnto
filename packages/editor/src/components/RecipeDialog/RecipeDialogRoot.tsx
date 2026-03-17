"use client";

import { useCallback } from "react";
import {
  Button,
  Dialog,
  DialogBody,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Input,
  Label,
} from "@bnto/ui";
import { useRecipeDialogForm } from "./useRecipeDialogForm";

/**
 * RecipeDialog — view and edit recipe metadata (name, and future fields).
 *
 * All form state lives in useRecipeDialogForm. This component is a
 * render shell — it composes the dialog layout and spreads form props.
 */

interface RecipeDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

function RecipeDialogRoot({ open, onOpenChange }: RecipeDialogProps) {
  const { name, setName, hasChanges, handleSubmit } = useRecipeDialogForm({
    open,
    onOpenChange,
  });

  const handleNameChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => setName(e.target.value),
    [setName],
  );

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent size="sm">
        <DialogHeader>
          <DialogTitle>Recipe Settings</DialogTitle>
          <DialogClose />
        </DialogHeader>
        <form onSubmit={handleSubmit}>
          <DialogBody>
            <fieldset className="space-y-1.5">
              <Label htmlFor="recipe-name">Name</Label>
              <Input
                id="recipe-name"
                value={name}
                onChange={handleNameChange}
                placeholder="Recipe name"
                autoFocus
              />
            </fieldset>
          </DialogBody>
          <DialogFooter>
            <Button type="submit" disabled={!hasChanges}>
              Save
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

export { RecipeDialogRoot };
