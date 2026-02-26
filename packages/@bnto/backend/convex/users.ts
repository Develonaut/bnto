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

/** Get remaining runs for the current user. */
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

/** Get usage analytics for the current user. */
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

/** Reset run counters for all users whose reset time has passed. */
export const resetRunCounters = internalMutation({
  args: {},
  handler: async (ctx) => {
    const now = Date.now();
    const nextMonth = new Date(now);
    nextMonth.setMonth(nextMonth.getMonth() + 1);
    nextMonth.setDate(1);
    nextMonth.setHours(0, 0, 0, 0);
    const nextReset = nextMonth.getTime();

    const users = await ctx.db.query("users").collect();
    for (const user of users) {
      if (user.runsResetAt <= now) {
        await ctx.db.patch(user._id, {
          runsUsed: 0,
          runsResetAt: nextReset,
        });
      }
    }
  },
});
