import { authComponent } from "../auth";
import type { Id } from "../_generated/dataModel";
import type { QueryCtx, MutationCtx } from "../_generated/server";

/**
 * Resolve the current authenticated user's app user ID.
 * Uses the Better Auth component to get the auth user, then
 * looks up the corresponding app user by the auth userId link.
 */
export async function getAppUserId(
  ctx: QueryCtx | MutationCtx,
): Promise<Id<"users"> | null> {
  const authUser = await authComponent.safeGetAuthUser(ctx);
  if (!authUser) return null;

  const appUser = await ctx.db
    .query("users")
    .withIndex("by_userId", (q) => q.eq("userId", authUser._id))
    .unique();

  return appUser?._id ?? null;
}
