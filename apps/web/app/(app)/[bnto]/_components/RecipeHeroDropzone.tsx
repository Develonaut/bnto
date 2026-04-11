"use client";

import { Badge, Button, DormantWrapper, FileUploadDropzone, Heading, UploadIcon } from "@bnto/ui";
import { useRecipeStepperDefn } from "../_stores/recipeStepperContext";

/**
 * Full-hero dropzone — the entire above-fold area is one drop target.
 *
 * Contains H1, description, feature badges, and upload prompt.
 * Inspired by Canva's approach: everything inside the CTA zone.
 */
export function RecipeHeroDropzone({
  h1,
  description,
  features,
}: {
  h1: string;
  description: string;
  features: string[];
}) {
  const defn = useRecipeStepperDefn();
  return (
    <FileUploadDropzone className="gap-6 px-6 py-12 sm:px-10 sm:py-16">
      <Heading level={1} data-testid="recipe-heading">
        {h1}
      </Heading>
      <p className="text-muted-foreground mx-auto max-w-lg text-sm leading-snug text-balance">
        {description}
      </p>
      <div className="flex flex-wrap justify-center gap-2">
        {features.map((f) => (
          <Badge key={f} variant="secondary" size="sm">
            {f}
          </Badge>
        ))}
      </div>
      <DormantWrapper>
        <Button variant="primary">
          <UploadIcon />
          Upload your files
        </Button>
      </DormantWrapper>
      <p className="text-muted-foreground text-sm">or drop here</p>
      <p className="text-muted-foreground text-xs">accepts {defn.acceptLabel}</p>
    </FileUploadDropzone>
  );
}
