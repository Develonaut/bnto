import { query, internalMutation } from "./_generated/server";
import { getAuthUserId } from "@convex-dev/auth/server";

const FREE_RUN_LIMIT = 5;

/** Get the current authenticated user with Bento fields. */
export const getMe = query({
  args: {},
  handler: async (ctx) => {
    const userId = await getAuthUserId(ctx);
    if (userId === null) return null;
    return ctx.db.get(userId);
  },
});

/** Get remaining runs for the current user. */
export const getRunsRemaining = query({
  args: {},
  handler: async (ctx) => {
    const userId = await getAuthUserId(ctx);
    if (userId === null) return null;
    const user = await ctx.db.get(userId);
    if (user === null) return null;
    const limit = user.runLimit ?? FREE_RUN_LIMIT;
    const used = user.runsUsed ?? 0;
    return Math.max(0, limit - used);
  },
});

/**
 * Initialize Bento-specific fields on a user after first sign-in.
 * Idempotent — skips if fields are already set.
 */
export const ensureUser = internalMutation({
  args: {},
  handler: async (ctx) => {
    const userId = await getAuthUserId(ctx);
    if (userId === null) return;
    const user = await ctx.db.get(userId);
    if (user === null) return;
    if (user.plan !== undefined) return;

    const now = Date.now();
    const nextMonth = new Date(now);
    nextMonth.setMonth(nextMonth.getMonth() + 1);
    nextMonth.setDate(1);
    nextMonth.setHours(0, 0, 0, 0);

    await ctx.db.patch(userId, {
      plan: "free",
      runsUsed: 0,
      runLimit: FREE_RUN_LIMIT,
      runsResetAt: nextMonth.getTime(),
    });
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
      if (
        user.runsResetAt !== undefined &&
        user.runsResetAt <= now
      ) {
        await ctx.db.patch(user._id, {
          runsUsed: 0,
          runsResetAt: nextReset,
        });
      }
    }
  },
});
