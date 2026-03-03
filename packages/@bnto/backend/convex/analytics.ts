import { query } from "./_generated/server";
import { getAppUserId } from "./_helpers/auth";

/** Lifetime usage analytics for the current user. */
export const getAnalytics = query({
  args: {},
  handler: async (ctx) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;
    const user = await ctx.db.get(userId);
    if (user === null) return null;
    return {
      plan: user.plan,
      totalRuns: user.totalRuns,
      lastRunAt: user.lastRunAt ?? null,
    };
  },
});
