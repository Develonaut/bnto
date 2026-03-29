/** Maps execution status to UI flow phase. */

export type BentoFlowPhase = "idle" | "running" | "completed" | "failed";

const PHASE_MAP: Record<string, BentoFlowPhase> = {
  processing: "running",
  completed: "completed",
  failed: "failed",
};

export function mapExecutionPhase(status: string): BentoFlowPhase {
  return PHASE_MAP[status] ?? "idle";
}
