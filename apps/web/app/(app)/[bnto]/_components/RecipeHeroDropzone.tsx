"use client";

import { Badge, Button, DormantWrapper, FileUploadDropzone, Heading, UploadIcon } from "@bnto/ui";
import { useRecipeStepperDefn } from "../_stores/recipeStepperContext";

/** Right padding per row to avoid mascot overlap. None needed for h1/description. */
const MASCOT_PR = "pr-28 sm:pr-36 lg:pr-0";

/**
 * Full-hero dropzone — the entire above-fold area is one drop target.
 *
 * Contains H1, description, feature badges, and upload prompt.
 * Inspired by Canva's approach: everything inside the CTA zone.
 *
 * Mascot avoidance is per-row: h1 and description sit above the mascot
 * and center freely. Lower rows (badges, button, helper text) get
 * right padding to stay clear.
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
    <FileUploadDropzone className="items-start gap-6 px-6 py-12 text-left sm:px-10 sm:py-16 md:items-center md:text-center">
      <Heading level={1} data-testid="recipe-heading">
        {h1}
      </Heading>
      <p className="text-muted-foreground max-w-lg text-sm leading-snug text-balance md:mx-auto">
        {description}
      </p>
      <div className={`flex flex-wrap gap-2 self-stretch md:justify-center ${MASCOT_PR}`}>
        {features.map((f) => (
          <Badge key={f} variant="secondary" size="sm">
            {f}
          </Badge>
        ))}
      </div>
      <div className={`self-stretch ${MASCOT_PR}`}>
        <DormantWrapper>
          <Button variant="primary">
            <UploadIcon />
            Upload your files
          </Button>
        </DormantWrapper>
      </div>
      <p className={`text-muted-foreground self-stretch text-sm ${MASCOT_PR}`}>or drop here</p>
      <p className={`text-muted-foreground self-stretch text-xs ${MASCOT_PR}`}>
        accepts {defn.acceptLabel}
      </p>
    </FileUploadDropzone>
  );
}
