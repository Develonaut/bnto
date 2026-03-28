import { ArrowLeftIcon, Button, DownloadIcon, Row, TrashIcon } from "@bnto/ui";
import { RunPlayButton } from "../RunPlayButton";
import type { BrowserFileResult } from "@bnto/core";
import type { FileProgress } from "../../store/types";
import { RunningBanner, CompletedBanner, FailedBanner, IdleBanner } from "./RunHeaderBanners";

interface RunHeaderProps {
  phase: string;
  inputFiles: File[];
  results: BrowserFileResult[];
  fileProgress: FileProgress | null;
  errors: string[];
  onBack: () => void;
  onClear: () => void;
  onDownloadAll: () => void;
  onRun: () => void;
}

function RunHeader(props: RunHeaderProps) {
  const {
    phase,
    inputFiles,
    results,
    fileProgress,
    errors,
    onBack,
    onClear,
    onDownloadAll,
    onRun,
  } = props;

  const isProcessing = phase === "running";
  const isDone = phase === "completed" || phase === "failed";
  const centerSlot = deriveCenterSlot(phase, results, fileProgress, errors);

  return (
    <div className="flex min-h-[4.5rem] items-center gap-4 bg-card px-3 py-2">
      <Row gap="xs">
        <Button
          variant="ghost"
          size="icon"
          elevation="sm"
          disabled={isProcessing}
          onClick={onBack}
          aria-label={isDone ? "Back to staging" : "Back to file selection"}
          data-testid="back-button"
        >
          <ArrowLeftIcon className="size-4" />
        </Button>
        <p className="shrink-0 text-sm font-medium text-foreground" data-testid="file-count">
          {inputFiles.length} {inputFiles.length === 1 ? "file" : "files"} selected
        </p>
      </Row>
      <div className="min-w-0 flex-1 border-border border-l border-r px-4">{centerSlot}</div>
      <HeaderActions
        phase={phase}
        results={results}
        onClear={onClear}
        onDownloadAll={onDownloadAll}
        onRun={onRun}
      />
    </div>
  );
}

/* ── Center slot — status banners ───────────────────────────── */

function deriveCenterSlot(
  phase: string,
  results: BrowserFileResult[],
  fileProgress: FileProgress | null,
  errors: string[],
) {
  if (phase === "running") return <RunningBanner fileProgress={fileProgress} />;
  if (phase === "completed") return <CompletedBanner results={results} />;
  if (phase === "failed") return <FailedBanner errors={errors} />;
  return <IdleBanner />;
}

/* ── Right side — action buttons ────────────────────────────── */

function HeaderActions({
  phase,
  results,
  onClear,
  onDownloadAll,
  onRun,
}: {
  phase: string;
  results: BrowserFileResult[];
  onClear: () => void;
  onDownloadAll: () => void;
  onRun: () => void;
}) {
  return (
    <Row gap="xs" className="ml-auto shrink-0">
      {phase === "completed" && (
        <Button
          variant="outline"
          size="icon"
          elevation="sm"
          onClick={onDownloadAll}
          aria-label={`Download all ${results.length} files`}
          data-testid="download-all-button"
        >
          <DownloadIcon className="size-4" />
        </Button>
      )}
      {phase === "idle" && (
        <Button variant="outline" size="icon" onClick={onClear} aria-label="Clear all files">
          <TrashIcon className="size-4" />
        </Button>
      )}
      <RunPlayButton onClick={onRun} />
    </Row>
  );
}

export { RunHeader };
export type { RunHeaderProps };
