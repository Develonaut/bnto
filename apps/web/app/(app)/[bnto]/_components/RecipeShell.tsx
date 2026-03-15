"use client";

import type { ReactNode } from "react";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { SlideUp, BouncyStagger, FileUpload, FileUploadDropzone, Grid, Stack } from "@bnto/ui";
import { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { DropzoneContent } from "./DropzoneContent";
import { ErrorCard } from "./ErrorCard";
import { FileCard } from "./FileCard";
import { PhaseIndicator } from "./PhaseIndicator";
import { RecipeConfigSection } from "./RecipeConfigSection";
import { RecipeResultsSection } from "./RecipeResultsSection";
import { RecipeToolbar } from "./RecipeToolbar";
import { ToolbarProgress } from "./ToolbarProgress";
import { SessionMarker } from "./SessionMarker";
import { deriveActivePhase } from "./phaseMapping";

/**
 * Recipe page interactive flow — client island.
 *
 * Composes the progressive phase flow:
 *   Phase 1 (Files)     → dropzone
 *   Phase 2 (Configure) → file grid + config panel
 *   Phase 3 (Results)   → execution progress / results
 *
 * Static header (h1, description, editor link) is server-rendered
 * in the parent page.tsx. This component handles the interactive flow only.
 */
export function RecipeShell({ entry, children }: { entry: BntoEntry; children?: ReactNode }) {
  const {
    isBrowserPath,
    acceptLabel,
    dropzoneAccept,
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

  const activePhase = deriveActivePhase(resolvedPhase, files.length);

  return (
    <div
      className="space-y-6"
      data-testid="bnto-shell"
      data-session="ready"
      data-execution-mode={isBrowserPath ? "browser" : "cloud"}
    >
      <SessionMarker />
      <PhaseIndicator activePhase={activePhase} />

      {/* Static header — server-rendered, passed as children from page.tsx */}
      {children}

      <FileUpload
        value={files}
        onValueChange={setFiles}
        accept={dropzoneAccept}
        multiple
        disabled={isProcessing}
      >
        {/* Phase 1: Dropzone — full width to match Phases 2–3 */}
        {activePhase === 1 && (
          <SlideUp>
            <FileUploadDropzone className="gap-3 px-4 py-8 sm:px-6 sm:py-10">
              <DropzoneContent label={acceptLabel} />
            </FileUploadDropzone>
          </SlideUp>
        )}

        {/* Phases 2–3: Toolbar persists, content below changes */}
        {(activePhase === 2 || activePhase === 3) && (
          <Stack className="gap-4 text-left">
            <RecipeToolbar
              activePhase={activePhase as 2 | 3}
              resolvedPhase={resolvedPhase}
              isProcessing={isProcessing}
              fileCount={files.length}
              onBack={activePhase === 3 ? handleResetExecution : () => setFiles([])}
              onRun={handleRun}
              onDownloadAll={downloadAll}
              centerContent={
                activePhase === 2 ? (
                  <RecipeConfigSection slug={entry.slug} config={config} onChange={setConfig} />
                ) : isBrowserPath &&
                  (browserExec.status === "processing" || browserExec.status === "completed") ? (
                  <ToolbarProgress execution={browserExec} />
                ) : undefined
              }
            />

            {/* Phase 3 (browser): Error card above the grid */}
            {activePhase === 3 &&
              isBrowserPath &&
              browserExec.status === "failed" &&
              browserExec.error && (
                <SlideUp>
                  <ErrorCard error={browserExec.error} />
                </SlideUp>
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

            {/* Persistent file grid — always visible in Phases 2 and 3.
             * Columns adapt to file count: 1 file = full width,
             * 2 files = 2 cols on md+, 3+ files = 2 cols on md / 3 cols on lg. */}
            <BouncyStagger asChild>
              <Grid
                cols={1}
                gap="sm"
                role="list"
                aria-label="Selected files"
                className={
                  files.length >= 3
                    ? "md:grid-cols-2 lg:grid-cols-3"
                    : files.length === 2
                      ? "md:grid-cols-2"
                      : undefined
                }
              >
                {files.map((file, i) => {
                  const result =
                    activePhase === 3 && isBrowserPath ? browserExec.results[i] : undefined;
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
              </Grid>
            </BouncyStagger>
          </Stack>
        )}
      </FileUpload>
    </div>
  );
}
