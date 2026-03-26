"use client";

import { useState } from "react";
import { useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import { core } from "@bnto/core";
import { createRecipeFlowStore } from "../_stores/recipeFlowStore";
import type { BntoEntry } from "@/lib/bntoRegistry";
import type { BntoConfigMap, BntoSlug } from "../_components/configs/types";
import { DEFAULT_CONFIGS } from "../_components/configs/types";
import type { RunPhase } from "../_components/RunButton";
import { toBrowserPhase, toCloudPhase } from "../_components/phaseMapping";
import { useRecipeDefinition } from "./useRecipeDefinition";
import { useRecipeActions } from "./useRecipeActions";

interface FlowParts {
  defn: ReturnType<typeof useRecipeDefinition>;
  storeState: ReturnType<typeof useRecipeFlowStore>;
  actions: ReturnType<typeof useRecipeActions>;
  browser: ReturnType<typeof useRecipeBrowserExec>;
  cloud: ReturnType<typeof useRecipeCloudExec>;
  resolvedPhase: RunPhase;
}

function assembleFlowResult({
  defn,
  storeState,
  actions,
  browser,
  cloud,
  resolvedPhase,
}: FlowParts) {
  return {
    ...defn,
    ...storeState,
    ...actions,
    config: storeState.config as BntoConfigMap[BntoSlug],
    browserExec: browser.exec,
    cloudExecution: cloud.execution,
    uploadProgress: cloud.uploadProgress,
    resolvedPhase,
    isProcessing: resolvedPhase === "uploading" || resolvedPhase === "running",
  };
}

/** Manages the full recipe page lifecycle — files, config, execution, results. */
export function useRecipeFlow({ entry }: { entry: BntoEntry }) {
  const storeState = useRecipeFlowStore(entry.slug);
  const defn = useRecipeDefinition(entry.slug);
  const browser = useRecipeBrowserExec();
  const cloud = useRecipeCloudExec(storeState.executionId);

  const resolvedPhase: RunPhase = defn.isBrowserPath
    ? toBrowserPhase(browser.exec.status)
    : toCloudPhase(storeState.cloudPhase, cloud.execution?.status);

  const actions = useRecipeActions({
    slug: entry.slug,
    store: storeState.store,
    files: storeState.files,
    isBrowserPath: defn.isBrowserPath,
    browserInstance: browser.instance,
    browserResults: browser.exec.results,
    definition: defn.definition,
    upload: cloud.upload,
    startCloudExec: cloud.startExec,
    resetUpload: cloud.resetUpload,
  });

  return assembleFlowResult({ defn, storeState, actions, browser, cloud, resolvedPhase });
}

/** Page-scoped store — files, config, cloud state. */
function useRecipeFlowStore(slug: string) {
  const [store] = useState(() =>
    createRecipeFlowStore((DEFAULT_CONFIGS[slug as BntoSlug] ?? {}) as Record<string, unknown>),
  );
  const state = useStore(
    store,
    useShallow((s) => ({
      files: s.files,
      config: s.config,
      executionId: s.executionId,
      cloudPhase: s.cloudPhase,
      clientError: s.clientError,
    })),
  );
  return { store, ...state };
}

/** Per-instance browser execution (isolated per page mount). */
function useRecipeBrowserExec() {
  const [instance] = useState(() => core.executions.createExecution());
  const exec = core.executions.useExecutionState(instance);
  return { instance, exec };
}

/** Cloud execution hooks — upload, start, status. */
function useRecipeCloudExec(executionId: string | null) {
  const { progress: uploadProgress, upload, reset: resetUpload } = core.uploads.useUploadFiles();
  const { mutateAsync: startExec } = core.executions.useRunPredefined();
  const { data: execution } = core.executions.useExecution(executionId ?? "");
  return { uploadProgress, upload, resetUpload, startExec, execution };
}
