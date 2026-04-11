import { Divider, Row, StepperContent, Text } from "@bnto/ui";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { RecipeStepper, RecipeStepperResultList } from "./RecipeStepper";
import { RecipeHeroMascot } from "./RecipeHeroMascot";
import { RecipeHeroDropzone } from "./RecipeHeroDropzone";
import { ConditionalStepperIndicator } from "./ConditionalStepperIndicator";
import { OpenInEditorLink } from "./OpenInEditorLink";
import { StepToolbar } from "./StepToolbar";

export function BntoRunStepper({ entry }: { entry: BntoEntry }) {
  return (
    <RecipeStepper key={entry.slug} entry={entry}>
      <StepperContent value="1">
        <div className="relative">
          <RecipeHeroDropzone
            h1={entry.h1}
            description={entry.description}
            features={entry.features}
          />
          <RecipeHeroMascot category={entry.category} />
        </div>
        <Row className="justify-center gap-2 pt-2">
          <Text size="sm" className="text-muted-foreground">
            Want to customize or build your own?
          </Text>
          <OpenInEditorLink slug={entry.slug} />
        </Row>
      </StepperContent>

      <ConditionalStepperIndicator />

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
