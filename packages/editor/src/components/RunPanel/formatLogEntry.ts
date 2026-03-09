import type { RunLogEntry } from "./types";

interface FormattedLogEntry {
  time: string;
  icon: string;
  message: string;
}

/**
 * Format a log entry into a human-readable line for the logs console.
 *
 * Each event type gets a distinct icon and descriptive message.
 */
function formatLogEntry(entry: RunLogEntry): FormattedLogEntry {
  const time = formatTime(entry.timestamp);
  const { event } = entry;

  switch (event.type) {
    case "PipelineStarted":
      return {
        time,
        icon: ">>",
        message: `Pipeline started — ${event.totalNodes} node(s), ${event.totalFiles} file(s)`,
      };
    case "NodeStarted":
      return {
        time,
        icon: "->",
        message: `[${event.nodeType}] Node ${event.nodeIndex + 1}/${event.totalNodes} started`,
      };
    case "FileProgress":
      return {
        time,
        icon: "  ",
        message: `File ${event.fileIndex + 1}/${event.totalFiles} — ${event.percent}% ${event.message}`,
      };
    case "NodeCompleted":
      return {
        time,
        icon: "ok",
        message: `Node completed — ${event.filesProcessed} file(s) in ${event.durationMs}ms`,
      };
    case "NodeFailed":
      return {
        time,
        icon: "!!",
        message: `Node failed — ${event.error}`,
      };
    case "PipelineCompleted":
      return {
        time,
        icon: "<<",
        message: `Pipeline completed — ${event.totalFilesProcessed} file(s) in ${event.durationMs}ms`,
      };
    case "PipelineFailed":
      return {
        time,
        icon: "!!",
        message: `Pipeline failed — ${event.error}`,
      };
    case "ValidationFailed":
      return {
        time,
        icon: "!!",
        message: `Validation — ${event.error}`,
      };
  }
}

/** Format a unix timestamp as HH:MM:SS.mmm */
function formatTime(timestamp: number): string {
  const d = new Date(timestamp);
  const h = String(d.getHours()).padStart(2, "0");
  const m = String(d.getMinutes()).padStart(2, "0");
  const s = String(d.getSeconds()).padStart(2, "0");
  const ms = String(d.getMilliseconds()).padStart(3, "0");
  return `${h}:${m}:${s}.${ms}`;
}

export { formatLogEntry };
