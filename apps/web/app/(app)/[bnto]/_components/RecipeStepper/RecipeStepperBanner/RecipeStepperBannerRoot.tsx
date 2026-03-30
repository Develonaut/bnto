"use client";

import { core } from "@bnto/core";
import { useRecipeStepperInstance } from "../../../_stores/recipeStepperContext";
import { CompletedBanner } from "./CompletedBanner";
import { FailedBanner } from "./FailedBanner";
import { IdleBanner } from "./IdleBanner";
import { ProcessingBanner } from "./ProcessingBanner";
import { dataAttrs } from "./dataAttrs";

const STATUS_CLASS =
  "order-last w-full md:order-none md:min-w-0 md:flex-1 md:border-l md:border-r md:border-border md:px-4";

/** Execution status banner — renders the appropriate banner for the current phase. */
export function RecipeStepperBanner() {
  const instance = useRecipeStepperInstance();
  const execution = core.executions.useExecutionState(instance);
  const { status } = execution;

  return (
    <div className={STATUS_CLASS}>
      <div data-testid="toolbar-progress" data-status={status} {...dataAttrs(execution)}>
        {status === "failed" && <FailedBanner error={execution.error ?? "Execution failed"} />}
        {status === "completed" && <CompletedBanner execution={execution} />}
        {status === "processing" && <ProcessingBanner execution={execution} />}
        {status === "idle" && <IdleBanner />}
      </div>
    </div>
  );
}
