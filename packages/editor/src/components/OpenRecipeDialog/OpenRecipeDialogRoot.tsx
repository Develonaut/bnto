"use client";

import { useCallback } from "react";
import type { Definition } from "@bnto/nodes";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogHeader,
  DialogTitle,
  Divider,
  type LucideIcon,
} from "@bnto/ui";
import { useEditor } from "../../context";
import { RecipePickerGrid } from "./RecipePickerGrid";
import { FileImportDropzone } from "./FileImportDropzone";

/**
 * OpenRecipeDialog — browse predefined recipes or import a .bnto.json file.
 *
 * Single panel: recipe list on top, divider, then a dropzone for importing
 * custom .bnto.json files below.
 *
 * Selecting a recipe or importing a file calls `loadDefinition` on the
 * editor store and closes the dialog.
 */

interface OpenRecipeDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  getIcon?: (slug: string) => LucideIcon;
}

function OpenRecipeDialogRoot({
  open,
  onOpenChange,
  getIcon,
}: OpenRecipeDialogProps) {
  const editor = useEditor();

  const handleLoad = useCallback(
    (definition: Definition) => {
      editor.definition.loadDefinition(definition);
      onOpenChange(false);
    },
    [editor, onOpenChange],
  );

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent size="lg">
        <DialogHeader className="pb-4">
          <DialogTitle>Open Recipe</DialogTitle>
          <DialogClose />
        </DialogHeader>
        <div className="flex h-[34rem] flex-col">
          <RecipePickerGrid onSelect={handleLoad} getIcon={getIcon} />
          <Divider label="or import a file" className="my-4 shrink-0" />
          <div className="shrink-0 px-1">
            <FileImportDropzone onImport={handleLoad} />
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}

export { OpenRecipeDialogRoot };
