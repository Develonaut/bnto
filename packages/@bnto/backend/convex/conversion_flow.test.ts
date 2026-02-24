import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { api } from "./_generated/api";
import {
  FREE_RUN_LIMIT,
  FREE_PLAN_RUN_LIMIT,
  nextMonthReset,
} from "./_test_helpers";

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
 * Create an anonymous user with runs and executions, simulating a user
 * who has been actively using bnto before deciding to sign up.
 */
async function seedAnonymousUserWithHistory(t: ReturnType<typeof convexTest>) {
  const { userId, executionIds, eventIds } = await t.run(async (ctx) => {
    const uid = await ctx.db.insert("users", {
      isAnonymous: true,
      plan: "free",
      runsUsed: 2,
      runLimit: FREE_RUN_LIMIT,
      runsResetAt: nextMonthReset(),
    });

    const eid1 = await ctx.db.insert("executions", {
      userId: uid,
      status: "completed",
      progress: [{ nodeId: "compress-1", status: "completed" }],
      outputFiles: [
        {
          key: "executions/exec-1/output/compressed.png",
          name: "compressed.png",
          sizeBytes: 12345,
          contentType: "image/png",
        },
      ],
      startedAt: Date.now() - 60000,
      completedAt: Date.now() - 55000,
    });

    const eid2 = await ctx.db.insert("executions", {
      userId: uid,
      status: "completed",
      progress: [{ nodeId: "resize-1", status: "completed" }],
      startedAt: Date.now() - 30000,
      completedAt: Date.now() - 25000,
    });

    const evId1 = await ctx.db.insert("executionEvents", {
      userId: uid,
      slug: "compress-images",
      timestamp: Date.now() - 60000,
      status: "completed",
      durationMs: 5000,
      executionId: eid1,
    });

    const evId2 = await ctx.db.insert("executionEvents", {
      userId: uid,
      slug: "resize-images",
      timestamp: Date.now() - 30000,
      status: "completed",
      durationMs: 3000,
      executionId: eid2,
    });

    return {
      userId: uid,
      executionIds: [eid1, eid2],
      eventIds: [evId1, evId2],
    };
  });

  const asAnonymous = t.withIdentity({
    subject: `${userId}|test-session-anon`,
  });

  return { userId, executionIds, eventIds, asAnonymous };
}

/**
 * Simulate the anonymous → authenticated upgrade that @convex-dev/auth's
 * createOrUpdateUser callback performs. Patches the existing user in-place
 * (same _id) with profile info and isAnonymous: false.
 *
 * Also upgrades runLimit to the free-tier plan limit, mirroring the
 * expected behavior when a user converts.
 */
async function simulateUpgrade(
  t: ReturnType<typeof convexTest>,
  userId: string,
) {
  await t.run(async (ctx) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any -- convex-test ctx.db.patch needs the Id type, userId is the same string
    await ctx.db.patch(userId as any, {
      email: "jane@example.com",
      name: "Jane Doe",
      isAnonymous: false,
      runLimit: FREE_PLAN_RUN_LIMIT,
    });
  });

  // Return a new identity context for the upgraded user.
  // Same userId, different session — mirrors real auth behavior.
  return t.withIdentity({
    subject: `${userId}|test-session-upgraded`,
  });
}

/**
 * Conversion Flow (C1-C3)
 *
 * Tests the anonymous → signup revenue funnel. When an anonymous user signs
 * up, their existing data (executions, events, run count) must carry over
 * seamlessly. The userId stays the same — no data migration needed.
 *
 * Spec: .claude/journeys/auth.md — "Conversion Flow"
 */
