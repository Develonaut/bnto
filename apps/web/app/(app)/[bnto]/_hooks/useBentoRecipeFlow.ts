"use client";

import { useCallback } from "react";
import type { BrowserFileResult } from "@bnto/core";
import { useRecipeDefinition } from "./useRecipeDefinition";
import { useBentoExecution } from "./useBentoExecution";
import { useBentoParameters } from "./useBentoParameters";
import type { extractProcessingNodes } from "../_utils/extractProcessingNodes";

export type { BentoFlowPhase } from "../_utils/mapExecutionPhase";

export interface BentoRecipeFlow {
  definition: ReturnType<typeof useRecipeDefinition>["definition"];
  recipe: ReturnType<typeof useRecipeDefinition>["recipe"];
  acceptLabel: string;
  dropzoneAccept: ReturnType<typeof useRecipeDefinition>["dropzoneAccept"];
  isBrowserPath: boolean;
  processingNodes: ReturnType<typeof extractProcessingNodes>;
  files: File[];
  setFiles: (files: File[]) => void;
  parameters: Record<string, unknown>;
  setParameter: (name: string, value: unknown) => void;
  phase: ReturnType<typeof useBentoExecution>["phase"];
  exec: ReturnType<typeof useBentoExecution>["exec"];
  handleRun: () => void;
  handleReset: () => void;
  downloadResult: (result: BrowserFileResult) => void;
  downloadAll: () => void;
}

/** Manages the bento grid recipe flow — files, parameters, execution, results. */
export function useBentoRecipeFlow(slug: string): BentoRecipeFlow {
  const defn = useRecipeDefinition(slug);
  const params = useBentoParameters(defn.definition);
  const execution = useBentoExecution(slug);
  const handleRun = useCallback(() => {
    if (defn.definition && params.files.length > 0)
      execution.run(defn.definition, params.files, params.parameters);
  }, [execution, defn.definition, params.files, params.parameters]);

  return {
    ...defn,
    ...params,
    phase: execution.phase,
    exec: execution.exec,
    handleRun,
    handleReset: execution.reset,
    downloadResult: execution.downloadResult,
    downloadAll: execution.downloadAll,
  };
}
