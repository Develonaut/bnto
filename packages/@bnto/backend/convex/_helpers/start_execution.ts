import type { MutationCtx } from "../_generated/server";
import type { Id } from "../_generated/dataModel";
import { internal } from "../_generated/api";
import { getAppUserId } from "./auth";
import { ConvexError } from "convex/values";

/** Authenticate and increment the user's run counter. Returns the user ID. */
async function authenticateAndCount(ctx: MutationCtx, now: number) {
  const userId = await getAppUserId(ctx);
  if (userId === null) throw new ConvexError("Not authenticated");

  const user = await ctx.db.get(userId);
  if (user === null) throw new ConvexError("User not found");

  await ctx.db.patch(userId, { totalRuns: user.totalRuns + 1, lastRunAt: now });
  return userId;
}

/** Insert execution + event records and schedule the engine action. */
async function createAndSchedule(
  ctx: MutationCtx,
  userId: Id<"users">,
  args: { slug: string; definition: unknown; recipeId?: Id<"recipes">; sessionId?: string },
  now: number,
) {
  const executionId = await ctx.db.insert("executions", {
    userId,
    recipeId: args.recipeId,
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

  await ctx.scheduler.runAfter(0, internal.execution_engine.executeWorkflow, {
    executionId,
    definition: args.definition,
    eventId,
    sessionId: args.sessionId,
  });

  return executionId;
}

/**
 * Shared logic for starting an execution — used by both `start` (stored
 * recipe) and `startPredefined` (inline definition) mutations.
 */
export async function startExecution(
  ctx: MutationCtx,
  args: { slug: string; definition: unknown; recipeId?: Id<"recipes">; sessionId?: string },
) {
  const now = Date.now();
  const userId = await authenticateAndCount(ctx, now);
  return createAndSchedule(ctx, userId, args, now);
}
