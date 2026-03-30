"use client";

import { Stack } from "@bnto/ui";
import type { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { DynamicRecipeConfig } from "./DynamicRecipeConfig";
import { RecipeToolbar } from "./RecipeToolbar";
import { RecipeResultsSection } from "./RecipeResultsSection";
import { RecipeFileGrid } from "./RecipeFileGrid";
import { ToolbarProgress } from "./ToolbarProgress";

interface RecipePhaseContentProps {
  activePhase: 2 | 3;
  flow: ReturnType<typeof useRecipeFlow>;
  onBack: () => void;
  onDeleteFile: (index: number) => () => void;
}

/**
 * Phases 2-3 content block.
 *
 * Composes toolbar, error cards, cloud results, and file grid
 * into the layout shown after files are selected.
 */
export function RecipePhaseContent(props: RecipePhaseContentProps) {
  const { activePhase, flow, onBack, onDeleteFile } = props;

  return (
    <Stack className="gap-4 text-left">
      <RecipeToolbar
        activePhase={activePhase}
        resolvedPhase={flow.resolvedPhase}
        isProcessing={flow.isProcessing}
        fileCount={flow.files.length}
        onBack={onBack}
        onRun={flow.handleRun}
        onDownloadAll={flow.downloadAll}
        centerContent={deriveCenterContent(activePhase, flow)}
      />
      <PhaseThreeCloudResults activePhase={activePhase} flow={flow} />
      <RecipeFileGrid
        files={flow.files}
        activePhase={activePhase}
        isBrowserPath={flow.isBrowserPath}
        browserExec={flow.browserExec}
        onDeleteFile={onDeleteFile}
        onDownload={flow.downloadResult}
      />
    </Stack>
  );
}

/** Derive the toolbar center content based on the active phase. */
function deriveCenterContent(activePhase: 2 | 3, flow: ReturnType<typeof useRecipeFlow>) {
  if (activePhase === 2 && flow.definition) {
    return (
      <DynamicRecipeConfig
        definition={flow.definition}
        config={flow.config}
        onChange={flow.setNodeParam}
      />
    );
  }
  if (flow.isBrowserPath) {
    return <ToolbarProgress execution={flow.browserExec} />;
  }
  return;
}

/** Cloud results section for Phase 3. */
function PhaseThreeCloudResults({
  activePhase,
  flow,
}: {
  activePhase: 2 | 3;
  flow: ReturnType<typeof useRecipeFlow>;
}) {
  if (activePhase !== 3 || flow.isBrowserPath) return null;
  return (
    <RecipeResultsSection
      isBrowserPath={false}
      resolvedPhase={flow.resolvedPhase}
      browserExec={flow.browserExec}
      onDownload={flow.downloadResult}
      executionId={flow.executionId}
      uploadProgress={flow.uploadProgress}
      clientError={flow.clientError}
    />
  );
}
