import type { MutationCtx } from "../_generated/server";
import type { Id } from "../_generated/dataModel";
import { internal } from "../_generated/api";
import { getAppUserId } from "./auth";
import { ConvexError } from "convex/values";

/**
 * Shared logic for starting an execution — used by both `start` (stored
 * workflow) and `startPredefined` (inline definition) mutations.
 *
 * Steps: auth check -> user lookup -> run counter increment -> execution
 * insert -> event insert -> schedule action.
 */
export async function startExecution(
  ctx: MutationCtx,
  args: {
    slug: string;
    definition: unknown;
    workflowId?: Id<"workflows">;
    sessionId?: string;
  },
) {
  const userId = await getAppUserId(ctx);
  if (userId === null) throw new ConvexError("Not authenticated");

  const user = await ctx.db.get(userId);
  if (user === null) throw new ConvexError("User not found");

  const now = Date.now();
  await ctx.db.patch(userId, {
    totalRuns: (user.totalRuns ?? 0) + 1,
    lastRunAt: now,
  });

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
    slug: args.slug,
    timestamp: now,
    status: "started",
    executionId,
  });

  await ctx.scheduler.runAfter(
    0,
    internal.execution_engine.executeWorkflow,
    {
      executionId,
      definition: args.definition,
      eventId,
      sessionId: args.sessionId,
    },
  );

  return executionId;
}
