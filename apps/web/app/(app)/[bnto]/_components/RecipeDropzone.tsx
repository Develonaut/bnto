"use client";

import { SlideUp, FileUploadDropzone } from "@bnto/ui";
import { DropzoneContent } from "./DropzoneContent";

interface RecipeDropzoneProps {
  acceptLabel: string;
}

/**
 * Phase 1 dropzone — full-width file upload area.
 *
 * Shown when no files are selected. Wraps the FileUploadDropzone
 * primitive with a slide-up entrance animation.
 */
export function RecipeDropzone({ acceptLabel }: RecipeDropzoneProps) {
  return (
    <SlideUp className="h-full">
      <FileUploadDropzone disableAnimation className="h-full gap-3 px-4 py-8 sm:px-6 sm:py-10">
        <DropzoneContent label={acceptLabel} />
      </FileUploadDropzone>
    </SlideUp>
  );
}
