import { useCallback } from "react";
import {
  FileList,
  FileListContent,
  FileListIcon,
  FileListItem,
  FileListName,
  IconBadge,
  XCircleIcon,
} from "@bnto/ui";
import type { BrowserFileResult } from "@bnto/core";
import { deriveFileResultProps } from "@bnto/core";
import type { FileProgress } from "../../store/types";
import { InputRow } from "./InputRow";
import { ResultRow } from "./ResultRow";

interface RunFileListProps {
  phase: string;
  inputFiles: File[];
  results: BrowserFileResult[];
  errors: string[];
  fileProgress: FileProgress | null;
  onRemove: (index: number) => void;
  onDownload: (result: BrowserFileResult) => void;
}

function RunFileList({
  phase,
  inputFiles,
  results,
  errors,
  fileProgress,
  onRemove,
  onDownload,
}: RunFileListProps) {
  if (phase === "failed") {
    return <FailedFileList errors={errors} results={results} onDownload={onDownload} />;
  }
  if (phase === "completed") {
    return <CompletedFileList results={results} onDownload={onDownload} />;
  }
  if (phase === "running") {
    return (
      <RunningFileList
        inputFiles={inputFiles}
        results={results}
        fileProgress={fileProgress}
        onDownload={onDownload}
      />
    );
  }
  return <IdleFileList inputFiles={inputFiles} onRemove={onRemove} />;
}

/* -- Per-item row wrappers (avoid inline arrow in JSX) ---------- */

function CompletedResultItem({
  result,
  index,
  onDownload,
}: {
  result: BrowserFileResult;
  index: number;
  onDownload: (r: BrowserFileResult) => void;
}) {
  const d = deriveFileResultProps(result);
  const handleDownload = useCallback(() => onDownload(result), [onDownload, result]);
  return (
    <ResultRow
      key={`${d.filename}-${index}`}
      filename={d.filename}
      extension={d.extension}
      outputSize={d.outputSize}
      originalSize={d.originalSize}
      savings={d.savings}
      onDownload={handleDownload}
    />
  );
}

function RunningRow({
  file,
  index,
  result,
  fileProgress,
  onDownload,
}: {
  file: File;
  index: number;
  result: BrowserFileResult | undefined;
  fileProgress: FileProgress | null;
  onDownload: (r: BrowserFileResult) => void;
}) {
  if (result) {
    return <CompletedResultItem result={result} index={index} onDownload={onDownload} />;
  }
  return (
    <InputRow
      key={`${file.name}-${file.size}-${index}`}
      file={file}
      processing={fileProgress?.fileIndex === index}
    />
  );
}

function IdleRow({
  file,
  index,
  onRemove,
}: {
  file: File;
  index: number;
  onRemove: (i: number) => void;
}) {
  const handleRemove = useCallback(() => onRemove(index), [onRemove, index]);
  return (
    <InputRow key={`${file.name}-${file.size}-${index}`} file={file} onRemove={handleRemove} />
  );
}

/* -- Error display ---------------------------------------------- */

function ErrorRow({ message }: { message: string }) {
  return (
    <FileListItem data-testid="error-row">
      <FileListIcon>
        <IconBadge variant="destructive" size="lg" aria-hidden="true">
          <XCircleIcon className="size-5" />
        </IconBadge>
      </FileListIcon>
      <FileListContent>
        <FileListName className="text-destructive">{message}</FileListName>
      </FileListContent>
    </FileListItem>
  );
}

/* -- Phase-specific lists --------------------------------------- */

function FailedFileList({
  errors,
  results,
  onDownload,
}: {
  errors: string[];
  results: BrowserFileResult[];
  onDownload: (r: BrowserFileResult) => void;
}) {
  return (
    <FileList as="ul" className="px-4 py-3">
      {errors.map((msg, i) => (
        <ErrorRow key={`error-${i}`} message={msg} />
      ))}
      {results.map((r, i) => (
        <CompletedResultItem
          key={`${r.filename}-${i}`}
          result={r}
          index={i}
          onDownload={onDownload}
        />
      ))}
    </FileList>
  );
}

function CompletedFileList({
  results,
  onDownload,
}: {
  results: BrowserFileResult[];
  onDownload: (r: BrowserFileResult) => void;
}) {
  return (
    <FileList as="ul" className="px-4 py-3">
      {results.map((r, i) => (
        <CompletedResultItem
          key={`${r.filename}-${i}`}
          result={r}
          index={i}
          onDownload={onDownload}
        />
      ))}
    </FileList>
  );
}

function RunningFileList({
  inputFiles,
  results,
  fileProgress,
  onDownload,
}: {
  inputFiles: File[];
  results: BrowserFileResult[];
  fileProgress: FileProgress | null;
  onDownload: (r: BrowserFileResult) => void;
}) {
  return (
    <FileList as="ul" className="px-4 py-3">
      {inputFiles.map((file, i) => (
        <RunningRow
          key={`${file.name}-${file.size}-${i}`}
          file={file}
          index={i}
          result={results[i]}
          fileProgress={fileProgress}
          onDownload={onDownload}
        />
      ))}
    </FileList>
  );
}

function IdleFileList({
  inputFiles,
  onRemove,
}: {
  inputFiles: File[];
  onRemove: (index: number) => void;
}) {
  return (
    <FileList as="ul" className="px-4 py-3">
      {inputFiles.map((file, i) => (
        <IdleRow key={`${file.name}-${file.size}-${i}`} file={file} index={i} onRemove={onRemove} />
      ))}
    </FileList>
  );
}

export { RunFileList };
export type { RunFileListProps };
