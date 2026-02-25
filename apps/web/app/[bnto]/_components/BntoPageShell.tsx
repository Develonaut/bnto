"use client";

import { useCallback, useState } from "react";
import {
  core,
  useRunQuota,
  useCurrentUser,
  useUploadFiles,
  useRunPredefined,
  useExecution,
  useBrowserExecution,
} from "@bnto/core";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { getRecipe } from "@/lib/menu";
import { BntoConfigPanel } from "./BntoConfigPanel";
import type { BntoConfigMap, BntoSlug } from "./configs/types";
import { DEFAULT_CONFIGS } from "./configs/types";
import { FileDropZone } from "./FileDropZone";
import { UpgradePrompt } from "./UpgradePrompt";
import { UploadProgress } from "./UploadProgress";
import { RunButton } from "./RunButton";
import type { RunPhase } from "./RunButton";
import { ExecutionProgress } from "./ExecutionProgress";
import { ExecutionResults } from "./ExecutionResults";
import { BrowserExecutionProgress } from "./BrowserExecutionProgress";
import { BrowserExecutionResults } from "./BrowserExecutionResults";
import { ErrorCard } from "./ErrorCard";
import { useBrowserEngine } from "@/lib/wasm/useBrowserEngine";
import { toBrowserPhase, toCloudPhase } from "./phaseMapping";

interface BntoPageShellProps {
  entry: BntoEntry;
}

/**
 * Client shell for bnto tool pages.
 *
 * Manages the full execution lifecycle with two paths:
 *
 * Browser path (Tier 1 bntos like compress-images):
 *   1. User drops files -> FileDropZone
 *   2. User clicks Run -> WASM processes files in a Web Worker
 *   3. Results appear as in-memory blobs -> download directly
 *
 * Cloud path (server-side bntos):
 *   1. User drops files -> FileDropZone
 *   2. User clicks Run -> upload files to R2
 *   3. Execution runs on Railway Go API
 *   4. Results downloaded from R2
 */
export function BntoPageShell({ entry }: BntoPageShellProps) {
  const { isPending: sessionPending, quotaExhausted } = useRunQuota();
  const { data: currentUser } = useCurrentUser();
  const [config, setConfig] = useState<BntoConfigMap[BntoSlug]>(
    DEFAULT_CONFIGS[entry.slug as BntoSlug] ?? {},
  );
  const [files, setFiles] = useState<File[]>([]);

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
  } = useBrowserExecution();

  // -- Cloud execution --
  const [executionId, setExecutionId] = useState<string | null>(null);
  const [cloudPhase, setCloudPhase] = useState<RunPhase>("idle");
  const [clientError, setClientError] = useState<string | null>(null);
  const { progress, upload, reset: resetUpload } = useUploadFiles();
  const { mutateAsync: startCloudExec } = useRunPredefined();
  const { data: cloudExecution } = useExecution(executionId ?? "");

  // -- Resolved phase (unified across both paths) --
  const resolvedPhase: RunPhase = isBrowserPath
    ? toBrowserPhase(browserExec.status)
    : toCloudPhase(cloudPhase, cloudExecution?.status);

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

    setClientError(null);
    try {
      setCloudPhase("uploading");
      const session = await upload(files);

      setCloudPhase("running");
      const id = await startCloudExec({
        slug: entry.slug,
        definition,
        sessionId: session.sessionId,
      });
      setExecutionId(String(id));
    } catch (e) {
      setCloudPhase("failed");
      setClientError(e instanceof Error ? e.message : "Something went wrong");
    }
  }, [
    entry.slug,
    files,
    config,
    isBrowserPath,
    browserExecute,
    upload,
    startCloudExec,
  ]);

  const handleReset = useCallback(() => {
    setFiles([]);
    if (isBrowserPath) {
      resetBrowser();
    } else {
      setExecutionId(null);
      setCloudPhase("idle");
      setClientError(null);
      resetUpload();
    }
  }, [isBrowserPath, resetBrowser, resetUpload]);

  const isProcessing =
    resolvedPhase === "uploading" || resolvedPhase === "running";

  return (
    <div
      className="container space-y-3 text-center"
      data-testid="bnto-shell"
      data-session={sessionPending ? "pending" : "ready"}
      data-user-id={
        process.env.NODE_ENV !== "production"
          ? (currentUser?.id ?? "")
          : undefined
      }
      data-execution-mode={isBrowserPath ? "browser" : "cloud"}
    >
      <h1 className="text-2xl tracking-tight md:text-4xl lg:text-5xl">
        {entry.h1}
      </h1>
      <p className="text-muted-foreground mx-auto max-w-xl leading-snug text-balance">
        {entry.description}
      </p>

      <div className="mx-auto max-w-2xl space-y-4 pt-4">
        {quotaExhausted && <UpgradePrompt slug={entry.slug} reason="quota" />}

        {!quotaExhausted && (
          <>
            <div className="text-left">
              <BntoConfigPanel
                slug={entry.slug}
                config={config}
                onChange={setConfig}
              />
            </div>

            <FileDropZone
              slug={entry.slug}
              value={files}
              onValueChange={setFiles}
              disabled={isProcessing}
            />

            {/* Browser execution feedback */}
            {isBrowserPath && browserExec.status === "processing" && (
              <BrowserExecutionProgress execution={browserExec} />
            )}
            {isBrowserPath && browserExec.status === "completed" && (
              <BrowserExecutionResults
                execution={browserExec}
                onDownload={downloadResult}
                onDownloadAll={downloadAll}
              />
            )}
            {isBrowserPath &&
              browserExec.status === "failed" &&
              browserExec.error && (
                <ErrorCard error={browserExec.error} />
              )}

            {/* Cloud execution feedback */}
            {!isBrowserPath &&
              progress.files.length > 0 &&
              resolvedPhase === "uploading" && (
                <UploadProgress files={progress.files} />
              )}
            {!isBrowserPath && executionId && resolvedPhase === "running" && (
              <ExecutionProgress executionId={executionId} />
            )}
            {!isBrowserPath &&
              executionId &&
              resolvedPhase === "completed" && (
                <ExecutionResults executionId={executionId} />
              )}
            {!isBrowserPath && executionId && resolvedPhase === "failed" && (
              <ExecutionProgress executionId={executionId} />
            )}
            {!isBrowserPath &&
              !executionId &&
              resolvedPhase === "failed" &&
              clientError && <ErrorCard error={clientError} />}

            <RunButton
              phase={resolvedPhase}
              hasFiles={files.length > 0}
              onRun={handleRun}
              onReset={handleReset}
            />
          </>
        )}
      </div>
    </div>
  );
}
