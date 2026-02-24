import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { api } from "./_generated/api";
import { FREE_RUN_LIMIT, nextMonthReset } from "./_test_helpers";

const modules = import.meta.glob("./**/*.ts");

// Use fake timers to prevent scheduled functions (executeWorkflow) from
// firing during tests. startPredefined schedules an internal action with
// delay 0 — without fake timers it runs asynchronously and causes
// "Write outside of transaction" errors when the test instance is disposed.
beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
});

/**
 * Create an anonymous user in the database and return an authenticated
 * test context that simulates what happens after signIn("anonymous"):
 * 1. User row created with isAnonymous: true + free-tier defaults
 * 2. Auth identity set so getAuthUserId() returns the user's _id
 *
 * @convex-dev/auth's getAuthUserId extracts userId from
 * identity.subject.split("|")[0], so we set subject to "userId|sessionId".
 */
async function seedAnonymousUser(
  t: ReturnType<typeof convexTest>,
  overrides?: { runsUsed?: number; runLimit?: number },
) {
  const userId = await t.run(async (ctx) => {
    return ctx.db.insert("users", {
      isAnonymous: true,
      plan: "free",
      runsUsed: overrides?.runsUsed ?? 0,
      runLimit: overrides?.runLimit ?? FREE_RUN_LIMIT,
      runsResetAt: nextMonthReset(),
    });
  });

  const asAnonymous = t.withIdentity({
    subject: `${userId}|test-session`,
  });

  return { userId, asAnonymous };
}

/**
 * Anonymous Edge Cases (A6-A7)
 *
 * Quota enforcement and session durability. These tests verify that
 * anonymous users hit quota errors (not auth errors) and that sessions
 * persist across simulated refreshes.
 *
 * Spec: .claude/journeys/auth.md — "Anonymous Edge Cases"
 */
