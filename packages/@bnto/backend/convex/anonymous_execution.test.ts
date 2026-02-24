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
async function seedAnonymousUser(t: ReturnType<typeof convexTest>) {
  const userId = await t.run(async (ctx) => {
    return ctx.db.insert("users", {
      isAnonymous: true,
      plan: "free",
      runsUsed: 0,
      runLimit: FREE_RUN_LIMIT,
      runsResetAt: nextMonthReset(),
    });
  });

  const asAnonymous = t.withIdentity({
    subject: `${userId}|test-session`,
  });

  return { userId, asAnonymous };
}

/**
 * Anonymous Execution Flow (A1-A5)
 *
 * Tests the happy path: anonymous user goes from landing to download.
 * Every auth gate must pass. These tests exercise the real Convex
 * mutations and queries with a simulated anonymous identity.
 *
 * Spec: .claude/journeys/auth.md — "Anonymous Execution Flow"
 */
describe("anonymous execution flow (A1-A5)", () => {
  describe("A1: anonymous session bootstrap", () => {
    it("getMe returns the anonymous user with isAnonymous: true", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      const user = await asAnonymous.query(api.users.getMe);

      expect(user).not.toBeNull();
      expect(user!._id).toBe(userId);
      expect(user!.isAnonymous).toBe(true);
      expect(user!.plan).toBe("free");
      expect(user!.runsUsed).toBe(0);
      expect(user!.runLimit).toBe(FREE_RUN_LIMIT);
    });

    it("getAppUserId returns non-null for anonymous session", async () => {
      const t = convexTest(schema, modules);
      const { asAnonymous } = await seedAnonymousUser(t);

      // getRunsRemaining calls getAppUserId internally.
      // Non-null result proves getAppUserId resolved the anonymous user.
      const remaining = await asAnonymous.query(api.users.getRunsRemaining);

      expect(remaining).not.toBeNull();
      expect(remaining).toBe(FREE_RUN_LIMIT);
    });

    it("unauthenticated user gets null from getMe", async () => {
      const t = convexTest(schema, modules);

      const user = await t.query(api.users.getMe);

      expect(user).toBeNull();
    });

    it("unauthenticated user gets null from getRunsRemaining", async () => {
      const t = convexTest(schema, modules);

      const remaining = await t.query(api.users.getRunsRemaining);

      expect(remaining).toBeNull();
    });
  });

  describe("A2: anonymous can access upload auth gate", () => {
    it("getMe returns plan for anonymous user (upload limit source)", async () => {
      const t = convexTest(schema, modules);
      const { asAnonymous } = await seedAnonymousUser(t);

      // generateUploadUrls (action) calls api.users.getMe to get the plan.
      // Actions require Node.js runtime (R2 calls) so we test the auth gate
      // they depend on: getMe returning the user's plan.
      const user = await asAnonymous.query(api.users.getMe);

      expect(user).not.toBeNull();
      expect(user!.plan).toBe("free");
    });

    it("unauthenticated user gets null from getMe (defaults to free)", async () => {
      const t = convexTest(schema, modules);

      // generateUploadUrls falls back to "free" when getMe returns null.
      // This test verifies the fallback path is exercised for unauth users.
      const user = await t.query(api.users.getMe);

      expect(user).toBeNull();
    });
  });

  describe("A3: anonymous can start execution", () => {
    it("startPredefined succeeds and creates execution record", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      const executionId = await asAnonymous.mutation(
        api.executions.startPredefined,
        {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        },
      );

      expect(executionId).toBeDefined();

      const execution = await t.run(async (ctx) => ctx.db.get(executionId));
      expect(execution).not.toBeNull();
      expect(execution!.userId).toBe(userId);
      expect(execution!.status).toBe("pending");
      expect(execution!.startedAt).toBeGreaterThan(0);
    });

    it("increments runsUsed after startPredefined", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      await asAnonymous.mutation(api.executions.startPredefined, {
        slug: "compress-images",
        definition: { type: "group", nodes: [] },
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.runsUsed).toBe(1);
    });

    it("logs execution event with correct fields", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      const executionId = await asAnonymous.mutation(
        api.executions.startPredefined,
        {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        },
      );

      const events = await t.run(async (ctx) => {
        return ctx.db
          .query("executionEvents")
          .withIndex("by_userId", (q) => q.eq("userId", userId))
          .collect();
      });

      expect(events).toHaveLength(1);
      expect(events[0].slug).toBe("compress-images");
      expect(events[0].status).toBe("started");
      expect(events[0].executionId).toBe(executionId);
      expect(events[0].userId).toBe(userId);
      expect(events[0].timestamp).toBeGreaterThan(0);
    });

    it("passes sessionId through to execution record", async () => {
      const t = convexTest(schema, modules);
      const { asAnonymous } = await seedAnonymousUser(t);

      const executionId = await asAnonymous.mutation(
        api.executions.startPredefined,
        {
          slug: "resize-images",
          definition: { type: "group", nodes: [] },
          sessionId: "session-abc-123",
        },
      );

      const execution = await t.run(async (ctx) => ctx.db.get(executionId));
      expect(execution!.sessionId).toBe("session-abc-123");
    });

    it("rejects unauthenticated user with Not authenticated", async () => {
      const t = convexTest(schema, modules);

      await expect(
        t.mutation(api.executions.startPredefined, {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        }),
      ).rejects.toThrow("Not authenticated");
    });
  });

  describe("A4: anonymous can subscribe to progress", () => {
    it("get returns the execution for the owning anonymous user", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          status: "running",
          progress: [{ nodeId: "resize-1", status: "completed" }],
          startedAt: Date.now(),
        });
      });

      const execution = await asAnonymous.query(api.executions.get, {
        id: executionId,
      });

      expect(execution).not.toBeNull();
      expect(execution!._id).toBe(executionId);
      expect(execution!.userId).toBe(userId);
      expect(execution!.status).toBe("running");
      expect(execution!.progress).toEqual([
        { nodeId: "resize-1", status: "completed" },
      ]);
    });

    it("returns null for another user's execution (ownership check)", async () => {
      const t = convexTest(schema, modules);
      const { asAnonymous: userA } = await seedAnonymousUser(t);
      const { userId: userBId } = await seedAnonymousUser(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId: userBId,
          status: "running",
          progress: [],
          startedAt: Date.now(),
        });
      });

      const execution = await userA.query(api.executions.get, {
        id: executionId,
      });

      expect(execution).toBeNull();
    });

    it("returns null for unauthenticated user", async () => {
      const t = convexTest(schema, modules);
      const { userId } = await seedAnonymousUser(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          status: "running",
          progress: [],
          startedAt: Date.now(),
        });
      });

      const execution = await t.query(api.executions.get, {
        id: executionId,
      });

      expect(execution).toBeNull();
    });
  });

  describe("A5: anonymous can access download auth gate", () => {
    it("get returns completed execution with outputFiles", async () => {
      const t = convexTest(schema, modules);
      const { userId, asAnonymous } = await seedAnonymousUser(t);

      const outputFiles = [
        {
          key: "executions/test-123/output/compressed.png",
          name: "compressed.png",
          sizeBytes: 12345,
          contentType: "image/png",
        },
      ];

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          status: "completed",
          progress: [],
          outputFiles,
          startedAt: Date.now(),
          completedAt: Date.now(),
        });
      });

      // generateDownloadUrls calls api.executions.get internally.
      // If get returns the completed execution, the download auth gate passes.
      const execution = await asAnonymous.query(api.executions.get, {
        id: executionId,
      });

      expect(execution).not.toBeNull();
      expect(execution!.status).toBe("completed");
      expect(execution!.outputFiles).toEqual(outputFiles);
    });

    it("blocks another user from accessing download gate", async () => {
      const t = convexTest(schema, modules);
      const { userId: ownerUserId } = await seedAnonymousUser(t);
      const { asAnonymous: otherUser } = await seedAnonymousUser(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId: ownerUserId,
          status: "completed",
          progress: [],
          outputFiles: [
            {
              key: "executions/test-456/output/result.png",
              name: "result.png",
              sizeBytes: 5678,
              contentType: "image/png",
            },
          ],
          startedAt: Date.now(),
          completedAt: Date.now(),
        });
      });

      const execution = await otherUser.query(api.executions.get, {
        id: executionId,
      });

      expect(execution).toBeNull();
    });

    it("blocks unauthenticated user from accessing download gate", async () => {
      const t = convexTest(schema, modules);
      const { userId } = await seedAnonymousUser(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          status: "completed",
          progress: [],
          outputFiles: [
            {
              key: "executions/test-789/output/output.csv",
              name: "output.csv",
              sizeBytes: 999,
              contentType: "text/csv",
            },
          ],
          startedAt: Date.now(),
          completedAt: Date.now(),
        });
      });

      const execution = await t.query(api.executions.get, {
        id: executionId,
      });

      expect(execution).toBeNull();
    });
  });
});
