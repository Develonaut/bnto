import { query, mutation, internalMutation } from "./_generated/server";
import { authComponent } from "./auth";
import { getAppUserId } from "./_helpers/auth";

const FREE_RUN_LIMIT = 5;

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

/**
 * Initialize Bnto-specific fields for a new user after first sign-in.
 * Creates the app user record linked to the Better Auth component user.
 * Idempotent — returns existing app user if already created.
 */
export const ensureUser = mutation({
  args: {},
  handler: async (ctx) => {
    const authUser = await authComponent.getAuthUser(ctx);

    const existing = await ctx.db
      .query("users")
      .withIndex("by_userId", (q) => q.eq("userId", authUser._id))
      .unique();

    if (existing) return existing._id;

    const now = Date.now();
    const nextMonth = new Date(now);
    nextMonth.setMonth(nextMonth.getMonth() + 1);
    nextMonth.setDate(1);
    nextMonth.setHours(0, 0, 0, 0);

    return ctx.db.insert("users", {
      userId: authUser._id,
      email: authUser.email,
      name: authUser.name,
      image: authUser.image ?? undefined,
      isAnonymous: authUser.isAnonymous ?? false,
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
      if (user.runsResetAt <= now) {
        await ctx.db.patch(user._id, {
          runsUsed: 0,
          runsResetAt: nextReset,
        });
      }
    }
  },
});
