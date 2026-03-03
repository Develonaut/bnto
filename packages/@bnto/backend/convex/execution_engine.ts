import { v } from "convex/values";
import { internalAction } from "./_generated/server";
import { internal } from "./_generated/api";

const POLL_INTERVAL_MS = 2000;
const MAX_POLL_ATTEMPTS = 450; // 15 min at 2s intervals

/**
 * Internal action: POST recipe definition to Go API, then poll for completion.
 * Writes progress updates to Convex via internal mutations.
 *
 * When sessionId is provided, the Go API uses it to locate input files
 * in R2 at `uploads/{sessionId}/` and writes output files back to R2
 * at `executions/{executionId}/output/`.
 */
export const executeWorkflow = internalAction({
  args: {
    executionId: v.id("executions"),
    definition: v.any(),
    eventId: v.optional(v.id("executionEvents")),
    sessionId: v.optional(v.string()),
  },
  handler: async (ctx, args) => {
    const startTime = Date.now();
    const goApiUrl = process.env.GO_API_URL;
    if (!goApiUrl) {
      await failExecution(ctx, args, "GO_API_URL not configured", startTime);
      return;
    }

    const goExecutionId = await callGoApi(
      ctx,
      args,
      goApiUrl,
      startTime,
    );
    if (!goExecutionId) return; // already failed inside callGoApi

    await ctx.runMutation(internal.executions.updateProgress, {
      executionId: args.executionId,
      status: "running",
      progress: [],
      goExecutionId,
    });

    await pollExecution(ctx, args, goApiUrl, goExecutionId, startTime);
  },
});

/** POST the recipe definition to the Go API. Returns the Go execution ID or null on failure. */
async function callGoApi(
  ctx: { runMutation: (...a: any[]) => Promise<any> }, // eslint-disable-line @typescript-eslint/no-explicit-any
  args: { executionId: any; definition: any; eventId?: any; sessionId?: string }, // eslint-disable-line @typescript-eslint/no-explicit-any
  goApiUrl: string,
  startTime: number,
): Promise<string | null> {
  try {
    const body = buildRunRequestBody(args.definition, args.sessionId);
    const response = await fetch(`${goApiUrl}/api/run`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    if (!response.ok) {
      const text = await response.text();
      throw new Error(`Go API returned ${response.status}: ${text}`);
    }
    const data = await response.json();
    return data.id;
  } catch (e) {
    await failExecution(
      ctx,
      args,
      `Failed to start execution: ${e instanceof Error ? e.message : String(e)}`,
      startTime,
    );
    return null;
  }
}

/** Poll the Go API for execution status until complete, failed, or timeout. */
async function pollExecution(
  ctx: { runMutation: (...a: any[]) => Promise<any> }, // eslint-disable-line @typescript-eslint/no-explicit-any
  args: { executionId: any; eventId?: any; sessionId?: string }, // eslint-disable-line @typescript-eslint/no-explicit-any
  goApiUrl: string,
  goExecutionId: string,
  startTime: number,
) {
  for (let attempt = 0; attempt < MAX_POLL_ATTEMPTS; attempt++) {
    await sleep(POLL_INTERVAL_MS);

    try {
      const response = await fetch(
        `${goApiUrl}/api/executions/${goExecutionId}`,
      );
      if (!response.ok) {
        throw new Error(`Poll returned ${response.status}`);
      }
      const execution = await response.json();

      if (execution.status === "completed") {
        await ctx.runMutation(internal.executions.complete, {
          executionId: args.executionId,
          result: execution.result ?? null,
          outputFiles: execution.outputFiles,
          eventId: args.eventId,
          startTime,
        });
        return;
      }

      if (execution.status === "failed") {
        await failExecution(
          ctx,
          args,
          execution.error ?? "Execution failed",
          startTime,
        );
        return;
      }

      await ctx.runMutation(internal.executions.updateProgress, {
        executionId: args.executionId,
        status: "running",
        progress: execution.progress ?? [],
        goExecutionId,
      });
    } catch (e) {
      await failExecution(
        ctx,
        args,
        `Poll error: ${e instanceof Error ? e.message : String(e)}`,
        startTime,
      );
      return;
    }
  }

  await failExecution(
    ctx,
    args,
    "Execution timed out (polling limit reached)",
    startTime,
  );
}

/** Record an execution failure via the internal mutation. */
async function failExecution(
  ctx: { runMutation: (...a: any[]) => Promise<any> }, // eslint-disable-line @typescript-eslint/no-explicit-any
  args: { executionId: any; eventId?: any }, // eslint-disable-line @typescript-eslint/no-explicit-any
  error: string,
  startTime: number,
) {
  await ctx.runMutation(internal.executions.fail, {
    executionId: args.executionId,
    error,
    eventId: args.eventId,
    startTime,
  });
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Build the JSON body for POST /api/run.
 * When a sessionId is present, the Go API pulls input files from R2.
 */
function buildRunRequestBody(
  definition: unknown,
  sessionId: string | undefined,
) {
  if (sessionId) {
    return { definition, sessionId };
  }
  return { definition };
}
