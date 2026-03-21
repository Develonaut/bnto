/** Default execution-related state — used for both initial state and resetRun. */

import type { ExecutionPhase, ExecutionState, FileProgress, RunLogEntry } from "./types";
import type { BrowserFileResult } from "@bnto/core";

interface ExecutionDefaults {
  executionState: ExecutionState;
  nodeProgress: Record<string, number>;
  executionPhase: ExecutionPhase;
  executionResults: BrowserFileResult[];
  executionErrors: string[];
  executionLogs: RunLogEntry[];
  executionFileProgress: FileProgress | null;
  executionInputFiles: File[];
}

const EXECUTION_DEFAULTS: ExecutionDefaults = {
  executionState: {},
  nodeProgress: {},
  executionPhase: "idle",
  executionResults: [],
  executionErrors: [],
  executionLogs: [],
  executionFileProgress: null,
  executionInputFiles: [],
};

export { EXECUTION_DEFAULTS };
