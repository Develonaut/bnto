import type { RunStep } from "./RunButton";

/**
 * Convert an execution status to a RunButton display step.
 *
 * The only rename is "processing" → "running" because the RunButton
 * calls it "running" (it doesn't know or care about the execution internals).
 */
export function toRunStep(status: string): RunStep {
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
