"use client";

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
import { useEditorExecutionContext } from "../../hooks/EditorExecutionContext";

/** Persistent banner that updates across running → completed → failed. */
function ExecutionBanner() {
  const { phase, results, errors, fileProgress, inputFiles, downloadAll } =
    useEditorExecutionContext();

  if (phase === "failed" || errors.length > 0) {
    const message = errors.length === 1 ? errors[0] : "Execution failed";
    return (
      <StatusBanner variant="error">
        <StatusBannerRow>
          <StatusBannerIcon>
            <XCircleIcon />
          </StatusBannerIcon>
          <StatusBannerLabel>{message}</StatusBannerLabel>
        </StatusBannerRow>
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
          onClick={downloadAll}
          aria-label={`Download all ${results.length} files`}
        />
      </StatusBannerRow>
    </StatusBanner>
  );
}

export { ExecutionBanner };
