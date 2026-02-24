import { getAuthUserId } from "@convex-dev/auth/server";
import type { Id } from "../_generated/dataModel";
import type { QueryCtx, MutationCtx } from "../_generated/server";

/**
 * Resolve the current authenticated user's ID.
 *
 * With @convex-dev/auth, the auth user ID IS the users table _id directly.
 * No two-hop lookup needed — getAuthUserId returns the users._id.
 */
export async function getAppUserId(
  ctx: QueryCtx | MutationCtx,
): Promise<Id<"users"> | null> {
  return await getAuthUserId(ctx);
}
