import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";

const modules = import.meta.glob("./**/*.ts");

/** Create a user with standard free-tier fields. */
async function seedUser(
  t: ReturnType<typeof convexTest>,
  overrides?: { totalRuns?: number; lastRunAt?: number },
) {
  return t.run(async (ctx) => {
    return ctx.db.insert("users", {
      email: "jane@example.com",
      plan: "free",
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

    it("increments totalRuns on execution", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      // Simulate what start_execution.ts does
      await t.run(async (ctx) => {
        const user = (await ctx.db.get(userId))!;
        await ctx.db.patch(userId, {
          totalRuns: user.totalRuns + 1,
          lastRunAt: Date.now(),
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.totalRuns).toBe(1);
    });

    it("accumulates totalRuns across multiple runs", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      // Simulate 5 runs
      for (let i = 0; i < 5; i++) {
        await t.run(async (ctx) => {
          const user = (await ctx.db.get(userId))!;
          await ctx.db.patch(userId, {
            totalRuns: user.totalRuns + 1,
            lastRunAt: Date.now(),
          });
        });
      }

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.totalRuns).toBe(5);
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
          totalRuns: user.totalRuns + 1,
          lastRunAt: firstRunTime,
        });
      });

      const afterFirst = await t.run(async (ctx) => ctx.db.get(userId));
      expect(afterFirst!.lastRunAt).toBe(firstRunTime);

      const secondRunTime = firstRunTime + 5000;
      await t.run(async (ctx) => {
        const user = (await ctx.db.get(userId))!;
        await ctx.db.patch(userId, {
          totalRuns: user.totalRuns + 1,
          lastRunAt: secondRunTime,
        });
      });

      const afterSecond = await t.run(async (ctx) => ctx.db.get(userId));
      expect(afterSecond!.lastRunAt).toBe(secondRunTime);
    });
  });
});
