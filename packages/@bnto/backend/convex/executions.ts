import { ConvexError, v } from "convex/values";
import {
  query,
  mutation,
  internalMutation,
  internalAction,
} from "./_generated/server";
import type { MutationCtx } from "./_generated/server";
import { internal } from "./_generated/api";
import { getAppUserId } from "./_helpers/auth";

const POLL_INTERVAL_MS = 2000;
const MAX_POLL_ATTEMPTS = 450; // 15 min at 2s intervals

/** Delay before deleting output files from R2 (2 hours). */
const R2_OUTPUT_CLEANUP_DELAY_MS = 2 * 60 * 60 * 1000;

/** Throws ConvexError if the user has exceeded their run quota. */
function enforceQuota(user: {
  isAnonymous: boolean;
  runsUsed: number;
  runLimit: number;
}) {
  if (user.isAnonymous) {
    const anonLimitStr = process.env.ANONYMOUS_RUN_LIMIT;
    const anonLimit = anonLimitStr ? parseInt(anonLimitStr, 10) : 3;
    if (user.runsUsed >= anonLimit) {
      throw new ConvexError({
        code: "ANONYMOUS_QUOTA_EXCEEDED",
        message: "Sign up for a free account to keep running workflows",
      });
    }
  }
  if (user.runsUsed >= user.runLimit) {
    throw new ConvexError({
      code: "RUN_LIMIT_REACHED",
      message: "Run limit reached",
    });
  }
}

/** Start a workflow execution. Checks run limits and schedules the action. */
export const start = mutation({
  args: {
    workflowId: v.id("workflows"),
    slug: v.optional(v.string()),
    sessionId: v.optional(v.string()),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) throw new ConvexError("Not authenticated");

    const user = await ctx.db.get(userId);
    if (user === null) throw new ConvexError("User not found");

    enforceQuota(user);

    const workflow = await ctx.db.get(args.workflowId);
    if (workflow === null || workflow.userId !== userId) {
      throw new Error("Workflow not found");
    }

    await ctx.db.patch(userId, { runsUsed: user.runsUsed + 1 });

    const now = Date.now();
    const executionId = await ctx.db.insert("executions", {
      userId,
      workflowId: args.workflowId,
      status: "pending",
      progress: [],
      sessionId: args.sessionId,
      startedAt: now,
    });

    const eventId = await ctx.db.insert("executionEvents", {
      userId,
      slug: args.slug ?? workflow.name,
      timestamp: now,
      status: "started",
      executionId,
    });

    await ctx.scheduler.runAfter(
      0,
      internal.executions.executeWorkflow,
      {
        executionId,
        definition: workflow.definition,
        eventId,
        sessionId: args.sessionId,
      },
    );

    return executionId;
  },
});

/** Get an execution by ID. Real-time subscription target. */
export const get = query({
  args: { id: v.id("executions") },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;
    const execution = await ctx.db.get(args.id);
    if (execution === null || execution.userId !== userId) return null;
    return execution;
  },
});

/** List executions for a workflow. */
export const listByWorkflow = query({
  args: { workflowId: v.id("workflows") },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return [];
    return ctx.db
      .query("executions")
      .withIndex("by_workflow", (q) => q.eq("workflowId", args.workflowId))
      .filter((q) => q.eq(q.field("userId"), userId))
      .order("desc")
      .take(50);
  },
});

/**
 * Internal action: POST workflow to Go API, then poll for completion.
 * Writes progress updates to Convex via internal mutations.
 *
 * When sessionId is provided, the Go API uses it to locate input files
 * in R2 at `uploads/{sessionId}/` and writes output files back to R2
 * at `executions/{executionId}/output/`.
 */
