"use client";

import type { BntoEntry } from "@/lib/bntoRegistry";
import { FileUpload, StepperContent } from "@bnto/ui";
import type { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { RecipeDropzone } from "./RecipeDropzone";
import { RecipePhaseContent } from "./RecipePhaseContent";

interface RecipeShellUploadProps {
  flow: ReturnType<typeof useRecipeFlow>;
  entry: BntoEntry;
  onClearFiles: () => void;
  onDeleteFile: (index: number) => () => void;
}

/** File upload wrapper — renders dropzone or phase content via Stepper. */
export function RecipeShellUpload({
  flow,
  entry,
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
      className="h-full [&>*]:flex-1"
    >
      <UploadPhases
        flow={flow}
        entry={entry}
        onClearFiles={onClearFiles}
        onDeleteFile={onDeleteFile}
      />
    </FileUpload>
  );
}

function UploadPhases({ flow, entry, onClearFiles, onDeleteFile }: RecipeShellUploadProps) {
  return (
    <>
      <StepperContent value="1">
        <RecipeDropzone acceptLabel={flow.acceptLabel} />
      </StepperContent>
      <StepperContent value="2">
        <RecipePhaseContent entry={entry} activePhase={2} flow={flow} onDeleteFile={onDeleteFile} />
      </StepperContent>
      <StepperContent value="3">
        <RecipePhaseContent entry={entry} activePhase={3} flow={flow} onDeleteFile={onDeleteFile} />
      </StepperContent>
    </>
  );
}
