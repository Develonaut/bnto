import { Divider, StepperContent } from "@bnto/ui";
import type { BntoEntry } from "@/lib/bntoRegistry";
import {
  RecipeStepper,
  RecipeStepperIndicator,
  RecipeStepperDropzone,
  RecipeStepperResultList,
} from "./RecipeStepper";
import { BntoHero } from "./BntoHero";
import { OpenInEditorLink } from "./OpenInEditorLink";
import { StepToolbar } from "./StepToolbar";

export function BntoRunStepper({ entry }: { entry: BntoEntry }) {
  return (
    <RecipeStepper key={entry.slug} entry={entry}>
      <RecipeStepperIndicator />
      <BntoHero h1={entry.h1} description={entry.description} />
      <div>
        <OpenInEditorLink slug={entry.slug} />
      </div>

      <StepperContent value="1">
        <RecipeStepperDropzone />
      </StepperContent>

      <StepperContent value="2">
        <StepToolbar />
        <Divider />
        <RecipeStepperResultList />
      </StepperContent>

      <StepperContent value="3">
        <StepToolbar />
        <Divider />
        <RecipeStepperResultList />
      </StepperContent>
    </RecipeStepper>
  );
}
