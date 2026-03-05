/**
 * Phase Mapping — Translates execution status into UI display phases.
 *
 * WHY THIS EXISTS:
 * Bnto has two execution paths — browser (WASM) and cloud (R2 + Go API).
 * Each path uses different status strings internally, but the RunButton
 * only understands one set of display phases: "idle", "uploading", "running",
 * "completed", "failed". These functions act as translators — they convert
 * each path's internal status into the unified RunPhase type that the UI uses.
 *
 * Think of it like an airport departures board: different airlines use
 * different internal systems, but the board shows the same set of statuses
 * (On Time, Boarding, Departed, Delayed) regardless of airline.
 *
 * BROWSER PATH:
 *   browserExec.status → toBrowserPhase() → RunPhase
 *   Statuses: "idle" | "processing" | "completed" | "failed"
 *   Simple 1:1 mapping (processing → running, everything else passes through).
 *
 * CLOUD PATH:
 *   localPhase + cloudExecution.status → toCloudPhase() → RunPhase
 *   More complex because the cloud path has TWO sources of truth:
 *     1. localPhase — what the client knows (e.g., "I started uploading")
 *     2. executionStatus — what the server reports (e.g., "running", "completed")
 *   The function merges these into a single display phase.
 */

import type { RunPhase } from "./RunButton";

/**
 * Convert a browser execution status to a RunButton display phase.
 *
 * The browser path is straightforward — the status comes from our local
 * state machine (browserExecutionStore.ts), so it maps almost 1:1.
 * The only rename is "processing" → "running" because the RunButton
 * calls it "running" (it doesn't know or care about browser vs cloud).
 *
 * @param status - The browser execution status from useBrowserExecution()
 * @returns The RunPhase the button should display
 */
export function toBrowserPhase(status: string): RunPhase {
  switch (status) {
    // "processing" is what our store calls it; "running" is what the UI shows
    case "processing":
      return "running";
    case "completed":
      return "completed";
    case "failed":
      return "failed";
    // Anything unexpected (including "idle") → show the idle/ready state
    default:
      return "idle";
  }
}

/**
 * Derive a RunButton display phase from the cloud execution path.
 *
 * The cloud path is trickier than the browser path because two different
 * systems contribute to the status:
 *
 *   1. CLIENT (localPhase): The React component tracks what it has done
 *      locally — "I uploaded the files" → "uploading", "I called the API"
 *      → "running". This is set immediately via useState.
 *
 *   2. SERVER (executionStatus): The Convex backend tracks the real
 *      execution status — "pending", "running", "completed", "failed".
 *      This arrives later via a real-time subscription (useExecution hook).
 *
 * WHY TWO SOURCES?
 * When the user clicks Run, we need to show progress immediately (uploading
 * files to R2). The server doesn't even know about the execution yet at
 * that point — it only gets created after the upload finishes. So the
 * client drives the early phases, and the server takes over once it has
 * real status to report.
 *
 * PRIORITY:
 *   - If the client says "uploading" → always show uploading (server
 *     doesn't know about the execution yet)
 *   - If the server has reported a status → use the server's status
 *     (it's the source of truth once the execution exists)
 *   - Otherwise → fall back to whatever the client thinks
 *
 * @param localPhase - What the client component tracks (uploading, running, etc.)
 * @param executionStatus - What the Convex backend reports (may be undefined
 *   if we haven't created the execution yet or haven't received data)
 * @returns The RunPhase the button should display
 */
export function toCloudPhase(localPhase: RunPhase, executionStatus: string | undefined): RunPhase {
  // Client is still uploading files to R2 — server doesn't know yet
  if (localPhase === "uploading") return "uploading";

  // No server status yet, or client hasn't started — use client's phase
  if (!executionStatus || localPhase === "idle") return localPhase;

  // Server has spoken — translate its status to our display phase
  switch (executionStatus) {
    case "pending":
    case "running":
      return "running";
    case "completed":
      return "completed";
    case "failed":
      return "failed";
    // Unknown server status — fall back to client's phase
    default:
      return localPhase;
  }
}

/**
 * Map the unified RunPhase + file count to the 3-step PhaseIndicator.
 *
 * Phase 1 = no files yet (upload prompt)
 * Phase 2 = files selected (configure)
 * Phase 3 = execution in progress or complete
 */
export function deriveActivePhase(resolvedPhase: RunPhase, fileCount: number): 1 | 2 | 3 {
  switch (resolvedPhase) {
    case "uploading":
    case "running":
    case "completed":
    case "failed":
      return 3;
    default:
      return fileCount > 0 ? 2 : 1;
  }
}
