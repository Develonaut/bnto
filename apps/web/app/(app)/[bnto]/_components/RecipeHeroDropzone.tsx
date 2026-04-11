"use client";

import { FileUploadDropzone, UploadIcon } from "@bnto/ui";
import { useRecipeStepperDefn } from "../_stores/recipeStepperContext";

/** Large prominent dropzone — the primary CTA on recipe landing pages. */
export function RecipeHeroDropzone() {
  const defn = useRecipeStepperDefn();
  return (
    <FileUploadDropzone className="gap-3 px-4 py-14 sm:px-6 sm:py-16">
      <div className="rounded-full bg-muted p-3 text-muted-foreground">
        <UploadIcon className="size-8" />
      </div>
      <div className="text-center">
        <p className="text-sm font-medium text-foreground">Drop your files here</p>
        <p className="mt-1 text-xs text-muted-foreground">
          or click to browse &middot; accepts {defn.acceptLabel}
        </p>
      </div>
    </FileUploadDropzone>
  );
}
