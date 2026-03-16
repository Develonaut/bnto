"use client";

/**
 * useRecipeDialogForm — form state for the recipe settings dialog.
 *
 * Owns the name field, sync-on-open, validation, and submit handler.
 * Designed so the internals can be swapped for a form library later
 * without changing the component.
 */

import { useCallback, useEffect, useState } from "react";
import type { FormEvent } from "react";
import { useEditor } from "../../context";

interface UseRecipeDialogFormOptions {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

function useRecipeDialogForm({ open, onOpenChange }: UseRecipeDialogFormOptions) {
  const editor = useEditor();
  const { recipeMetadata } = editor.definition.useDefinition();
  const [name, setName] = useState(recipeMetadata.name);

  // Sync local state when dialog opens
  useEffect(() => {
    if (open) setName(recipeMetadata.name);
  }, [open, recipeMetadata.name]);

  const trimmed = name.trim();
  const hasChanges = trimmed.length > 0 && trimmed !== recipeMetadata.name;

  const handleSubmit = useCallback(
    (e: FormEvent) => {
      e.preventDefault();
      if (!hasChanges || !trimmed) return;
      editor.definition.setRecipeMetadata({ ...recipeMetadata, name: trimmed });
      onOpenChange(false);
    },
    [editor, recipeMetadata, trimmed, hasChanges, onOpenChange],
  );

  return { name, setName, hasChanges, handleSubmit };
}

export { useRecipeDialogForm };
