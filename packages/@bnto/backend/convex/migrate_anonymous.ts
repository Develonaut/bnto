import { v } from "convex/values";
import { internalMutation } from "./_generated/server";

/**
 * Migrate data from an anonymous user to a newly authenticated user.
 *
 * Called during account upgrade (anonymous -> real account) via the
 * Better Auth onLinkAccount callback. Re-associates all user-owned
 * records from the old anonymous app user to the new authenticated app user.
 *
 * Args are Better Auth component user IDs (strings), not app user doc IDs.
 */
export const migrateAnonymousData = internalMutation({
  args: {
    anonymousAuthUserId: v.string(),
    newAuthUserId: v.string(),
  },
  handler: async (ctx, { anonymousAuthUserId, newAuthUserId }) => {
    const oldAppUser = await ctx.db
      .query("users")
      .withIndex("by_userId", (q) => q.eq("userId", anonymousAuthUserId))
      .unique();

    const newAppUser = await ctx.db
      .query("users")
      .withIndex("by_userId", (q) => q.eq("userId", newAuthUserId))
      .unique();

    if (!oldAppUser || !newAppUser) return;

    // Migrate workflows
    const workflows = await ctx.db
      .query("workflows")
      .withIndex("by_user", (q) => q.eq("userId", oldAppUser._id))
      .collect();
    for (const workflow of workflows) {
      await ctx.db.patch(workflow._id, { userId: newAppUser._id });
    }

    // Migrate executions
    const executions = await ctx.db
      .query("executions")
      .withIndex("by_user", (q) => q.eq("userId", oldAppUser._id))
      .collect();
    for (const execution of executions) {
      await ctx.db.patch(execution._id, { userId: newAppUser._id });
    }

    // Transfer run counts from anonymous to new user
    await ctx.db.patch(newAppUser._id, {
      runsUsed: (newAppUser.runsUsed ?? 0) + (oldAppUser.runsUsed ?? 0),
    });

    // Mark old app user as no longer anonymous (cleaned up by cron later)
    await ctx.db.patch(oldAppUser._id, { isAnonymous: false });
  },
});
