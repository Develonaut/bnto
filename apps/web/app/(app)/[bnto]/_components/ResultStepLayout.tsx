import { Divider, StepperIndicator } from "@bnto/ui";
import { RecipeStepperResultList } from "./RecipeStepper";
import { RecipeHeroSidebar } from "./RecipeHeroSidebar";
import { StepToolbar } from "./StepToolbar";

/** Shared sidebar + results grid layout used by steps 2 and 3. */
export function ResultStepLayout({
  h1,
  description,
  features,
  category,
}: {
  h1: string;
  description: string;
  features: string[];
  category: string;
}) {
  return (
    <div className="lg:grid lg:grid-cols-[300px_auto_1fr_1fr] lg:gap-6">
      <div className="mb-6 lg:sticky lg:top-24 lg:mb-0 lg:self-start">
        <RecipeHeroSidebar
          h1={h1}
          description={description}
          features={features}
          category={category}
        />
      </div>
      <Divider orientation="vertical" className="hidden lg:flex" />
      <div className="col-span-2">
        <StepperIndicator className="mb-4 hidden lg:flex" />
        <div className="pb-4">
          <StepToolbar />
        </div>
        <Divider className="mb-4" />
        <div className="max-h-[60vh] overflow-y-auto p-2 -m-2">
          <RecipeStepperResultList />
        </div>
      </div>
    </div>
  );
}
