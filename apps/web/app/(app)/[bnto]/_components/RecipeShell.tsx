"use client";

import { type ReactNode, useCallback } from "react";
import type { BntoEntry } from "@/lib/bntoRegistry";
import {
  ArrowLeftIcon,
  Button,
  DownloadIcon,
  Grid,
  GridItem,
  LoaderIcon,
  PlayIcon,
  RotateCcwIcon,
  Stepper,
  Surface,
  Text,
} from "@bnto/ui";
import { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { RecipeConfigSection } from "./RecipeConfigSection";
import type { RunPhase } from "./RunButton";
import { RecipeShellUpload } from "./RecipeShellUpload";
import { SessionMarker } from "./SessionMarker";
import { ToolbarProgress } from "./ToolbarProgress";
import { deriveActivePhase } from "./phaseMapping";

const noop = () => {};

const PHASE_LABELS = { 1: "Files", 2: "Configure", 3: "Results" } as const;

/**
 * Recipe page interactive flow -- client island.
 *
 * Bento box layout:
 *   Top-left:      current phase indicator
 *   Top-mid-left:  run button
 *   Bottom-left:   file upload / status / file list
 *   Right:         config panel (full height)
 */
export function RecipeShell({ entry }: { entry: BntoEntry }) {
  const flow = useRecipeFlow({ entry });
  const activePhase = deriveActivePhase(flow.resolvedPhase, flow.files.length);
  const handleClearFiles = useCallback(() => flow.setFiles([]), [flow]);
  const handleDeleteFile = useCallback(
    (index: number) => () => flow.setFiles(flow.files.filter((_, j) => j !== index)),
    [flow],
  );

  return (
    <RecipeShellLayout activePhase={activePhase} isBrowserPath={flow.isBrowserPath}>
      <SessionMarker />

      <Surface asChild elevation="none" variant="muted" rounded="xl">
        <Grid
          cols={4}
          gap="md"
          className={`p-3 lg:h-[65svh] ${activePhase === 1 ? "grid-rows-[auto_minmax(0,1fr)]" : "grid-rows-[auto_auto_minmax(0,1fr)]"}`}
        >
          {/* Row 1: Step */}
          <GridItem>
            <Surface
              elevation="sm"
              className="flex h-full flex-col items-center justify-center gap-2 p-4"
            >
              <Text className="text-muted-foreground text-xs uppercase tracking-wider">Step</Text>
              <Button
                variant="ghost"
                size="icon"
                elevation="sm"
                disabled={activePhase === 1}
                onClick={activePhase === 3 ? flow.handleResetExecution : handleClearFiles}
                aria-label="Go back"
              >
                <ArrowLeftIcon className="size-4" />
              </Button>
              <Text className="text-sm font-medium">{PHASE_LABELS[activePhase]}</Text>
            </Surface>
          </GridItem>

          {/* Row 1: Run */}
          <GridItem>
            <Surface
              elevation="sm"
              className="flex h-full flex-col items-center justify-center gap-2 p-4"
            >
              <RunCell
                resolvedPhase={flow.resolvedPhase}
                hasFiles={flow.files.length > 0}
                onRun={flow.handleRun}
              />
            </Surface>
          </GridItem>

          {/* Row 1: Config panel */}
          <GridItem colSpan={2}>
            <Surface elevation="sm" className="h-full p-6">
              <RecipeConfigSection
                slug={entry.slug}
                config={flow.config}
                onChange={flow.setConfig}
              />
            </Surface>
          </GridItem>

          {/* Row 2: Status + Download (phases 2/3 only) */}
          {activePhase !== 1 && (
            <>
              <GridItem colSpan={3}>
                <StatusCell activePhase={activePhase} flow={flow} />
              </GridItem>
              <GridItem>
                <Surface elevation="sm" className="flex h-full items-center justify-center p-4">
                  <Button
                    variant="outline"
                    size="icon"
                    elevation="sm"
                    disabled={flow.resolvedPhase !== "completed"}
                    onClick={flow.downloadAll}
                    aria-label="Download all"
                    data-testid="download-all-button"
                  >
                    <DownloadIcon className="size-4" />
                  </Button>
                </Surface>
              </GridItem>
            </>
          )}

          {/* Last row: File upload / file list (full width) */}
          <GridItem colSpan={4} className="[&>*]:h-full">
            <RecipeShellUpload
              flow={flow}
              entry={entry}
              onClearFiles={handleClearFiles}
              onDeleteFile={handleDeleteFile}
            />
          </GridItem>
        </Grid>
      </Surface>
    </RecipeShellLayout>
  );
}

/** Status cell — file count in phase 2, progress banner in phase 3. */
function StatusCell({
  activePhase,
  flow,
}: {
  activePhase: 2 | 3;
  flow: ReturnType<typeof useRecipeFlow>;
}) {
  return (
    <Surface elevation="sm" className="flex h-full items-center p-4">
      {activePhase === 2 ? (
        <Text className="text-sm font-medium" data-testid="file-count">
          {flow.files.length} {flow.files.length === 1 ? "file" : "files"} selected
        </Text>
      ) : (
        <ToolbarProgress execution={flow.browserExec} />
      )}
    </Surface>
  );
}

const RUN_LABELS: Record<RunPhase, string> = {
  idle: "Run",
  uploading: "Uploading",
  running: "Running",
  completed: "Rerun",
  failed: "Retry",
};

/** Run button cell with phase-aware icon and disabled state. */
function RunCell({
  resolvedPhase,
  hasFiles,
  onRun,
}: {
  resolvedPhase: RunPhase;
  hasFiles: boolean;
  onRun: () => void;
}) {
  const isProcessing = resolvedPhase === "uploading" || resolvedPhase === "running";
  const isDone = resolvedPhase === "completed" || resolvedPhase === "failed";

  return (
    <>
      <Text className="text-muted-foreground text-xs uppercase tracking-wider">Action</Text>
      <Button
        size="icon"
        elevation="sm"
        variant={resolvedPhase === "failed" ? "outline" : "primary"}
        disabled={!hasFiles || isProcessing}
        onClick={onRun}
        aria-label={RUN_LABELS[resolvedPhase]}
        data-testid="run-button"
        data-phase={resolvedPhase}
      >
        {isProcessing && <LoaderIcon className="size-4 motion-safe:animate-spin" />}
        {isDone && <RotateCcwIcon className="size-4" />}
        {!isProcessing && !isDone && <PlayIcon className="size-4" />}
      </Button>
      <Text className="text-sm font-medium">{RUN_LABELS[resolvedPhase]}</Text>
    </>
  );
}

function RecipeShellLayout({
  activePhase,
  isBrowserPath,
  children,
}: {
  activePhase: 1 | 2 | 3;
  isBrowserPath: boolean;
  children: ReactNode;
}) {
  return (
    <Stepper
      value={String(activePhase)}
      onValueChange={noop}
      data-testid="bnto-shell"
      data-session="ready"
      data-execution-mode={isBrowserPath ? "browser" : "cloud"}
    >
      {children}
    </Stepper>
  );
}
