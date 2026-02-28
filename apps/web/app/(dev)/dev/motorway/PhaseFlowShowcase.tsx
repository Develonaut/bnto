"use client";

import { useState, useMemo } from "react";
import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { FileUpload } from "@/components/ui/FileUpload";
import { Label } from "@/components/ui/Label";
import { Slider } from "@/components/ui/Slider";
import { Heading } from "@/components/ui/Heading";
import {
  ArrowLeftIcon,
  ArrowRightIcon,
  DownloadIcon,
  TrashIcon,
  UploadIcon,
} from "@/components/ui/icons";
import { FileCard } from "@/app/(app)/[bnto]/_components/FileCard";
import { RunButton } from "@/app/(app)/[bnto]/_components/RunButton";
import { ToolbarProgress } from "@/app/(app)/[bnto]/_components/ToolbarProgress";
import { PhaseIndicator } from "@/app/(app)/[bnto]/_components/PhaseIndicator";
import type { RunPhase } from "@/app/(app)/[bnto]/_components/RunButton";

type SimPhase = "idle" | "files-selected" | "processing" | "completed";

const PHASE_LABELS: Record<SimPhase, string> = {
  idle: "Phase 1: Dropzone",
  "files-selected": "Phase 2: Configure",
  processing: "Phase 3: Processing",
  completed: "Phase 3: Completed",
};

const PHASE_ORDER: SimPhase[] = ["idle", "files-selected", "processing", "completed"];

/** Create mock File objects for the showcase. */
function createMockFiles(): File[] {
  return [
    new File(["x".repeat(2_400_000)], "vacation-photo.jpg", { type: "image/jpeg" }),
    new File(["x".repeat(1_800_000)], "profile-pic.png", { type: "image/png" }),
    new File(["x".repeat(3_100_000)], "hero-banner.webp", { type: "image/webp" }),
  ];
}

/** Create mock BrowserExecution for different states. */
function createMockExecution(
  phase: SimPhase,
  files: File[],
): BrowserExecution {
  const base: BrowserExecution = {
    id: "mock-exec",
    status: "idle",
    fileProgress: null,
    results: [],
  };

  if (phase === "processing") {
    return {
      ...base,
      status: "processing",
      startedAt: Date.now() - 3000,
      fileProgress: {
        fileIndex: 1,
        totalFiles: files.length,
        percent: 64,
        message: "Compressing profile-pic.png\u2026",
      },
      results: [mockResult(files[0]!)],
    };
  }

  if (phase === "completed") {
    return {
      ...base,
      status: "completed",
      startedAt: Date.now() - 5000,
      completedAt: Date.now(),
      results: files.map((f) => mockResult(f)),
    };
  }

  return base;
}

function mockResult(file: File): BrowserFileResult {
  const ratio = 0.35 + Math.random() * 0.3; // 35-65% of original
  return {
    blob: new Blob(["x".repeat(Math.floor(file.size * ratio))], { type: file.type }),
    filename: file.name.replace(/\./, "-compressed."),
    mimeType: file.type,
    metadata: {
      originalSize: file.size,
      ratio,
    },
  };
}

/**
 * Interactive showcase for stepping through recipe page phases.
 *
 * Simulates all 4 states: dropzone, file list, processing, completed.
 * Use the arrow buttons to step forward/back and observe transitions.
 */
