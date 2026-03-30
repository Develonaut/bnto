/**
 * Imperative action factory for the recipe flow.
 *
 * All actions read state at call time via store.getState().
 * Cloud function refs are read lazily via the `refs` accessor so
 * this file has ZERO imports from the context layer (no cycle).
 */

import { core, definitionToPipeline } from "@bnto/core";
import type { BrowserFileResult, ExecutionInstance, Definition } from "@bnto/core";
import type { StoreApi } from "zustand";
import type { RecipeFlowState } from "./recipeFlowStore";

// ---------------------------------------------------------------------------
// Public interface
// ---------------------------------------------------------------------------

export interface RecipeFlowActions {
  run(): void;
  back(): void;
  setFiles(files: File[]): void;
  setNodeParam(nodeId: string, paramName: string, value: unknown): void;
  deleteFile(index: number): void;
  downloadResult(result: BrowserFileResult): void;
  downloadAll(): void;
  resetExecution(): void;
}

/** Lazy accessors for mutable module refs — avoids circular imports. */
export interface RecipeFlowRefs {
  getBrowserResults(): BrowserFileResult[];
  getUploadFn(): ((files: File[]) => Promise<{ sessionId: string }>) | null;
  getStartCloudExecFn():
    | ((args: { slug: string; definition: Definition; sessionId: string }) => Promise<unknown>)
    | null;
  getResetUploadFn(): (() => void) | null;
}

/** Captures all factory arguments for inner helpers. */
interface ActionContext {
  store: StoreApi<RecipeFlowState>;
  browserInstance: ExecutionInstance;
  refs: RecipeFlowRefs;
  slug: string;
  isBrowserPath: boolean;
  definition: Definition | undefined;
}

// ---------------------------------------------------------------------------
// Factory
// ---------------------------------------------------------------------------

export function createRecipeFlowActions(
  store: StoreApi<RecipeFlowState>,
  browserInstance: ExecutionInstance,
  refs: RecipeFlowRefs,
  slug: string,
  isBrowserPath: boolean,
  definition: Definition | undefined,
): RecipeFlowActions {
  const ctx: ActionContext = { store, browserInstance, refs, slug, isBrowserPath, definition };
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
    core.telemetry.capture("files_added", {
      slug: ctx.slug,
      fileCount: files.length,
      totalBytes: files.reduce((sum, f) => sum + f.size, 0),
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
  const results = ctx.refs.getBrowserResults();
  core.executions.downloadAllResults(results, ctx.slug);
  core.telemetry.capture("result_downloaded", { slug: ctx.slug, fileCount: results.length });
}

function resetExecutionAction(ctx: ActionContext) {
  if (ctx.isBrowserPath) {
    ctx.browserInstance.reset();
  } else {
    ctx.store.setState({ executionId: null, cloudPhase: "idle" as const, clientError: null });
    ctx.refs.getResetUploadFn()?.();
  }
}

function backAction(ctx: ActionContext) {
  if (ctx.store.getState().activeStep === 3) resetExecutionAction(ctx);
  else setFilesAction(ctx, []);
}

async function runAction(ctx: ActionContext) {
  const { files, config } = ctx.store.getState();
  if (files.length === 0) return;
  const runCtx = buildRunContext(ctx.slug, files, ctx.isBrowserPath);
  core.telemetry.capture("recipe_run_started", runCtx.props);
  if (ctx.isBrowserPath) {
    await runBrowserPath(ctx.browserInstance, ctx.slug, files, config, runCtx);
  } else {
    await runCloudPath(ctx.store, ctx.refs, ctx.slug, files, ctx.definition, runCtx);
  }
}

// ---------------------------------------------------------------------------
// Run context + execution paths
// ---------------------------------------------------------------------------

interface RunContext {
  props: Record<string, unknown>;
  startTime: number;
}

function buildRunContext(slug: string, files: File[], isBrowserPath: boolean): RunContext {
  return {
    props: {
      slug,
      fileCount: files.length,
      totalBytes: files.reduce((sum, f) => sum + f.size, 0),
      executionPath: isBrowserPath ? "browser" : "cloud",
    },
    startTime: Date.now(),
  };
}

async function runBrowserPath(
  instance: ExecutionInstance,
  slug: string,
  files: File[],
  config: Record<string, Record<string, unknown>>,
  ctx: RunContext,
) {
  const recipe = core.registry.getRecipeBySlug(slug);
  if (!recipe) throw new Error(`No browser implementation for slug "${slug}"`);

  const pipeline = definitionToPipeline(recipe.definition, config);
  const result = await instance.run(pipeline, files);
  const durationMs = Date.now() - ctx.startTime;

  if (result.status === "completed" && result.results.length > 0) {
    captureCompleted(ctx.props, durationMs, result.results);
    core.executions.downloadAllResults(
      result.results as Parameters<typeof core.executions.downloadAllResults>[0],
      slug,
    );
  } else if (result.status === "failed") {
    captureFailed(ctx.props, durationMs, result.error ?? "unknown");
  }
}

async function runCloudPath(
  store: StoreApi<RecipeFlowState>,
  refs: RecipeFlowRefs,
  slug: string,
  files: File[],
  definition: Definition | undefined,
  ctx: RunContext,
) {
  if (!definition) return;
  const upload = refs.getUploadFn();
  const startCloudExec = refs.getStartCloudExecFn();
  if (!upload || !startCloudExec) return;

  try {
    store.getState().startUpload();
    const session = await upload(files);
    const id = await startCloudExec({ slug, definition, sessionId: session.sessionId });
    store.getState().startExecution(String(id));
  } catch (e) {
    captureFailed(
      ctx.props,
      Date.now() - ctx.startTime,
      e instanceof Error ? e.message : "unknown",
    );
    store.getState().failCloud(e instanceof Error ? e.message : "Something went wrong");
  }
}

// ---------------------------------------------------------------------------
// Telemetry helpers
// ---------------------------------------------------------------------------

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
  core.telemetry.capture("recipe_run_failed", { ...props, durationMs, error });
}
