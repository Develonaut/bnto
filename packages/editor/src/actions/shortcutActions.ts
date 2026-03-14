/**
 * shortcutActions — pure guard/dispatch logic for keyboard shortcuts.
 *
 * Each function decides whether a shortcut should fire and returns
 * the action to take. The hook (`useEditorShortcuts`) calls these
 * and dispatches the result. Pure functions = trivially testable
 * without React or ReactFlow context.
 */

import type { ExecutionPhase, PanelState } from "../store/types";

/** Returns the node ID to delete, or null if nothing should happen. */
function resolveDelete(selectedNodeId: string | null): string | null {
  return selectedNodeId;
}

/** Returns true if a run should be triggered. */
function canTriggerRun(phase: ExecutionPhase, inputFileCount: number): boolean {
  if (phase === "running") return false;
  return inputFileCount > 0;
}

/** Describes what Escape should do based on current state. */
interface EscapeResult {
  closeConfig: boolean;
  closeRun: boolean;
  deselect: boolean;
}

function resolveEscape(selectedNodeId: string | null, panels: PanelState): EscapeResult {
  return {
    closeConfig: panels.config,
    closeRun: panels.run,
    deselect: selectedNodeId !== null,
  };
}

export { resolveDelete, canTriggerRun, resolveEscape };
export type { EscapeResult };
