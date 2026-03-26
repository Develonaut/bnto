/** Store actions: execution lifecycle (run, reset, files, download). */

import { core } from "@bnto/core";
import type { BrowserFileResult } from "@bnto/core";
import type { EditorStore } from "./types";
import { runExecution } from "../actions/runExecution";
import { EXECUTION_DEFAULTS } from "./executionDefaults";

type Setter = (partial: Partial<EditorStore> | ((s: EditorStore) => Partial<EditorStore>)) => void;
type Getter = () => EditorStore;

function createExecutionActions(set: Setter, get: Getter) {
  return {
    runExecution: async (files: File[]) => {
      await runExecution(set, get, files);
    },

    resetRun: () => {
      set(EXECUTION_DEFAULTS);
    },

    setInputFiles: (files: File[]) => {
      set({ executionInputFiles: files });
    },

    downloadResult: (file: BrowserFileResult) => {
      core.executions.downloadResult(file);
    },

    downloadAllResults: async () => {
      const results = get().executionResults;
      if (results.length === 0) return;
      await core.executions.downloadAllResults(results, "editor-results");
    },
  };
}

export { createExecutionActions };
