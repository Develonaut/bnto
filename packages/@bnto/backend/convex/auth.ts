import { convexAuth, getAuthUserId } from "@convex-dev/auth/server";
import { Anonymous } from "@convex-dev/auth/providers/Anonymous";
import { Password } from "@convex-dev/auth/providers/Password";
import type { Id } from "./_generated/dataModel";
import {
  handleExistingUser,
  handleNewUser,
} from "./_helpers/user_lifecycle";

/**
 * Password provider with profile extraction.
 *
 * The `profile()` callback receives ALL params from the client's `signIn()`
 * call. We extract standard fields (email, name) and also forward
 * `anonymousUserId` if present — this is how the client tells the backend
 * which anonymous user to upgrade during password sign-up.
 */
const PasswordProvider = Password({
  profile(params) {
    return {
      email: params.email as string,
      ...(params.name ? { name: params.name as string } : {}),
      ...(params.anonymousUserId
        ? { anonymousUserId: params.anonymousUserId as string }
        : {}),
    };
  },
});

export const { auth, signIn, signOut, store, isAuthenticated } = convexAuth({
  providers: [Anonymous, PasswordProvider],
  callbacks: {
    /**
     * Custom account linking for anonymous → password upgrade.
     *
     * Four paths:
     * 1. Library finds existing user (same provider+account) → patch in-place
     * 2. No existing user, but client passed anonymousUserId → look up that
     *    user, verify it's anonymous, and upgrade in-place (preserves _id)
     * 3. Fallback: check getAuthUserId(ctx) for active anonymous session
     * 4. No match → create fresh user
     *
     * Path 2 is the primary mechanism — the client reads the anonymous userId
     * from the session before navigating to sign-up, and passes it through
     * the signIn() params → profile() → here. This is reliable because it
     * doesn't depend on the mutation context inheriting auth state.
     *
     * Path 3 is a fallback for cases where the client didn't pass the ID
     * (e.g., direct navigation to /signin without an anonymous session).
     */
    async createOrUpdateUser(ctx, args) {
      const { existingUserId } = args;

      // Path 1: Library matched an existing user (same provider+accountId).
      if (existingUserId) {
        return handleExistingUser(
          ctx,
          existingUserId,
          args.profile,
          args.provider,
        );
      }

      // Path 2: Client explicitly passed the anonymous userId.
      const candidateId = args.profile.anonymousUserId as string | undefined;
      if (candidateId) {
        const candidate = await ctx.db.get(candidateId as Id<"users">);
        if (candidate?.isAnonymous) {
          return handleExistingUser(
            ctx,
            candidate._id,
            args.profile,
            args.provider,
          );
        }
      }

      // Path 3: Fallback — check if mutation context has an anonymous session.
      const currentUserId = await getAuthUserId(ctx);
      if (currentUserId) {
        const currentUser = await ctx.db.get(currentUserId);
        if (currentUser?.isAnonymous) {
          return handleExistingUser(
            ctx,
            currentUser._id,
            args.profile,
            args.provider,
          );
        }
      }

      // Path 4: Fresh signup — no anonymous session to upgrade.
      return handleNewUser(ctx, args.profile, args.provider);
    },
  },
});
