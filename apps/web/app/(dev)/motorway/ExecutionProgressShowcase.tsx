"use client";

import { useState, useEffect, useCallback, useRef } from "react";
import type { BrowserExecution, BrowserFileResult } from "@bnto/core";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { Button } from "@/components/ui/Button";
import { PlayIcon, RotateCcwIcon } from "@/components/ui/icons";
import { BrowserExecutionProgress } from "@/app/(app)/[bnto]/_components/BrowserExecutionProgress";
import { ToolbarProgress } from "@/app/(app)/[bnto]/_components/ToolbarProgress";

/* ── Scenario config ─────────────────────────────────────────── */

interface MockFile {
  name: string;
  size: number;
  type: string;
}

interface ScenarioConfig {
  id: string;
  label: string;
  description: string;
  files: MockFile[];
  /** Milliseconds between each progress tick. Lower = faster. */
  tickMs: number;
  /** How many percent points each tick advances the current file. */
  percentPerTick: number;
}

const SCENARIOS: ScenarioConfig[] = [
  {
    id: "single-file",
    label: "Single File",
    description: "One image, smooth 0\u2192100% fill.",
    files: [{ name: "vacation-photo.jpg", size: 2_400_000, type: "image/jpeg" }],
    tickMs: 60,
    percentPerTick: 2,
  },
  {
    id: "three-file-batch",
    label: "3-File Batch",
    description: "Three files sequentially. Watch the file counter and overall progress.",
    files: [
      { name: "vacation-photo.jpg", size: 2_400_000, type: "image/jpeg" },
      { name: "profile-pic.png", size: 1_800_000, type: "image/png" },
      { name: "hero-banner.webp", size: 3_100_000, type: "image/webp" },
    ],
    tickMs: 40,
    percentPerTick: 3,
  },
  {
    id: "five-file-batch",
    label: "5-File Batch",
    description: "Larger batch. Overall progress increments in smaller steps per file.",
    files: [
      { name: "photo-001.jpg", size: 3_200_000, type: "image/jpeg" },
      { name: "photo-002.jpg", size: 2_100_000, type: "image/jpeg" },
      { name: "photo-003.png", size: 4_500_000, type: "image/png" },
      { name: "photo-004.webp", size: 1_900_000, type: "image/webp" },
      { name: "photo-005.jpg", size: 2_800_000, type: "image/jpeg" },
    ],
    tickMs: 30,
    percentPerTick: 4,
  },
];

/* ── Simulation hook ─────────────────────────────────────────── */

interface SimState {
  phase: "idle" | "processing" | "completed";
  fileIndex: number;
  percent: number;
  results: BrowserFileResult[];
  startedAt?: number;
  completedAt?: number;
}

function mockResult(file: MockFile): BrowserFileResult {
  const ratio = 0.45;
  return {
    blob: new Blob(["x".repeat(Math.floor(file.size * ratio))], { type: file.type }),
    filename: file.name.replace(/\./, "-compressed."),
    mimeType: file.type,
    metadata: { originalSize: file.size, ratio },
  };
}

