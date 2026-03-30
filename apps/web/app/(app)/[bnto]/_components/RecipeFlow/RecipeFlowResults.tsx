"use client";

import type { BrowserExecution } from "@bnto/core";
import { StepperContent } from "@bnto/ui";
import {
  useRecipeFlowStore,
  useRecipeFlowDefn,
  useRecipeFlowCloudRefs,
} from "../../_stores/recipeFlowContext";
import { RecipeResultsSection } from "../RecipeResultsSection";

const noop = () => {};

/** Stub — cloud path never reads browser execution state. */
const CLOUD_BROWSER_EXEC: BrowserExecution = {
  id: "",
  status: "idle",
  results: [],
  fileProgress: null,
};

/**
 * Phase 3 cloud results only.
 *
 * Browser execution results are rendered inline by RecipeFilesGrid
 * (via CompletedFileRow per file). This component handles cloud
 * results which have a separate upload/execution lifecycle.
 */
export function RecipeFlowResults() {
  const step = useRecipeFlowStore((s) => s.activeStep);
  const { isBrowserPath } = useRecipeFlowDefn();
  const resolvedPhase = useRecipeFlowStore((s) => s.resolvedPhase);
  const executionId = useRecipeFlowStore((s) => s.executionId);
  const clientError = useRecipeFlowStore((s) => s.clientError);
  const cloudRefs = useRecipeFlowCloudRefs();

  // Browser results handled by RecipeFilesGrid — skip here.
  if (step !== 3 || isBrowserPath) return null;

  return (
    <StepperContent value="3">
      <RecipeResultsSection
        isBrowserPath={false}
        resolvedPhase={resolvedPhase}
        browserExec={CLOUD_BROWSER_EXEC}
        onDownload={noop}
        executionId={executionId}
        uploadProgress={cloudRefs.getUploadProgress()}
        clientError={clientError}
      />
    </StepperContent>
  );
}
