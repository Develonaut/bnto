"use client";

import { useLayoutEffect } from "react";
import { core } from "@bnto/core";
import type { ExecutionInstance } from "@bnto/core";
import type { StoreApi } from "zustand/vanilla";
import type { RecipeFlowState } from "../../_stores/recipeFlowStore";
import type { RecipeDefn } from "../../_stores/recipeFlowContext";
import { toBrowserStep, toCloudStep, deriveActiveStep } from "../stepMapping";
import { useStore } from "zustand";

/** Derive activeStep + resolvedPhase from reactive inputs and sync to store. */
export function useSyncDerivedStep(
  storeApi: StoreApi<RecipeFlowState>,
  instance: ExecutionInstance,
  defn: RecipeDefn,
) {
  const fileCount = useStore(storeApi, (s) => s.files.length);
  const cloudPhase = useStore(storeApi, (s) => s.cloudPhase);
  const browserExec = core.executions.useExecutionState(instance);
  const { data: cloudExecution } = core.executions.useExecution(
    storeApi.getState().executionId ?? "",
  );

  const resolvedPhase = defn.isBrowserPath
    ? toBrowserStep(browserExec.status)
    : toCloudStep(cloudPhase, cloudExecution?.status);
  const activeStep = deriveActiveStep(resolvedPhase, fileCount);

  useLayoutEffect(() => {
    const prev = storeApi.getState();
    if (prev.activeStep !== activeStep || prev.resolvedPhase !== resolvedPhase) {
      storeApi.setState({
        activeStep,
        resolvedPhase,
        isProcessing: resolvedPhase === "uploading" || resolvedPhase === "running",
      });
    }
  }, [storeApi, activeStep, resolvedPhase]);
}
