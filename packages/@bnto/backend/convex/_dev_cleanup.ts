import { internalMutation } from "./_generated/server";

/**
 * Predefined test accounts that persist across runs.
 * These are stable identities for tests that need auth without the signup flow.
 * Must match the emails in apps/web/e2e/accounts.ts.
 */
const PREDEFINED_EMAILS = new Set([
  "e2e-basic@test.bnto.dev",
  "e2e-pro@test.bnto.dev",
]);

/**
 * Ephemeral test account cleanup — deletes throwaway @test.bnto.dev users
 * while preserving predefined test accounts.
 *
 * Called by Playwright global teardown after E2E runs. Processes one user at
 * a time to keep reads bounded well within Convex's 4096 read limit.
 *
 * Cascade order (respects FK dependencies):
 * 1. Find ephemeral users (email ends with @test.bnto.dev, not predefined)
 * 2. Per user: collect authSessions, authAccounts
 * 3. Per session: delete authRefreshTokens, authVerifiers
 * 4. Per account: delete authVerificationCodes
 * 5. Delete authSessions, authAccounts
 * 6. Delete app tables: workflows, executions → executionLogs, executionEvents
 * 7. Delete authRateLimits for ephemeral test emails
 * 8. Delete users
 */
export const cleanTestAccounts = internalMutation({
  args: {},
  handler: async (ctx) => {
    const testDomain = "@test.bnto.dev";
    let deletedUsers = 0;
    let deletedTotal = 0;

    // Scan users table for test accounts.
    // E2E runs create ~5-10 accounts. With cleanup after every suite, this is
    // a small number and .collect() is safe here.
    const allUsers = await ctx.db.query("users").collect();
    const testUsers = allUsers.filter(
      (u) =>
        u.email &&
        u.email.endsWith(testDomain) &&
        !PREDEFINED_EMAILS.has(u.email),
    );

    for (const user of testUsers) {
      // --- Auth cascade: sessions → refresh tokens + verifiers ---
      const sessions = await ctx.db
        .query("authSessions")
        .withIndex("userId", (q) => q.eq("userId", user._id))
        .collect();

      for (const session of sessions) {
        // Delete refresh tokens for this session
        const refreshTokens = await ctx.db
          .query("authRefreshTokens")
          .withIndex("sessionId", (q) => q.eq("sessionId", session._id))
          .collect();
        for (const rt of refreshTokens) {
          await ctx.db.delete(rt._id);
          deletedTotal++;
        }

        // Delete verifiers for this session (PKCE)
        // authVerifiers has sessionId as optional field with signature index,
        // but no sessionId index — scan and filter
        const verifiers = await ctx.db.query("authVerifiers").collect();
        for (const v of verifiers) {
          if (v.sessionId === session._id) {
            await ctx.db.delete(v._id);
            deletedTotal++;
          }
        }

        await ctx.db.delete(session._id);
        deletedTotal++;
      }

      // --- Auth cascade: accounts → verification codes ---
      const accounts = await ctx.db
        .query("authAccounts")
        .withIndex("userIdAndProvider", (q) => q.eq("userId", user._id))
        .collect();

      for (const account of accounts) {
        const codes = await ctx.db
          .query("authVerificationCodes")
          .withIndex("accountId", (q) => q.eq("accountId", account._id))
          .collect();
        for (const code of codes) {
          await ctx.db.delete(code._id);
          deletedTotal++;
        }

        await ctx.db.delete(account._id);
        deletedTotal++;
      }

      // --- App tables: workflows ---
      const workflows = await ctx.db
        .query("workflows")
        .withIndex("by_user", (q) => q.eq("userId", user._id))
        .collect();
      for (const wf of workflows) {
        await ctx.db.delete(wf._id);
        deletedTotal++;
      }

      // --- App tables: executions → executionLogs ---
      const executions = await ctx.db
        .query("executions")
        .withIndex("by_user", (q) => q.eq("userId", user._id))
        .collect();
      for (const exec of executions) {
        const logs = await ctx.db
          .query("executionLogs")
          .withIndex("by_execution", (q) => q.eq("executionId", exec._id))
          .collect();
        for (const log of logs) {
          await ctx.db.delete(log._id);
          deletedTotal++;
        }
        await ctx.db.delete(exec._id);
        deletedTotal++;
      }

      // --- App tables: executionEvents ---
      const events = await ctx.db
        .query("executionEvents")
        .withIndex("by_userId", (q) => q.eq("userId", user._id))
        .collect();
      for (const event of events) {
        await ctx.db.delete(event._id);
        deletedTotal++;
      }

      // --- Delete the user ---
      await ctx.db.delete(user._id);
      deletedUsers++;
      deletedTotal++;
    }

    // --- Rate limits: clean up entries for ephemeral test email identifiers ---
    const rateLimits = await ctx.db.query("authRateLimits").collect();
    for (const rl of rateLimits) {
      if (
        rl.identifier.endsWith(testDomain) &&
        !PREDEFINED_EMAILS.has(rl.identifier)
      ) {
        await ctx.db.delete(rl._id);
        deletedTotal++;
      }
    }

    if (deletedUsers > 0) {
      console.log(
        `_dev_cleanup: deleted ${deletedUsers} test users, ${deletedTotal} total records`,
      );
    }

    return { deletedUsers, deletedTotal };
  },
});
