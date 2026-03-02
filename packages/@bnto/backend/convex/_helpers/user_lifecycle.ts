import type { MutationCtx } from "../_generated/server";

type AuthProfile = {
  email?: string;
  name?: string;
  image?: string;
};

/**
 * Create a brand new user with free-tier defaults.
 */
export async function handleNewUser(
  ctx: MutationCtx,
  profile: AuthProfile,
) {
  return ctx.db.insert("users", {
    name: profile.name,
    email: profile.email,
    image: profile.image,
    plan: "free",
    totalRuns: 0,
  });
}
