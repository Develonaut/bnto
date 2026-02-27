"use client";

import type { BntoEntry } from "@/lib/bntoRegistry";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { FileUpload } from "@/components/ui/FileUpload";
import { Heading } from "@/components/ui/Heading";
import { Tabs } from "@/components/ui/Tabs";
import { UploadIcon, XIcon } from "@/components/ui/icons";
import { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { getAcceptedTypes, toDropzoneAccept } from "../_lib/getAcceptedTypes";
import { BntoConfigPanel } from "./BntoConfigPanel";
import { UpgradePrompt } from "./UpgradePrompt";
import { UploadProgress } from "./UploadProgress";
import { RunButton } from "./RunButton";
import { ExecutionProgress } from "./ExecutionProgress";
import { ExecutionResults } from "./ExecutionResults";
import { BrowserExecutionProgress } from "./BrowserExecutionProgress";
import { BrowserExecutionResults } from "./BrowserExecutionResults";
import { ErrorCard } from "./ErrorCard";

type Step = "upload" | "files" | "processing" | "results";

interface BntoPageShellProps {
  entry: BntoEntry;
}

/**
 * Derive the active tab step from the execution phase and file count.
 *
 * - No files + idle → dropzone
 * - Has files + idle → file list with controls
 * - Uploading/running → processing feedback
 * - Completed/failed → results or error
 */
function deriveStep(phase: string, fileCount: number): Step {
  switch (phase) {
    case "uploading":
    case "running":
      return "processing";
    case "completed":
    case "failed":
      return "results";
    default:
      return fileCount > 0 ? "files" : "upload";
  }
}

/**
 * Client shell for bnto tool pages.
 *
 * Composes Tabs + FileUpload primitives to manage the full lifecycle:
 *   upload → file list → processing → results
 *
 * The active tab is derived from useRecipeFlow state — no manual tab switching.
 * Two execution paths (browser WASM vs cloud R2+Go) share the same tab structure.
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

  const { label } = getAcceptedTypes(entry.slug);
  const accept = toDropzoneAccept(entry.slug);
  const currentStep = deriveStep(resolvedPhase, files.length);

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
          <FileUpload
            value={files}
            onValueChange={setFiles}
            accept={accept}
            multiple
            disabled={isProcessing}
          >
            <div className="text-left">
              <BntoConfigPanel
                slug={entry.slug}
                config={config}
                onChange={setConfig}
              />
            </div>

            <Tabs value={currentStep}>
              {/* No visible tab bar — step derived from state */}
              <Tabs.List className="hidden">
                <Tabs.Trigger value="upload" />
                <Tabs.Trigger value="files" />
                <Tabs.Trigger value="processing" />
                <Tabs.Trigger value="results" />
              </Tabs.List>

              <Tabs.Content value="upload">
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
              </Tabs.Content>

              <Tabs.Content value="files">
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <p className="text-sm font-medium text-foreground">
                      {files.length} {files.length === 1 ? "file" : "files"} selected
                    </p>
                    <FileUpload.Clear asChild>
                      <Button variant="outline" elevation="md">
                        Clear all
                      </Button>
                    </FileUpload.Clear>
                  </div>

                  <FileUpload.List>
                    {files.map((file, i) => (
                      <FileUpload.Item
                        key={`${file.name}-${file.size}-${file.lastModified}`}
                        value={file}
                        index={i}
                      >
                        <Card
                          className="flex items-center gap-3 rounded-lg px-4 py-3"
                          elevation="sm"
                        >
                          <FileUpload.ItemMetadata />
                          <FileUpload.ItemActions>
                            <FileUpload.ItemDelete asChild>
                              <Button variant="outline" size="icon-sm" elevation="sm">
                                <XIcon className="size-4" />
                              </Button>
                            </FileUpload.ItemDelete>
                          </FileUpload.ItemActions>
                        </Card>
                      </FileUpload.Item>
                    ))}
                  </FileUpload.List>
                </div>
              </Tabs.Content>

              <Tabs.Content value="processing">
                {/* Browser execution progress */}
                {isBrowserPath && browserExec.status === "processing" && (
                  <BrowserExecutionProgress execution={browserExec} />
                )}

                {/* Cloud execution progress */}
                {!isBrowserPath &&
                  uploadProgress.files.length > 0 &&
                  resolvedPhase === "uploading" && (
                    <UploadProgress files={uploadProgress.files} />
                  )}
                {!isBrowserPath && executionId && resolvedPhase === "running" && (
                  <ExecutionProgress executionId={executionId} />
                )}
              </Tabs.Content>

              <Tabs.Content value="results">
                {/* Browser results */}
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

                {/* Cloud results */}
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
              </Tabs.Content>
            </Tabs>

            <RunButton
              phase={resolvedPhase}
              hasFiles={files.length > 0}
              onRun={handleRun}
              onReset={handleReset}
            />
          </FileUpload>
        )}
      </div>
    </div>
  );
}
