"use client";

import { core } from "@bnto/core";
import {
  useRecipeFlowStore,
  useRecipeFlowDefn,
  useRecipeFlowInstance,
  useRecipeFlowActions,
} from "../../_stores/recipeFlowContext";
import { DynamicRecipeConfig } from "../DynamicRecipeConfig";
import { ToolbarProgress } from "../ToolbarProgress";

const STATUS_CLASS =
  "order-last w-full md:order-none md:min-w-0 md:flex-1 md:border-l md:border-r md:border-border md:px-4";

/** Center content slot — config (step 2) or progress (step 3). */
export function RecipeFlowStatus() {
  const step = useRecipeFlowStore((s) => s.activeStep);
  const config = useRecipeFlowStore((s) => s.config);
  const { definition } = useRecipeFlowDefn();
  const actions = useRecipeFlowActions();
  const instance = useRecipeFlowInstance();
  const browserExec = core.executions.useExecutionState(instance);

  if (step === 2 && definition) {
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

  if (step === 3) {
    return (
      <div className={STATUS_CLASS}>
        <ToolbarProgress execution={browserExec} />
      </div>
    );
  }

  return null;
}
