"use client";

import { useCallback } from "react";
import {
  Button,
  CheckCircle2Icon,
  DownloadIcon,
  LoaderIcon,
  StatusBanner,
  StatusBannerIcon,
  StatusBannerLabel,
  StatusBannerProgress,
  StatusBannerRow,
  StatusBannerSpacer,
  XCircleIcon,
} from "@bnto/ui";
import { formatFileSize } from "@bnto/ui";
import { computeTotalSaved } from "@bnto/core";
import { useEditor } from "../../context";

/** Persistent banner that updates across running -> completed -> failed. */
function ExecutionBanner() {
  const editor = useEditor();
  const { phase, results, errors, logs, fileProgress, inputFiles } =
    editor.execution.useExecution();

  const handleErrorClick = useCallback(
    (index: number) => {
      // Find the matching log entry to get the nodeId
      const errorLogs = logs.filter(
        (l) => l.event.type === "ValidationFailed" || l.event.type === "NodeFailed",
      );
      const entry = errorLogs[index];
      if (entry && "nodeId" in entry.event && entry.event.nodeId) {
        editor.nodes.selectNode(entry.event.nodeId);
      }
    },
    [logs, editor],
  );

  if (phase === "failed" || errors.length > 0) {
    const title =
      errors.length === 0
        ? "Execution failed"
        : errors.length === 1
          ? "1 issue found"
          : `${errors.length} issues found`;

    return (
      <StatusBanner variant="error">
        <StatusBannerRow>
          <StatusBannerIcon>
            <XCircleIcon />
          </StatusBannerIcon>
          <StatusBannerLabel>{title}</StatusBannerLabel>
        </StatusBannerRow>
        {errors.length > 0 && (
          <ul className="flex flex-col gap-1 pl-6 text-xs text-destructive">
            {errors.map((err, i) => (
              <li key={i} className="list-disc">
                <button
                  type="button"
                  className="text-left underline-offset-2 hover:underline"
                  onClick={() => handleErrorClick(i)}
                >
                  {err}
                </button>
              </li>
            ))}
          </ul>
        )}
      </StatusBanner>
    );
  }

  if (phase === "running") {
    const percent = fileProgress?.overallPercent ?? 0;
    const label = fileProgress
      ? `Processing file ${fileProgress.fileIndex + 1} of ${fileProgress.totalFiles}...`
      : inputFiles.length > 0
        ? `0 of ${inputFiles.length} files`
        : "Initializing...";

    return (
      <StatusBanner variant="processing">
        <StatusBannerRow>
          <StatusBannerIcon>
            <LoaderIcon className="motion-safe:animate-spin" />
          </StatusBannerIcon>
          <StatusBannerLabel muted>{label}</StatusBannerLabel>
        </StatusBannerRow>
        <StatusBannerProgress value={percent} />
      </StatusBanner>
    );
  }

  const saved = computeTotalSaved(results);
  return (
    <StatusBanner variant="success">
      <StatusBannerRow>
        <StatusBannerIcon>
          <CheckCircle2Icon />
        </StatusBannerIcon>
        <StatusBannerLabel>
          {results.length} {results.length === 1 ? "file" : "files"} processed
        </StatusBannerLabel>
        <StatusBannerSpacer />
        {saved > 0 && (
          <StatusBannerLabel muted mono>
            {formatFileSize(saved)} saved
          </StatusBannerLabel>
        )}
        <Button
          variant="outline"
          size="sm"
          icon={<DownloadIcon />}
          onClick={() => editor.execution.downloadAllResults()}
          aria-label={`Download all ${results.length} files`}
        />
      </StatusBannerRow>
    </StatusBanner>
  );
}

export { ExecutionBanner };
