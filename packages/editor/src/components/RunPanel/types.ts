import type { PipelineEvent } from "@bnto/core";

/** A single log entry — a pipeline event with a capture timestamp. */
interface RunLogEntry {
  timestamp: number;
  event: PipelineEvent;
}

export type { RunLogEntry };
