"use client";

import {
  Button,
  CheckCircle2Icon,
  DownloadIcon,
  formatFileSize,
  LinearProgress,
  LoaderIcon,
  ResultFileCard,
  Text,
  XCircleIcon,
} from "@bnto/ui";
import type { BrowserFileResult } from "@bnto/core";
import { useFileResultProps } from "@bnto/core";
import { useEditorExecutionContext } from "../../hooks/EditorExecutionContext";

/**
 * ResultsTab — consumes execution state directly from context.
 *
 * Running: LinearProgress with file counter.
 * Completed: summary header + ResultFileCards + download all.
 * Failed: error messages.
 * Idle: prompt.
 */
function ResultsTab() {
  const { phase, results, errors, fileProgress, inputFiles, downloadAll } =
    useEditorExecutionContext();

  if (phase === "idle") {
    return (
      <div className="flex h-full items-center justify-center p-4">
        <Text size="xs" color="muted">
          Run a recipe to see results.
        </Text>
      </div>
    );
  }

  if (phase === "running") {
    return (
      <div className="flex h-full flex-col gap-4 p-4">
        <RunningProgress />
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col overflow-y-auto">
      {errors.length > 0 && <ErrorSection />}

      {results.length > 0 && (
        <>
          <CompletedSummary />
          <div className="flex flex-col gap-1.5 p-2">
            {results.map((file, i) => (
              <ResultRow key={`${file.filename}-${i}`} result={file} />
            ))}
          </div>
        </>
      )}

      {results.length === 0 && errors.length === 0 && (
        <div className="flex h-full items-center justify-center p-4">
          <Text size="xs" color="muted">
            No output files.
          </Text>
        </div>
      )}
    </div>
  );
}

/** Single result row using the shared ResultFileCard. */
function ResultRow({ result }: { result: BrowserFileResult }) {
  const { downloadFile } = useEditorExecutionContext();
  const props = useFileResultProps(result);

  return (
    <ResultFileCard
      filename={props.filename}
      extension={props.extension}
      outputSize={props.outputSize}
      originalSize={props.originalSize}
      savings={props.savings}
      action={
        <Button
          variant="outline"
          size="sm"
          icon={<DownloadIcon />}
          onClick={() => downloadFile(result)}
          aria-label={`Download ${result.filename}`}
        />
      }
    />
  );
}

/** Progress bar with file counter while execution is running. */
function RunningProgress() {
  const { fileProgress, inputFiles } = useEditorExecutionContext();
  const totalFiles = inputFiles.length;

  if (!fileProgress) {
    return (
      <LinearProgress
        value={0}
        icon={<LoaderIcon className="size-4 shrink-0 text-primary motion-safe:animate-spin" />}
        label={totalFiles > 0 ? `0 of ${totalFiles} files` : "Initializing..."}
        valueLabel=""
      />
    );
  }

  return (
    <LinearProgress
      value={fileProgress.overallPercent}
      icon={<LoaderIcon className="size-4 shrink-0 text-primary motion-safe:animate-spin" />}
      label={`Processing file ${fileProgress.fileIndex + 1} of ${fileProgress.totalFiles}...`}
    />
  );
}

/** Summary header showing file count, total savings, and download all button. */
function CompletedSummary() {
  const { results, downloadAll } = useEditorExecutionContext();

  const totalSaved = results.reduce((acc, r) => {
    const orig = r.metadata.originalSize as number | undefined;
    return orig != null ? acc + (orig - r.blob.size) : acc;
  }, 0);

  const label = `${results.length} ${results.length === 1 ? "file" : "files"} processed`;
  const saved = totalSaved > 0 ? `${formatFileSize(totalSaved)} saved` : null;

  return (
    <div className="flex items-center justify-between border-b border-border px-3 py-2.5">
      <div className="flex items-center gap-2">
        <CheckCircle2Icon className="size-4 text-primary" />
        <Text size="xs" className="font-medium">
          {label}
        </Text>
        {saved && (
          <Text size="xs" color="muted" className="font-mono tabular-nums">
            {saved}
          </Text>
        )}
      </div>
      {results.length > 1 && (
        <Button
          variant="outline"
          size="sm"
          icon={<DownloadIcon />}
          onClick={downloadAll}
          aria-label={`Download all ${results.length} files`}
        />
      )}
    </div>
  );
}

/** Error messages section. */
function ErrorSection() {
  const { errors } = useEditorExecutionContext();

  return (
    <div className="flex flex-col gap-1 border-b border-border p-3">
      {errors.map((error, i) => (
        <div key={i} className="flex items-start gap-2">
          <XCircleIcon className="mt-0.5 size-3.5 shrink-0 text-destructive" />
          <Text size="xs" className="text-destructive">
            {error}
          </Text>
        </div>
      ))}
    </div>
  );
}

export { ResultsTab };
