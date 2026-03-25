import { FileList } from "@bnto/ui";
import type { BrowserFileResult } from "@bnto/core";
import { deriveFileResultProps } from "@bnto/core";
import type { FileProgress } from "../../store/types";
import { InputRow } from "./InputRow";
import { ResultRow } from "./ResultRow";

interface RunFileListProps {
  phase: string;
  inputFiles: File[];
  results: BrowserFileResult[];
  fileProgress: FileProgress | null;
  onRemove: (index: number) => void;
  onDownload: (result: BrowserFileResult) => void;
}

function RunFileList({
  phase,
  inputFiles,
  results,
  fileProgress,
  onRemove,
  onDownload,
}: RunFileListProps) {
  if (phase === "completed" || phase === "failed") {
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

/* ── Phase-specific lists ────────────────────────────────────── */

function CompletedFileList({
  results,
  onDownload,
}: {
  results: BrowserFileResult[];
  onDownload: (r: BrowserFileResult) => void;
}) {
  return (
    <FileList as="ul" className="px-4 py-3">
      {results.map((r, i) => {
        const d = deriveFileResultProps(r);
        return (
          <ResultRow
            key={`${d.filename}-${i}`}
            filename={d.filename}
            extension={d.extension}
            outputSize={d.outputSize}
            originalSize={d.originalSize}
            savings={d.savings}
            onDownload={() => onDownload(r)}
          />
        );
      })}
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
      {inputFiles.map((file, i) => {
        const result = results[i];
        if (result) {
          const d = deriveFileResultProps(result);
          return (
            <ResultRow
              key={`${d.filename}-${i}`}
              filename={d.filename}
              extension={d.extension}
              outputSize={d.outputSize}
              originalSize={d.originalSize}
              savings={d.savings}
              onDownload={() => onDownload(result)}
            />
          );
        }
        return (
          <InputRow
            key={`${file.name}-${file.size}-${i}`}
            file={file}
            processing={fileProgress?.fileIndex === i}
          />
        );
      })}
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
        <InputRow key={`${file.name}-${file.size}-${i}`} file={file} onRemove={() => onRemove(i)} />
      ))}
    </FileList>
  );
}

export { RunFileList };
export type { RunFileListProps };
