import { query } from "./_generated/server";
import { getAppUserId } from "./_helpers/auth";

/**
 * Clean usage analytics for the current user.
 *
 * Returns lifetime analytics fields only — no quota/billing fields.
 * Quota (server-node run limits) lives in `users.getServerQuota`.
 */
export const getAnalytics = query({
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
    };
  },
});
