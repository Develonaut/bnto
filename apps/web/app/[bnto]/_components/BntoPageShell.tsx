"use client";

import type { BntoEntry } from "@/lib/bntoRegistry";
import { Heading } from "@/components/ui/Heading";
import { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { BntoConfigPanel } from "./BntoConfigPanel";
import { FileDropZone } from "./FileDropZone";
import { UpgradePrompt } from "./UpgradePrompt";
import { UploadProgress } from "./UploadProgress";
import { RunButton } from "./RunButton";
import { ExecutionProgress } from "./ExecutionProgress";
import { ExecutionResults } from "./ExecutionResults";
import { BrowserExecutionProgress } from "./BrowserExecutionProgress";
import { BrowserExecutionResults } from "./BrowserExecutionResults";
import { ErrorCard } from "./ErrorCard";

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
  const {
    sessionPending,
    quotaExhausted,
    currentUser,
    isBrowserPath,
    files,
    config,
    setFiles,
    setConfig,
    browserExec,
    downloadResult,
    downloadAll,
    executionId,
    uploadProgress,
    resolvedPhase,
    isProcessing,
    clientError,
    handleRun,
    handleReset,
  } = useRecipeFlow({ entry });

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
      <Heading level={1}>
        {entry.h1}
      </Heading>
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
              uploadProgress.files.length > 0 &&
              resolvedPhase === "uploading" && (
                <UploadProgress files={uploadProgress.files} />
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
