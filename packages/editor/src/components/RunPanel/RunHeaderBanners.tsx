import {
  CheckCircle2Icon,
  LoaderIcon,
  StatusBanner,
  StatusBannerIcon,
  StatusBannerLabel,
  StatusBannerProgress,
  StatusBannerRow,
  StatusBannerSpacer,
  XCircleIcon,
  formatFileSize,
} from "@bnto/ui";
import type { BrowserFileResult } from "@bnto/core";
import { computeTotalSaved } from "@bnto/core";
import type { FileProgress } from "../../store/types";

function RunningBanner({ fileProgress }: { fileProgress: FileProgress | null }) {
  const label = fileProgress
    ? `Processing file ${fileProgress.fileIndex + 1} of ${fileProgress.totalFiles}...`
    : "Initializing...";

  return (
    <StatusBanner>
      <StatusBannerRow>
        <StatusBannerIcon>
          <LoaderIcon className="motion-safe:animate-spin" />
        </StatusBannerIcon>
        <StatusBannerLabel muted>{label}</StatusBannerLabel>
      </StatusBannerRow>
      <StatusBannerProgress value={fileProgress?.overallPercent ?? 0} />
    </StatusBanner>
  );
}

function CompletedBanner({ results }: { results: BrowserFileResult[] }) {
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
        {saved > 0 && (
          <>
            <StatusBannerSpacer />
            <StatusBannerLabel muted mono>
              {formatFileSize(saved)} saved
            </StatusBannerLabel>
          </>
        )}
      </StatusBannerRow>
      <StatusBannerProgress value={100} variant="success" />
    </StatusBanner>
  );
}

function FailedBanner({ errors }: { errors: string[] }) {
  const title =
    errors.length === 0
      ? "Execution failed"
      : errors.length === 1
        ? errors[0]
        : `${errors.length} issues found`;

  return (
    <StatusBanner variant="error">
      <StatusBannerRow>
        <StatusBannerIcon>
          <XCircleIcon />
        </StatusBannerIcon>
        <StatusBannerLabel>{title}</StatusBannerLabel>
      </StatusBannerRow>
      <StatusBannerProgress value={100} variant="error" />
    </StatusBanner>
  );
}

function IdleBanner() {
  return (
    <StatusBanner>
      <StatusBannerRow>
        <StatusBannerLabel muted>Ready to run</StatusBannerLabel>
      </StatusBannerRow>
      <StatusBannerProgress value={0} />
    </StatusBanner>
  );
}

export { RunningBanner, CompletedBanner, FailedBanner, IdleBanner };
