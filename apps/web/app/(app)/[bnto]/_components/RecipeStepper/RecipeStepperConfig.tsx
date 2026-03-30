"use client";

import {
  useRecipeStepperStore,
  useRecipeStepperDefn,
  useRecipeStepperActions,
} from "../../_stores/recipeStepperContext";
import { DynamicRecipeConfig } from "../DynamicRecipeConfig";

const STATUS_CLASS =
  "order-last w-full md:order-none md:min-w-0 md:flex-1 md:border-l md:border-r md:border-border md:px-4";

/** Config panel — renders DynamicRecipeConfig for the current definition. */
export function RecipeStepperConfig() {
  const config = useRecipeStepperStore((s) => s.config);
  const { definition } = useRecipeStepperDefn();
  const actions = useRecipeStepperActions();

  if (!definition) return null;

  return (
    <div className={STATUS_CLASS}>
      <DynamicRecipeConfig
        definition={definition}
        config={config}
        onChange={actions.setNodeParam}
      />
    </div>
  );
}
