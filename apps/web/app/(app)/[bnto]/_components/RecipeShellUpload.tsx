"use client";

import { FileUpload, StepperContent } from "@bnto/ui";
import type { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { RecipeDropzone } from "./RecipeDropzone";
import { RecipePhaseContent } from "./RecipePhaseContent";

interface RecipeShellUploadProps {
  flow: ReturnType<typeof useRecipeFlow>;
  onClearFiles: () => void;
  onDeleteFile: (index: number) => () => void;
}

/** File upload wrapper — renders dropzone or phase content via Stepper. */
export function RecipeShellUpload({ flow, onClearFiles, onDeleteFile }: RecipeShellUploadProps) {
  return (
    <FileUpload
      value={flow.files}
      onValueChange={flow.setFiles}
      accept={flow.dropzoneAccept}
      multiple
      disabled={flow.isProcessing}
    >
      <UploadPhases flow={flow} onClearFiles={onClearFiles} onDeleteFile={onDeleteFile} />
    </FileUpload>
  );
}

function UploadPhases({ flow, onClearFiles, onDeleteFile }: RecipeShellUploadProps) {
  return (
    <>
      <StepperContent value="1">
        <RecipeDropzone acceptLabel={flow.acceptLabel} />
      </StepperContent>
      <StepperContent value="2">
        <RecipePhaseContent
          activePhase={2}
          flow={flow}
          onBack={onClearFiles}
          onDeleteFile={onDeleteFile}
        />
      </StepperContent>
      <StepperContent value="3">
        <RecipePhaseContent
          activePhase={3}
          flow={flow}
          onBack={flow.handleResetExecution}
          onDeleteFile={onDeleteFile}
        />
      </StepperContent>
    </>
  );
}
