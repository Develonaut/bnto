import { v } from "convex/values";
import { query, internalMutation } from "./_generated/server";
import { getAppUserId } from "./_helpers/auth";

/** List logs for an execution. Verifies ownership via the execution record. */
export const list = query({
  args: { executionId: v.id("executions") },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return [];

    const execution = await ctx.db.get(args.executionId);
    if (execution === null || execution.userId !== userId) return [];

    return ctx.db
      .query("executionLogs")
      .withIndex("by_execution", (q) =>
        q.eq("executionId", args.executionId),
      )
      .collect();
  },
});

/** Insert a log entry for an execution. */
export const insert = internalMutation({
  args: {
    executionId: v.id("executions"),
    nodeId: v.string(),
    level: v.union(
      v.literal("info"),
      v.literal("warn"),
      v.literal("error"),
      v.literal("debug"),
    ),
    message: v.string(),
    timestamp: v.number(),
  },
  handler: async (ctx, args) => {
    await ctx.db.insert("executionLogs", args);
  },
});
