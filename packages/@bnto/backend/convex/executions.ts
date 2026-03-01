import { ConvexError, v } from "convex/values";
import { query, mutation, internalMutation } from "./_generated/server";
import type { MutationCtx } from "./_generated/server";
import { internal } from "./_generated/api";
import { getAppUserId } from "./_helpers/auth";
import { startExecution } from "./_helpers/start_execution";

/** Delay before deleting output files from R2 (2 hours). */
const R2_OUTPUT_CLEANUP_DELAY_MS = 2 * 60 * 60 * 1000;

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

    const workflow = await ctx.db.get(args.workflowId);
    if (workflow === null || workflow.userId !== userId) {
      throw new ConvexError("Workflow not found");
    }

    return startExecution(ctx, {
      slug: args.slug ?? workflow.name,
      definition: workflow.definition,
      workflowId: args.workflowId,
      sessionId: args.sessionId,
    });
  },
});

/** Start a predefined bnto execution. No stored workflow required. */
export const startPredefined = mutation({
  args: {
    slug: v.string(),
    definition: v.any(),
    sessionId: v.optional(v.string()),
  },
  handler: async (ctx, args) => {
    return startExecution(ctx, {
      slug: args.slug,
      definition: args.definition,
      sessionId: args.sessionId,
    });
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

/** Paginated execution history for the current user (most recent first). */
export const listByUser = query({
  args: { paginationOpts: v.any() },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) {
      return { page: [], isDone: true, continueCursor: "" };
    }
    return ctx.db
      .query("executions")
      .withIndex("by_user", (q) => q.eq("userId", userId))
      .order("desc")
      .paginate(args.paginationOpts);
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
    result: v.any(),
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
