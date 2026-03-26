/**
 * logEventToConsole — mirrors execution events to the browser console.
 *
 * Gated behind NEXT_PUBLIC_DEBUG_LOGS=1. Only active in dev.
 * Color-coded by event severity: blue for info, green for success, red for errors.
 */

import type { PipelineEvent } from "@bnto/core";

type LoggableEvent =
  | PipelineEvent
  | { type: "ValidationFailed"; nodeId: string; field: string; error: string };

const ENABLED = process.env.NEXT_PUBLIC_DEBUG_LOGS === "1";

/** Info events: blue styled console.log */
function logInfo(msg: string, ...args: unknown[]) {
  console.log(`%c${msg}`, "color: #6b9bd2; font-weight: bold", ...args);
}

/** Success events: green styled console.log */
function logSuccess(msg: string, ...args: unknown[]) {
  console.log(`%c${msg}`, "color: #6b9; font-weight: bold", ...args);
}

/** Dispatch table mapping event types to their log handlers. */
const EVENT_HANDLERS: Record<string, (event: LoggableEvent) => void> = {
  PipelineStarted: (e) => {
    const ev = e as Extract<PipelineEvent, { type: "PipelineStarted" }>;
    logInfo("[bnto] >> Pipeline started — %d node(s), %d file(s)", ev.totalNodes, ev.totalFiles);
  },
  NodeStarted: (e) => {
    const ev = e as Extract<PipelineEvent, { type: "NodeStarted" }>;
    logInfo("[bnto] -> [%s] Node %d/%d started", ev.nodeType, ev.nodeIndex + 1, ev.totalNodes);
  },
  FileProgress: (e) => {
    const ev = e as Extract<PipelineEvent, { type: "FileProgress" }>;
    console.log(
      "[bnto]    File %d/%d — %d%% %s",
      ev.fileIndex + 1,
      ev.totalFiles,
      ev.percent,
      ev.message,
    );
  },
  NodeCompleted: (e) => {
    const ev = e as Extract<PipelineEvent, { type: "NodeCompleted" }>;
    logSuccess("[bnto] ok Node completed — %d file(s) in %dms", ev.filesProcessed, ev.durationMs);
  },
  NodeFailed: (e) => {
    console.error(
      "[bnto] !! Node failed — %s",
      (e as Extract<PipelineEvent, { type: "NodeFailed" }>).error,
    );
  },
  PipelineCompleted: (e) => {
    const ev = e as Extract<PipelineEvent, { type: "PipelineCompleted" }>;
    logSuccess(
      "[bnto] << Pipeline completed — %d file(s) in %dms",
      ev.totalFilesProcessed,
      ev.durationMs,
    );
  },
  PipelineFailed: (e) => {
    console.error(
      "[bnto] !! Pipeline failed — %s",
      (e as Extract<PipelineEvent, { type: "PipelineFailed" }>).error,
    );
  },
  ValidationFailed: (e) => {
    console.warn("[bnto] !! Validation — %s", (e as { error: string }).error);
  },
};

function logEventToConsole(event: LoggableEvent): void {
  if (!ENABLED) return;
  EVENT_HANDLERS[event.type]?.(event);
}

export { logEventToConsole };
