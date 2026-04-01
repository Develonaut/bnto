"use client";

import {
  useRecipeStepperStore,
  useRecipeStepperDefn,
  useRecipeStepperActions,
} from "../../_stores/recipeStepperContext";
import { DynamicRecipeConfig } from "../DynamicRecipeConfig";

/** Inner content for the recipe config dialog — reads store, renders form. */
export function RecipeStepperConfigDialogContent() {
  const config = useRecipeStepperStore((s) => s.config);
  const files = useRecipeStepperStore((s) => s.files);
  const { definition } = useRecipeStepperDefn();
  const actions = useRecipeStepperActions();

  if (!definition) return null;

  return (
    <DynamicRecipeConfig
      definition={definition}
      config={config}
      files={files}
      onChange={actions.setNodeParam}
    />
  );
}
