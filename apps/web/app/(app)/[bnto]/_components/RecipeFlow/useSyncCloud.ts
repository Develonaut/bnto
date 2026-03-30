"use client";

import { core } from "@bnto/core";
import type { ExecutionInstance } from "@bnto/core";
import type { StoreApi } from "zustand/vanilla";
import type { RecipeFlowState } from "../../_stores/recipeFlowStore";
import type { MutableCloudRefs } from "../../_stores/createMutableCloudRefs";

/** Sync cloud hook values + browser results into the mutable refs. */
export function useSyncCloud(
  storeApi: StoreApi<RecipeFlowState>,
  cloudRefs: MutableCloudRefs,
  instance: ExecutionInstance,
) {
  const { progress, upload, reset: resetUpload } = core.uploads.useUploadFiles();
  const { mutateAsync: startExec } = core.executions.useRunPredefined();
  core.executions.useExecution(storeApi.getState().executionId ?? "");

  cloudRefs.syncUploadFn(upload);
  cloudRefs.syncStartCloudExecFn(startExec);
  cloudRefs.syncResetUploadFn(resetUpload);
  cloudRefs.syncUploadProgress(progress);

  const browserExec = core.executions.useExecutionState(instance);
  cloudRefs.syncBrowserResults(browserExec.results);
}
