"use client";

import type { BntoEntry } from "@/lib/bntoRegistry";
import { FileUpload } from "@bnto/ui";
import type { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { RecipeDropzone } from "./RecipeDropzone";
import { RecipePhaseContent } from "./RecipePhaseContent";

interface RecipeShellUploadProps {
  flow: ReturnType<typeof useRecipeFlow>;
  entry: BntoEntry;
  activePhase: 1 | 2 | 3;
  onClearFiles: () => void;
  onDeleteFile: (index: number) => () => void;
}

/** File upload wrapper — renders dropzone or phase 2-3 content. */
export function RecipeShellUpload({
  flow,
  entry,
  activePhase,
  onClearFiles,
  onDeleteFile,
}: RecipeShellUploadProps) {
  return (
    <FileUpload
      value={flow.files}
      onValueChange={flow.setFiles}
      accept={flow.dropzoneAccept}
      multiple
      disabled={flow.isProcessing}
    >
      {activePhase === 1 && <RecipeDropzone acceptLabel={flow.acceptLabel} />}
      {(activePhase === 2 || activePhase === 3) && (
        <RecipePhaseContent
          entry={entry}
          activePhase={activePhase as 2 | 3}
          flow={flow}
          onBack={activePhase === 3 ? flow.handleResetExecution : onClearFiles}
          onDeleteFile={onDeleteFile}
        />
      )}
    </FileUpload>
  );
}