export function PhaseFlowShowcase() {
  const [simPhase, setSimPhase] = useState<SimPhase>("idle");
  const phaseIndex = PHASE_ORDER.indexOf(simPhase);

  const mockFiles = useMemo(() => createMockFiles(), []);
  const mockExec = useMemo(() => createMockExecution(simPhase, mockFiles), [simPhase, mockFiles]);

  const files = simPhase === "idle" ? [] : mockFiles;
  const activePhase: 1 | 2 | 3 =
    simPhase === "idle" ? 1 : simPhase === "files-selected" ? 2 : 3;
  const resolvedPhase: RunPhase =
    simPhase === "processing" ? "running"
    : simPhase === "completed" ? "completed"
    : "idle";

  const prev = () => setSimPhase(PHASE_ORDER[Math.max(0, phaseIndex - 1)]!);
  const next = () => setSimPhase(PHASE_ORDER[Math.min(PHASE_ORDER.length - 1, phaseIndex + 1)]!);

  return (
    <div className="space-y-6">
      {/* Phase stepper controls */}
      <div className="flex items-center gap-3">
        <Button variant="outline" size="icon" onClick={prev} disabled={phaseIndex === 0}>
          <ArrowLeftIcon className="size-4" />
        </Button>
        <div className="flex flex-1 gap-2">
          {PHASE_ORDER.map((p) => (
            <button
              key={p}
              onClick={() => setSimPhase(p)}
              className={`rounded-lg px-3 py-1.5 text-xs font-medium transition-colors ${
                p === simPhase
                  ? "bg-primary text-primary-foreground"
                  : "bg-muted text-muted-foreground hover:bg-muted/80"
              }`}
            >
              {PHASE_LABELS[p]}
            </button>
          ))}
        </div>
        <Button variant="outline" size="icon" onClick={next} disabled={phaseIndex === PHASE_ORDER.length - 1}>
          <ArrowRightIcon className="size-4" />
        </Button>
      </div>

      {/* Simulated recipe shell */}
      <div className="space-y-6 text-center">
        <PhaseIndicator activePhase={activePhase} />
        <Heading level={2}>Compress Images</Heading>
        <p className="text-muted-foreground mx-auto max-w-xl text-sm leading-snug text-balance">
          Reduce image file sizes while maintaining visual quality.
        </p>

          <FileUpload
            value={files}
            onValueChange={() => {}}
            accept={{ "image/*": [] }}
            multiple
            disabled={simPhase === "processing"}
          >
            {/* Phase 1: Dropzone */}
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
                      or click to browse &middot; accepts JPEG, PNG, WebP
                    </p>
                  </div>
                </FileUpload.Dropzone>
              </Animate.SlideUp>
            )}

            {/* Phases 2-3: Toolbar + persistent file grid */}
            {(activePhase === 2 || activePhase === 3) && (
              <div className="space-y-4 text-left">
                {/* Toolbar */}
                <div className="flex min-h-10 flex-col gap-3 md:flex-row md:items-center md:gap-4">
                  <div className="flex items-center gap-2">
                    <Button
                      variant="ghost"
                      size="icon"
                      elevation="sm"
                      disabled={simPhase === "processing"}
                      onClick={prev}
                    >
                      <ArrowLeftIcon className="size-4" />
                    </Button>
                    <p className="shrink-0 text-sm font-medium text-foreground">
                      {files.length} files selected
                    </p>
                    {/* Mobile actions */}
                    <div className="ml-auto flex shrink-0 items-center gap-2 md:hidden">
                      {activePhase === 3 && (
                        <Button variant="outline" size="icon" elevation="sm" disabled={resolvedPhase !== "completed"} aria-label="Download all">
                          <DownloadIcon className="size-4" />
                        </Button>
                      )}
                      {activePhase === 2 && (
                        <Button variant="outline" size="icon" elevation="md">
                          <TrashIcon className="size-4" />
                        </Button>
                      )}
                      <RunButton
                        phase={resolvedPhase}
                        hasFiles={files.length > 0}
                        onRun={next}
                      />
                    </div>
                  </div>

                  {/* Center: config (Phase 2) or progress (Phase 3) */}
                  {activePhase === 2 && (
                    <div className="min-w-0 flex-1 border-border md:mx-4 md:border-l md:border-r md:px-4">
                      <div className="flex w-full flex-col gap-3">
                        <div className="flex items-baseline justify-between gap-2">
                          <Label className="text-muted-foreground text-sm">Quality</Label>
                          <span className="text-muted-foreground shrink-0 font-mono text-sm tabular-nums">
                            80%
                          </span>
                        </div>
                        <Slider
                          className="w-full"
                          defaultValue={[80]}
                          min={1}
                          max={100}
                          step={1}
                        />
                        <p className="min-h-4 text-xs text-muted-foreground">Lower values reduce file size more but also lower quality</p>
                      </div>
                    </div>
                  )}
                  {activePhase === 3 && (
                    <div className="min-w-0 flex-1 border-border md:mx-4 md:border-l md:border-r md:px-4">
                      <ToolbarProgress execution={mockExec} />
                    </div>
                  )}

                  {/* Desktop actions */}
                  <div className="ml-auto hidden shrink-0 items-center gap-2 md:flex">
                    {activePhase === 3 && (
                      <Button variant="outline" size="icon" elevation="sm" disabled={resolvedPhase !== "completed"} aria-label="Download all">
                        <DownloadIcon className="size-4" />
                      </Button>
                    )}
                    {activePhase === 2 && (
                      <Button variant="outline" size="icon" elevation="md">
                        <TrashIcon className="size-4" />
                      </Button>
                    )}
                    <RunButton
                      phase={resolvedPhase}
                      hasFiles={files.length > 0}
                      onRun={next}
                    />
                  </div>
                </div>

                {/* Persistent file grid */}
                <Animate.BouncyStagger className="flex flex-col gap-2">
                  {files.map((file, i) => {
                    const result = activePhase === 3 ? mockExec.results[i] : undefined;
                    const isFileProcessing =
                      simPhase === "processing" &&
                      mockExec.fileProgress?.fileIndex === i;

                    return (
                      <FileCard
                        key={file.name}
                        file={file}
                        result={result}
                        isProcessing={isFileProcessing}
                        isExecuting={activePhase === 3}
                        onDelete={() => {}}
                        onDownload={() => {}}
                      />
                    );
                  })}
                </Animate.BouncyStagger>
              </div>
            )}
          </FileUpload>
        </div>
    </div>
  );
}
