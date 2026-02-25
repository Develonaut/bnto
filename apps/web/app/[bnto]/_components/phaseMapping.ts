import type { RunPhase } from "./RunButton";

/** Map browser execution status to RunButton phase. */
export function toBrowserPhase(status: string): RunPhase {
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

/** Derive display phase from local cloud phase + backend execution status. */
export function toCloudPhase(
  localPhase: RunPhase,
  executionStatus: string | undefined,
): RunPhase {
  if (localPhase === "uploading") return "uploading";
  if (!executionStatus || localPhase === "idle") return localPhase;

  switch (executionStatus) {
    case "pending":
    case "running":
      return "running";
    case "completed":
      return "completed";
    case "failed":
      return "failed";
    default:
      return localPhase;
  }
}
