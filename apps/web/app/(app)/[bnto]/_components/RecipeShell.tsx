"use client";

import { type ReactNode, useCallback } from "react";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { Stepper, StepperList, StepperStep, StepperIndicator } from "@bnto/ui";
import { useRecipeFlow } from "../_hooks/useRecipeFlow";
import { RecipeShellUpload } from "./RecipeShellUpload";
import { SessionMarker } from "./SessionMarker";
import { deriveActivePhase } from "./phaseMapping";

const noop = () => {};

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
    <RecipeShellLayout activePhase={activePhase} isBrowserPath={flow.isBrowserPath}>
      <SessionMarker />
      <StepperIndicator />
      {children}
      <RecipeShellUpload
        flow={flow}
        entry={entry}
        onClearFiles={handleClearFiles}
        onDeleteFile={handleDeleteFile}
      />
    </RecipeShellLayout>
  );
}

function RecipeShellLayout({
  activePhase,
  isBrowserPath,
  children,
}: {
  activePhase: 1 | 2 | 3;
  isBrowserPath: boolean;
  children: ReactNode;
}) {
  return (
    <Stepper
      value={String(activePhase)}
      onValueChange={noop}
      className="space-y-6"
      data-testid="bnto-shell"
      data-session="ready"
      data-execution-mode={isBrowserPath ? "browser" : "cloud"}
    >
      <StepperList>
        <StepperStep value="1" label="Files" />
        <StepperStep value="2" label="Configure" />
        <StepperStep value="3" label="Results" />
      </StepperList>
      {children}
    </Stepper>
  );
}
