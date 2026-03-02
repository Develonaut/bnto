import { convexAuth } from "@convex-dev/auth/server";
import { Password } from "@convex-dev/auth/providers/Password";
import { handleNewUser } from "./_helpers/user_lifecycle";

/**
 * Password provider with profile extraction.
 *
 * The `profile()` callback receives ALL params from the client's `signIn()`
 * call. We extract standard fields (email, name).
 */
const PasswordProvider = Password({
  profile(params) {
    return {
      email: params.email as string,
      ...(params.name ? { name: params.name as string } : {}),
    };
  },
});

export const { auth, signIn, signOut, store, isAuthenticated } = convexAuth({
  providers: [PasswordProvider],
  callbacks: {
    /**
     * Create or update a user on sign-in/sign-up.
     *
     * Two paths:
     * 1. Library finds existing user (same provider+account) → patch profile
     * 2. No existing user → create fresh user
     */
    async createOrUpdateUser(ctx, args) {
      const { existingUserId } = args;

      // Path 1: Library matched an existing user (same provider+accountId).
      // Patch profile fields (name, email, image) if they changed.
      if (existingUserId) {
        const updates: Partial<{
          email: string;
          name: string;
          image: string;
        }> = {};

        if (args.profile.email) updates.email = args.profile.email as string;
        if (args.profile.name) updates.name = args.profile.name as string;
        if (args.profile.image !== undefined)
          updates.image = args.profile.image as string;

        if (Object.keys(updates).length > 0) {
          await ctx.db.patch(existingUserId, updates);
        }

        return existingUserId;
      }

      // Path 2: Fresh signup — create a new user.
      return handleNewUser(ctx, args.profile);
    },
  },
});
