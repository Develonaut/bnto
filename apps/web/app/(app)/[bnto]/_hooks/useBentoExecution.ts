"use client";

import { useState, useCallback, useMemo } from "react";
import { core, definitionToPipeline } from "@bnto/core";
import type { Definition, BrowserFileResult } from "@bnto/core";
import { mapExecutionPhase } from "../_utils/mapExecutionPhase";

/** Manages execution lifecycle — run, reset, download. */
export function useBentoExecution(slug: string) {
  const [instance] = useState(() => core.executions.createExecution());
  const exec = core.executions.useExecutionState(instance);
  const phase = useMemo(() => mapExecutionPhase(exec.status), [exec.status]);

  const run = useCallback(
    (definition: Definition, files: File[], parameters: Record<string, unknown>) => {
      const pipeline = definitionToPipeline(definition, parameters);
      instance.run(pipeline, files);
      core.telemetry.capture("recipe_run", { slug, fileCount: files.length, source: "bento-grid" });
    },
    [instance, slug],
  );

  const reset = useCallback(() => {
    instance.reset();
  }, [instance]);

  const downloadResult = useCallback((result: BrowserFileResult) => {
    core.executions.downloadResult(result);
  }, []);

  const downloadAll = useCallback(() => {
    core.executions.downloadAllResults(exec.results, slug);
    core.telemetry.capture("result_downloaded", { slug, fileCount: exec.results.length });
  }, [exec.results, slug]);

  return { exec, phase, run, reset, downloadResult, downloadAll };
}
