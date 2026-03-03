"use client";

import { useCallback, useState } from "react";
import { useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import { core, createRecipeFlowStore } from "@bnto/core";
import type { BrowserFileResult } from "@bnto/core";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { getRecipe } from "@/lib/menu";
import type { BntoConfigMap, BntoSlug } from "../_components/configs/types";
import { DEFAULT_CONFIGS } from "../_components/configs/types";
import type { RunPhase } from "../_components/RunButton";
import { toBrowserPhase, toCloudPhase } from "../_components/phaseMapping";

/**
 * Manages the full recipe page lifecycle — files, config, execution, results.
 *
 * Composes three layers:
 *   1. Page-scoped recipeFlowStore (files, config, cloud state)
 *   2. Per-instance browser execution (WASM via @bnto/core)
 *   3. Cloud execution (R2 upload + Railway via @bnto/core)
 *
 * RecipeShell consumes this hook and renders the return values —
 * no state management in the component.
 */
export function useRecipeFlow({ entry }: { entry: BntoEntry }) {
  // -- Page-scoped store (created once per mount via lazy init) --
  const [store] = useState(() =>
    createRecipeFlowStore(
      (DEFAULT_CONFIGS[entry.slug as BntoSlug] ?? {}) as Record<string, unknown>,
    ),
  );

  const { files, config, executionId, cloudPhase, clientError } = useStore(
    store,
    useShallow((s) => ({
      files: s.files,
      config: s.config,
      executionId: s.executionId,
      cloudPhase: s.cloudPhase,
      clientError: s.clientError,
    })),
  );

  const { data: currentUser } = core.user.useCurrentUser();

  // -- Execution path: browser (WASM) vs cloud (R2 + Go API) --
  const isBrowserPath = core.executions.hasImplementation(entry.slug);

  // -- Per-instance browser execution (isolated store per page mount) --
  const [browserInstance] = useState(() => core.executions.createExecution());
  const browserExec = core.executions.useExecutionState(browserInstance);

  // -- Cloud execution --
  const { progress: uploadProgress, upload, reset: resetUpload } =
    core.uploads.useUploadFiles();
  const { mutateAsync: startCloudExec } = core.executions.useRunPredefined();
  const { data: cloudExecution } = core.executions.useExecution(
    executionId ?? "",
  );

  // -- Resolved phase (unified across both paths) --
  const resolvedPhase: RunPhase = isBrowserPath
    ? toBrowserPhase(browserExec.status)
    : toCloudPhase(cloudPhase, cloudExecution?.status);

  // -- Actions --
  const setFiles = useCallback(
    (newFiles: File[]) => {
      store.getState().setFiles(newFiles);
      if (newFiles.length > 0) {
        core.telemetry.capture("files_added", {
          slug: entry.slug,
          fileCount: newFiles.length,
          totalBytes: newFiles.reduce((sum, f) => sum + f.size, 0),
        });
      }
    },
    [store, entry.slug],
  );

  const setConfig = useCallback(
    (newConfig: BntoConfigMap[BntoSlug]) =>
      store.getState().setConfig(newConfig as Record<string, unknown>),
    [store],
  );

  const downloadResult = useCallback((result: BrowserFileResult) => {
    core.executions.downloadResult(result);
  }, []);

  const handleDownloadAll = useCallback(() => {
    core.executions.downloadAllResults(browserExec.results, entry.slug);
    core.telemetry.capture("result_downloaded", {
      slug: entry.slug,
      fileCount: browserExec.results.length,
    });
  }, [browserExec.results, entry.slug]);

  const handleRun = useCallback(async () => {
    if (files.length === 0) return;

    const totalBytes = files.reduce((sum, f) => sum + f.size, 0);
    const runProps = {
      slug: entry.slug,
      fileCount: files.length,
      totalBytes,
      executionPath: isBrowserPath ? "browser" : "cloud",
    };

    core.telemetry.capture("recipe_run_started", runProps);
    const startTime = Date.now();

    if (isBrowserPath) {
      const result = await browserInstance.run(
        entry.slug,
        files,
        config as Record<string, unknown>,
      );

      const durationMs = Date.now() - startTime;

      if (result.status === "completed" && result.results.length > 0) {
        const outputBytes = result.results.reduce((sum, r) => sum + r.blob.size, 0);
        core.telemetry.capture("recipe_run_completed", {
          ...runProps,
          durationMs,
          outputFileCount: result.results.length,
          outputBytes,
        });
        await core.executions.downloadAllResults(result.results, entry.slug);
      } else if (result.status === "failed") {
        core.telemetry.capture("recipe_run_failed", {
          ...runProps,
          durationMs,
          error: result.error ?? "unknown",
        });
      }
      return;
    }

    // Cloud path
    const definition = getRecipe(entry.slug)?.definition;
    if (!definition) return;

    try {
      store.getState().startUpload();
      const session = await upload(files);
      const id = await startCloudExec({
        slug: entry.slug,
        definition,
        sessionId: session.sessionId,
      });
      store.getState().startExecution(String(id));
    } catch (e) {
      const durationMs = Date.now() - startTime;
      core.telemetry.capture("recipe_run_failed", {
        ...runProps,
        durationMs,
        error: e instanceof Error ? e.message : "unknown",
      });
      store.getState().failCloud(
        e instanceof Error ? e.message : "Something went wrong",
      );
    }
  }, [
    entry.slug,
    files,
    config,
    isBrowserPath,
    browserInstance,
    upload,
    startCloudExec,
    store,
  ]);

  const handleResetExecution = useCallback(() => {
    if (isBrowserPath) {
      browserInstance.reset();
    } else {
      store.setState({
        executionId: null,
        cloudPhase: "idle" as const,
        clientError: null,
      });
      resetUpload();
    }
  }, [isBrowserPath, browserInstance, resetUpload, store]);

  const handleReset = useCallback(() => {
    setFiles([]);
    handleResetExecution();
  }, [setFiles, handleResetExecution]);

  return {
    currentUser: currentUser ?? null,
    // Execution path
    isBrowserPath,
    // Files + config (from store)
    files,
    config: config as BntoConfigMap[BntoSlug],
    setFiles,
    setConfig,
    // Browser execution
    browserExec,
    downloadResult,
    downloadAll: handleDownloadAll,
    // Cloud execution
    executionId,
    cloudPhase,
    clientError,
    uploadProgress,
    cloudExecution,
    // Unified
    resolvedPhase,
    isProcessing: resolvedPhase === "uploading" || resolvedPhase === "running",
    handleRun,
    handleReset,
    handleResetExecution,
  };
}
