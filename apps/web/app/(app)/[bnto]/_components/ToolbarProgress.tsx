"use client";

import type { BrowserExecution } from "@bnto/core";
import { CheckCircle2Icon, LoaderIcon } from "@/components/ui/icons";
import { LinearProgress } from "@/components/ui/LinearProgress";
import { formatFileSize } from "@/src/utils/formatFileSize";

interface ToolbarProgressProps {
  execution: BrowserExecution;
}

/**
 * Compact inline progress/stats display for the toolbar center area.
 *
 * During processing: spinner + file counter + progress bar
 * After completion: checkmark + summary stats (files processed, total saved)
 */
export function ToolbarProgress({ execution }: ToolbarProgressProps) {
  if (execution.status === "completed") {
    const totalSaved = execution.results.reduce((acc, r) => {
      const orig = r.metadata.originalSize as number | undefined;
      return orig != null ? acc + (orig - r.blob.size) : acc;
    }, 0);

    const label = `${execution.results.length} ${execution.results.length === 1 ? "file" : "files"} processed`;
    const saved = totalSaved > 0 ? formatFileSize(totalSaved) + " saved" : undefined;

    return (
      <LinearProgress
        value={100}
        icon={<CheckCircle2Icon className="size-4 shrink-0 text-primary" />}
        label={label}
        valueLabel={saved ?? "100%"}
      />
    );
  }

  const { fileProgress } = execution;

  if (!fileProgress) {
    return (
      <LinearProgress
        value={0}
        icon={<LoaderIcon className="size-4 shrink-0 text-primary motion-safe:animate-spin" />}
        label="Initializing\u2026"
        valueLabel=""
      />
    );
  }

  return (
    <LinearProgress
      value={fileProgress.percent}
      icon={<LoaderIcon className="size-4 shrink-0 text-primary motion-safe:animate-spin" />}
      label={`Processing file ${fileProgress.fileIndex + 1} of ${fileProgress.totalFiles}\u2026`}
    />
  );
}
