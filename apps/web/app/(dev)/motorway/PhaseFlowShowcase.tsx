"use client";

import { useState, useMemo } from "react";
import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import {
  SlideUp,
  BouncyStagger,
  ArrowLeftIcon,
  ArrowRightIcon,
  Button,
  FileUpload,
  FileUploadDropzone,
  Heading,
} from "@bnto/ui";
import { DropzoneContent } from "@/app/(app)/[bnto]/_components/DropzoneContent";
import { FileCard } from "@/app/(app)/[bnto]/_components/FileCard";
import { PhaseIndicator } from "@/app/(app)/[bnto]/_components/PhaseIndicator";
import { RecipeConfigSection } from "@/app/(app)/[bnto]/_components/RecipeConfigSection";
import { RecipeToolbar } from "@/app/(app)/[bnto]/_components/RecipeToolbar";
import { ToolbarProgress } from "@/app/(app)/[bnto]/_components/ToolbarProgress";
import type { RunPhase } from "@/app/(app)/[bnto]/_components/RunButton";
import { DEFAULT_CONFIGS } from "@/app/(app)/[bnto]/_components/configs/types";
import type { BntoConfigMap } from "@/app/(app)/[bnto]/_components/configs/types";

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
function createMockExecution(phase: SimPhase, files: File[]): BrowserExecution {
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
        overallPercent: Math.round((1 * 100 + 64) / files.length),
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
  const [config, setConfig] = useState<BntoConfigMap["compress-images"]>(
    DEFAULT_CONFIGS["compress-images"],
  );
  const phaseIndex = PHASE_ORDER.indexOf(simPhase);

  const mockFiles = useMemo(() => createMockFiles(), []);
  const mockExec = useMemo(() => createMockExecution(simPhase, mockFiles), [simPhase, mockFiles]);

  const files = simPhase === "idle" ? [] : mockFiles;
  const activePhase: 1 | 2 | 3 = simPhase === "idle" ? 1 : simPhase === "files-selected" ? 2 : 3;
  const resolvedPhase: RunPhase =
    simPhase === "processing" ? "running" : simPhase === "completed" ? "completed" : "idle";

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
        <Button
          variant="outline"
          size="icon"
          onClick={next}
          disabled={phaseIndex === PHASE_ORDER.length - 1}
        >
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
            <SlideUp>
              <FileUploadDropzone className="gap-3 px-4 py-8 sm:px-6 sm:py-10">
                <DropzoneContent label="JPEG, PNG, WebP" />
              </FileUploadDropzone>
            </SlideUp>
          )}

          {/* Phases 2-3: Toolbar + persistent file grid */}
          {(activePhase === 2 || activePhase === 3) && (
            <div className="space-y-4 text-left">
              <RecipeToolbar
                activePhase={activePhase as 2 | 3}
                resolvedPhase={resolvedPhase}
                isProcessing={simPhase === "processing"}
                fileCount={files.length}
                onBack={prev}
                onRun={next}
                onDownloadAll={() => {}}
                centerContent={
                  activePhase === 2 ? (
                    <RecipeConfigSection
                      slug="compress-images"
                      config={config}
                      onChange={(c) => setConfig(c as BntoConfigMap["compress-images"])}
                    />
                  ) : (
                    <ToolbarProgress execution={mockExec} />
                  )
                }
              />

              {/* Persistent file grid */}
              <BouncyStagger className="flex flex-col gap-2">
                {files.map((file, i) => {
                  const result = activePhase === 3 ? mockExec.results[i] : undefined;
                  const isFileProcessing =
                    simPhase === "processing" && mockExec.fileProgress?.fileIndex === i;

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
              </BouncyStagger>
            </div>
          )}
        </FileUpload>
      </div>
    </div>
  );
}
