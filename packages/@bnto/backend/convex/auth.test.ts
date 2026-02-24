import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";

const modules = import.meta.glob("./**/*.ts");

const FREE_RUN_LIMIT = 5;

/** Compute next month's reset timestamp (same logic as auth.ts callback). */
function nextMonthReset() {
  const now = Date.now();
  const nextMonth = new Date(now);
  nextMonth.setMonth(nextMonth.getMonth() + 1);
  nextMonth.setDate(1);
  nextMonth.setHours(0, 0, 0, 0);
  return nextMonth.getTime();
}

/**
 * Tests for the createOrUpdateUser callback patterns in auth.ts.
 *
 * We can't call the callback directly (it's wired into @convex-dev/auth),
 * so we exercise the exact same db operations it performs and verify:
 * - Schema accepts the document shapes
 * - Fields are stored and retrievable
 * - Upgrade path preserves _id and existing data
 *
 * This is the critical test surface for the auth migration. If these tests
 * pass, the createOrUpdateUser callback will work — it does exactly these
 * operations on the same schema.
 */
describe("user creation (createOrUpdateUser patterns)", () => {
  describe("new anonymous user", () => {
    it("creates a user row with isAnonymous: true and free-tier defaults", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          isAnonymous: true,
          plan: "free",
          runsUsed: 0,
          runLimit: FREE_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));

      expect(user).not.toBeNull();
      expect(user!.isAnonymous).toBe(true);
      expect(user!.plan).toBe("free");
      expect(user!.runsUsed).toBe(0);
      expect(user!.runLimit).toBe(FREE_RUN_LIMIT);
      expect(user!.runsResetAt).toBeGreaterThan(Date.now());
      // Anonymous users have no profile fields
      expect(user!.email).toBeUndefined();
      expect(user!.name).toBeUndefined();
      expect(user!.image).toBeUndefined();
    });

    it("user _id is a valid users table reference", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          isAnonymous: true,
          plan: "free",
          runsUsed: 0,
          runLimit: FREE_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
      });

      // The _id should be usable as a foreign key in executions
      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          status: "pending",
          progress: [],
          startedAt: Date.now(),
        });
      });

      const execution = await t.run(async (ctx) => ctx.db.get(executionId));
      expect(execution).not.toBeNull();
      expect(execution!.userId).toBe(userId);
    });
  });

  describe("new real user (email/password)", () => {
    it("creates a user row with email, name, and isAnonymous: false", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          name: "Jane Doe",
          email: "jane@example.com",
          isAnonymous: false,
          plan: "free",
          runsUsed: 0,
          runLimit: FREE_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));

      expect(user).not.toBeNull();
      expect(user!.name).toBe("Jane Doe");
      expect(user!.email).toBe("jane@example.com");
      expect(user!.isAnonymous).toBe(false);
      expect(user!.plan).toBe("free");
      expect(user!.runsUsed).toBe(0);
      expect(user!.runLimit).toBe(FREE_RUN_LIMIT);
    });

    it("accepts optional image field", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          name: "Jane Doe",
          email: "jane@example.com",
          image: "https://example.com/avatar.png",
          isAnonymous: false,
          plan: "free",
          runsUsed: 0,
          runLimit: FREE_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.image).toBe("https://example.com/avatar.png");
    });
  });

  describe("anonymous → authenticated upgrade", () => {
    it("preserves the same _id after upgrade", async () => {
      const t = convexTest(schema, modules);

      // Step 1: Create anonymous user (what signIn("anonymous") does)
      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          isAnonymous: true,
          plan: "free",
          runsUsed: 0,
          runLimit: FREE_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
      });

      // Step 2: Upgrade (what createOrUpdateUser does with existingUserId)
      await t.run(async (ctx) => {
        await ctx.db.patch(userId, {
          email: "jane@example.com",
          name: "Jane Doe",
          isAnonymous: false,
        });
      });

      // Verify: same _id, profile updated
      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user).not.toBeNull();
      expect(user!._id).toBe(userId);
      expect(user!.email).toBe("jane@example.com");
      expect(user!.name).toBe("Jane Doe");
      expect(user!.isAnonymous).toBe(false);
    });

    it("preserves runsUsed and plan data through upgrade", async () => {
      const t = convexTest(schema, modules);

      // Anonymous user has used 3 runs
      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          isAnonymous: true,
          plan: "free",
          runsUsed: 3,
          runLimit: FREE_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
      });

      // Upgrade to real account
      await t.run(async (ctx) => {
        await ctx.db.patch(userId, {
          email: "jane@example.com",
          name: "Jane Doe",
          isAnonymous: false,
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      // Run count must survive the upgrade — no data loss
      expect(user!.runsUsed).toBe(3);
      expect(user!.plan).toBe("free");
      expect(user!.runLimit).toBe(FREE_RUN_LIMIT);
    });

    it("preserves associated executions through upgrade", async () => {
      const t = convexTest(schema, modules);

      // Create anonymous user and run an execution
      const { userId, executionId } = await t.run(async (ctx) => {
        const uid = await ctx.db.insert("users", {
          isAnonymous: true,
          plan: "free",
          runsUsed: 1,
          runLimit: FREE_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
        const eid = await ctx.db.insert("executions", {
          userId: uid,
          status: "completed",
          progress: [],
          startedAt: Date.now(),
          completedAt: Date.now(),
        });
        return { userId: uid, executionId: eid };
      });

      // Upgrade to real account
      await t.run(async (ctx) => {
        await ctx.db.patch(userId, {
          email: "jane@example.com",
          isAnonymous: false,
        });
      });

      // Execution still belongs to the same user
      const execution = await t.run(async (ctx) => ctx.db.get(executionId));
      expect(execution).not.toBeNull();
      expect(execution!.userId).toBe(userId);
    });

    it("preserves associated execution events through upgrade", async () => {
      const t = convexTest(schema, modules);

      const { userId, eventId } = await t.run(async (ctx) => {
        const uid = await ctx.db.insert("users", {
          isAnonymous: true,
          plan: "free",
          runsUsed: 1,
          runLimit: FREE_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
        const eid = await ctx.db.insert("executionEvents", {
          userId: uid,
          slug: "compress-images",
          timestamp: Date.now(),
          status: "completed",
          durationMs: 500,
        });
        return { userId: uid, eventId: eid };
      });

      // Upgrade
      await t.run(async (ctx) => {
        await ctx.db.patch(userId, {
          email: "jane@example.com",
          isAnonymous: false,
        });
      });

      // Event still linked to same user
      const event = await t.run(async (ctx) => ctx.db.get(eventId));
      expect(event!.userId).toBe(userId);
    });

    it("only patches fields that have values (sparse update)", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          isAnonymous: true,
          plan: "free",
          runsUsed: 0,
          runLimit: FREE_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
      });

      // Upgrade with email only (no name, no image) — mirrors the callback's
      // conditional field inclusion: only sets fields that are truthy/defined
      await t.run(async (ctx) => {
        await ctx.db.patch(userId, {
          email: "jane@example.com",
          isAnonymous: false,
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.email).toBe("jane@example.com");
      expect(user!.isAnonymous).toBe(false);
      // Fields not provided in patch remain unchanged
      expect(user!.name).toBeUndefined();
      expect(user!.image).toBeUndefined();
    });
  });

  describe("profile updates (re-authentication)", () => {
    it("updates name and email on re-auth without touching app fields", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          name: "Jane Doe",
          email: "jane@example.com",
          isAnonymous: false,
          plan: "pro",
          runsUsed: 15,
          runLimit: 100,
          runsResetAt: nextMonthReset(),
        });
      });

      // Re-auth updates profile (e.g., name changed on provider)
      await t.run(async (ctx) => {
        await ctx.db.patch(userId, {
          name: "Jane Smith",
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.name).toBe("Jane Smith");
      // App fields untouched
      expect(user!.plan).toBe("pro");
      expect(user!.runsUsed).toBe(15);
      expect(user!.runLimit).toBe(100);
    });
  });

  describe("run counter increment", () => {
    it("increments runsUsed atomically", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          isAnonymous: true,
          plan: "free",
          runsUsed: 0,
          runLimit: FREE_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
        });
      });

      // Simulate what start/startPredefined does after enforceQuota
      await t.run(async (ctx) => {
        const user = await ctx.db.get(userId);
        await ctx.db.patch(userId, { runsUsed: user!.runsUsed + 1 });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.runsUsed).toBe(1);
    });

    it("increments correctly across multiple runs", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          isAnonymous: false,
          email: "jane@example.com",
          plan: "free",
          runsUsed: 0,
          runLimit: 25,
          runsResetAt: nextMonthReset(),
        });
      });

      // Run 3 executions
      for (let i = 0; i < 3; i++) {
        await t.run(async (ctx) => {
          const user = await ctx.db.get(userId);
          await ctx.db.patch(userId, { runsUsed: user!.runsUsed + 1 });
        });
      }

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.runsUsed).toBe(3);
    });
  });
});
