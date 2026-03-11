/**
 * logEventToConsole — mirrors execution events to the browser console.
 *
 * Gated behind NEXT_PUBLIC_DEBUG_LOGS=1. Only active in dev.
 * Color-coded by event severity: blue for info, green for success, red for errors.
 */

import type { PipelineEvent } from "@bnto/core";

type LoggableEvent = PipelineEvent | { type: "ValidationFailed"; nodeId: string; field: string; error: string };

const ENABLED = process.env.NEXT_PUBLIC_DEBUG_LOGS === "1";

function logEventToConsole(event: LoggableEvent): void {
  if (!ENABLED) return;
  switch (event.type) {
    case "PipelineStarted":
      console.log(
        "%c[bnto] >> Pipeline started — %d node(s), %d file(s)",
        "color: #6b9bd2; font-weight: bold",
        event.totalNodes,
        event.totalFiles,
      );
      break;

    case "NodeStarted":
      console.log(
        "%c[bnto] -> [%s] Node %d/%d started",
        "color: #6b9bd2",
        event.nodeType,
        event.nodeIndex + 1,
        event.totalNodes,
      );
      break;

    case "FileProgress":
      console.log(
        "[bnto]    File %d/%d — %d%% %s",
        event.fileIndex + 1,
        event.totalFiles,
        event.percent,
        event.message,
      );
      break;

    case "NodeCompleted":
      console.log(
        "%c[bnto] ok Node completed — %d file(s) in %dms",
        "color: #6b9",
        event.filesProcessed,
        event.durationMs,
      );
      break;

    case "NodeFailed":
      console.error("[bnto] !! Node failed — %s", event.error);
      break;

    case "PipelineCompleted":
      console.log(
        "%c[bnto] << Pipeline completed — %d file(s) in %dms",
        "color: #6b9; font-weight: bold",
        event.totalFilesProcessed,
        event.durationMs,
      );
      break;

    case "PipelineFailed":
      console.error("[bnto] !! Pipeline failed — %s", event.error);
      break;

    case "ValidationFailed":
      console.warn("[bnto] !! Validation — %s", event.error);
      break;
  }
}

export { logEventToConsole };
