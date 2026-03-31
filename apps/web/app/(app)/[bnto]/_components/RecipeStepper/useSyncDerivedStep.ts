"use client";

import { useLayoutEffect } from "react";
import { core } from "@bnto/core";
import type { ExecutionInstance } from "@bnto/core";
import type { StoreApi } from "zustand/vanilla";
import type { RecipeStepperState } from "../../_stores/recipeStepperStore";
import { toRunStep } from "./toRunStep";
import { deriveActiveStep } from "./deriveActiveStep";
import { useStore } from "zustand";

/** Derive activeStep + resolvedStep from reactive inputs and sync to store. */
export function useSyncDerivedStep(
  storeApi: StoreApi<RecipeStepperState>,
  instance: ExecutionInstance,
) {
  const fileCount = useStore(storeApi, (s) => s.files.length);
  const execution = core.executions.useExecutionState(instance);

  const resolvedStep = toRunStep(execution.status);
  const activeStep = deriveActiveStep(resolvedStep, fileCount);

  useLayoutEffect(() => {
    const prev = storeApi.getState();
    if (prev.activeStep !== activeStep || prev.resolvedStep !== resolvedStep) {
      storeApi.setState({
        activeStep,
        resolvedStep,
        isProcessing: resolvedStep === "running",
      });
    }
  }, [storeApi, activeStep, resolvedStep]);
}
