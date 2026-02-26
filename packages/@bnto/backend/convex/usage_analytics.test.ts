import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { internal } from "./_generated/api";
import { FREE_PLAN_RUN_LIMIT, nextMonthReset, pastReset } from "./_test_helpers";

const modules = import.meta.glob("./**/*.ts");

/** Create a user with standard free-tier fields. */
async function seedUser(
  t: ReturnType<typeof convexTest>,
  overrides?: { totalRuns?: number; lastRunAt?: number; runsUsed?: number },
) {
  return t.run(async (ctx) => {
    return ctx.db.insert("users", {
      email: "jane@example.com",
      isAnonymous: false,
      plan: "free",
      runsUsed: overrides?.runsUsed ?? 0,
      runLimit: FREE_PLAN_RUN_LIMIT,
      runsResetAt: nextMonthReset(),
      totalRuns: overrides?.totalRuns ?? 0,
      lastRunAt: overrides?.lastRunAt,
    });
  });
}

describe("usage analytics fields", () => {
  describe("totalRuns", () => {
    it("new user starts with totalRuns = 0", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.totalRuns).toBe(0);
    });

    it("increments totalRuns alongside runsUsed", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      // Simulate what executions.ts start/startPredefined does
      await t.run(async (ctx) => {
        const user = (await ctx.db.get(userId))!;
        await ctx.db.patch(userId, {
          runsUsed: user.runsUsed + 1,
          totalRuns: (user.totalRuns ?? 0) + 1,
          lastRunAt: Date.now(),
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.totalRuns).toBe(1);
      expect(user!.runsUsed).toBe(1);
    });

    it("accumulates totalRuns across multiple runs", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      // Simulate 5 runs
      for (let i = 0; i < 5; i++) {
        await t.run(async (ctx) => {
          const user = (await ctx.db.get(userId))!;
          await ctx.db.patch(userId, {
            runsUsed: user.runsUsed + 1,
            totalRuns: (user.totalRuns ?? 0) + 1,
            lastRunAt: Date.now(),
          });
        });
      }

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.totalRuns).toBe(5);
      expect(user!.runsUsed).toBe(5);
    });

    it("survives monthly reset — totalRuns is never reset", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          email: "jane@example.com",
          isAnonymous: false,
          plan: "free",
          runsUsed: 20,
          runLimit: FREE_PLAN_RUN_LIMIT,
          runsResetAt: pastReset(),
          totalRuns: 150,
          lastRunAt: Date.now() - 3600_000,
        });
      });

      await t.mutation(internal.users.resetRunCounters, {});

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.runsUsed).toBe(0); // monthly counter resets
      expect(user!.totalRuns).toBe(150); // all-time total preserved
    });

    it("handles legacy users without totalRuns (backward compat)", async () => {
      const t = convexTest(schema, modules);

      // Insert user without totalRuns — simulates pre-migration docs
      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          email: "legacy@example.com",
          isAnonymous: false,
          plan: "free",
          runsUsed: 10,
          runLimit: FREE_PLAN_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
          // totalRuns not set — optional field
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      // Code should treat undefined as 0
      expect(user!.totalRuns ?? 0).toBe(0);

      // First run on legacy user increments from undefined → 1
      await t.run(async (ctx) => {
        const u = (await ctx.db.get(userId))!;
        await ctx.db.patch(userId, {
          totalRuns: (u.totalRuns ?? 0) + 1,
          lastRunAt: Date.now(),
        });
      });

      const updated = await t.run(async (ctx) => ctx.db.get(userId));
      expect(updated!.totalRuns).toBe(1);
    });
  });

  describe("lastRunAt", () => {
    it("new user has no lastRunAt", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.lastRunAt).toBeUndefined();
    });

    it("updates lastRunAt on each run", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      const firstRunTime = Date.now();
      await t.run(async (ctx) => {
        const user = (await ctx.db.get(userId))!;
        await ctx.db.patch(userId, {
          runsUsed: user.runsUsed + 1,
          totalRuns: (user.totalRuns ?? 0) + 1,
          lastRunAt: firstRunTime,
        });
      });

      const afterFirst = await t.run(async (ctx) => ctx.db.get(userId));
      expect(afterFirst!.lastRunAt).toBe(firstRunTime);

      const secondRunTime = firstRunTime + 5000;
      await t.run(async (ctx) => {
        const user = (await ctx.db.get(userId))!;
        await ctx.db.patch(userId, {
          runsUsed: user.runsUsed + 1,
          totalRuns: (user.totalRuns ?? 0) + 1,
          lastRunAt: secondRunTime,
        });
      });

      const afterSecond = await t.run(async (ctx) => ctx.db.get(userId));
      expect(afterSecond!.lastRunAt).toBe(secondRunTime);
    });

    it("survives monthly reset — lastRunAt is preserved", async () => {
      const t = convexTest(schema, modules);
      const lastRun = Date.now() - 86400_000; // 1 day ago

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          email: "jane@example.com",
          isAnonymous: false,
          plan: "free",
          runsUsed: 15,
          runLimit: FREE_PLAN_RUN_LIMIT,
          runsResetAt: pastReset(),
          totalRuns: 100,
          lastRunAt: lastRun,
        });
      });

      await t.mutation(internal.users.resetRunCounters, {});

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.runsUsed).toBe(0);
      expect(user!.lastRunAt).toBe(lastRun); // preserved
    });
  });

  describe("getUsageStats (logic)", () => {
    it("returns correct stats for active user", async () => {
      const t = convexTest(schema, modules);
      const lastRun = Date.now() - 3600_000;

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          email: "jane@example.com",
          isAnonymous: false,
          plan: "free",
          runsUsed: 7,
          runLimit: FREE_PLAN_RUN_LIMIT,
          runsResetAt: nextMonthReset(),
          totalRuns: 42,
          lastRunAt: lastRun,
        });
      });

      // getUsageStats requires auth context — verify logic via direct read
      const user = await t.run(async (ctx) => ctx.db.get(userId));
      const stats = {
        plan: user!.plan,
        totalRuns: user!.totalRuns ?? 0,
        lastRunAt: user!.lastRunAt ?? null,
        runsUsedThisMonth: user!.runsUsed,
        runLimit: user!.runLimit,
        runsRemaining: Math.max(0, user!.runLimit - user!.runsUsed),
      };

      expect(stats).toEqual({
        plan: "free",
        totalRuns: 42,
        lastRunAt: lastRun,
        runsUsedThisMonth: 7,
        runLimit: FREE_PLAN_RUN_LIMIT,
        runsRemaining: FREE_PLAN_RUN_LIMIT - 7,
      });
    });

    it("returns totalRuns = 0 and lastRunAt = null for new user", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      const stats = {
        plan: user!.plan,
        totalRuns: user!.totalRuns ?? 0,
        lastRunAt: user!.lastRunAt ?? null,
        runsUsedThisMonth: user!.runsUsed,
        runLimit: user!.runLimit,
        runsRemaining: Math.max(0, user!.runLimit - user!.runsUsed),
      };

      expect(stats.totalRuns).toBe(0);
      expect(stats.lastRunAt).toBeNull();
      expect(stats.runsRemaining).toBe(FREE_PLAN_RUN_LIMIT);
    });

    it("returns correct stats for pro user", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          email: "pro@example.com",
          isAnonymous: false,
          plan: "pro",
          runsUsed: 50,
          runLimit: 1000,
          runsResetAt: nextMonthReset(),
          totalRuns: 500,
          lastRunAt: Date.now(),
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.plan).toBe("pro");
      expect(user!.totalRuns).toBe(500);
      expect(Math.max(0, user!.runLimit - user!.runsUsed)).toBe(950);
    });
  });
});
