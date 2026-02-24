import { convexAuth } from "@convex-dev/auth/server";
import { Anonymous } from "@convex-dev/auth/providers/Anonymous";
import { Password } from "@convex-dev/auth/providers/Password";

const FREE_RUN_LIMIT = 5;

export const { auth, signIn, signOut, store, isAuthenticated } = convexAuth({
  providers: [Anonymous, Password],
  callbacks: {
    /**
     * Controls user creation and upgrade flow.
     *
     * - New anonymous user: creates user doc with default free-tier fields.
     * - New real user (email/password): creates user doc with isAnonymous: false.
     * - Anonymous → real upgrade: patches existing user, preserves the same _id
     *   so all workflows, executions, and run counts stay associated.
     */
    async createOrUpdateUser(ctx, args) {
      if (args.existingUserId) {
        // Upgrading: anonymous → real account, or updating profile.
        // Patch in-place — same _id, no data migration needed.
        // Build typed patch — only include fields that have real values.
        // eslint-disable-next-line @typescript-eslint/no-explicit-any -- Convex patch accepts Record-style objects; profile fields are untyped from @convex-dev/auth
        const updates: Record<string, any> = {};
        if (args.profile.email) updates.email = args.profile.email;
        if (args.profile.name) updates.name = args.profile.name;
        if (args.profile.image !== undefined) updates.image = args.profile.image;
        // If signing in with a real provider, mark as non-anonymous.
        if (args.provider?.id !== "anonymous") {
          updates.isAnonymous = false;
        }
        if (Object.keys(updates).length > 0) {
          await ctx.db.patch(args.existingUserId, updates);
        }
        return args.existingUserId;
      }

      // New user — initialize with free-tier defaults.
      const now = Date.now();
      const nextMonth = new Date(now);
      nextMonth.setMonth(nextMonth.getMonth() + 1);
      nextMonth.setDate(1);
      nextMonth.setHours(0, 0, 0, 0);

      return ctx.db.insert("users", {
        name: args.profile.name,
        email: args.profile.email,
        image: args.profile.image,
        isAnonymous: args.provider?.id === "anonymous",
        plan: "free",
        runsUsed: 0,
        runLimit: FREE_RUN_LIMIT,
        runsResetAt: nextMonth.getTime(),
      });
    },
  },
});
