"use client";

import { useLayoutEffect } from "react";
import { core } from "@bnto/core";
import type { ExecutionInstance } from "@bnto/core";
import type { StoreApi } from "zustand/vanilla";
import type { RecipeFlowState } from "../../_stores/recipeFlowStore";
import { toRunPhase, deriveActiveStep } from "../stepMapping";
import { useStore } from "zustand";

/** Derive activeStep + resolvedPhase from reactive inputs and sync to store. */
export function useSyncDerivedStep(
  storeApi: StoreApi<RecipeFlowState>,
  instance: ExecutionInstance,
) {
  const fileCount = useStore(storeApi, (s) => s.files.length);
  const browserExec = core.executions.useExecutionState(instance);

  const resolvedPhase = toRunPhase(browserExec.status);
  const activeStep = deriveActiveStep(resolvedPhase, fileCount);

  useLayoutEffect(() => {
    const prev = storeApi.getState();
    if (prev.activeStep !== activeStep || prev.resolvedPhase !== resolvedPhase) {
      storeApi.setState({
        activeStep,
        resolvedPhase,
        isProcessing: resolvedPhase === "running",
      });
    }
  }, [storeApi, activeStep, resolvedPhase]);
}
