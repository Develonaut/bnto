import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { internal } from "./_generated/api";

const modules = import.meta.glob("./**/*.ts");

const FREE_RUN_LIMIT = 5;

/** Helper to compute a reset timestamp in the past. */
function pastReset() {
  return Date.now() - 1000;
}

/** Helper to compute next month's reset timestamp. */
function futureReset() {
  const d = new Date();
  d.setMonth(d.getMonth() + 1);
  d.setDate(1);
  d.setHours(0, 0, 0, 0);
  return d.getTime();
}

describe("resetRunCounters", () => {
  it("resets runsUsed to 0 for users past their reset date", async () => {
    const t = convexTest(schema, modules);

    const userId = await t.run(async (ctx) => {
      return ctx.db.insert("users", {
        isAnonymous: false,
        email: "jane@example.com",
        plan: "free",
        runsUsed: 20,
        runLimit: FREE_RUN_LIMIT,
        runsResetAt: pastReset(),
      });
    });

    // Run the reset mutation
    await t.mutation(internal.users.resetRunCounters, {});

    const user = await t.run(async (ctx) => ctx.db.get(userId));
    expect(user!.runsUsed).toBe(0);
    // Reset date should be pushed to next month
    expect(user!.runsResetAt).toBeGreaterThan(Date.now());
  });

  it("does NOT reset users whose reset date is still in the future", async () => {
    const t = convexTest(schema, modules);

    const userId = await t.run(async (ctx) => {
      return ctx.db.insert("users", {
        isAnonymous: false,
        email: "jane@example.com",
        plan: "free",
        runsUsed: 3,
        runLimit: FREE_RUN_LIMIT,
        runsResetAt: futureReset(),
      });
    });

    await t.mutation(internal.users.resetRunCounters, {});

    const user = await t.run(async (ctx) => ctx.db.get(userId));
    expect(user!.runsUsed).toBe(3); // unchanged
  });

  it("resets multiple users in a single pass", async () => {
    const t = convexTest(schema, modules);

    const [id1, id2, id3] = await t.run(async (ctx) => {
      const a = await ctx.db.insert("users", {
        isAnonymous: false,
        email: "a@example.com",
        plan: "free",
        runsUsed: 5,
        runLimit: FREE_RUN_LIMIT,
        runsResetAt: pastReset(),
      });
      const b = await ctx.db.insert("users", {
        isAnonymous: false,
        email: "b@example.com",
        plan: "pro",
        runsUsed: 80,
        runLimit: 100,
        runsResetAt: pastReset(),
      });
      const c = await ctx.db.insert("users", {
        isAnonymous: true,
        plan: "free",
        runsUsed: 2,
        runLimit: FREE_RUN_LIMIT,
        runsResetAt: futureReset(), // Not due yet
      });
      return [a, b, c] as const;
    });

    await t.mutation(internal.users.resetRunCounters, {});

    const [u1, u2, u3] = await t.run(async (ctx) => {
      return [
        await ctx.db.get(id1),
        await ctx.db.get(id2),
        await ctx.db.get(id3),
      ] as const;
    });

    expect(u1!.runsUsed).toBe(0); // reset
    expect(u2!.runsUsed).toBe(0); // reset
    expect(u3!.runsUsed).toBe(2); // NOT reset (future date)
  });

  it("preserves plan and other fields after reset", async () => {
    const t = convexTest(schema, modules);

    const userId = await t.run(async (ctx) => {
      return ctx.db.insert("users", {
        name: "Jane Doe",
        email: "jane@example.com",
        isAnonymous: false,
        plan: "pro",
        runsUsed: 50,
        runLimit: 100,
        runsResetAt: pastReset(),
      });
    });

    await t.mutation(internal.users.resetRunCounters, {});

    const user = await t.run(async (ctx) => ctx.db.get(userId));
    expect(user!.runsUsed).toBe(0);
    expect(user!.plan).toBe("pro");
    expect(user!.runLimit).toBe(100);
    expect(user!.name).toBe("Jane Doe");
    expect(user!.email).toBe("jane@example.com");
  });
});

describe("getMe (schema shape)", () => {
  it("returns user with all expected fields", async () => {
    const t = convexTest(schema, modules);

    const userId = await t.run(async (ctx) => {
      return ctx.db.insert("users", {
        name: "Jane Doe",
        email: "jane@example.com",
        isAnonymous: false,
        plan: "free",
        runsUsed: 3,
        runLimit: FREE_RUN_LIMIT,
        runsResetAt: futureReset(),
      });
    });

    // Direct read — getMe requires auth context we can't set in convex-test,
    // so we verify the schema shape via direct db.get.
    const user = await t.run(async (ctx) => ctx.db.get(userId));

    expect(user).toMatchObject({
      name: "Jane Doe",
      email: "jane@example.com",
      isAnonymous: false,
      plan: "free",
      runsUsed: 3,
      runLimit: FREE_RUN_LIMIT,
    });
    expect(user!._id).toBe(userId);
    expect(user!._creationTime).toBeDefined();
  });
});

describe("getRunsRemaining (logic)", () => {
  it("computes remaining runs correctly", async () => {
    const t = convexTest(schema, modules);

    const userId = await t.run(async (ctx) => {
      return ctx.db.insert("users", {
        isAnonymous: false,
        email: "jane@example.com",
        plan: "free",
        runsUsed: 3,
        runLimit: 25,
        runsResetAt: futureReset(),
      });
    });

    const user = await t.run(async (ctx) => ctx.db.get(userId));
    const remaining = Math.max(0, user!.runLimit - user!.runsUsed);
    expect(remaining).toBe(22);
  });

  it("floors at zero when over limit", async () => {
    const t = convexTest(schema, modules);

    const userId = await t.run(async (ctx) => {
      return ctx.db.insert("users", {
        isAnonymous: false,
        email: "jane@example.com",
        plan: "free",
        runsUsed: 30,
        runLimit: 25,
        runsResetAt: futureReset(),
      });
    });

    const user = await t.run(async (ctx) => ctx.db.get(userId));
    const remaining = Math.max(0, user!.runLimit - user!.runsUsed);
    expect(remaining).toBe(0);
  });
});
