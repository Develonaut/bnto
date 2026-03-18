"use client";

import { useCallback, useState } from "react";
import { core } from "@bnto/core";
import {
  Button,
  Dialog,
  DialogBody,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  EllipsisVerticalIcon,
  Input,
  Label,
} from "@bnto/ui";

/**
 * RecipeInfoButton — ellipsis icon that opens a rename dialog.
 *
 * Uses `dormant` prop: grounded + muted by default, wakes on ancestor
 * .group hover. Lets users rename recipes without opening the editor.
 */
export function RecipeInfoButton({ recipeId, recipeName }: RecipeInfoButtonProps) {
  const [open, setOpen] = useState(false);
  const [name, setName] = useState(recipeName);

  const handleOpen = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      setName(recipeName);
      setOpen(true);
    },
    [recipeName],
  );

  const recipe = open ? core.recipes.get(recipeId) : undefined;
  const hasChanges = name.trim().length > 0 && name.trim() !== recipeName;

  const handleSave = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      if (!recipe || !hasChanges) return;
      core.recipes.save(recipe.definition, {
        id: recipe.id,
        name: name.trim(),
        type: recipe.type,
        version: recipe.version,
        cloudId: recipe.cloudId,
      });
      setOpen(false);
    },
    [recipe, name, hasChanges],
  );

  const handleNameChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => setName(e.target.value),
    [],
  );

  return (
    <>
      <Button
        icon={<EllipsisVerticalIcon />}
        dormant
        onClick={handleOpen}
        aria-label={`Info for ${recipeName}`}
        data-testid="recipe-info"
      />
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent size="sm">
          <DialogHeader>
            <DialogTitle>Recipe Info</DialogTitle>
            <DialogClose />
          </DialogHeader>
          <form onSubmit={handleSave}>
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
              <DialogClose asChild>
                <Button variant="ghost">Cancel</Button>
              </DialogClose>
              <Button type="submit" disabled={!hasChanges}>
                Save
              </Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
    </>
  );
}

type RecipeInfoButtonProps = {
  recipeId: string;
  recipeName: string;
};
