"use client";

import type { ReactNode } from "react";
import { Stepper, StepperList, StepperStep } from "@bnto/ui";
import { useRecipeFlowStore, useRecipeFlowDefn } from "../../_stores/recipeFlowContext";

const noop = () => {};

/** Stepper wrapper — reads active step from the recipe flow store. */
export function RecipeFlowStepper({ children }: { children: ReactNode }) {
  const step = useRecipeFlowStore((s) => s.activeStep);
  const defn = useRecipeFlowDefn();

  return (
    <Stepper
      value={String(step)}
      onValueChange={noop}
      className="space-y-6"
      data-testid="bnto-shell"
      data-session="ready"
      data-execution-mode={defn.isBrowserPath ? "browser" : "cloud"}
    >
      <StepperList>
        <StepperStep value="1" label="Files" />
        <StepperStep value="2" label="Configure" />
        <StepperStep value="3" label="Results" />
      </StepperList>
      {children}
    </Stepper>
  );
}
