import { createClient, type GenericCtx } from "@convex-dev/better-auth";
import { convex } from "@convex-dev/better-auth/plugins";
import { betterAuth } from "better-auth/minimal";
import { anonymous } from "better-auth/plugins";
import type { GenericActionCtx } from "convex/server";
import type { DataModel } from "./_generated/dataModel";
import { components, internal } from "./_generated/api";
import authConfig from "./auth.config";

export const authComponent = createClient<DataModel>(components.betterAuth);

export const createAuth = (ctx: GenericCtx<DataModel>) =>
  betterAuth({
    baseURL: process.env.SITE_URL!,
    database: authComponent.adapter(ctx),
    emailAndPassword: { enabled: true },
    socialProviders: {
      google: {
        clientId: process.env.AUTH_GOOGLE_ID!,
        clientSecret: process.env.AUTH_GOOGLE_SECRET!,
      },
      discord: {
        clientId: process.env.AUTH_DISCORD_ID!,
        clientSecret: process.env.AUTH_DISCORD_SECRET!,
      },
    },
    plugins: [
      convex({ authConfig }),
      anonymous({
        // Prevents race condition where anonymous user is deleted during
        // the sign-in request itself (Better Auth issue #5824)
        disableDeleteAnonymousUser: true,
        onLinkAccount: async ({ anonymousUser, newUser }) => {
          // Migrate app data from the old anonymous user to the new authenticated user.
          // ctx is captured from the createAuth closure — it's an ActionCtx when called
          // from the HTTP handler, which has runMutation().
          const actionCtx = ctx as GenericActionCtx<DataModel>;
          await actionCtx.runMutation(
            internal.migrate_anonymous.migrateAnonymousData,
            {
              anonymousAuthUserId: anonymousUser.user.id,
              newAuthUserId: newUser.user.id,
            },
          );
        },
      }),
      // TODO: Add Cloudflare Turnstile CAPTCHA before production to prevent
      // database-bloat attacks from automated anonymous sign-in requests.
    ],
  });
