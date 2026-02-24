import { ConvexError } from "convex/values";

const DEFAULT_ANONYMOUS_LIMIT = 3;

/**
 * Throws ConvexError if the user has exceeded their run quota.
 *
 * Anonymous users have a separate, lower limit (ANONYMOUS_RUN_LIMIT env var,
 * defaults to 3). Real users are governed by their plan's runLimit.
 */
export function enforceQuota(user: {
  isAnonymous?: boolean;
  runsUsed: number;
  runLimit: number;
}) {
  if (user.isAnonymous === true) {
    const anonLimitStr = process.env.ANONYMOUS_RUN_LIMIT;
    const anonLimit = anonLimitStr ? parseInt(anonLimitStr, 10) : DEFAULT_ANONYMOUS_LIMIT;
    if (user.runsUsed >= anonLimit) {
      throw new ConvexError({
        code: "ANONYMOUS_QUOTA_EXCEEDED",
        message: "Sign up for a free account to keep running workflows",
      });
    }
  }
  if (user.runsUsed >= user.runLimit) {
    throw new ConvexError({
      code: "RUN_LIMIT_REACHED",
      message: "Run limit reached",
    });
  }
}
