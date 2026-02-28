"use client";

import { useCallback, useEffect, useState } from "react";
import { useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import { core } from "@bnto/core";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { getRecipe } from "@/lib/menu";
import type { BntoConfigMap, BntoSlug } from "../_components/configs/types";
import { DEFAULT_CONFIGS } from "../_components/configs/types";
import type { RunPhase } from "../_components/RunButton";
import { toBrowserPhase, toCloudPhase } from "../_components/phaseMapping";
import { useBrowserEngine } from "@/lib/wasm/useBrowserEngine";

/**
 * Manages the full recipe page lifecycle — files, config, execution, results.
 *
 * Composes three layers:
 *   1. Page-scoped recipeFlowStore (files, config, cloud state)
 *   2. Browser execution (WASM via @bnto/core)
 *   3. Cloud execution (R2 upload + Railway via @bnto/core)
 *
 * RecipeShell consumes this hook and renders the return values —
 * no state management in the component.
 */
export function useRecipeFlow({ entry }: { entry: BntoEntry }) {
  // -- Page-scoped store (created once per mount via lazy init) --
  const [store] = useState(() =>
    core.recipe.createStore(
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

  // -- Session / quota --
  const { isPending: sessionPending, quotaExhausted } = core.user.useRunQuota();
  const { data: currentUser } = core.user.useCurrentUser();

  // -- Execution path: browser (WASM) vs cloud (R2 + Go API) --
  const isBrowserPath = core.browser.hasImplementation(entry.slug);
  useBrowserEngine(isBrowserPath);

  // -- Browser execution --
  const {
    execution: browserExec,
    execute: browserExecute,
    downloadResult,
    downloadAll,
    reset: resetBrowser,
  } = core.browser.useBrowserExecution();

  // Reset singleton browser execution store on mount so stale state
  // from a previous recipe doesn't trigger auto-download or show old results.
  useEffect(() => {
    core.browser.reset();
  }, []);

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
    (newFiles: File[]) => store.getState().setFiles(newFiles),
    [store],
  );

  const setConfig = useCallback(
    (newConfig: BntoConfigMap[BntoSlug]) =>
      store.getState().setConfig(newConfig as Record<string, unknown>),
    [store],
  );

  const handleDownloadAll = useCallback(
    () => downloadAll(entry.slug),
    [downloadAll, entry.slug],
  );

  const handleRun = useCallback(async () => {
    if (files.length === 0) return;

    if (isBrowserPath) {
      await browserExecute(
        entry.slug,
        files,
        config as Record<string, unknown>,
      );
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
      store.getState().failCloud(
        e instanceof Error ? e.message : "Something went wrong",
      );
    }
  }, [
    entry.slug,
    files,
    config,
    isBrowserPath,
    browserExecute,
    upload,
    startCloudExec,
    store,
  ]);

  const handleResetExecution = useCallback(() => {
    if (isBrowserPath) {
      resetBrowser();
    } else {
      store.setState({
        executionId: null,
        cloudPhase: "idle" as const,
        clientError: null,
      });
      resetUpload();
    }
  }, [isBrowserPath, resetBrowser, resetUpload, store]);

  const handleReset = useCallback(() => {
    setFiles([]);
    handleResetExecution();
  }, [setFiles, handleResetExecution]);

  return {
    // Session
    sessionPending,
    quotaExhausted,
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
