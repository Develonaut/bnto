"use client";

import { useCallback, useMemo } from "react";
import { Divider, FileUpload, FileUploadDropzone, toDropzoneAccept } from "@bnto/ui";
import { useEditor } from "../../context";
import { DropzoneContent } from "../DropzoneContent";
import { RunHeader } from "./RunHeader";
import { RunFileList } from "./RunFileList";

/**
 * RunTab — merged input staging + execution results.
 *
 * Three screens, navigable via back button:
 * 1. Dropzone (idle, no files)
 * 2. Files selected (idle, has files) — StatusBanner + file list
 * 3. Results (running/completed/failed) — StatusBanner + file list
 *
 * Back steps: results → files selected → dropzone.
 */
function RunTab() {
  const editor = useEditor();
  const { inputFiles, fileAccept, phase, results, fileProgress, errors } =
    editor.execution.useExecution();

  const setFiles = useCallback((files: File[]) => editor.execution.setInputFiles(files), [editor]);

  const removeFile = useCallback(
    (index: number) => setFiles(inputFiles.filter((_, i) => i !== index)),
    [inputFiles, setFiles],
  );

  const handleBack = useCallback(() => {
    if (phase === "idle") {
      setFiles([]);
    } else {
      const files = inputFiles;
      editor.execution.resetRun();
      editor.execution.setInputFiles(files);
    }
  }, [phase, inputFiles, editor, setFiles]);

  const acceptLabel = fileAccept && fileAccept !== "*/*" ? fileAccept : "all files";
  const dropzoneAccept = useMemo(
    () => (fileAccept ? toDropzoneAccept(fileAccept) : undefined),
    [fileAccept],
  );

  const showDropzone = phase === "idle" && inputFiles.length === 0;

  return (
    <div className="flex h-full flex-col overflow-hidden rounded-[inherit] bg-input">
      <FileUpload
        value={inputFiles}
        onValueChange={setFiles}
        accept={dropzoneAccept}
        multiple
        className="flex h-full flex-col gap-0"
      >
        {showDropzone ? (
          <div className="min-h-0 flex-1 p-3">
            <FileUploadDropzone disableAnimation className="h-full gap-3 p-6">
              <DropzoneContent label={acceptLabel} />
            </FileUploadDropzone>
          </div>
        ) : (
          <>
            <RunHeader
              phase={phase}
              inputFiles={inputFiles}
              results={results}
              fileProgress={fileProgress}
              errors={errors}
              onBack={handleBack}
              onClear={() => setFiles([])}
              onDownloadAll={() => editor.execution.downloadAllResults()}
            />
            <Divider />
            <div className="min-h-0 flex-1 overflow-y-auto">
              <RunFileList
                phase={phase}
                inputFiles={inputFiles}
                results={results}
                fileProgress={fileProgress}
                onRemove={removeFile}
                onDownload={(r) => editor.execution.downloadResult(r)}
              />
            </div>
          </>
        )}
      </FileUpload>
    </div>
  );
}

export { RunTab };
