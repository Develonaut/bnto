"use client";

import { useMemo } from "react";
import type { BrowserExecution, BrowserFileResult, BrowserFileProgress } from "@bnto/core";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { BrowserExecutionProgress } from "@/app/(app)/[bnto]/_components/BrowserExecutionProgress";
import { ToolbarProgress } from "@/app/(app)/[bnto]/_components/ToolbarProgress";

interface Scenario {
  id: string;
  label: string;
  execution: BrowserExecution;
}

/** Create a deterministic mock result with a fixed compression ratio. */
function mockResult(originalSize: number, filename: string, mimeType: string): BrowserFileResult {
  const ratio = 0.45;
  return {
    blob: new Blob(["x".repeat(Math.floor(originalSize * ratio))], { type: mimeType }),
    filename,
    mimeType,
    metadata: { originalSize, ratio },
  };
}

/** Build all 6 scenarios with deterministic data. */
function buildScenarios(): Scenario[] {
  const startedAt = Date.now() - 3000;

  const mkProgress = (
    fileIndex: number,
    totalFiles: number,
    percent: number,
    overallPercent: number,
    message: string,
  ): BrowserFileProgress => ({ fileIndex, totalFiles, percent, overallPercent, message });

  return [
    {
      id: "initializing",
      label: "Initializing",
      execution: {
        id: "mock-init",
        status: "processing",
        fileProgress: null,
        results: [],
        startedAt,
      },
    },
    {
      id: "single-file-50",
      label: "Single file at 50%",
      execution: {
        id: "mock-single-50",
        status: "processing",
        fileProgress: mkProgress(0, 1, 50, 50, "Compressing\u2026"),
        results: [],
        startedAt,
      },
    },
    {
      id: "multi-file-early",
      label: "Multi-file: file 1/3 at 60%",
      execution: {
        id: "mock-multi-early",
        status: "processing",
        fileProgress: mkProgress(0, 3, 60, 20, "Compressing\u2026"),
        results: [],
        startedAt,
      },
    },
    {
      id: "multi-file-mid",
      label: "Multi-file: file 2/3 at 80%",
      execution: {
        id: "mock-multi-mid",
        status: "processing",
        fileProgress: mkProgress(1, 3, 80, 60, "Compressing\u2026"),
        results: [mockResult(2_400_000, "photo-1-compressed.jpg", "image/jpeg")],
        startedAt,
      },
    },
    {
      id: "multi-file-near",
      label: "Multi-file: file 3/3 at 90%",
      execution: {
        id: "mock-multi-near",
        status: "processing",
        fileProgress: mkProgress(2, 3, 90, 97, "Compressing\u2026"),
        results: [
          mockResult(2_400_000, "photo-1-compressed.jpg", "image/jpeg"),
          mockResult(1_800_000, "photo-2-compressed.png", "image/png"),
        ],
        startedAt,
      },
    },
    {
      id: "completed",
      label: "Completed",
      execution: {
        id: "mock-completed",
        status: "completed",
        fileProgress: null,
        results: [
          mockResult(2_400_000, "photo-1-compressed.jpg", "image/jpeg"),
          mockResult(1_800_000, "photo-2-compressed.png", "image/png"),
          mockResult(3_100_000, "photo-3-compressed.webp", "image/webp"),
        ],
        startedAt: Date.now() - 5000,
        completedAt: Date.now(),
      },
    },
  ];
}

/**
 * Renders 6 execution progress scenarios side-by-side.
 *
 * Each scenario shows both the card view (BrowserExecutionProgress)
 * and the compact toolbar view (ToolbarProgress).
 */
export function ExecutionProgressShowcase() {
  const scenarios = useMemo(() => buildScenarios(), []);

  return (
    <div className="space-y-6">
      {scenarios.map((scenario) => (
        <div
          key={scenario.id}
          data-testid={`scenario-${scenario.id}`}
          className="space-y-3 rounded-xl border border-border bg-muted/30 p-4"
        >
          <div className="flex items-baseline gap-2">
            <Heading level={3} size="xs">{scenario.label}</Heading>
            <Text size="xs" color="muted" className="font-mono">
              {scenario.id}
            </Text>
          </div>

          <div className="grid grid-cols-1 gap-4 lg:grid-cols-2">
            {/* Card view */}
            <div>
              <Text size="xs" color="muted" className="mb-2">Card view</Text>
              <BrowserExecutionProgress execution={scenario.execution} />
            </div>

            {/* Compact toolbar view */}
            <div>
              <Text size="xs" color="muted" className="mb-2">Toolbar view</Text>
              <div className="rounded-lg border border-border bg-card p-4">
                <ToolbarProgress execution={scenario.execution} />
              </div>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}
