"use client";

import { useCallback, useMemo, useState } from "react";
import { useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import { core, deriveAcceptedTypes } from "@bnto/core";
import { createRecipeFlowStore } from "../_stores/recipeFlowStore";
import type { BrowserFileResult } from "@bnto/core";
import { getRecipeBySlug } from "@bnto/nodes";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { toDropzoneAccept } from "@bnto/ui";
import type { BntoConfigMap, BntoSlug } from "../_components/configs/types";
import { DEFAULT_CONFIGS } from "../_components/configs/types";
import type { RunPhase } from "../_components/RunButton";
import { toBrowserPhase, toCloudPhase } from "../_components/phaseMapping";
import { runRecipeAction } from "./runRecipeAction";

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

  // -- Definition + I/O config (derived from input node, not slug) --
  const recipe = getRecipeBySlug(entry.slug);
  const definition = recipe?.definition;
  const { acceptLabel, dropzoneAccept } = useMemo(() => {
    if (!definition) return { acceptLabel: "files", dropzoneAccept: undefined };
    const { accept, label } = deriveAcceptedTypes(definition);
    return { acceptLabel: label, dropzoneAccept: toDropzoneAccept(accept) };
  }, [definition]);

  // -- Execution path: browser (WASM) vs cloud (R2 + Go API) --
  // All predefined recipes with definitions are browser-capable (Tier 1 = WASM).
  const isBrowserPath = !!recipe;

  // -- Per-instance browser execution (isolated store per page mount) --
  const [browserInstance] = useState(() => core.executions.createExecution());
  const browserExec = core.executions.useExecutionState(browserInstance);

  // -- Cloud execution --
  const { progress: uploadProgress, upload, reset: resetUpload } = core.uploads.useUploadFiles();
  const { mutateAsync: startCloudExec } = core.executions.useRunPredefined();
  const { data: cloudExecution } = core.executions.useExecution(executionId ?? "");

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

  const handleRun = useCallback(
    () =>
      runRecipeAction({
        slug: entry.slug,
        files,
        config: config as Record<string, unknown>,
        isBrowserPath,
        browserInstance,
        definition,
        upload,
        startCloudExec,
        onStartUpload: () => store.getState().startUpload(),
        onStartExecution: (id) => store.getState().startExecution(id),
        onFail: (msg) => store.getState().failCloud(msg),
      }),
    [
      entry.slug,
      files,
      config,
      definition,
      isBrowserPath,
      browserInstance,
      upload,
      startCloudExec,
      store,
    ],
  );

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
    // I/O (derived from definition's input node)
    acceptLabel,
    dropzoneAccept,
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
