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
  EllipsisVerticalIcon,
  Input,
  Label,
  Menu,
  MenuContent,
  MenuItem,
  MenuTrigger,
  PenLineIcon,
  TrashIcon,
} from "@bnto/ui";

interface RecipeCardMenuProps {
  recipeId: string;
  recipeName: string;
}

/**
 * Ellipsis menu for a saved recipe card — Rename + Delete.
 *
 * Uses `dormant` trigger so the button is grounded/muted by default
 * and wakes on ancestor `.group` hover. Parent wraps this in
 * `<CardActionArea>` to block propagation from the trigger.
 */
export function RecipeCardMenu({ recipeId, recipeName }: RecipeCardMenuProps) {
  const [renameOpen, setRenameOpen] = useState(false);
  const [deleteOpen, setDeleteOpen] = useState(false);
  const [name, setName] = useState(recipeName);

  /* ── Rename ────────────────────────────────────────────────── */

  const recipe = renameOpen ? core.recipes.get(recipeId) : undefined;
  const hasChanges = name.trim().length > 0 && name.trim() !== recipeName;

  const handleRenameOpen = useCallback(() => {
    setName(recipeName);
    setRenameOpen(true);
  }, [recipeName]);

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
      setRenameOpen(false);
    },
    [recipe, name, hasChanges],
  );

  /* ── Delete ────────────────────────────────────────────────── */

  const handleDelete = useCallback(() => {
    core.recipes.remove(recipeId);
    setDeleteOpen(false);
  }, [recipeId]);

  return (
    <>
      <Menu>
        <MenuTrigger
          icon={<EllipsisVerticalIcon />}
          dormant
          aria-label={`Actions for ${recipeName}`}
          data-testid="recipe-menu"
        />
        <MenuContent align="end">
          <MenuItem onClick={handleRenameOpen}>
            <PenLineIcon />
            Rename
          </MenuItem>
          <MenuItem onClick={() => setDeleteOpen(true)} className="text-destructive">
            <TrashIcon />
            Delete
          </MenuItem>
        </MenuContent>
      </Menu>

      {/* Rename dialog */}
      <Dialog open={renameOpen} onOpenChange={setRenameOpen}>
        <DialogContent size="sm">
          <DialogHeader>
            <DialogTitle>Rename recipe</DialogTitle>
            <DialogClose />
          </DialogHeader>
          <form onSubmit={handleSave}>
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

      {/* Delete confirmation dialog */}
      <Dialog open={deleteOpen} onOpenChange={setDeleteOpen}>
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
              <Button variant="ghost">Cancel</Button>
            </DialogClose>
            <Button
              variant="destructive"
              onClick={handleDelete}
              data-testid="confirm-delete-recipe"
            >
              Delete
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
