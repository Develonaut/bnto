import {
  RecipeStepperToolbar,
  RecipeStepperBackButton,
  RecipeStepperConfigButton,
  RecipeStepperBanner,
  RecipeStepperActions,
  RunRecipeButton,
} from "./RecipeStepper";
import { ConditionalStepperIndicator } from "./ConditionalStepperIndicator";

export function StepToolbar() {
  return (
    <RecipeStepperToolbar>
      <RecipeStepperActions className="shrink-0">
        <RecipeStepperBackButton />
      </RecipeStepperActions>
      <div className="flex-1 lg:hidden">
        <ConditionalStepperIndicator />
      </div>
      <RecipeStepperBanner />
      <RecipeStepperActions className="ml-auto shrink-0">
        <RecipeStepperConfigButton />
        <RunRecipeButton />
      </RecipeStepperActions>
    </RecipeStepperToolbar>
  );
}
