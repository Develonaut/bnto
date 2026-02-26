import type { MutationCtx } from "../_generated/server";
import type { Id } from "../_generated/dataModel";
import {
  getAnonymousRunLimit,
  getFreePlanRunLimit,
} from "./run_limits";

type AuthProfile = {
  email?: string;
  name?: string;
  image?: string;
  _anonymousUserId?: string;
};

type AuthProvider = {
  id?: string;
};

/**
 * Handle anonymous -> real account upgrade.
 * Patches the existing user in-place so the same _id is preserved.
 */
export async function handleExistingUser(
  ctx: MutationCtx,
  existingUserId: Id<"users">,
  profile: AuthProfile,
  provider?: AuthProvider,
) {
  const updates: Partial<{
    email: string;
    name: string;
    image: string;
    isAnonymous: boolean;
    runLimit: number;
  }> = {};

  if (profile.email) updates.email = profile.email;
  if (profile.name) updates.name = profile.name;
  if (profile.image !== undefined) updates.image = profile.image;

  // If signing in with a real provider, mark as non-anonymous
  // and upgrade runLimit to the full free-tier allowance.
  if (provider?.id !== "anonymous") {
    updates.isAnonymous = false;
    updates.runLimit = getFreePlanRunLimit();
  }

  if (Object.keys(updates).length > 0) {
    await ctx.db.patch(existingUserId, updates);
  }

  return existingUserId;
}

/**
 * Create a brand new user with plan-appropriate run limits.
 * Anonymous users get a lower limit; real users get the full free tier.
 */
export async function handleNewUser(
  ctx: MutationCtx,
  profile: AuthProfile,
  provider?: AuthProvider,
) {
  const isAnonymous = provider?.id === "anonymous";
  const runLimit = isAnonymous
    ? getAnonymousRunLimit()
    : getFreePlanRunLimit();

  const now = Date.now();
  const nextMonth = new Date(now);
  nextMonth.setMonth(nextMonth.getMonth() + 1);
  nextMonth.setDate(1);
  nextMonth.setHours(0, 0, 0, 0);

  return ctx.db.insert("users", {
    name: profile.name,
    email: profile.email,
    image: profile.image,
    isAnonymous,
    plan: "free",
    runsUsed: 0,
    runLimit,
    runsResetAt: nextMonth.getTime(),
    totalRuns: 0,
  });
}

/**
 * Detect anonymous -> password upgrade via profile._anonymousUserId.
 * Returns the existing anonymous user's ID if valid, or null.
 */
export async function resolveAnonymousUpgrade(
  ctx: MutationCtx,
  existingUserId: Id<"users"> | string | null | undefined,
  profile: AuthProfile,
): Promise<Id<"users"> | null> {
  if (existingUserId) return null; // already has a known user

  const candidateId = profile._anonymousUserId;
  if (!candidateId) return null;

  const candidate = await ctx.db.get(candidateId as Id<"users">);
  if (candidate?.isAnonymous) {
    return candidate._id;
  }

  return null;
}
