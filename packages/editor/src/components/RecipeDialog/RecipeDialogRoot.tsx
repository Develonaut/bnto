"use client";

import { useCallback, useEffect, useState } from "react";
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
import { useEditor } from "../../context";

/**
 * RecipeDialog — view and edit recipe metadata (name, and future fields).
 *
 * Pre-populates with the current recipeMetadata. On save, updates
 * metadata via the definition service and closes.
 */

interface RecipeDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

function RecipeDialogRoot({ open, onOpenChange }: RecipeDialogProps) {
  const editor = useEditor();
  const { recipeMetadata } = editor.definition.useDefinition();
  const [name, setName] = useState(recipeMetadata.name);

  // Sync local state when dialog opens
  useEffect(() => {
    if (open) setName(recipeMetadata.name);
  }, [open, recipeMetadata.name]);

  const trimmed = name.trim();
  const hasChanges = trimmed.length > 0 && trimmed !== recipeMetadata.name;

  const handleSave = useCallback(() => {
    if (!trimmed) return;
    editor.definition.setRecipeMetadata({ ...recipeMetadata, name: trimmed });
    onOpenChange(false);
  }, [editor, recipeMetadata, trimmed, onOpenChange]);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent size="sm">
        <DialogHeader>
          <DialogTitle>Recipe Settings</DialogTitle>
          <DialogClose />
        </DialogHeader>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            if (hasChanges) handleSave();
          }}
        >
          <DialogBody>
            <fieldset className="space-y-1.5">
              <Label htmlFor="recipe-name">Name</Label>
              <Input
                id="recipe-name"
                value={name}
                onChange={(e) => setName(e.target.value)}
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
