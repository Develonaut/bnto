"use client";

import { useCallback } from "react";
import { core } from "@bnto/core";
import type { BrowserFileResult, Definition, ExecutionInstance } from "@bnto/core";
import type { RecipeFlowState } from "../_stores/recipeFlowStore";
import { runRecipeAction } from "./runRecipeAction";

interface UseRecipeActionsParams {
  slug: string;
  store: RecipeFlowStore;
  files: File[];
  isBrowserPath: boolean;
  browserInstance: ExecutionInstance;
  browserResults: BrowserFileResult[];
  definition: Definition | undefined;
  upload: (files: File[]) => Promise<{ sessionId: string }>;
  startCloudExec: (args: {
    slug: string;
    definition: Definition;
    sessionId: string;
  }) => Promise<unknown>;
  resetUpload: () => void;
}

type RecipeFlowStore = {
  getState: () => RecipeFlowState;
  setState: (partial: Partial<RecipeFlowState>) => void;
};

/** Recipe page action callbacks — files, config, run, reset, download. */
export function useRecipeActions(params: UseRecipeActionsParams) {
  const { slug, store, browserResults } = params;
  const fileActions = useFileActions(slug, store);
  const downloadActions = useDownloadActions(slug, browserResults);
  const handleRun = useRunAction(params);
  const handleResetExecution = useResetAction(params);

  return {
    ...fileActions,
    ...downloadActions,
    handleRun,
    handleResetExecution,
    handleReset: useCallback(() => {
      fileActions.setFiles([]);
      handleResetExecution();
    }, [fileActions, handleResetExecution]),
  };
}

/** File and config mutation callbacks. */
function useFileActions(slug: string, store: RecipeFlowStore) {
  const setFiles = useCallback(
    (newFiles: File[]) => {
      store.getState().setFiles(newFiles);
      if (newFiles.length > 0) {
        core.telemetry.capture("files_added", {
          slug,
          fileCount: newFiles.length,
          totalBytes: newFiles.reduce((sum, f) => sum + f.size, 0),
        });
      }
    },
    [store, slug],
  );

  const setNodeParam = useCallback(
    (nodeId: string, paramName: string, value: unknown) =>
      store.getState().setNodeParam(nodeId, paramName, value),
    [store],
  );

  return { setFiles, setNodeParam };
}

/** Download result callbacks. */
function useDownloadActions(slug: string, browserResults: BrowserFileResult[]) {
  const downloadResult = useCallback((result: BrowserFileResult) => {
    core.executions.downloadResult(result);
  }, []);

  const downloadAll = useCallback(() => {
    core.executions.downloadAllResults(browserResults, slug);
    core.telemetry.capture("result_downloaded", { slug, fileCount: browserResults.length });
  }, [browserResults, slug]);

  return { downloadResult, downloadAll };
}

/** Run execution callback. */
function useRunAction({
  slug,
  store,
  files,
  isBrowserPath,
  browserInstance,
  definition,
  upload,
  startCloudExec,
}: UseRecipeActionsParams) {
  return useCallback(
    () =>
      runRecipeAction({
        slug,
        files,
        config: store.getState().config,
        isBrowserPath,
        browserInstance,
        definition,
        upload,
        startCloudExec,
        onStartUpload: () => store.getState().startUpload(),
        onStartExecution: (id) => store.getState().startExecution(id),
        onFail: (msg) => store.getState().failCloud(msg),
      }),
    [slug, files, definition, isBrowserPath, browserInstance, upload, startCloudExec, store],
  );
}

/** Reset execution callback. */
function useResetAction({
  isBrowserPath,
  browserInstance,
  store,
  resetUpload,
}: UseRecipeActionsParams) {
  return useCallback(() => {
    if (isBrowserPath) {
      browserInstance.reset();
    } else {
      store.setState({ executionId: null, cloudPhase: "idle" as const, clientError: null });
      resetUpload();
    }
  }, [isBrowserPath, browserInstance, resetUpload, store]);
}
