import { convexAuth } from "@convex-dev/auth/server";
import { Anonymous } from "@convex-dev/auth/providers/Anonymous";
import { Password } from "@convex-dev/auth/providers/Password";
import { Value } from "convex/values";
import {
  getAnonymousRunLimit,
  getFreePlanRunLimit,
} from "./_helpers/run_limits";
import { Id } from "./_generated/dataModel";

/**
 * Custom Password provider that detects anonymous → password upgrades.
 *
 * PROBLEM: @convex-dev/auth v0.0.90 creates a new user when an anonymous
 * user signs up with a password. The library resolves `existingUserId`
 * from the authAccounts table (same provider only), not from the active
 * session. Since the anonymous user has no password account, the library
 * sees no existing user and creates a new one.
 *
 * COMPLICATION: The `createOrUpdateUser` callback runs inside the `store`
 * internal mutation, which has NO auth context — `ctx.auth.getUserIdentity()`
 * returns null. So we can't detect the anonymous session there.
 *
 * FIX: The Password provider's `authorize` function runs in the `signIn`
 * ACTION, which DOES have auth context. We wrap `authorize` to:
 * 1. Extract the current userId from the JWT (action has auth)
 * 2. Inject it into `params` as `_anonymousUserId`
 * 3. The `profile` callback forwards it into the profile object
 * 4. `createOrUpdateUser` reads it from `args.profile._anonymousUserId`
 *    and upgrades the anonymous user in-place
 *
 * This preserves the same `_id` through the conversion, so all executions,
 * run counts, and events stay associated with the user.
 */
function PasswordWithAnonymousUpgrade() {
  const base = Password({
    profile(params) {
      return {
        email: params.email as string,
        ...(params.name ? { name: params.name as string } : {}),
        // Forward the anonymous userId injected by the authorize wrapper.
        // Only present during signUp when caller has an active session.
        ...(params._anonymousUserId
          ? { _anonymousUserId: params._anonymousUserId }
          : {}),
        // eslint-disable-next-line @typescript-eslint/no-explicit-any -- _anonymousUserId is a transient field not in the users schema; cast needed to pass through profile
      } as any;
    },
  });

  // The real authorize lives in the internal options object.
  // eslint-disable-next-line @typescript-eslint/no-explicit-any -- accessing internal ConvexCredentials options to wrap authorize
  const realAuthorize = (base as any).options.authorize;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any -- wrapping internal authorize with typed params
  (base as any).options.authorize = async (
    params: Record<string, Value | undefined>,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any -- action ctx type is internal to @convex-dev/auth
    ctx: any,
  ) => {
    // Security: Always clean client-injected _anonymousUserId first.
    // We only trust the value extracted from the server-side JWT.
    delete params._anonymousUserId;

    // During signUp, detect if the caller has an active anonymous session.
    // The action context HAS auth — the store mutation does not.
    if (params.flow === "signUp") {
      try {
        const identity = await ctx.auth.getUserIdentity();
        if (identity?.subject) {
          // JWT subject format: "userId|sessionId"
          const [userId] = identity.subject.split("|");
          if (userId) {
            params._anonymousUserId = userId;
          }
        }
      } catch {
        // No identity — fresh signup, not an upgrade
      }
    }

    return realAuthorize(params, ctx);
  };

  return base;
}

export const { auth, signIn, signOut, store, isAuthenticated } = convexAuth({
  providers: [Anonymous, PasswordWithAnonymousUpgrade()],
  callbacks: {
    /**
     * Controls user creation and upgrade flow.
     *
     * - New anonymous user: creates user doc with ANONYMOUS_RUN_LIMIT.
     * - New real user (email/password): creates user doc with FREE_PLAN_RUN_LIMIT.
     * - Anonymous → real upgrade: patches existing user, bumps runLimit to
     *   FREE_PLAN_RUN_LIMIT, preserves the same _id so all workflows,
     *   executions, and run counts stay associated.
     *
     * The anonymous userId is passed through `args.profile._anonymousUserId`
     * by the PasswordWithAnonymousUpgrade wrapper (see above).
     */
    async createOrUpdateUser(ctx, args) {
      let { existingUserId } = args;

      // WORKAROUND: Detect anonymous → password upgrade via profile.
      // The PasswordWithAnonymousUpgrade wrapper injects _anonymousUserId
      // from the action's JWT into the profile object. We read it here
      // and verify the user is actually anonymous before upgrading.
      if (!existingUserId && args.profile?._anonymousUserId) {
        const candidateId = args.profile._anonymousUserId as Id<"users">;
        const candidate = await ctx.db.get(candidateId);
        if (candidate?.isAnonymous) {
          existingUserId = candidate._id;
        }
      }

      if (existingUserId) {
        // Upgrading: anonymous → real account, or updating profile.
        // Patch in-place — same _id, no data migration needed.
        // eslint-disable-next-line @typescript-eslint/no-explicit-any -- Convex patch accepts Record-style objects; profile fields are untyped from @convex-dev/auth
        const updates: Record<string, any> = {};
        if (args.profile.email) updates.email = args.profile.email;
        if (args.profile.name) updates.name = args.profile.name;
        if (args.profile.image !== undefined) updates.image = args.profile.image;
        // If signing in with a real provider, mark as non-anonymous
        // and upgrade runLimit to the full free-tier allowance.
        if (args.provider?.id !== "anonymous") {
          updates.isAnonymous = false;
          updates.runLimit = getFreePlanRunLimit();
        }
        if (Object.keys(updates).length > 0) {
          await ctx.db.patch(existingUserId, updates);
        }
        return existingUserId;
      }

      // New user — initialize with plan-appropriate run limit.
      const isAnonymous = args.provider?.id === "anonymous";
      const runLimit = isAnonymous
        ? getAnonymousRunLimit()
        : getFreePlanRunLimit();

      const now = Date.now();
      const nextMonth = new Date(now);
      nextMonth.setMonth(nextMonth.getMonth() + 1);
      nextMonth.setDate(1);
      nextMonth.setHours(0, 0, 0, 0);

      return ctx.db.insert("users", {
        name: args.profile.name,
        email: args.profile.email,
        image: args.profile.image,
        isAnonymous,
        plan: "free",
        runsUsed: 0,
        runLimit,
        runsResetAt: nextMonth.getTime(),
        totalRuns: 0,
      });
    },
  },
});

