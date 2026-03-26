"use client";

import { type ReactNode, useCallback } from "react";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { PhaseIndicator } from "./PhaseIndicator";
import { RecipeShellUpload } from "./RecipeShellUpload";
import { SessionMarker } from "./SessionMarker";
import { deriveActivePhase } from "./phaseMapping";

/**
 * Recipe page interactive flow -- client island.
 *
 * Composes the progressive phase flow:
 *   Phase 1 (Files)     -> dropzone
 *   Phase 2 (Configure) -> file grid + config panel
 *   Phase 3 (Results)   -> execution progress / results
 */
export function RecipeShell({ entry, children }: { entry: BntoEntry; children?: ReactNode }) {
  const flow = useRecipeFlow({ entry });
  const activePhase = deriveActivePhase(flow.resolvedPhase, flow.files.length);
  const handleClearFiles = useCallback(() => flow.setFiles([]), [flow]);
  const handleDeleteFile = useCallback(
    (index: number) => () => flow.setFiles(flow.files.filter((_, j) => j !== index)),
    [flow],
  );

  return (
    <div
      className="space-y-6"
      data-testid="bnto-shell"
      data-session="ready"
      data-execution-mode={flow.isBrowserPath ? "browser" : "cloud"}
    >
      <SessionMarker />
      <PhaseIndicator activePhase={activePhase} />
      {children}
      <RecipeShellUpload
        flow={flow}
        entry={entry}
        activePhase={activePhase}
        onClearFiles={handleClearFiles}
        onDeleteFile={handleDeleteFile}
      />
    </div>
  );
}
