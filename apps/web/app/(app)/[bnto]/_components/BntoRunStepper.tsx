import { Divider, Heading, StepperContent } from "@bnto/ui";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { RecipeStepper, RecipeStepperResultList } from "./RecipeStepper";
import { RecipeHeroMascot } from "./RecipeHeroMascot";
import { RecipeHeroDropzone } from "./RecipeHeroDropzone";
import { ConditionalStepperIndicator } from "./ConditionalStepperIndicator";
import { RecipeDetailsSection } from "./RecipeDetailsSection";
import { OpenInEditorLink } from "./OpenInEditorLink";
import { StepToolbar } from "./StepToolbar";

export function BntoRunStepper({ entry }: { entry: BntoEntry }) {
  return (
    <RecipeStepper key={entry.slug} entry={entry}>
      <RecipeHeroMascot category={entry.category} />
      <Heading level={1} data-testid="recipe-heading">
        {entry.h1}
      </Heading>

      <StepperContent value="1">
        <RecipeHeroDropzone />
      </StepperContent>

      <ConditionalStepperIndicator />

      <StepperContent value="2">
        <OpenInEditorLink slug={entry.slug} />
        <StepToolbar />
        <Divider />
        <RecipeStepperResultList />
      </StepperContent>

      <StepperContent value="3">
        <StepToolbar />
        <Divider />
        <RecipeStepperResultList />
      </StepperContent>

      <RecipeDetailsSection description={entry.description} features={entry.features} />
    </RecipeStepper>
  );
}
