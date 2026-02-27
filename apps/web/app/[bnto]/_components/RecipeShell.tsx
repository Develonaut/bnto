"use client";

import type { BntoEntry } from "@/lib/bntoRegistry";
import { Animate } from "@/components/ui/Animate";
import { Container } from "@/components/ui/Container";
import { FileUpload } from "@/components/ui/FileUpload";
import { Grid } from "@/components/ui/Grid";
import { Heading } from "@/components/ui/Heading";
import { UploadIcon } from "@/components/ui/icons";
import { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { getAcceptedTypes, toDropzoneAccept } from "../_lib/getAcceptedTypes";
import { FileCard } from "./FileCard";
import { PhaseIndicator } from "./PhaseIndicator";
import { RecipeConfigSection } from "./RecipeConfigSection";
import { RecipeResultsSection } from "./RecipeResultsSection";
import { RunButton } from "./RunButton";
import type { RunPhase } from "./RunButton";
import { UpgradePrompt } from "./UpgradePrompt";

/** Map the unified RunPhase + file count to the 3-step PhaseIndicator. */
function deriveActivePhase(resolvedPhase: RunPhase, fileCount: number): 1 | 2 | 3 {
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
    handleReset,
  } = useRecipeFlow({ entry });

  const { label } = getAcceptedTypes(entry.slug);
  const accept = toDropzoneAccept(entry.slug);
  const activePhase = deriveActivePhase(resolvedPhase, files.length);

  return (
    <Container
      size="sm"
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
      <Animate.FadeIn>
        <Heading level={1}>{entry.h1}</Heading>
      </Animate.FadeIn>
      <p className="text-muted-foreground mx-auto max-w-xl leading-snug text-balance">
        {entry.description}
      </p>

      <PhaseIndicator activePhase={activePhase} />

      {quotaExhausted && <UpgradePrompt slug={entry.slug} reason="quota" />}

      {!quotaExhausted && (
        <FileUpload
          value={files}
          onValueChange={setFiles}
          accept={accept}
          multiple
          disabled={isProcessing}
        >
          {/* Phase 1: Dropzone */}
          {activePhase === 1 && (
            <Animate.SlideUp>
              <FileUpload.Dropzone className="gap-3 px-6 py-10">
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

          {/* Phase 2: File grid + config */}
          {activePhase === 2 && (
            <div className="space-y-4">
              <Animate.Stagger>
                <Grid cols={{ mobile: 2, desktop: 3 }} gap="sm">
                  {files.map((file, i) => (
                    <FileCard
                      key={`${file.name}-${file.size}-${file.lastModified}`}
                      file={file}
                      onRemove={() => setFiles(files.filter((_, j) => j !== i))}
                      disabled={isProcessing}
                      index={i}
                    />
                  ))}
                </Grid>
              </Animate.Stagger>

              <div className="text-left">
                <RecipeConfigSection
                  slug={entry.slug}
                  config={config}
                  onChange={setConfig}
                />
              </div>
            </div>
          )}

          {/* Phase 3: Results */}
          {activePhase === 3 && (
            <RecipeResultsSection
              isBrowserPath={isBrowserPath}
              resolvedPhase={resolvedPhase}
              browserExec={browserExec}
              onDownload={downloadResult}
              onDownloadAll={downloadAll}
              executionId={executionId}
              uploadProgress={uploadProgress}
              clientError={clientError}
            />
          )}

          <RunButton
            phase={resolvedPhase}
            hasFiles={files.length > 0}
            onRun={handleRun}
            onReset={handleReset}
          />
        </FileUpload>
      )}
    </Container>
  );
}
