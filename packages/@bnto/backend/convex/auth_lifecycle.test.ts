import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { api } from "./_generated/api";

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
 * Create a real (email/password) user in the database and return an
 * authenticated test context. Mirrors what @convex-dev/auth's
 * createOrUpdateUser callback does for Password provider sign-ups.
 */
async function seedRealUser(
  t: ReturnType<typeof convexTest>,
  overrides?: { email?: string; name?: string },
) {
  const email = overrides?.email ?? "jane@example.com";
  const name = overrides?.name ?? "Jane Doe";

  const userId = await t.run(async (ctx) => {
    return ctx.db.insert("users", {
      email,
      name,
      plan: "free",
      totalRuns: 0,
    });
  });

  const asUser = t.withIdentity({
    subject: `${userId}|test-session-real`,
  });

  return { userId, asUser };
}

/**
 * Auth Lifecycle (S1-S3)
 *
 * Tests standard email sign-in, sign-out, and the auth API surface.
 * Verifies that authenticated users can access protected queries/mutations,
 * and that unauthenticated requests are correctly rejected.
 *
 * Spec: .claude/journeys/auth.md — "Standard Auth Lifecycle"
 */
describe("auth lifecycle (S1-S3)", () => {
  describe("S1: email sign-in works", () => {
    it("getMe returns the authenticated user with correct fields", async () => {
      const t = convexTest(schema, modules);
      const { userId, asUser } = await seedRealUser(t);

      const user = await asUser.query(api.users.getMe);

      expect(user).not.toBeNull();
      expect(user!._id).toBe(userId);
      expect(user!.email).toBe("jane@example.com");
      expect(user!.name).toBe("Jane Doe");
      expect(user!.plan).toBe("free");
      expect(user!.totalRuns).toBe(0);
    });

    it("authenticated user can start a predefined execution", async () => {
      const t = convexTest(schema, modules);
      const { userId, asUser } = await seedRealUser(t);

      const executionId = await asUser.mutation(
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
    });

    it("authenticated user can query their own execution", async () => {
      const t = convexTest(schema, modules);
      const { userId, asUser } = await seedRealUser(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          status: "completed",
          progress: [{ nodeId: "resize-1", status: "completed" }],
          startedAt: Date.now(),
          completedAt: Date.now(),
        });
      });

      const execution = await asUser.query(api.executions.get, {
        id: executionId,
      });

      expect(execution).not.toBeNull();
      expect(execution!._id).toBe(executionId);
      expect(execution!.status).toBe("completed");
    });
  });

  describe("S2: sign-out clears session", () => {
    it("getMe returns null after sign-out (no identity)", async () => {
      const t = convexTest(schema, modules);
      await seedRealUser(t);

      // After sign-out, there is no identity — bare `t` has no auth context
      const user = await t.query(api.users.getMe);

      expect(user).toBeNull();
    });

    it("startPredefined rejects after sign-out", async () => {
      const t = convexTest(schema, modules);
      await seedRealUser(t);

      // No identity context — simulates post-sign-out state
      await expect(
        t.mutation(api.executions.startPredefined, {
          slug: "compress-images",
          definition: { type: "group", nodes: [] },
        }),
      ).rejects.toThrow("Not authenticated");
    });

    it("executions.get returns null after sign-out", async () => {
      const t = convexTest(schema, modules);
      const { userId } = await seedRealUser(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          status: "completed",
          progress: [],
          startedAt: Date.now(),
          completedAt: Date.now(),
        });
      });

      // No identity — can't access the execution
      const execution = await t.query(api.executions.get, {
        id: executionId,
      });

      expect(execution).toBeNull();
    });
  });

  describe("S3: auth API surface", () => {
    it("two email users cannot see each other's executions", async () => {
      const t = convexTest(schema, modules);
      const { userId: userAId, asUser: asUserA } = await seedRealUser(t, {
        email: "alice@example.com",
        name: "Alice",
      });
      const { asUser: asUserB } = await seedRealUser(t, {
        email: "bob@example.com",
        name: "Bob",
      });

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId: userAId,
          status: "running",
          progress: [{ nodeId: "compress-1", status: "running" }],
          startedAt: Date.now(),
        });
      });

      // Owner can see it
      const ownExec = await asUserA.query(api.executions.get, {
        id: executionId,
      });
      expect(ownExec).not.toBeNull();
      expect(ownExec!._id).toBe(executionId);

      // Other user cannot
      const otherExec = await asUserB.query(api.executions.get, {
        id: executionId,
      });
      expect(otherExec).toBeNull();
    });

    it("getMe returns correct shape for email user", async () => {
      const t = convexTest(schema, modules);
      const { asUser } = await seedRealUser(t);

      const user = await asUser.query(api.users.getMe);

      expect(user).not.toBeNull();
      // All expected fields present
      expect(user).toHaveProperty("_id");
      expect(user).toHaveProperty("_creationTime");
      expect(user).toHaveProperty("email");
      expect(user).toHaveProperty("name");
      expect(user).toHaveProperty("plan");
      expect(user).toHaveProperty("totalRuns");
      // Type correctness
      expect(typeof user!.email).toBe("string");
      expect(typeof user!.name).toBe("string");
      expect(typeof user!.plan).toBe("string");
      expect(typeof user!.totalRuns).toBe("number");
    });

    it("invalid identity (non-existent user) returns null from getMe", async () => {
      const t = convexTest(schema, modules);

      // Identity pointing to a userId that doesn't exist in the users table
      const asBogus = t.withIdentity({
        subject: "nonexistent-user-id|fake-session",
      });

      const user = await asBogus.query(api.users.getMe);

      expect(user).toBeNull();
    });

    it("listByRecipe returns empty for unauthenticated user", async () => {
      const t = convexTest(schema, modules);
      const { userId } = await seedRealUser(t);

      const recipeId = await t.run(async (ctx) => {
        return ctx.db.insert("recipes", {
          userId,
          name: "Test Recipe",
          definition: { type: "group", nodes: [] },
          version: 1,
          isPublic: false,
          createdAt: Date.now(),
          updatedAt: Date.now(),
        });
      });

      // No identity — simulates unauthenticated state
      const executions = await t.query(api.executions.listByRecipe, {
        recipeId,
      });

      expect(executions).toEqual([]);
    });

    it("listByRecipe returns executions for the owning user", async () => {
      const t = convexTest(schema, modules);
      const { userId, asUser } = await seedRealUser(t);

      const recipeId = await t.run(async (ctx) => {
        return ctx.db.insert("recipes", {
          userId,
          name: "Test Recipe",
          definition: { type: "group", nodes: [] },
          version: 1,
          isPublic: false,
          createdAt: Date.now(),
          updatedAt: Date.now(),
        });
      });

      await t.run(async (ctx) => {
        await ctx.db.insert("executions", {
          userId,
          recipeId,
          status: "completed",
          progress: [],
          startedAt: Date.now(),
          completedAt: Date.now(),
        });
      });

      const executions = await asUser.query(api.executions.listByRecipe, {
        recipeId,
      });

      expect(executions).toHaveLength(1);
      expect(executions[0].recipeId).toBe(recipeId);
      expect(executions[0].userId).toBe(userId);
    });
  });
});
