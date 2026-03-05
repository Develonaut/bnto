import { internalMutation } from "./_generated/server";
import type { MutationCtx } from "./_generated/server";
import type { Id } from "./_generated/dataModel";

/**
 * Predefined test accounts that persist across runs.
 * These are stable identities for tests that need auth without the signup flow.
 * Must match the emails in apps/web/e2e/accounts.ts.
 */
const PREDEFINED_EMAILS = new Set([
  "e2e-basic@test.bnto.dev",
  "e2e-pro@test.bnto.dev",
]);

const TEST_DOMAIN = "@test.bnto.dev";

/** Delete auth sessions and their dependent records (refresh tokens, verifiers). */
async function deleteAuthSessions(ctx: MutationCtx, userId: Id<"users">) {
  const sessions = await ctx.db
    .query("authSessions")
    .withIndex("userId", (q) => q.eq("userId", userId))
    .collect();

  let deleted = 0;
  for (const session of sessions) {
    const refreshTokens = await ctx.db
      .query("authRefreshTokens")
      .withIndex("sessionId", (q) => q.eq("sessionId", session._id))
      .collect();
    for (const rt of refreshTokens) {
      await ctx.db.delete(rt._id);
      deleted++;
    }

    // authVerifiers has no sessionId index — scan and filter
    const verifiers = await ctx.db.query("authVerifiers").collect();
    for (const v of verifiers) {
      if (v.sessionId === session._id) {
        await ctx.db.delete(v._id);
        deleted++;
      }
    }

    await ctx.db.delete(session._id);
    deleted++;
  }
  return deleted;
}

/** Delete auth accounts and their verification codes. */
async function deleteAuthAccounts(ctx: MutationCtx, userId: Id<"users">) {
  const accounts = await ctx.db
    .query("authAccounts")
    .withIndex("userIdAndProvider", (q) => q.eq("userId", userId))
    .collect();

  let deleted = 0;
  for (const account of accounts) {
    const codes = await ctx.db
      .query("authVerificationCodes")
      .withIndex("accountId", (q) => q.eq("accountId", account._id))
      .collect();
    for (const code of codes) {
      await ctx.db.delete(code._id);
      deleted++;
    }
    await ctx.db.delete(account._id);
    deleted++;
  }
  return deleted;
}

/** Delete app-level data for a user (recipes, executions, logs, events). */
async function deleteAppData(ctx: MutationCtx, userId: Id<"users">) {
  let deleted = 0;

  const recipes = await ctx.db
    .query("recipes")
    .withIndex("by_user", (q) => q.eq("userId", userId))
    .collect();
  for (const recipe of recipes) {
    await ctx.db.delete(recipe._id);
    deleted++;
  }

  const executions = await ctx.db
    .query("executions")
    .withIndex("by_user", (q) => q.eq("userId", userId))
    .collect();
  for (const exec of executions) {
    const logs = await ctx.db
      .query("executionLogs")
      .withIndex("by_execution", (q) => q.eq("executionId", exec._id))
      .collect();
    for (const log of logs) {
      await ctx.db.delete(log._id);
      deleted++;
    }
    await ctx.db.delete(exec._id);
    deleted++;
  }

  const events = await ctx.db
    .query("executionEvents")
    .withIndex("by_userId", (q) => q.eq("userId", userId))
    .collect();
  for (const event of events) {
    await ctx.db.delete(event._id);
    deleted++;
  }

  return deleted;
}

/** Delete ephemeral rate limit entries for test emails. */
async function deleteEphemeralRateLimits(ctx: MutationCtx) {
  const rateLimits = await ctx.db.query("authRateLimits").collect();
  let deleted = 0;
  for (const rl of rateLimits) {
    if (
      rl.identifier.endsWith(TEST_DOMAIN) &&
      !PREDEFINED_EMAILS.has(rl.identifier)
    ) {
      await ctx.db.delete(rl._id);
      deleted++;
    }
  }
  return deleted;
}

/**
 * Ephemeral test account cleanup — deletes throwaway @test.bnto.dev users
 * while preserving predefined test accounts.
 *
 * Called by Playwright global teardown after E2E runs.
 *
 * Cascade order (respects FK dependencies):
 * 1. Auth sessions → refresh tokens + verifiers
 * 2. Auth accounts → verification codes
 * 3. App tables: recipes, executions → logs, events
 * 4. Rate limits for ephemeral emails
 * 5. User record
 */
export const cleanTestAccounts = internalMutation({
  args: {},
  handler: async (ctx) => {
    let deletedUsers = 0;
    let deletedTotal = 0;

    const allUsers = await ctx.db.query("users").collect();
    const testUsers = allUsers.filter(
      (u) =>
        u.email &&
        u.email.endsWith(TEST_DOMAIN) &&
        !PREDEFINED_EMAILS.has(u.email),
    );

    for (const user of testUsers) {
      deletedTotal += await deleteAuthSessions(ctx, user._id);
      deletedTotal += await deleteAuthAccounts(ctx, user._id);
      deletedTotal += await deleteAppData(ctx, user._id);

      await ctx.db.delete(user._id);
      deletedUsers++;
      deletedTotal++;
    }

    deletedTotal += await deleteEphemeralRateLimits(ctx);

    if (deletedUsers > 0) {
      console.log(
        `_dev_cleanup: deleted ${deletedUsers} test users, ${deletedTotal} total records`,
      );
    }

    return { deletedUsers, deletedTotal };
  },
});
