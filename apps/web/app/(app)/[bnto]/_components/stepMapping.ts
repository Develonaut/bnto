/**
 * Step Mapping — Translates execution status into UI display steps.
 *
 * Converts internal status strings into the unified RunPhase type
 * that the UI uses for the RunButton and stepper.
 *
 * Execution status → toRunPhase() → RunPhase
 * Statuses: "idle" | "processing" | "completed" | "failed"
 * Simple 1:1 mapping (processing → running, everything else passes through).
 */

import type { RunPhase } from "./RunButton";

/**
 * Convert an execution status to a RunButton display phase.
 *
 * The only rename is "processing" → "running" because the RunButton
 * calls it "running" (it doesn't know or care about the execution internals).
 *
 * @param status - The execution status from useExecutionState()
 * @returns The RunPhase the button should display
 */
export function toRunPhase(status: string): RunPhase {
  switch (status) {
    case "processing":
      return "running";
    case "completed":
      return "completed";
    case "failed":
      return "failed";
    default:
      return "idle";
  }
}

/**
 * Map the unified RunPhase + file count to the 3-step stepper position.
 *
 * Step 1 = no files yet (upload prompt)
 * Step 2 = files selected (configure)
 * Step 3 = execution in progress or complete
 */
export function deriveActiveStep(resolvedPhase: RunPhase, fileCount: number): 1 | 2 | 3 {
  switch (resolvedPhase) {
    case "running":
    case "completed":
    case "failed":
      return 3;
    default:
      return fileCount > 0 ? 2 : 1;
  }
}
