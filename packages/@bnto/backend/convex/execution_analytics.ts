import { v } from "convex/values";
import { query } from "./_generated/server";
import { getAppUserId } from "./_helpers/auth";

/**
 * Aggregate execution events by slug for the authenticated user.
 *
 * Returns per-slug stats: total runs, completed/failed counts,
 * average duration, and most recent run timestamp. Sorted by
 * total runs descending (most-used bntos first).
 *
 * Drives the "most used tools" and per-tool stats in the dashboard.
 */
export const aggregateBySlug = query({
  args: {},
  handler: async (ctx) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return [];

    // Safety limit: load at most 10,000 events for aggregation.
    // Users with higher volume will need a pre-computed aggregation strategy.
    const events = await ctx.db
      .query("executionEvents")
      .withIndex("by_userId", (q) => q.eq("userId", userId))
      .take(10_000);

    // Group events by slug and compute per-slug aggregates
    const bySlug = new Map<
      string,
      {
        totalRuns: number;
        completedRuns: number;
        failedRuns: number;
        totalDurationMs: number;
        durationCount: number;
        lastRunAt: number;
      }
    >();

    for (const event of events) {
      const entry = bySlug.get(event.slug) ?? {
        totalRuns: 0,
        completedRuns: 0,
        failedRuns: 0,
        totalDurationMs: 0,
        durationCount: 0,
        lastRunAt: 0,
      };

      entry.totalRuns += 1;
      if (event.status === "completed") entry.completedRuns += 1;
      if (event.status === "failed") entry.failedRuns += 1;
      if (event.durationMs !== undefined) {
        entry.totalDurationMs += event.durationMs;
        entry.durationCount += 1;
      }
      if (event.timestamp > entry.lastRunAt) {
        entry.lastRunAt = event.timestamp;
      }

      bySlug.set(event.slug, entry);
    }

    // Convert to array with computed averages, sorted by totalRuns desc
    return Array.from(bySlug.entries())
      .map(([slug, stats]) => ({
        slug,
        totalRuns: stats.totalRuns,
        completedRuns: stats.completedRuns,
        failedRuns: stats.failedRuns,
        avgDurationMs:
          stats.durationCount > 0
            ? Math.round(stats.totalDurationMs / stats.durationCount)
            : null,
        lastRunAt: stats.lastRunAt,
      }))
      .sort((a, b) => b.totalRuns - a.totalRuns);
  },
});

/**
 * List execution events for the authenticated user within a date range.
 *
 * Uses the `by_userId_timestamp` compound index for efficient range queries.
 * Returns events in descending order (most recent first), with a configurable
 * limit to prevent unbounded reads.
 *
 * Drives the execution history page with date filters.
 */
export const listByDateRange = query({
  args: {
    from: v.number(),
    to: v.number(),
    limit: v.optional(v.number()),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return [];

    const take = args.limit ?? 100;

    return ctx.db
      .query("executionEvents")
      .withIndex("by_userId_timestamp", (q) =>
        q.eq("userId", userId).gte("timestamp", args.from).lte("timestamp", args.to),
      )
      .order("desc")
      .take(take);
  },
});

/**
 * Summary stats for the authenticated user within a date range.
 *
 * Returns aggregate counts (total, completed, failed) and average duration
 * for a specific time window. Useful for period-over-period comparisons
 * (this week vs last week, this month vs last month).
 */
export const summaryByDateRange = query({
  args: {
    from: v.number(),
    to: v.number(),
  },
  handler: async (ctx, args) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;

    // Safety limit: cap at 10,000 events per summary query.
    const events = await ctx.db
      .query("executionEvents")
      .withIndex("by_userId_timestamp", (q) =>
        q.eq("userId", userId).gte("timestamp", args.from).lte("timestamp", args.to),
      )
      .take(10_000);

    let completedRuns = 0;
    let failedRuns = 0;
    let totalDurationMs = 0;
    let durationCount = 0;

    for (const event of events) {
      if (event.status === "completed") completedRuns += 1;
      if (event.status === "failed") failedRuns += 1;
      if (event.durationMs !== undefined) {
        totalDurationMs += event.durationMs;
        durationCount += 1;
      }
    }

    return {
      totalRuns: events.length,
      completedRuns,
      failedRuns,
      avgDurationMs:
        durationCount > 0
          ? Math.round(totalDurationMs / durationCount)
          : null,
    };
  },
});
