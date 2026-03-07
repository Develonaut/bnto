import { core, flattenDefinition } from "@bnto/core";
import type { ExecutionInstance } from "@bnto/core";
import type { Definition } from "@bnto/nodes";
import { getRecipeBySlug } from "@bnto/nodes";

interface RunRecipeParams {
  slug: string;
  files: File[];
  config: Record<string, unknown>;
  isBrowserPath: boolean;
  browserInstance: ExecutionInstance;
  definition: Definition | undefined;
  upload: (files: File[]) => Promise<{ sessionId: string }>;
  startCloudExec: (args: {
    slug: string;
    definition: Definition;
    sessionId: string;
  }) => Promise<unknown>;
  onStartUpload: () => void;
  onStartExecution: (id: string) => void;
  onFail: (message: string) => void;
}

/**
 * Execute a recipe via browser (WASM) or cloud (R2 + Go API).
 *
 * Pure action function -- no React, no store access.
 * Tracks telemetry events at start, completion, and failure.
 */
export async function runRecipeAction(params: RunRecipeParams) {
  const { slug, files, isBrowserPath } = params;
  if (files.length === 0) return;

  const totalBytes = files.reduce((sum, f) => sum + f.size, 0);
  const runProps = {
    slug,
    fileCount: files.length,
    totalBytes,
    executionPath: isBrowserPath ? "browser" : "cloud",
  };

  core.telemetry.capture("recipe_run_started", runProps);
  const startTime = Date.now();

  if (isBrowserPath) {
    await runBrowserPath(params, runProps, startTime);
    return;
  }

  await runCloudPath(params, runProps, startTime);
}

async function runBrowserPath(
  { browserInstance, slug, files, config }: RunRecipeParams,
  runProps: Record<string, unknown>,
  startTime: number,
) {
  // Look up the recipe definition and build a pipeline from it
  const recipe = getRecipeBySlug(slug);
  if (!recipe) {
    throw new Error(`No recipe definition for slug "${slug}"`);
  }

  // Merge user config into the processing node's parameters
  const definition = mergeConfigIntoDefinition(recipe.definition, config);
  const pipeline = flattenDefinition(definition);

  const result = await browserInstance.run(pipeline, files);
  const durationMs = Date.now() - startTime;

  if (result.status === "completed" && result.results.length > 0) {
    const outputBytes = result.results.reduce((sum, r) => sum + r.blob.size, 0);
    core.telemetry.capture("recipe_run_completed", {
      ...runProps,
      durationMs,
      outputFileCount: result.results.length,
      outputBytes,
    });
    await core.executions.downloadAllResults(result.results, slug);
  } else if (result.status === "failed") {
    core.telemetry.capture("recipe_run_failed", {
      ...runProps,
      durationMs,
      error: result.error ?? "unknown",
    });
  }
}

/**
 * Merge user config (quality, format, width, etc.) into the processing
 * node's parameters. Processing nodes are those that aren't input/output.
 */
function mergeConfigIntoDefinition(
  definition: Definition,
  config: Record<string, unknown>,
): Definition {
  if (!definition.nodes || Object.keys(config).length === 0) return definition;

  return {
    ...definition,
    nodes: definition.nodes.map((node) => {
      if (node.type === "input" || node.type === "output") return node;
      return {
        ...node,
        parameters: { ...node.parameters, ...config },
      };
    }),
  };
}

async function runCloudPath(
  {
    definition,
    slug,
    files,
    upload,
    startCloudExec,
    onStartUpload,
    onStartExecution,
    onFail,
  }: RunRecipeParams,
  runProps: Record<string, unknown>,
  startTime: number,
) {
  if (!definition) return;

  try {
    onStartUpload();
    const session = await upload(files);
    const id = await startCloudExec({
      slug,
      definition,
      sessionId: session.sessionId,
    });
    onStartExecution(String(id));
  } catch (e) {
    const durationMs = Date.now() - startTime;
    core.telemetry.capture("recipe_run_failed", {
      ...runProps,
      durationMs,
      error: e instanceof Error ? e.message : "unknown",
    });
    onFail(e instanceof Error ? e.message : "Something went wrong");
  }
}
