import { internalMutation } from "../_generated/server";

/**
 * One-shot cleanup mutation for Sprint 3A Wave 5.
 *
 * Removes remnants of the anonymous user system:
 * 1. Deletes all executionEvents with no userId (orphaned anonymous events)
 * 2. Backfills any users missing totalRuns with 0
 *
 * Run against production BEFORE deploying the strict schema:
 *   npx convex run --prod _helpers/cleanup_anonymous:execute
 *
 * TODO: Delete this file after running the migration in production.
 */
export const execute = internalMutation({
  args: {},
  handler: async (ctx) => {
    // --- Step 1: Delete orphaned executionEvents (no userId) ---
    let deletedEvents = 0;
    let hasMore = true;

    while (hasMore) {
      const batch = await ctx.db
        .query("executionEvents")
        .withIndex("by_userId", (q) => q.eq("userId", undefined))
        .take(100);

      for (const event of batch) {
        await ctx.db.delete(event._id);
        deletedEvents++;
      }

      hasMore = batch.length === 100;
    }

    // --- Step 2: Backfill users missing totalRuns ---
    let backfilledUsers = 0;
    const allUsers = await ctx.db.query("users").collect();

    for (const user of allUsers) {
      if (user.totalRuns === undefined) {
        await ctx.db.patch(user._id, { totalRuns: 0 });
        backfilledUsers++;
      }
    }

    return {
      deletedEvents,
      backfilledUsers,
    };
  },
});
