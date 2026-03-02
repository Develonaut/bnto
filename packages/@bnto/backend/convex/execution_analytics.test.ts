import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { api } from "./_generated/api";
import type { Id } from "./_generated/dataModel";

const modules = import.meta.glob("./**/*.ts");

/** Create a user and return their ID. */
async function seedUser(t: ReturnType<typeof convexTest>) {
  return t.run(async (ctx) => {
    return ctx.db.insert("users", {
      email: "jane@example.com",
      plan: "free",
      totalRuns: 0,
    });
  });
}

/** Insert an execution event with sensible defaults. */
async function seedEvent(
  t: ReturnType<typeof convexTest>,
  overrides: {
    userId?: Id<"users">;
    slug?: string;
    timestamp?: number;
    status?: "started" | "completed" | "failed";
    durationMs?: number;
  },
) {
  return t.run(async (ctx) => {
    return ctx.db.insert("executionEvents", {
      userId: overrides.userId,
      slug: overrides.slug ?? "compress-images",
      timestamp: overrides.timestamp ?? Date.now(),
      status: overrides.status ?? "completed",
      durationMs: overrides.durationMs,
    });
  });
}

describe("execution_analytics", () => {
  describe("aggregateBySlug", () => {
    it("returns empty array for unauthenticated user", async () => {
      const t = convexTest(schema, modules);

      const result = await t.query(api.execution_analytics.aggregateBySlug, {});
      expect(result).toEqual([]);
    });

    it("returns empty array for user with no events", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.aggregateBySlug,
        {},
      );
      expect(result).toEqual([]);
    });

    it("groups events by slug with correct counts", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      // 3 compress-images, 2 clean-csv, 1 rename-files
      await seedEvent(t, { userId, slug: "compress-images", status: "completed", durationMs: 100 });
      await seedEvent(t, { userId, slug: "compress-images", status: "completed", durationMs: 200 });
      await seedEvent(t, { userId, slug: "compress-images", status: "failed", durationMs: 50 });
      await seedEvent(t, { userId, slug: "clean-csv", status: "completed", durationMs: 300 });
      await seedEvent(t, { userId, slug: "clean-csv", status: "completed", durationMs: 500 });
      await seedEvent(t, { userId, slug: "rename-files", status: "completed", durationMs: 80 });

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.aggregateBySlug,
        {},
      );

      expect(result).toHaveLength(3);

      // Sorted by totalRuns desc
      expect(result[0].slug).toBe("compress-images");
      expect(result[0].totalRuns).toBe(3);
      expect(result[0].completedRuns).toBe(2);
      expect(result[0].failedRuns).toBe(1);

      expect(result[1].slug).toBe("clean-csv");
      expect(result[1].totalRuns).toBe(2);
      expect(result[1].completedRuns).toBe(2);
      expect(result[1].failedRuns).toBe(0);

      expect(result[2].slug).toBe("rename-files");
      expect(result[2].totalRuns).toBe(1);
    });

    it("computes average duration correctly (rounded)", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      await seedEvent(t, { userId, slug: "compress-images", status: "completed", durationMs: 100 });
      await seedEvent(t, { userId, slug: "compress-images", status: "completed", durationMs: 200 });
      await seedEvent(t, { userId, slug: "compress-images", status: "completed", durationMs: 300 });

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.aggregateBySlug,
        {},
      );

      expect(result[0].avgDurationMs).toBe(200); // (100 + 200 + 300) / 3
    });

    it("returns null avgDurationMs when no events have duration", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      // Events with status "started" typically have no durationMs
      await seedEvent(t, { userId, slug: "compress-images", status: "started" });
      await seedEvent(t, { userId, slug: "compress-images", status: "started" });

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.aggregateBySlug,
        {},
      );

      expect(result[0].avgDurationMs).toBeNull();
    });

    it("tracks lastRunAt as the most recent timestamp per slug", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      await seedEvent(t, { userId, slug: "compress-images", timestamp: 1000, status: "completed" });
      await seedEvent(t, { userId, slug: "compress-images", timestamp: 3000, status: "completed" });
      await seedEvent(t, { userId, slug: "compress-images", timestamp: 2000, status: "completed" });

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.aggregateBySlug,
        {},
      );

      expect(result[0].lastRunAt).toBe(3000);
    });

    it("excludes events from other users", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      const otherUserId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          email: "other@example.com",
          plan: "free",
          totalRuns: 0,
        });
      });

      await seedEvent(t, { userId, slug: "compress-images", status: "completed" });
      await seedEvent(t, { userId: otherUserId, slug: "compress-images", status: "completed" });
      await seedEvent(t, { userId: otherUserId, slug: "clean-csv", status: "completed" });

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.aggregateBySlug,
        {},
      );

      expect(result).toHaveLength(1);
      expect(result[0].slug).toBe("compress-images");
      expect(result[0].totalRuns).toBe(1);
    });
  });

  describe("listByDateRange", () => {
    it("returns empty array for unauthenticated user", async () => {
      const t = convexTest(schema, modules);

      const result = await t.query(api.execution_analytics.listByDateRange, {
        from: 0,
        to: Date.now(),
      });
      expect(result).toEqual([]);
    });

    it("returns events within the specified date range", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      // Events at timestamps 1000, 2000, 3000, 4000, 5000
      for (let i = 1; i <= 5; i++) {
        await seedEvent(t, {
          userId,
          slug: `slug-${i}`,
          timestamp: i * 1000,
          status: "completed",
        });
      }

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.listByDateRange,
        { from: 2000, to: 4000 },
      );

      expect(result).toHaveLength(3);
      // Ordered desc
      expect(result[0].timestamp).toBe(4000);
      expect(result[1].timestamp).toBe(3000);
      expect(result[2].timestamp).toBe(2000);
    });

    it("returns empty array when no events fall in range", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      await seedEvent(t, { userId, slug: "compress-images", timestamp: 1000, status: "completed" });

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.listByDateRange,
        { from: 5000, to: 10000 },
      );

      expect(result).toEqual([]);
    });

    it("respects the limit parameter", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      for (let i = 1; i <= 10; i++) {
        await seedEvent(t, {
          userId,
          slug: "compress-images",
          timestamp: i * 1000,
          status: "completed",
        });
      }

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.listByDateRange,
        { from: 0, to: 20000, limit: 3 },
      );

      expect(result).toHaveLength(3);
    });

    it("defaults to limit of 100", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      // Insert 5 events — all should be returned with default limit
      for (let i = 1; i <= 5; i++) {
        await seedEvent(t, {
          userId,
          slug: "compress-images",
          timestamp: i * 1000,
          status: "completed",
        });
      }

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.listByDateRange,
        { from: 0, to: 20000 },
      );

      expect(result).toHaveLength(5);
    });

    it("excludes events from other users", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      const otherUserId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          email: "other@example.com",
          plan: "free",
          totalRuns: 0,
        });
      });

      await seedEvent(t, { userId, slug: "compress-images", timestamp: 1000, status: "completed" });
      await seedEvent(t, { userId: otherUserId, slug: "clean-csv", timestamp: 2000, status: "completed" });

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.listByDateRange,
        { from: 0, to: 10000 },
      );

      expect(result).toHaveLength(1);
      expect(result[0].slug).toBe("compress-images");
    });
  });

  describe("summaryByDateRange", () => {
    it("returns null for unauthenticated user", async () => {
      const t = convexTest(schema, modules);

      const result = await t.query(api.execution_analytics.summaryByDateRange, {
        from: 0,
        to: Date.now(),
      });
      expect(result).toBeNull();
    });

    it("returns zeroed stats when no events exist in range", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.summaryByDateRange,
        { from: 0, to: Date.now() },
      );

      expect(result).toEqual({
        totalRuns: 0,
        completedRuns: 0,
        failedRuns: 0,
        avgDurationMs: null,
      });
    });

    it("computes correct aggregate stats for a date range", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      // 2 completed, 1 failed, 1 started — all within range [1000, 5000]
      await seedEvent(t, { userId, timestamp: 1000, status: "completed", durationMs: 100 });
      await seedEvent(t, { userId, timestamp: 2000, status: "completed", durationMs: 300 });
      await seedEvent(t, { userId, timestamp: 3000, status: "failed", durationMs: 50 });
      await seedEvent(t, { userId, timestamp: 4000, status: "started" });
      // Outside range — should not be counted
      await seedEvent(t, { userId, timestamp: 10000, status: "completed", durationMs: 999 });

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.summaryByDateRange,
        { from: 1000, to: 5000 },
      );

      expect(result).toEqual({
        totalRuns: 4,
        completedRuns: 2,
        failedRuns: 1,
        avgDurationMs: 150, // (100 + 300 + 50) / 3 = 150
      });
    });

    it("returns null avgDurationMs when no events have duration", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      await seedEvent(t, { userId, timestamp: 1000, status: "started" });
      await seedEvent(t, { userId, timestamp: 2000, status: "started" });

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.summaryByDateRange,
        { from: 0, to: 10000 },
      );

      expect(result!.avgDurationMs).toBeNull();
      expect(result!.totalRuns).toBe(2);
    });

    it("excludes events from other users", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      const otherUserId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          email: "other@example.com",
          plan: "free",
          totalRuns: 0,
        });
      });

      await seedEvent(t, { userId, timestamp: 1000, status: "completed", durationMs: 100 });
      await seedEvent(t, { userId: otherUserId, timestamp: 2000, status: "completed", durationMs: 200 });

      const result = await t.withIdentity({ subject: userId }).query(
        api.execution_analytics.summaryByDateRange,
        { from: 0, to: 10000 },
      );

      expect(result!.totalRuns).toBe(1);
      expect(result!.avgDurationMs).toBe(100);
    });
  });
});
