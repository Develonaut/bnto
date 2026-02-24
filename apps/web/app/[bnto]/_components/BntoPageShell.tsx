"use client";

import { useCallback, useState } from "react";
import {
  useRunQuota,
  useUploadFiles,
  useRunPredefined,
  useExecution,
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

interface BntoPageShellProps {
  entry: BntoEntry;
}

/**
 * Client shell for bnto tool pages.
 *
 * Manages the full execution lifecycle:
 *   1. User drops files → FileDropZone (controlled)
 *   2. User clicks Run → upload files to R2 via presigned URLs
 *   3. Upload completes → start predefined execution with sessionId
 *   4. Execution progress streams via Convex real-time subscription
 *   5. Execution completes → show output files with download buttons
 */
export function BntoPageShell({ entry }: BntoPageShellProps) {
  const { isPending: sessionPending, quotaExhausted } = useRunQuota();
  const [config, setConfig] = useState<BntoConfigMap[BntoSlug]>(
    DEFAULT_CONFIGS[entry.slug as BntoSlug] ?? {},
  );
  const [files, setFiles] = useState<File[]>([]);
  const [executionId, setExecutionId] = useState<string | null>(null);
  const [phase, setPhase] = useState<RunPhase>("idle");

  const { progress, upload, reset: resetUpload } = useUploadFiles();
  const { mutateAsync: startExecution } = useRunPredefined();
  const { data: execution } = useExecution(executionId ?? "");

  // Sync phase from execution status
  const resolvedPhase = resolvePhase(phase, execution?.status);

  const handleRun = useCallback(async () => {
    const definition = getRecipe(entry.slug)?.definition;
    if (!definition || files.length === 0) return;

    try {
      setPhase("uploading");
      const session = await upload(files);

      setPhase("running");
      const id = await startExecution({
        slug: entry.slug,
        definition,
        sessionId: session.sessionId,
      });
      setExecutionId(String(id));
    } catch {
      setPhase("failed");
    }
  }, [entry.slug, files, upload, startExecution]);

  const handleReset = useCallback(() => {
    setFiles([]);
    setExecutionId(null);
    setPhase("idle");
    resetUpload();
  }, [resetUpload]);

  const isProcessing = resolvedPhase === "uploading" || resolvedPhase === "running";

  return (
    <div
      className="container space-y-3 text-center"
      data-testid="bnto-shell"
      data-session={sessionPending ? "pending" : "ready"}
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

            {progress.files.length > 0 && resolvedPhase === "uploading" && (
              <UploadProgress files={progress.files} />
            )}

            {executionId && resolvedPhase === "running" && (
              <ExecutionProgress executionId={executionId} />
            )}

            {executionId && resolvedPhase === "completed" && (
              <ExecutionResults executionId={executionId} />
            )}

            {executionId && resolvedPhase === "failed" && (
              <ExecutionProgress executionId={executionId} />
            )}

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

/** Derive display phase from local phase + backend execution status. */
function resolvePhase(
  localPhase: RunPhase,
  executionStatus: string | undefined,
): RunPhase {
  if (localPhase === "uploading") return "uploading";
  if (!executionStatus || localPhase === "idle") return localPhase;

  switch (executionStatus) {
    case "pending":
    case "running":
      return "running";
    case "completed":
      return "completed";
    case "failed":
      return "failed";
    default:
      return localPhase;
  }
}
