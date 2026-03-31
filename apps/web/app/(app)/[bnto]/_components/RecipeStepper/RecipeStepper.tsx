"use client";

import type { ReactNode } from "react";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { RecipeStepperProvider } from "./RecipeStepperProvider";
import { RecipeStepperRoot } from "./RecipeStepperRoot";

interface RecipeStepperProps {
  entry: BntoEntry;
  children: ReactNode;
}

/** Recipe stepper root — provider + stepper shell + file upload. */
export function RecipeStepper({ entry, children }: RecipeStepperProps) {
  return (
    <RecipeStepperProvider entry={entry}>
      <RecipeStepperRoot>{children}</RecipeStepperRoot>
    </RecipeStepperProvider>
  );
}
