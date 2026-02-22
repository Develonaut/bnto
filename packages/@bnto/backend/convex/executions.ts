import { ConvexError, v } from "convex/values";
import {
  query,
  mutation,
  internalMutation,
  internalAction,
} from "./_generated/server";
import { internal } from "./_generated/api";
import { getAppUserId } from "./_helpers/auth";

const POLL_INTERVAL_MS = 2000;
const MAX_POLL_ATTEMPTS = 450; // 15 min at 2s intervals

/** Start a workflow execution. Checks run limits and schedules the action. */
export const start = mutation({
  args: {
    workflowId: v.id("workflows"),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) throw new ConvexError("Not authenticated");

    const user = await ctx.db.get(userId);
    if (user === null) throw new ConvexError("User not found");

    // Quota enforcement — anonymous users have a separate limit from env var
    if (user.isAnonymous) {
      const anonLimitStr = process.env.ANONYMOUS_RUN_LIMIT;
      const anonLimit = anonLimitStr ? parseInt(anonLimitStr, 10) : 3;
      const used = user.runsUsed ?? 0;
      if (used >= anonLimit) {
        throw new ConvexError({
          code: "ANONYMOUS_QUOTA_EXCEEDED",
          message: "Sign up for a free account to keep running workflows",
        });
      }
    }

    const limit = user.runLimit ?? 5;
    const used = user.runsUsed ?? 0;
    if (used >= limit) {
      throw new ConvexError({
        code: "RUN_LIMIT_REACHED",
        message: "Run limit reached",
      });
    }

    const workflow = await ctx.db.get(args.workflowId);
    if (workflow === null || workflow.userId !== userId) {
      throw new Error("Workflow not found");
    }

    await ctx.db.patch(userId, { runsUsed: used + 1 });

    const now = Date.now();
    const executionId = await ctx.db.insert("executions", {
      userId,
      workflowId: args.workflowId,
      status: "pending",
      progress: [],
      startedAt: now,
    });

    await ctx.scheduler.runAfter(
      0,
      internal.executions.executeWorkflow,
      { executionId, definition: workflow.definition },
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
 */
export const executeWorkflow = internalAction({
  args: {
    executionId: v.id("executions"),
    definition: v.any(), // eslint-disable-line @typescript-eslint/no-explicit-any
  },
  handler: async (ctx, args) => {
    const goApiUrl = process.env.GO_API_URL;
    if (!goApiUrl) {
      await ctx.runMutation(internal.executions.fail, {
        executionId: args.executionId,
        error: "GO_API_URL not configured",
      });
      return;
    }

    let goExecutionId: string;
    try {
      const response = await fetch(`${goApiUrl}/api/run`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(args.definition),
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
            });
          } else {
            await ctx.runMutation(internal.executions.fail, {
              executionId: args.executionId,
              error: execution.error ?? "Execution failed",
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
        });
        return;
      }
    }

    await ctx.runMutation(internal.executions.fail, {
      executionId: args.executionId,
      error: "Execution timed out (polling limit reached)",
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

/** Mark an execution as completed. */
export const complete = internalMutation({
  args: {
    executionId: v.id("executions"),
    result: v.any(), // eslint-disable-line @typescript-eslint/no-explicit-any
  },
  handler: async (ctx, args) => {
    await ctx.db.patch(args.executionId, {
      status: "completed" as const,
      result: args.result,
      completedAt: Date.now(),
    });
  },
});

/** Mark an execution as failed. */
export const fail = internalMutation({
  args: {
    executionId: v.id("executions"),
    error: v.string(),
  },
  handler: async (ctx, args) => {
    await ctx.db.patch(args.executionId, {
      status: "failed" as const,
      error: args.error,
      completedAt: Date.now(),
    });
  },
});

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
