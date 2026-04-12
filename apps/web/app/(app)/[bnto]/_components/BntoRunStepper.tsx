import { Row, StepperContent, Text } from "@bnto/ui";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { RecipeStepper } from "./RecipeStepper";
import { RecipeHeroMascot } from "./RecipeHeroMascot";
import { RecipeHeroDropzone } from "./RecipeHeroDropzone";
import { ResultStepLayout } from "./ResultStepLayout";
import { OpenInEditorLink } from "./OpenInEditorLink";

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
        <Row className="justify-center gap-2 pr-28 pt-2 sm:pr-36 lg:pr-0">
          <Text size="sm" className="text-muted-foreground">
            Want to customize or build your own?
          </Text>
          <OpenInEditorLink slug={entry.slug} />
        </Row>
      </StepperContent>

      <StepperContent value="2">
        <ResultStepLayout
          h1={entry.h1}
          description={entry.description}
          features={entry.features}
          category={entry.category}
        />
      </StepperContent>

      <StepperContent value="3">
        <ResultStepLayout
          h1={entry.h1}
          description={entry.description}
          features={entry.features}
          category={entry.category}
        />
      </StepperContent>
    </RecipeStepper>
  );
}
