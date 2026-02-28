"use client";

import { useEffect, useRef } from "react";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { Animate } from "@/components/ui/Animate";
import { Container } from "@/components/ui/Container";
import { FileUpload } from "@/components/ui/FileUpload";
import { Heading } from "@/components/ui/Heading";
import { UploadIcon } from "@/components/ui/icons";
import { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { getAcceptedTypes, toDropzoneAccept } from "../_lib/getAcceptedTypes";
import { ErrorCard } from "./ErrorCard";
import { FileCard } from "./FileCard";
import { PhaseIndicator } from "./PhaseIndicator";
import { RecipeConfigSection } from "./RecipeConfigSection";
import { RecipeResultsSection } from "./RecipeResultsSection";
import { RecipeToolbar } from "./RecipeToolbar";
import type { RunPhase } from "./RunButton";
import { ToolbarProgress } from "./ToolbarProgress";
import { UpgradePrompt } from "./UpgradePrompt";

/** Map the unified RunPhase + file count to the 3-step PhaseIndicator. */
function deriveActivePhase(
  resolvedPhase: RunPhase,
  fileCount: number,
): 1 | 2 | 3 {
  switch (resolvedPhase) {
    case "uploading":
    case "running":
    case "completed":
    case "failed":
      return 3;
    default:
      return fileCount > 0 ? 2 : 1;
  }
}

/**
 * Recipe page orchestrator with Motorway styling.
 *
 * Composes the progressive phase flow:
 *   Phase 1 (Files)     → dropzone
 *   Phase 2 (Configure) → file grid + config panel
 *   Phase 3 (Results)   → execution progress / results
 *
 * Uses useRecipeFlow for all state — this component is pure composition.
 */
export function RecipeShell({ entry }: { entry: BntoEntry }) {
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
    handleResetExecution,
  } = useRecipeFlow({ entry });

  const { label } = getAcceptedTypes(entry.slug);
  const accept = toDropzoneAccept(entry.slug);
  const activePhase = deriveActivePhase(resolvedPhase, files.length);

  const didAutoDownload = useRef(false);

  // FIXME: Code smell.
  // Reset the auto-download guard when execution resets
  useEffect(() => {
    if (resolvedPhase === "idle") didAutoDownload.current = false;
  }, [resolvedPhase]);

  // Auto-download when execution completes
  useEffect(() => {
    if (resolvedPhase === "completed" && !didAutoDownload.current) {
      didAutoDownload.current = true;
      downloadAll();
    }
  }, [resolvedPhase, downloadAll]);

  return (
    <Container
      size="md"
      className="space-y-6 text-center"
      data-testid="bnto-shell"
      data-session={sessionPending ? "pending" : "ready"}
      data-user-id={
        process.env.NODE_ENV !== "production"
          ? (currentUser?.id ?? "")
          : undefined
      }
      data-execution-mode={isBrowserPath ? "browser" : "cloud"}
    >
      <PhaseIndicator activePhase={activePhase} />

      <Animate.FadeIn>
        <Heading level={1}>{entry.h1}</Heading>
      </Animate.FadeIn>
      <p className="text-muted-foreground mx-auto max-w-xl leading-snug text-balance">
        {entry.description}
      </p>

      {quotaExhausted && <UpgradePrompt slug={entry.slug} reason="quota" />}

      {!quotaExhausted && (
        <FileUpload
          value={files}
          onValueChange={setFiles}
          accept={accept}
          multiple
          disabled={isProcessing}
        >
          {/* Phase 1: Dropzone — full width to match Phases 2–3 */}
          {activePhase === 1 && (
            <Animate.SlideUp>
              <FileUpload.Dropzone className="gap-3 px-4 py-8 sm:px-6 sm:py-10">
                <div className="rounded-full bg-muted p-3 text-muted-foreground">
                  <UploadIcon className="size-6" />
                </div>
                <div className="text-center">
                  <p className="text-sm font-medium text-foreground">
                    Drag & drop files here
                  </p>
                  <p className="mt-1 text-xs text-muted-foreground">
                    or click to browse &middot; accepts {label}
                  </p>
                </div>
              </FileUpload.Dropzone>
            </Animate.SlideUp>
          )}

          {/* Phases 2–3: Toolbar persists, content below changes */}
          {(activePhase === 2 || activePhase === 3) && (
            <div className="space-y-4 text-left">
              <RecipeToolbar
                activePhase={activePhase as 2 | 3}
                resolvedPhase={resolvedPhase}
                isProcessing={isProcessing}
                fileCount={files.length}
                onBack={
                  activePhase === 3
                    ? handleResetExecution
                    : () => setFiles([])
                }
                onRun={handleRun}
                onDownloadAll={downloadAll}
                centerContent={
                  activePhase === 2 ? (
                    <RecipeConfigSection
                      slug={entry.slug}
                      config={config}
                      onChange={setConfig}
                    />
                  ) : isBrowserPath &&
                    (browserExec.status === "processing" ||
                      browserExec.status === "completed") ? (
                    <ToolbarProgress execution={browserExec} />
                  ) : undefined
                }
              />

              {/* Phase 3 (browser): Error card above the grid */}
              {activePhase === 3 &&
                isBrowserPath &&
                browserExec.status === "failed" &&
                browserExec.error && (
                  <Animate.SlideUp>
                    <ErrorCard error={browserExec.error} />
                  </Animate.SlideUp>
                )}

              {/* Phase 3 (cloud): Upload/execution progress — cloud path not yet persistent */}
              {activePhase === 3 && !isBrowserPath && (
                <RecipeResultsSection
                  isBrowserPath={false}
                  resolvedPhase={resolvedPhase}
                  browserExec={browserExec}
                  onDownload={downloadResult}
                  executionId={executionId}
                  uploadProgress={uploadProgress}
                  clientError={clientError}
                />
              )}

              {/* Persistent file grid — always visible in Phases 2 and 3 */}
              <Animate.BouncyStagger className="flex flex-col gap-2">
                {files.map((file, i) => {
                  const result =
                    activePhase === 3 && isBrowserPath
                      ? browserExec.results[i]
                      : undefined;
                  const isFileProcessing =
                    activePhase === 3 &&
                    isBrowserPath &&
                    browserExec.status === "processing" &&
                    browserExec.fileProgress?.fileIndex === i;

                  return (
                    <FileCard
                      key={`${file.name}-${file.size}-${file.lastModified}`}
                      file={file}
                      result={result}
                      isProcessing={isFileProcessing}
                      isExecuting={activePhase === 3}
                      onDelete={() => setFiles(files.filter((_, j) => j !== i))}
                      onDownload={downloadResult}
                    />
                  );
                })}
              </Animate.BouncyStagger>
            </div>
          )}
        </FileUpload>
      )}
    </Container>
  );
}
