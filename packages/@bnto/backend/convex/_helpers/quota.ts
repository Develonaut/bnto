import { ConvexError } from "convex/values";

/**
 * Throws ConvexError if the user has exceeded their run quota.
 *
 * Uses the `runLimit` field on the user record — the single source of truth.
 * Anonymous users get ANONYMOUS_RUN_LIMIT (env var, default 3) written at
 * creation time. Real users get FREE_PLAN_RUN_LIMIT (env var, default 25).
 * Both are set in auth.ts createOrUpdateUser.
 *
 * The error code differs so the UI can show the right CTA:
 * - Anonymous: "Sign up to keep running" (ANONYMOUS_QUOTA_EXCEEDED)
 * - Real user: "Upgrade to Pro" (RUN_LIMIT_REACHED)
 */
export function enforceQuota(user: {
  isAnonymous?: boolean;
  runsUsed: number;
  runLimit: number;
}) {
  if (user.runsUsed >= user.runLimit) {
    if (user.isAnonymous === true) {
      throw new ConvexError({
        code: "ANONYMOUS_QUOTA_EXCEEDED",
        message: "Sign up for a free account to keep running workflows",
      });
    }
    throw new ConvexError({
      code: "RUN_LIMIT_REACHED",
      message: "Run limit reached",
    });
  }
}
