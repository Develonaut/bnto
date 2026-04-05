import {
  RecipeStepperToolbar,
  RecipeStepperBackButton,
  RecipeStepperConfigButton,
  RecipeStepperBanner,
  RecipeStepperActions,
  RunRecipeButton,
} from "./RecipeStepper";

export function StepToolbar() {
  return (
    <RecipeStepperToolbar>
      <RecipeStepperActions className="shrink-0">
        <RecipeStepperBackButton />
      </RecipeStepperActions>
      <RecipeStepperBanner />
      <RecipeStepperActions className="ml-auto shrink-0">
        <RecipeStepperConfigButton />
        <RunRecipeButton />
      </RecipeStepperActions>
    </RecipeStepperToolbar>
  );
}