describe("anonymous edge cases (A6-A7)", () => {
  describe("A6: anonymous quota enforcement", () => {
    it("rejects with quota error (not auth error) when limit is reached", async () => {
      const t = convexTest(schema, modules);
      // Default anonymous limit is 3 (enforceQuota DEFAULT_ANONYMOUS_LIMIT).
      // Seed the user at exactly 3 runs used — the next call should reject.
      const { asAnonymous } = await seedAnonymousUser(t, { runsUsed: 3 });

      // The critical distinction: quota error ("Sign up for a free account"),
      // not auth error ("Not authenticated"). If the user is authenticated
      // but over quota, the message must guide them to sign up — not suggest
      // they aren't logged in.
      await expect(
        asAnonymous.mutation(api.executions.startPredefined, {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        }),
      ).rejects.toThrow("Sign up for a free account");
    });

    it("allows runs below the anonymous limit", async () => {
      const t = convexTest(schema, modules);
      const { asAnonymous } = await seedAnonymousUser(t, { runsUsed: 2 });

      const executionId = await asAnonymous.mutation(
        api.executions.startPredefined,
        {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        },
      );

      expect(executionId).toBeDefined();
    });

    it("rejects above the anonymous limit", async () => {
      const t = convexTest(schema, modules);
      const { asAnonymous } = await seedAnonymousUser(t, { runsUsed: 5 });

      await expect(
        asAnonymous.mutation(api.executions.startPredefined, {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        }),
      ).rejects.toThrow();
    });

    it("does not increment runsUsed when quota is exceeded", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t, {
        runsUsed: 3,
      });

      await expect(
        asAnonymous.mutation(api.executions.startPredefined, {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        }),
      ).rejects.toThrow();

      // Verify runsUsed didn't change — quota check happens before increment
      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.runsUsed).toBe(3);
    });

    it("does not create execution record when quota is exceeded", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t, {
        runsUsed: 3,
      });

      await expect(
        asAnonymous.mutation(api.executions.startPredefined, {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        }),
      ).rejects.toThrow();

      // No execution record should exist for this user
      const executions = await t.run(async (ctx) => {
        return ctx.db.query("executions").collect();
      });
      const userExecutions = executions.filter(
        (e) => e.userId === userId,
      );
      expect(userExecutions).toHaveLength(0);
    });

    it("exhausts quota incrementally across multiple runs", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      // Run 1 — should succeed
      await asAnonymous.mutation(api.executions.startPredefined, {
        slug: "compress-images",
        definition: { type: "group", nodes: [] },
      });

      // Run 2 — should succeed
      await asAnonymous.mutation(api.executions.startPredefined, {
        slug: "resize-images",
        definition: { type: "group", nodes: [] },
      });

      // Run 3 — should succeed (at limit - 1)
      await asAnonymous.mutation(api.executions.startPredefined, {
        slug: "clean-csv",
        definition: { type: "group", nodes: [] },
      });

      // Verify runsUsed is now at the anonymous limit
      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.runsUsed).toBe(3);

      // Run 4 — should fail with quota error, not auth error
      await expect(
        asAnonymous.mutation(api.executions.startPredefined, {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        }),
      ).rejects.toThrow("Sign up for a free account");
    });
  });

  describe("A7: anonymous session persists across refresh", () => {
    it("same identity returns same user after simulated refresh", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      // First "session" — get user info
      const userBefore = await asAnonymous.query(api.users.getMe);
      expect(userBefore).not.toBeNull();
      expect(userBefore!._id).toBe(userId);

      // Simulate refresh: create a new identity context with the same
      // userId|sessionId (browser cookie persists the session across refresh).
      const afterRefresh = t.withIdentity({
        subject: `${userId}|test-session`,
      });

      // After "refresh" — same user returned
      const userAfter = await afterRefresh.query(api.users.getMe);
      expect(userAfter).not.toBeNull();
      expect(userAfter!._id).toBe(userId);
      expect(userAfter!.isAnonymous).toBe(true);
    });

    it("runsUsed persists across simulated refresh", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      // Start a run
      await asAnonymous.mutation(api.executions.startPredefined, {
        slug: "compress-images",
        definition: { type: "group", nodes: [] },
      });

      // Verify runsUsed incremented
      const userBefore = await asAnonymous.query(api.users.getMe);
      expect(userBefore!.runsUsed).toBe(1);

      // Simulate refresh with same session
      const afterRefresh = t.withIdentity({
        subject: `${userId}|test-session`,
      });

      // runsUsed survives the refresh
      const userAfter = await afterRefresh.query(api.users.getMe);
      expect(userAfter!.runsUsed).toBe(1);
    });

    it("execution records persist across simulated refresh", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      // Start a run and get the execution ID
      const executionId = await asAnonymous.mutation(
        api.executions.startPredefined,
        {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        },
      );

      // Simulate refresh with same session
      const afterRefresh = t.withIdentity({
        subject: `${userId}|test-session`,
      });

      // Execution is still accessible after refresh
      const execution = await afterRefresh.query(api.executions.get, {
        id: executionId,
      });
      expect(execution).not.toBeNull();
      expect(execution!._id).toBe(executionId);
      expect(execution!.userId).toBe(userId);
    });

    it("getRunsRemaining persists across simulated refresh", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      // Start two runs
      await asAnonymous.mutation(api.executions.startPredefined, {
        slug: "compress-images",
        definition: { type: "group", nodes: [] },
      });
      await asAnonymous.mutation(api.executions.startPredefined, {
        slug: "resize-images",
        definition: { type: "group", nodes: [] },
      });

      const remainingBefore = await asAnonymous.query(
        api.users.getRunsRemaining,
      );
      expect(remainingBefore).toBe(FREE_RUN_LIMIT - 2);

      // Simulate refresh with same session
      const afterRefresh = t.withIdentity({
        subject: `${userId}|test-session`,
      });

      const remainingAfter = await afterRefresh.query(
        api.users.getRunsRemaining,
      );
      expect(remainingAfter).toBe(FREE_RUN_LIMIT - 2);
    });

    it("mutations still work after simulated refresh", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      // Start a run in the first session
      await asAnonymous.mutation(api.executions.startPredefined, {
        slug: "compress-images",
        definition: { type: "group", nodes: [] },
      });

      // Simulate refresh
      const afterRefresh = t.withIdentity({
        subject: `${userId}|test-session`,
      });

      // Can still start executions after refresh
      const executionId = await afterRefresh.mutation(
        api.executions.startPredefined,
        {
          slug: "resize-images",
          definition: { type: "group", nodes: [] },
        },
      );
      expect(executionId).toBeDefined();

      // Verify runsUsed reflects both runs
      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.runsUsed).toBe(2);
    });
  });
});
