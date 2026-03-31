"use client";

import type { ReactNode } from "react";
import { Stepper, StepperList, StepperStep, FileUpload } from "@bnto/ui";
import {
  useRecipeStepperStore,
  useRecipeStepperActions,
  useRecipeStepperDefn,
} from "../../_stores/recipeStepperContext";

const noop = () => {};

/** Inner shell — reads step from store, renders Stepper + FileUpload. */
export function RecipeStepperRoot({ children }: { children: ReactNode }) {
  const step = useRecipeStepperStore((s) => s.activeStep);
  const files = useRecipeStepperStore((s) => s.files);
  const isProcessing = useRecipeStepperStore((s) => s.isProcessing);
  const actions = useRecipeStepperActions();
  const defn = useRecipeStepperDefn();

  return (
    <Stepper
      value={String(step)}
      onValueChange={noop}
      data-testid="bnto-shell"
      data-session="ready"
    >
      <StepperList>
        <StepperStep value="1" label="Files" />
        <StepperStep value="2" label="Configure" />
        <StepperStep value="3" label="Results" />
      </StepperList>
      <FileUpload
        value={files}
        onValueChange={actions.setFiles}
        accept={defn.dropzoneAccept}
        multiple
        disabled={isProcessing}
        className="gap-6"
      >
        {children}
      </FileUpload>
    </Stepper>
  );
}
