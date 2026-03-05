import { v } from "convex/values";
import { query, mutation, internalMutation } from "./_generated/server";
import { getAppUserId } from "./_helpers/auth";

/**
 * Log the start of a Bnto execution.
 * Requires authentication — unauthenticated browser-only (WASM) executions
 * are not tracked server-side (they run free, unlimited, no session needed).
 *
 * NOTE: Both `start` and `startPredefined` in executions.ts also insert
 * events directly for server-side executions. This mutation is the client-
 * facing path used by browser-only (WASM) executions that have no server-
 * side execution record.
 */
export const logStart = mutation({
  args: {
    slug: v.string(),
    executionId: v.optional(v.id("executions")),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;

    return ctx.db.insert("executionEvents", {
      userId,
      slug: args.slug,
      timestamp: Date.now(),
      status: "started",
      executionId: args.executionId,
    });
  },
});

/** Mark an execution event as completed with duration. Internal only. */
export const logComplete = internalMutation({
  args: {
    eventId: v.id("executionEvents"),
    durationMs: v.number(),
  },
  handler: async (ctx, args) => {
    const event = await ctx.db.get(args.eventId);
    if (event === null) return;

    await ctx.db.patch(args.eventId, {
      status: "completed" as const,
      durationMs: args.durationMs,
    });
  },
});

/** Mark an execution event as failed with duration. Internal only. */
export const logFail = internalMutation({
  args: {
    eventId: v.id("executionEvents"),
    durationMs: v.number(),
  },
  handler: async (ctx, args) => {
    const event = await ctx.db.get(args.eventId);
    if (event === null) return;

    await ctx.db.patch(args.eventId, {
      status: "failed" as const,
      durationMs: args.durationMs,
    });
  },
});

/**
 * Mark an execution event as completed. Client-facing variant.
 * Used by browser-only executions to update the event after WASM finishes.
 */
export const completeEvent = mutation({
  args: {
    eventId: v.id("executionEvents"),
    durationMs: v.number(),
    status: v.union(v.literal("completed"), v.literal("failed")),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return;

    const event = await ctx.db.get(args.eventId);
    if (event === null) return;
    if (String(event.userId) !== String(userId)) return;

    await ctx.db.patch(args.eventId, {
      status: args.status,
      durationMs: args.durationMs,
    });
  },
});

/**
 * Batch-insert execution events from local history (migration on signup).
 * TODO: Add deduplication by slug+timestamp to prevent duplicate migration entries
 */
export const migrateFromLocal = mutation({
  args: {
    entries: v.array(
      v.object({
        slug: v.string(),
        timestamp: v.number(),
        durationMs: v.number(),
        status: v.union(v.literal("completed"), v.literal("failed")),
      }),
    ),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return { migrated: 0 };

    let migrated = 0;
    for (const entry of args.entries) {
      await ctx.db.insert("executionEvents", {
        userId,
        slug: entry.slug,
        timestamp: entry.timestamp,
        durationMs: entry.durationMs,
        status: entry.status,
      });
      migrated++;
    }
    return { migrated };
  },
});

/** List execution events for the authenticated user, most recent first. */
export const listByUser = query({
  args: {
    limit: v.optional(v.number()),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return [];

    const take = args.limit ?? 50;
    return ctx.db
      .query("executionEvents")
      .withIndex("by_userId", (q) => q.eq("userId", userId))
      .order("desc")
      .take(take);
  },
});

/** Paginated execution events for authenticated user. Used by usePaginatedQuery. */
export const listByUserPaginated = query({
  args: { paginationOpts: v.any() },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) {
      return { page: [], isDone: true, continueCursor: "" };
    }
    return ctx.db
      .query("executionEvents")
      .withIndex("by_userId", (q) => q.eq("userId", userId))
      .order("desc")
      .paginate(args.paginationOpts);
  },
});
