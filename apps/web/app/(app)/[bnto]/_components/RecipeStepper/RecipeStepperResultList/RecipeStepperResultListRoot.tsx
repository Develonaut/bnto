"use client";

import { useCallback } from "react";
import { core } from "@bnto/core";
import { BouncyStagger, Grid } from "@bnto/ui";
import {
  useRecipeStepperStore,
  useRecipeStepperInstance,
  useRecipeStepperActions,
} from "../../../_stores/recipeStepperContext";
import { RecipeStepperResultListItem } from "./RecipeStepperResultListItemRoot";

/** File grid — reads files and execution state from the stepper store. */
export function RecipeStepperResultList() {
  const step = useRecipeStepperStore((s) => s.activeStep);
  const files = useRecipeStepperStore((s) => s.files);
  const instance = useRecipeStepperInstance();
  const execution = core.executions.useExecutionState(instance);
  const actions = useRecipeStepperActions();
  const handleDeleteFile = useCallback((i: number) => () => actions.deleteFile(i), [actions]);

  return (
    <BouncyStagger asChild>
      <Grid cols={1} gap="sm" role="list" aria-label="Selected files">
        {files.map((file, i) => {
          const result = step === 3 ? execution.results[i] : undefined;
          const isFileProcessing =
            step === 3 &&
            execution.status === "processing" &&
            execution.fileProgress?.fileIndex === i;

          return (
            <RecipeStepperResultListItem
              key={`${file.name}-${file.size}-${file.lastModified}`}
              file={file}
              result={result}
              isProcessing={isFileProcessing}
              isExecuting={step === 3}
              onDelete={handleDeleteFile(i)}
              onDownload={actions.downloadResult}
            />
          );
        })}
      </Grid>
    </BouncyStagger>
  );
}