describe("conversion flow (C1-C3)", () => {
  describe("C1: anonymous → signup conversion", () => {
    it("preserves the same userId after upgrade", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUserWithHistory(t);

      // Verify user is anonymous before upgrade
      const before = await asAnonymous.query(api.users.getMe);
      expect(before).not.toBeNull();
      expect(before!.isAnonymous).toBe(true);
      expect(before!._id).toBe(userId);

      // Simulate upgrade
      const asConverted = await simulateUpgrade(t, userId);

      // Verify same userId, now authenticated
      const after = await asConverted.query(api.users.getMe);
      expect(after).not.toBeNull();
      expect(after!._id).toBe(userId);
      expect(after!.isAnonymous).toBe(false);
      expect(after!.email).toBe("jane@example.com");
      expect(after!.name).toBe("Jane Doe");
    });

    it("preserves existing run count through upgrade", async () => {
      const t = convexTest(schema, modules);
      const { userId } = await seedAnonymousUserWithHistory(t);

      const asConverted = await simulateUpgrade(t, userId);

      const user = await asConverted.query(api.users.getMe);
      expect(user).not.toBeNull();
      // Run count must survive — no reset on upgrade
      expect(user!.runsUsed).toBe(2);
    });

    it("existing executions still belong to the converted user", async () => {
      const t = convexTest(schema, modules);
      const { userId, executionIds } = await seedAnonymousUserWithHistory(t);

      await simulateUpgrade(t, userId);

      // Verify executions still reference the same userId
      for (const execId of executionIds) {
        const execution = await t.run(async (ctx) => ctx.db.get(execId));
        expect(execution).not.toBeNull();
        expect(execution!.userId).toBe(userId);
      }
    });

    it("existing execution events still belong to the converted user", async () => {
      const t = convexTest(schema, modules);
      const { userId, eventIds } = await seedAnonymousUserWithHistory(t);

      await simulateUpgrade(t, userId);

      for (const eventId of eventIds) {
        const event = await t.run(async (ctx) => ctx.db.get(eventId));
        expect(event).not.toBeNull();
        expect(event!.userId).toBe(userId);
      }
    });

    it("converted user can start new executions", async () => {
      const t = convexTest(schema, modules);
      const { userId } = await seedAnonymousUserWithHistory(t);
      const asConverted = await simulateUpgrade(t, userId);

      const executionId = await asConverted.mutation(
        api.executions.startPredefined,
        {
          slug: "clean-csv",
          definition: { type: "group", nodes: [] },
        },
      );

      expect(executionId).toBeDefined();

      const execution = await t.run(async (ctx) => ctx.db.get(executionId));
      expect(execution).not.toBeNull();
      expect(execution!.userId).toBe(userId);
      expect(execution!.status).toBe("pending");
    });
  });

  describe("C2: converted user retains access to pre-conversion data", () => {
    it("api.executions.get returns pre-conversion execution", async () => {
      const t = convexTest(schema, modules);
      const { userId, executionIds } = await seedAnonymousUserWithHistory(t);
      const asConverted = await simulateUpgrade(t, userId);

      // Query pre-conversion executions through the auth-gated API
      for (const execId of executionIds) {
        const execution = await asConverted.query(api.executions.get, {
          id: execId,
        });
        expect(execution).not.toBeNull();
        expect(execution!._id).toBe(execId);
        expect(execution!.userId).toBe(userId);
      }
    });

    it("pre-conversion execution with outputFiles is accessible", async () => {
      const t = convexTest(schema, modules);
      const { userId, executionIds } = await seedAnonymousUserWithHistory(t);
      const asConverted = await simulateUpgrade(t, userId);

      // First execution was seeded with outputFiles
      const execution = await asConverted.query(api.executions.get, {
        id: executionIds[0],
      });

      expect(execution).not.toBeNull();
      expect(execution!.status).toBe("completed");
      expect(execution!.outputFiles).toBeDefined();
      expect(execution!.outputFiles).toHaveLength(1);
      expect(execution!.outputFiles![0].name).toBe("compressed.png");
    });

    it("another user cannot access the converted user's executions", async () => {
      const t = convexTest(schema, modules);
      const { executionIds } = await seedAnonymousUserWithHistory(t);

      // Create a separate user
      const otherUserId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          email: "other@example.com",
          isAnonymous: false,
          plan: "free",
          runsUsed: 0,
          runLimit: FREE_PLAN_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
      });
      const asOther = t.withIdentity({
        subject: `${otherUserId}|test-session-other`,
      });

      // Other user cannot see the converted user's executions
      const execution = await asOther.query(api.executions.get, {
        id: executionIds[0],
      });
      expect(execution).toBeNull();
    });

    it("getRunsRemaining works for converted user", async () => {
      const t = convexTest(schema, modules);
      const { userId } = await seedAnonymousUserWithHistory(t);
      const asConverted = await simulateUpgrade(t, userId);

      const remaining = await asConverted.query(api.users.getRunsRemaining);

      expect(remaining).not.toBeNull();
      // User had 2 runs, upgraded to 25 limit
      expect(remaining).toBe(FREE_PLAN_RUN_LIMIT - 2);
    });
  });

  describe("C3: converted user gets full quota", () => {
    it("quota uses plan limit, not anonymous limit, after upgrade", async () => {
      const t = convexTest(schema, modules);
      const { userId } = await seedAnonymousUserWithHistory(t);
      const asConverted = await simulateUpgrade(t, userId);

      const user = await asConverted.query(api.users.getMe);
      expect(user).not.toBeNull();
      // Plan limit should be free-tier (FREE_PLAN_RUN_LIMIT), not anonymous (ANONYMOUS_RUN_LIMIT)
      expect(user!.runLimit).toBe(FREE_PLAN_RUN_LIMIT);
      // Existing run count preserved (not reset)
      expect(user!.runsUsed).toBe(2);
      expect(user!.isAnonymous).toBe(false);
    });

    it("converted user can run past the old anonymous limit", async () => {
      const t = convexTest(schema, modules);

      // Create anonymous user at the anonymous limit (3 runs)
      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          isAnonymous: true,
          plan: "free",
          runsUsed: 3,
          runLimit: FREE_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
      });

      // Verify anonymous user would be blocked by anonymous limit
      const asAnonymous = t.withIdentity({
        subject: `${userId}|test-session-anon`,
      });
      await expect(
        asAnonymous.mutation(api.executions.startPredefined, {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        }),
      ).rejects.toThrow();

      // Upgrade — now limit is 25, not 3
      const asConverted = await simulateUpgrade(t, userId);

      // Should succeed — 3 runs used, 25 limit
      const executionId = await asConverted.mutation(
        api.executions.startPredefined,
        {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        },
      );

      expect(executionId).toBeDefined();
    });

    it("quota error after upgrade uses RUN_LIMIT_REACHED, not ANONYMOUS_QUOTA_EXCEEDED", async () => {
      const t = convexTest(schema, modules);

      // Create a converted user already at the free plan limit
      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          isAnonymous: false,
          email: "jane@example.com",
          plan: "free",
          runsUsed: FREE_PLAN_RUN_LIMIT,
          runLimit: FREE_PLAN_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
      });

      const asConverted = t.withIdentity({
        subject: `${userId}|test-session-converted`,
      });

      // Must throw the plan-level quota error ("Run limit reached"),
      // NOT the anonymous quota error ("Sign up for a free account")
      await expect(
        asConverted.mutation(api.executions.startPredefined, {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        }),
      ).rejects.toThrow("Run limit reached");
    });

    it("run count increments correctly after upgrade", async () => {
      const t = convexTest(schema, modules);
      const { userId } = await seedAnonymousUserWithHistory(t);
      const asConverted = await simulateUpgrade(t, userId);

      // User had 2 runs before upgrade. Run one more.
      await asConverted.mutation(api.executions.startPredefined, {
        slug: "rename-files",
        definition: { type: "group", nodes: [] },
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.runsUsed).toBe(3);
    });

    it("execution events after upgrade link to the same userId", async () => {
      const t = convexTest(schema, modules);
      const { userId } = await seedAnonymousUserWithHistory(t);
      const asConverted = await simulateUpgrade(t, userId);

      await asConverted.mutation(api.executions.startPredefined, {
        slug: "clean-csv",
        definition: { type: "group", nodes: [] },
      });

      // Should have 3 events total (2 pre-upgrade + 1 post-upgrade)
      const events = await t.run(async (ctx) => {
        return ctx.db
          .query("executionEvents")
          .withIndex("by_userId", (q) => q.eq("userId", userId))
          .collect();
      });

      expect(events).toHaveLength(3);
      // All events linked to the same userId
      for (const event of events) {
        expect(event.userId).toBe(userId);
      }
      // Latest event is the post-upgrade one
      const latest = events[events.length - 1];
      expect(latest.slug).toBe("clean-csv");
      expect(latest.status).toBe("started");
    });
  });
});
