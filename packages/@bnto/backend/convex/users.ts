import { query, internalMutation } from "./_generated/server";
import { getAppUserId } from "./_helpers/auth";

/** Get the current authenticated user with Bnto fields. */
export const getMe = query({
  args: {},
  handler: async (ctx) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;
    return ctx.db.get(userId);
  },
});

/**
 * Server-node execution quota for the current user.
 *
 * Returns monthly server-node usage, limit, and remaining allowance.
 * Browser executions are unlimited and not tracked here.
 *
 * Replaces the old `getRunsRemaining` (which implied all runs were capped).
 */
export const getServerQuota = query({
  args: {},
  handler: async (ctx) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;
    const user = await ctx.db.get(userId);
    if (user === null) return null;
    return {
      serverRunsUsed: user.runsUsed,
      serverRunLimit: user.runLimit,
      serverRunsRemaining: Math.max(0, user.runLimit - user.runsUsed),
    };
  },
});

/**
 * @deprecated Use `analytics.getAnalytics` for usage stats and
 * `users.getServerQuota` for quota. This function mixes concerns.
 */
export const getUsageStats = query({
  args: {},
  handler: async (ctx) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;
    const user = await ctx.db.get(userId);
    if (user === null) return null;
    return {
      plan: user.plan,
      totalRuns: user.totalRuns ?? 0,
      lastRunAt: user.lastRunAt ?? null,
      runsUsedThisMonth: user.runsUsed,
      runLimit: user.runLimit,
      runsRemaining: Math.max(0, user.runLimit - user.runsUsed),
    };
  },
});

/**
 * @deprecated Use `users.getServerQuota` instead.
 * Kept for backward compatibility during migration.
 */
export const getRunsRemaining = query({
  args: {},
  handler: async (ctx) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;
    const user = await ctx.db.get(userId);
    if (user === null) return null;
    return Math.max(0, user.runLimit - user.runsUsed);
  },
});

/** Reset server-node run counters for users whose reset time has passed.
 *  Uses the `by_runsResetAt` index to only fetch users due for reset,
 *  with a batch limit to stay within Convex mutation budgets. */
export const resetRunCounters = internalMutation({
  args: {},
  handler: async (ctx) => {
    const now = Date.now();
    const nextMonth = new Date(now);
    nextMonth.setMonth(nextMonth.getMonth() + 1);
    nextMonth.setDate(1);
    nextMonth.setHours(0, 0, 0, 0);
    const nextReset = nextMonth.getTime();

    const dueUsers = await ctx.db
      .query("users")
      .withIndex("by_runsResetAt", (q) => q.lte("runsResetAt", now))
      .take(100);

    for (const user of dueUsers) {
      await ctx.db.patch(user._id, {
        runsUsed: 0,
        runsResetAt: nextReset,
      });
    }
  },
});