function useSimulation(config: ScenarioConfig) {
  const [state, setState] = useState<SimState>({
    phase: "idle",
    fileIndex: 0,
    percent: 0,
    results: [],
  });

  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const clearTick = useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  }, []);

  const run = useCallback(() => {
    clearTick();
    setState({
      phase: "processing",
      fileIndex: 0,
      percent: 0,
      results: [],
      startedAt: Date.now(),
    });
  }, [clearTick]);

  const reset = useCallback(() => {
    clearTick();
    setState({ phase: "idle", fileIndex: 0, percent: 0, results: [] });
  }, [clearTick]);

  // Drive the simulation forward on an interval
  useEffect(() => {
    if (state.phase !== "processing") return;

    intervalRef.current = setInterval(() => {
      setState((prev) => {
        if (prev.phase !== "processing") return prev;

        const nextPercent = prev.percent + config.percentPerTick;

        // Current file not done yet — just advance percent
        if (nextPercent < 100) {
          return { ...prev, percent: nextPercent };
        }

        // Current file done — add result, move to next file
        const completedFile = config.files[prev.fileIndex]!;
        const newResults = [...prev.results, mockResult(completedFile)];
        const nextFileIndex = prev.fileIndex + 1;

        // All files done
        if (nextFileIndex >= config.files.length) {
          return {
            ...prev,
            phase: "completed",
            fileIndex: prev.fileIndex,
            percent: 100,
            results: newResults,
            completedAt: Date.now(),
          };
        }

        // Start next file
        return {
          ...prev,
          fileIndex: nextFileIndex,
          percent: 0,
          results: newResults,
        };
      });
    }, config.tickMs);

    return () => clearTick();
  }, [state.phase, config.tickMs, config.percentPerTick, config.files, clearTick]);

  // Build the BrowserExecution from simulation state
  const execution: BrowserExecution = {
    id: `sim-${config.id}`,
    status: state.phase === "processing" ? "processing"
          : state.phase === "completed" ? "completed"
          : "idle",
    fileProgress: state.phase === "processing" ? {
      fileIndex: state.fileIndex,
      totalFiles: config.files.length,
      percent: state.percent,
      overallPercent: Math.round(
        ((state.fileIndex * 100) + state.percent) / config.files.length,
      ),
      message: `Compressing ${config.files[state.fileIndex]?.name ?? ""}\u2026`,
    } : null,
    results: state.results,
    startedAt: state.startedAt,
    completedAt: state.completedAt,
  };

  return { execution, phase: state.phase, run, reset };
}

/* ── Scenario card ───────────────────────────────────────────── */

function ScenarioCard({ config }: { config: ScenarioConfig }) {
  const { execution, phase, run, reset } = useSimulation(config);

  return (
    <div
      data-testid={`scenario-${config.id}`}
      data-scenario-status={phase}
      className="space-y-3 rounded-xl border border-border bg-muted/30 p-4"
    >
      <div className="flex items-center justify-between gap-2">
        <div className="flex items-baseline gap-2">
          <Heading level={3} size="xs">{config.label}</Heading>
          <Text size="xs" color="muted">
            {config.files.length} {config.files.length === 1 ? "file" : "files"}
          </Text>
        </div>

        {phase === "idle" && (
          <Button
            size="icon"
            onClick={run}
            aria-label={`Run ${config.label}`}
            data-testid={`run-${config.id}`}
          >
            <PlayIcon className="size-4" />
          </Button>
        )}
        {phase === "completed" && (
          <Button
            variant="outline"
            size="icon"
            onClick={reset}
            aria-label={`Reset ${config.label}`}
            data-testid={`reset-${config.id}`}
          >
            <RotateCcwIcon className="size-4" />
          </Button>
        )}
        {phase === "processing" && (
          <Text size="xs" color="muted" className="font-mono tabular-nums">
            {execution.fileProgress?.overallPercent ?? 0}%
          </Text>
        )}
      </div>

      <Text size="xs" color="muted">{config.description}</Text>

      {phase !== "idle" && (
        <div className="grid grid-cols-1 gap-4 lg:grid-cols-2">
          <div>
            <Text size="xs" color="muted" className="mb-2">Card view</Text>
            <BrowserExecutionProgress execution={execution} />
          </div>
          <div>
            <Text size="xs" color="muted" className="mb-2">Toolbar view</Text>
            <div className="rounded-lg border border-border bg-card p-4">
              <ToolbarProgress execution={execution} />
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

/* ── Showcase ────────────────────────────────────────────────── */

/**
 * Interactive execution progress showcase.
 *
 * Each scenario has a Run button that kicks off a timed simulation.
 * Progress ticks through initializing → per-file processing → completed,
 * driving the real BrowserExecutionProgress and ToolbarProgress components.
 */
export function ExecutionProgressShowcase() {
  return (
    <div className="space-y-6">
      {SCENARIOS.map((config) => (
        <ScenarioCard key={config.id} config={config} />
      ))}
    </div>
  );
}
