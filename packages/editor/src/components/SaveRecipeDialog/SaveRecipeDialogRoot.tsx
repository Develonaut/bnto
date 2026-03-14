"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import {
  Button,
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Input,
  Label,
  SaveIcon,
  Stack,
  Text,
} from "@bnto/ui";
import { useEditor } from "../../context";

/**
 * SaveRecipeDialog — prompts the user for a recipe name and saves.
 *
 * Uses the editor's exportAsRecipe() to get a valid Definition,
 * then calls the provided onSave callback with the name + definition.
 * The parent component (EditorToolbar or the app page) provides
 * the actual save mutation — the editor package has no Convex dependency.
 */

interface SaveRecipeDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSave: (args: { name: string; definition: unknown }) => Promise<void>;
  /** Whether the save mutation is in flight. */
  isSaving?: boolean;
}

function SaveRecipeDialogRoot({
  open,
  onOpenChange,
  onSave,
  isSaving = false,
}: SaveRecipeDialogProps) {
  const editor = useEditor();
  const { recipeMetadata } = editor.definition.useDefinition();
  const [name, setName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  // Pre-fill name from recipe metadata when dialog opens
  useEffect(() => {
    if (open) {
      const defaultName = recipeMetadata.name !== "Untitled Recipe" ? recipeMetadata.name : "";
      setName(defaultName);
      setError(null);
      // Focus input after render
      requestAnimationFrame(() => inputRef.current?.select());
    }
  }, [open, recipeMetadata.name]);

  const handleSave = useCallback(async () => {
    const trimmed = name.trim();
    if (!trimmed) {
      setError("Recipe name is required");
      return;
    }

    const result = editor.definition.exportAsRecipe();
    if (!result.recipe) {
      setError("Recipe has validation errors");
      return;
    }

    try {
      await onSave({ name: trimmed, definition: result.recipe.definition });
      editor.definition.resetDirty();
      onOpenChange(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to save");
    }
  }, [name, editor, onSave, onOpenChange]);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === "Enter" && !isSaving) {
        e.preventDefault();
        handleSave();
      }
    },
    [handleSave, isSaving],
  );

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent size="sm">
        <DialogHeader>
          <DialogTitle>Save Recipe</DialogTitle>
          <DialogClose />
        </DialogHeader>

        <Stack className="gap-4 py-4">
          <Stack className="gap-1.5">
            <Label htmlFor="recipe-name">Name</Label>
            <Input
              ref={inputRef}
              id="recipe-name"
              value={name}
              onChange={(e) => {
                setName(e.target.value);
                if (error) setError(null);
              }}
              onKeyDown={handleKeyDown}
              placeholder="My Recipe"
              data-testid="save-recipe-name"
            />
            {error && (
              <Text size="sm" className="text-destructive">
                {error}
              </Text>
            )}
          </Stack>
        </Stack>

        <DialogFooter>
          <DialogClose asChild>
            <Button variant="ghost" elevation="sm">
              Cancel
            </Button>
          </DialogClose>
          <Button
            variant="primary"
            elevation="sm"
            onClick={handleSave}
            disabled={isSaving || !name.trim()}
            data-testid="save-recipe-confirm"
          >
            <SaveIcon />
            {isSaving ? "Saving…" : "Save"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

export { SaveRecipeDialogRoot };
