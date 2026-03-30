"use client";

import type { BntoEntry } from "@/lib/bntoRegistry";
import { Surface } from "@bnto/ui";
import type { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { RecipeResultsSection } from "./RecipeResultsSection";
import { RecipeFileGrid } from "./RecipeFileGrid";

interface RecipePhaseContentProps {
  entry: BntoEntry;
  activePhase: 2 | 3;
  flow: ReturnType<typeof useRecipeFlow>;
  onDeleteFile: (index: number) => () => void;
}

/**
 * Phases 2-3 file list content.
 *
 * Scrollable file grid with optional cloud results section.
 */
export function RecipePhaseContent(props: RecipePhaseContentProps) {
  const { activePhase, flow, onDeleteFile } = props;

  return (
    <Surface elevation="sm" className="h-full">
      <div className="h-full overflow-y-auto p-4">
        <PhaseThreeCloudResults activePhase={activePhase} flow={flow} />
        <RecipeFileGrid
          files={flow.files}
          activePhase={activePhase}
          isBrowserPath={flow.isBrowserPath}
          browserExec={flow.browserExec}
          onDeleteFile={onDeleteFile}
          onDownload={flow.downloadResult}
        />
      </div>
    </Surface>
  );
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