export const executeWorkflow = internalAction({
  args: {
    executionId: v.id("executions"),
    definition: v.any(), // eslint-disable-line @typescript-eslint/no-explicit-any
    eventId: v.optional(v.id("executionEvents")),
    sessionId: v.optional(v.string()),
  },
  handler: async (ctx, args) => {
    const startTime = Date.now();
    const goApiUrl = process.env.GO_API_URL;
    if (!goApiUrl) {
      await ctx.runMutation(internal.executions.fail, {
        executionId: args.executionId,
        error: "GO_API_URL not configured",
        eventId: args.eventId,
        startTime,
      });
      return;
    }

    let goExecutionId: string;
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
      goExecutionId = data.id;
    } catch (e) {
      await ctx.runMutation(internal.executions.fail, {
        executionId: args.executionId,
        error: `Failed to start execution: ${e instanceof Error ? e.message : String(e)}`,
        eventId: args.eventId,
        startTime,
      });
      return;
    }

    await ctx.runMutation(internal.executions.updateProgress, {
      executionId: args.executionId,
      status: "running",
      progress: [],
      goExecutionId,
    });

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

        if (
          execution.status === "completed" ||
          execution.status === "failed"
        ) {
          if (execution.status === "completed") {
            await ctx.runMutation(internal.executions.complete, {
              executionId: args.executionId,
              result: execution.result ?? null,
              outputFiles: execution.outputFiles,
              eventId: args.eventId,
              startTime,
            });
          } else {
            await ctx.runMutation(internal.executions.fail, {
              executionId: args.executionId,
              error: execution.error ?? "Execution failed",
              eventId: args.eventId,
              startTime,
            });
          }
          return;
        }

        await ctx.runMutation(internal.executions.updateProgress, {
          executionId: args.executionId,
          status: "running",
          progress: execution.progress ?? [],
          goExecutionId,
        });
      } catch (e) {
        await ctx.runMutation(internal.executions.fail, {
          executionId: args.executionId,
          error: `Poll error: ${e instanceof Error ? e.message : String(e)}`,
          eventId: args.eventId,
          startTime,
        });
        return;
      }
    }

    await ctx.runMutation(internal.executions.fail, {
      executionId: args.executionId,
      error: "Execution timed out (polling limit reached)",
      eventId: args.eventId,
      startTime,
    });
  },
});

/** Update execution progress from the polling action. */
export const updateProgress = internalMutation({
  args: {
    executionId: v.id("executions"),
    status: v.union(v.literal("pending"), v.literal("running")),
    progress: v.array(
      v.object({ nodeId: v.string(), status: v.string() }),
    ),
    goExecutionId: v.string(),
  },
  handler: async (ctx, args) => {
    await ctx.db.patch(args.executionId, {
      status: args.status,
      progress: args.progress,
      goExecutionId: args.goExecutionId,
    });
  },
});

/** Mark an execution as completed. Also updates the linked execution event.
 *  Schedules R2 cleanup: input session (immediate), output files (2 hours). */
export const complete = internalMutation({
  args: {
    executionId: v.id("executions"),
    result: v.any(), // eslint-disable-line @typescript-eslint/no-explicit-any
    outputFiles: v.optional(
      v.array(
        v.object({
          key: v.string(),
          name: v.string(),
          sizeBytes: v.number(),
          contentType: v.string(),
        }),
      ),
    ),
    eventId: v.optional(v.id("executionEvents")),
    startTime: v.optional(v.number()),
  },
  handler: async (ctx, args) => {
    const now = Date.now();
    const execution = await ctx.db.get(args.executionId);

    await ctx.db.patch(args.executionId, {
      status: "completed" as const,
      result: args.result,
      outputFiles: args.outputFiles,
      completedAt: now,
    });

    if (args.eventId) {
      const durationMs = args.startTime ? now - args.startTime : 0;
      await ctx.db.patch(args.eventId, {
        status: "completed" as const,
        durationMs,
      });
    }

    await scheduleR2Cleanup(ctx, args.executionId, execution?.sessionId, {
      hasOutputFiles: (args.outputFiles?.length ?? 0) > 0,
    });
  },
});

/** Mark an execution as failed. Also updates the linked execution event.
 *  Schedules R2 cleanup for any transit files. */
export const fail = internalMutation({
  args: {
    executionId: v.id("executions"),
    error: v.string(),
    eventId: v.optional(v.id("executionEvents")),
    startTime: v.optional(v.number()),
  },
  handler: async (ctx, args) => {
    const now = Date.now();
    const execution = await ctx.db.get(args.executionId);

    await ctx.db.patch(args.executionId, {
      status: "failed" as const,
      error: args.error,
      completedAt: now,
    });

    if (args.eventId) {
      const durationMs = args.startTime ? now - args.startTime : 0;
      await ctx.db.patch(args.eventId, {
        status: "failed" as const,
        durationMs,
      });
    }

    await scheduleR2Cleanup(ctx, args.executionId, execution?.sessionId);
  },
});

/**
 * Schedule R2 cleanup for transit files.
 * Input session cleanup is a safety net — the Go API deletes input files
 * immediately after download. Output cleanup runs after 2 hours to give
 * users time to download.
 */
async function scheduleR2Cleanup(
  ctx: MutationCtx,
  executionId: string,
  sessionId: string | undefined,
  options?: { hasOutputFiles?: boolean },
) {
  if (sessionId) {
    await ctx.scheduler.runAfter(0, internal.cleanup.deleteByPrefix, {
      prefix: `uploads/${sessionId}/`,
    });
  }
  if (options?.hasOutputFiles) {
    await ctx.scheduler.runAfter(
      R2_OUTPUT_CLEANUP_DELAY_MS,
      internal.cleanup.deleteByPrefix,
      { prefix: `executions/${executionId}/output/` },
    );
  }
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
