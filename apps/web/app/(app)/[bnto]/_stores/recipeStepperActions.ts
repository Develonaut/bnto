/**
 * Imperative action factory for the recipe stepper.
 *
 * All actions read state at call time via store.getState().
 * Browser results are read lazily via the `refs` accessor so
 * this file has ZERO imports from the context layer (no cycle).
 */

import { core, definitionToPipeline, bucketFileSize, categorizeError } from "@bnto/core";
import type { BrowserFileResult, ExecutionInstance } from "@bnto/core";
import type { StoreApi } from "zustand";
import type { RecipeStepperState } from "./recipeStepperStore";
import { missingRequiredParams } from "../_utils/missingRequiredParams";

// ---------------------------------------------------------------------------
// Public interface
// ---------------------------------------------------------------------------

export interface RecipeStepperActions {
  run(): void;
  back(): void;
  setFiles(files: File[]): void;
  setNodeParam(nodeId: string, paramName: string, value: unknown): void;
  deleteFile(index: number): void;
  downloadResult(result: BrowserFileResult): void;
  downloadAll(): void;
  resetExecution(): void;
}

/** Lazy accessor for browser results — avoids stale captures. */
export interface RecipeStepperRefs {
  getBrowserResults(): BrowserFileResult[];
}

/** Captures all factory arguments for inner helpers. */
interface ActionContext {
  store: StoreApi<RecipeStepperState>;
  instance: ExecutionInstance;
  refs: RecipeStepperRefs;
  slug: string;
}

// ---------------------------------------------------------------------------
// Factory
// ---------------------------------------------------------------------------

export function createRecipeStepperActions(
  store: StoreApi<RecipeStepperState>,
  instance: ExecutionInstance,
  refs: RecipeStepperRefs,
  slug: string,
): RecipeStepperActions {
  const ctx: ActionContext = { store, instance, refs, slug };
  return {
    run: () => runAction(ctx),
    back: () => backAction(ctx),
    setFiles: (files) => setFilesAction(ctx, files),
    setNodeParam: (nodeId, paramName, value) =>
      ctx.store.getState().setNodeParam(nodeId, paramName, value),
    deleteFile: (index) => deleteFileAction(ctx, index),
    downloadResult,
    downloadAll: () => downloadAllAction(ctx),
    resetExecution: () => resetExecutionAction(ctx),
  };
}

// ---------------------------------------------------------------------------
// Action implementations (outer scope — each under 20 lines)
// ---------------------------------------------------------------------------

function downloadResult(result: BrowserFileResult) {
  core.executions.downloadResult(result);
}

function setFilesAction(ctx: ActionContext, files: File[]) {
  ctx.store.getState().setFiles(files);
  if (files.length > 0) {
    const totalBytes = files.reduce((sum, f) => sum + f.size, 0);
    core.telemetry.capture("files_added", {
      slug: ctx.slug,
      fileCount: files.length,
      totalBytes,
      sizeBucket: bucketFileSize(totalBytes),
      fileTypes: countFileTypes(files),
    });
  }
}

function deleteFileAction(ctx: ActionContext, index: number) {
  const { files } = ctx.store.getState();
  setFilesAction(
    ctx,
    files.filter((_, j) => j !== index),
  );
}

function downloadAllAction(ctx: ActionContext) {
  downloadResults(ctx, ctx.refs.getBrowserResults(), "manual");
}

/**
 * Download every result and record it. The automatic download after a run
 * and the `downloadAll` action both come here; `trigger` keeps the automatic
 * download apart from downloads a user asks for.
 */
function downloadResults(
  ctx: ActionContext,
  results: BrowserFileResult[],
  trigger: "auto" | "manual",
) {
  core.executions.downloadAllResults(results, ctx.slug);
  core.telemetry.capture("result_downloaded", {
    slug: ctx.slug,
    fileCount: results.length,
    trigger,
  });
}

function resetExecutionAction(ctx: ActionContext) {
  ctx.instance.reset();
}

function backAction(ctx: ActionContext) {
  if (ctx.store.getState().activeStep === 3) resetExecutionAction(ctx);
  else setFilesAction(ctx, []);
}

async function runAction(ctx: ActionContext) {
  const { files, config } = ctx.store.getState();
  if (files.length === 0) return;
  const recipe = core.registry.getRecipeBySlug(ctx.slug);
  if (!recipe) throw new Error(`No implementation for slug "${ctx.slug}"`);
  // Run stays disabled until every required param has a value (ISS-73).
  if (missingRequiredParams(recipe.definition, config).length > 0) return;

  const props = runProps(ctx.slug, files);
  const startTime = Date.now();
  core.telemetry.capture("recipe_run_started", props);

  const pipeline = definitionToPipeline(recipe.definition, config);
  const result = await ctx.instance.run(pipeline, files);
  const durationMs = Date.now() - startTime;

  if (result.status === "completed" && result.results.length > 0) {
    captureCompleted(props, durationMs, result.results);
    downloadResults(ctx, result.results, "auto");
  } else if (result.status === "failed") {
    captureFailed(props, durationMs, result.error ?? "unknown");
  }
}

// ---------------------------------------------------------------------------
// Telemetry helpers
// ---------------------------------------------------------------------------

/** What every run event says about the run's files. */
function runProps(slug: string, files: File[]) {
  const totalBytes = files.reduce((sum, f) => sum + f.size, 0);
  return {
    slug,
    fileCount: files.length,
    totalBytes,
    sizeBucket: bucketFileSize(totalBytes),
    fileTypes: countFileTypes(files),
  };
}

function captureCompleted(
  props: Record<string, unknown>,
  durationMs: number,
  results: { blob: Blob }[],
) {
  core.telemetry.capture("recipe_run_completed", {
    ...props,
    durationMs,
    outputFileCount: results.length,
    outputBytes: results.reduce((sum, r) => sum + r.blob.size, 0),
  });
}

function captureFailed(props: Record<string, unknown>, durationMs: number, error: string) {
  core.telemetry.capture("recipe_run_failed", {
    ...props,
    durationMs,
    error,
    errorCategory: categorizeError(error),
  });
}

/** Count occurrences of each MIME type in a file list. */
function countFileTypes(files: File[]): Record<string, number> {
  const counts: Record<string, number> = {};
  for (const file of files) {
    const type = file.type || "unknown";
    counts[type] = (counts[type] ?? 0) + 1;
  }
  return counts;
}
