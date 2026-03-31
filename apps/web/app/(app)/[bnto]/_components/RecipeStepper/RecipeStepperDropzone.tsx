"use client";

import { SlideUp, FileUploadDropzone, UploadIcon } from "@bnto/ui";
import { useRecipeStepperDefn } from "../../_stores/recipeStepperContext";

/** Dropzone — renders the file picker for recipe input files. */
export function RecipeStepperDropzone() {
  const defn = useRecipeStepperDefn();
  return (
    <SlideUp>
      <FileUploadDropzone className="gap-3 px-4 py-8 sm:px-6 sm:py-10">
        <div className="rounded-full bg-muted p-3 text-muted-foreground">
          <UploadIcon className="size-6" />
        </div>
        <div className="text-center">
          <p className="text-sm font-medium text-foreground">Drag & drop files here</p>
          <p className="mt-1 text-xs text-muted-foreground">
            or click to browse &middot; accepts {defn.acceptLabel}
          </p>
        </div>
      </FileUploadDropzone>
    </SlideUp>
  );
}
