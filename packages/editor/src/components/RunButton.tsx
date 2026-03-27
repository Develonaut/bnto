"use client";

import { forwardRef, useCallback, useRef } from "react";
import { useExecution } from "../hooks/useExecution";
import { RunPlayButton } from "./RunPlayButton";

/**
 * RunButton — toolbar run button with hidden file input for selecting files.
 *
 * Wraps RunPlayButton with file-picker logic:
 * - No files staged → click opens file picker → runs on selection
 * - Files staged (via RunTab) → click runs staged files immediately
 * - Running → disabled (handled by RunPlayButton)
 * - Completed/failed → click reruns with same files
 */
function RunButton() {
  const { inputFiles, fileAccept, run } = useExecution();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const hasFiles = inputFiles.length > 0;

  const handleClick = useCallback(() => {
    if (hasFiles) {
      run(inputFiles);
      return;
    }
    fileInputRef.current?.click();
  }, [hasFiles, inputFiles, run]);

  const handleFileChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const files = e.target.files;
      if (!files || files.length === 0) return;
      run(Array.from(files));
      e.target.value = "";
    },
    [run],
  );

  return (
    <>
      <RunFileInput ref={fileInputRef} accept={fileAccept} onChange={handleFileChange} />
      <RunPlayButton onClick={handleClick} />
    </>
  );
}

interface RunFileInputProps {
  accept: string | undefined;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

const RunFileInput = forwardRef<HTMLInputElement, RunFileInputProps>(function RunFileInput(
  { accept, onChange },
  ref,
) {
  return (
    <input
      ref={ref}
      type="file"
      multiple
      accept={accept}
      aria-label="Select files to process"
      className="hidden"
      onChange={onChange}
      data-testid="run-file-input"
    />
  );
});

export { RunButton };
