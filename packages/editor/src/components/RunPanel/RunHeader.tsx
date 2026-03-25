import {
  ArrowLeftIcon,
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
  TrashIcon,
  XCircleIcon,
  formatFileSize,
} from "@bnto/ui";
import type { BrowserFileResult } from "@bnto/core";
import { computeTotalSaved } from "@bnto/core";
import type { FileProgress } from "../../store/types";

interface RunHeaderProps {
  phase: string;
  inputFiles: File[];
  results: BrowserFileResult[];
  fileProgress: FileProgress | null;
  errors: string[];
  onBack: () => void;
  onClear: () => void;
  onDownloadAll: () => void;
}

function RunHeader(props: RunHeaderProps) {
  const { phase } = props;
  if (phase === "idle") return <IdleHeader {...props} />;
  if (phase === "failed") return <FailedHeader {...props} />;
  if (phase === "running") return <RunningHeader {...props} />;
  return <CompletedHeader {...props} />;
}

/* ── Phase-specific headers ─────────────────────────────────── */

function HeaderShell({ onBack, children }: { onBack: () => void; children: React.ReactNode }) {
  return (
    <div className="flex shrink-0 items-center gap-2 bg-card px-3 py-2">
      <Button variant="ghost" size="icon" onClick={onBack} aria-label="Back">
        <ArrowLeftIcon />
      </Button>
      {children}
    </div>
  );
}

function IdleHeader({ inputFiles, onBack, onClear }: RunHeaderProps) {
  const count = inputFiles.length;
  return (
    <HeaderShell onBack={onBack}>
      <StatusBanner variant="secondary" dense className="flex-1">
        <StatusBannerRow>
          <StatusBannerLabel>
            {count} {count === 1 ? "file" : "files"} selected
          </StatusBannerLabel>
          <StatusBannerSpacer />
          <Button
            variant="outline"
            size="icon"
            icon={<TrashIcon />}
            onClick={onClear}
            aria-label="Clear all files"
          />
        </StatusBannerRow>
      </StatusBanner>
    </HeaderShell>
  );
}

function FailedHeader({ errors, onBack }: RunHeaderProps) {
  const title =
    errors.length === 0
      ? "Execution failed"
      : errors.length === 1
        ? "1 issue found"
        : `${errors.length} issues found`;

  return (
    <HeaderShell onBack={onBack}>
      <StatusBanner variant="error" dense className="flex-1">
        <StatusBannerRow>
          <StatusBannerIcon>
            <XCircleIcon />
          </StatusBannerIcon>
          <StatusBannerLabel>{title}</StatusBannerLabel>
        </StatusBannerRow>
      </StatusBanner>
    </HeaderShell>
  );
}

function RunningHeader({ inputFiles, fileProgress, onBack }: RunHeaderProps) {
  const label = fileProgress
    ? `Processing file ${fileProgress.fileIndex + 1} of ${fileProgress.totalFiles}...`
    : `0 of ${inputFiles.length} files`;

  return (
    <HeaderShell onBack={onBack}>
      <StatusBanner variant="processing" dense className="flex-1">
        <StatusBannerRow>
          <StatusBannerIcon>
            <LoaderIcon className="motion-safe:animate-spin" />
          </StatusBannerIcon>
          <StatusBannerLabel muted>{label}</StatusBannerLabel>
        </StatusBannerRow>
        <StatusBannerProgress value={fileProgress?.overallPercent ?? 0} />
      </StatusBanner>
    </HeaderShell>
  );
}

function CompletedHeader({ results, onBack, onDownloadAll }: RunHeaderProps) {
  const saved = computeTotalSaved(results);
  return (
    <HeaderShell onBack={onBack}>
      <StatusBanner variant="success" dense className="flex-1">
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
            size="icon"
            icon={<DownloadIcon />}
            onClick={onDownloadAll}
            aria-label={`Download all ${results.length} files`}
          />
        </StatusBannerRow>
      </StatusBanner>
    </HeaderShell>
  );
}

export { RunHeader };
export type { RunHeaderProps };
