import { ConvexError, v } from "convex/values";
import { query, mutation, internalMutation } from "./_generated/server";
import { getAppUserId } from "./_helpers/auth";

/**
 * Log the start of a Bnto execution.
 * Accepts either an authenticated userId (derived from session) or a
 * browser fingerprint for anonymous users. At least one must be provided.
 *
 * NOTE: Both `start` and `startPredefined` in executions.ts also insert
 * events directly for server-side executions. This mutation is the client-
 * facing path used by browser-only (WASM) executions that have no server-
 * side execution record.
 */
export const logStart = mutation({
  args: {
    slug: v.string(),
    fingerprint: v.optional(v.string()),
    executionId: v.optional(v.id("executions")),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);

    if (userId === null && !args.fingerprint) {
      throw new ConvexError(
        "Either an authenticated session or a fingerprint is required",
      );
    }

    return ctx.db.insert("executionEvents", {
      userId: userId ?? undefined,
      fingerprint: args.fingerprint,
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

/** List execution events for a browser fingerprint, most recent first.
 *  PUBLIC: Used by anonymous quota display on the client. The fingerprint
 *  is opaque (hash), not PII. Rate limiting should be added at the
 *  transport layer if enumeration becomes a concern. */
export const listByFingerprint = query({
  args: {
    fingerprint: v.string(),
    limit: v.optional(v.number()),
  },
  handler: async (ctx, args) => {
    const take = args.limit ?? 50;
    return ctx.db
      .query("executionEvents")
      .withIndex("by_fingerprint", (q) =>
        q.eq("fingerprint", args.fingerprint),
      )
      .order("desc")
      .take(take);
  },
});

/** Count runs for a fingerprint in the current month. For anonymous quota checks.
 *  PUBLIC: Used by the client to display remaining anonymous runs.
 *  See listByFingerprint note on enumeration risk. */
export const countByFingerprint = query({
  args: {
    fingerprint: v.string(),
  },
  handler: async (ctx, args) => {
    const now = new Date();
    const monthStart = new Date(
      now.getFullYear(),
      now.getMonth(),
      1,
    ).getTime();

    // Only the count is needed -- use .take() with a safety cap rather than
    // .collect() to prevent unbounded reads for heavy anonymous users.
    const events = await ctx.db
      .query("executionEvents")
      .withIndex("by_fingerprint_timestamp", (q) =>
        q.eq("fingerprint", args.fingerprint).gte("timestamp", monthStart),
      )
      .take(1000);

    return events.length;
  },
});
