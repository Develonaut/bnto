import { internalMutation, internalAction } from "./_generated/server";
import { internal } from "./_generated/api";

/** Executions older than this are considered stale and will be failed. */
const STALE_THRESHOLD_MS = 2 * 60 * 60 * 1000; // 2 hours

/** Maximum executions to process per cron run (Convex mutation budget). */
const MAX_PER_RUN = 100;

/**
 * Mark stale executions as failed (DB-only, no R2 scheduling).
 *
 * Catches executions stuck in pending/running for >2 hours.
 * Likely caused by Go API crash, network drop, or polling timeout edge case.
 *
 * Returns the list of cleaned executions so the orchestrating action
 * can schedule R2 cleanup separately.
 */
export const markStaleAsFailed = internalMutation({
  args: {},
  handler: async (ctx) => {
    const cutoff = Date.now() - STALE_THRESHOLD_MS;
    const now = Date.now();

    // Query pending and running executions separately using the compound
    // index, then merge. Two indexed queries are far cheaper than a full
    // table scan with .filter().
    const pendingStale = await ctx.db
      .query("executions")
      .withIndex("by_status_startedAt", (q) =>
        q.eq("status", "pending").lt("startedAt", cutoff),
      )
      .take(MAX_PER_RUN);

    const runningStale = await ctx.db
      .query("executions")
      .withIndex("by_status_startedAt", (q) =>
        q.eq("status", "running").lt("startedAt", cutoff),
      )
      .take(MAX_PER_RUN);

    const stale = [...pendingStale, ...runningStale].slice(0, MAX_PER_RUN);

    const cleaned: Array<{ id: string; sessionId?: string }> = [];

    for (const execution of stale) {
      await ctx.db.patch(execution._id, {
        status: "failed" as const,
        error: "Execution timed out (stale cleanup)",
        completedAt: now,
      });
      cleaned.push({
        id: execution._id,
        sessionId: execution.sessionId ?? undefined,
      });
    }

    if (cleaned.length > 0) {
      console.log(
        `markStaleAsFailed: marked ${cleaned.length} stale executions as failed`,
      );
    }

    return { cleaned };
  },
});

/**
 * Orchestrates stale execution cleanup: marks DB records as failed,
 * then schedules R2 cleanup for associated transit files.
 *
 * This is the cron target. Runs hourly.
 */
export const cleanupStaleExecutions = internalAction({
  args: {},
  handler: async (ctx): Promise<{ cleaned: number }> => {
    const { cleaned } = await ctx.runMutation(
      internal.cleanup_stale.markStaleAsFailed,
    ) as { cleaned: Array<{ id: string; sessionId?: string }> };

    for (const exec of cleaned) {
      if (exec.sessionId) {
        await ctx.runAction(internal.cleanup.deleteByPrefix, {
          prefix: `uploads/${exec.sessionId}/`,
        });
      }
      await ctx.runAction(internal.cleanup.deleteByPrefix, {
        prefix: `executions/${exec.id}/output/`,
      });
    }

    return { cleaned: cleaned.length };
  },
});
