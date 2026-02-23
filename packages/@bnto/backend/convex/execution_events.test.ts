import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { internal, api } from "./_generated/api";

const modules = import.meta.glob("./**/*.ts");

describe("executionEvents", () => {
  describe("logComplete", () => {
    it("sets status to completed and records durationMs", async () => {
      const t = convexTest(schema, modules);

      // Insert a test event directly
      const eventId = await t.run(async (ctx) => {
        return ctx.db.insert("executionEvents", {
          slug: "compress-images",
          timestamp: Date.now(),
          status: "started",
          fingerprint: "test-fingerprint",
        });
      });

      await t.mutation(internal.execution_events.logComplete, {
        eventId,
        durationMs: 1500,
      });

      // Verify the event was updated
      const event = await t.run(async (ctx) => {
        return ctx.db.get(eventId);
      });

      expect(event).not.toBeNull();
      expect(event!.status).toBe("completed");
      expect(event!.durationMs).toBe(1500);
    });

    it("does nothing if event does not exist", async () => {
      const t = convexTest(schema, modules);

      // Insert and delete to get a valid-format ID that doesn't exist
      const eventId = await t.run(async (ctx) => {
        const id = await ctx.db.insert("executionEvents", {
          slug: "test",
          timestamp: Date.now(),
          status: "started",
        });
        await ctx.db.delete(id);
        return id;
      });

      // Should not throw
      await t.mutation(internal.execution_events.logComplete, {
        eventId,
        durationMs: 1000,
      });
    });
  });

  describe("logFail", () => {
    it("sets status to failed and records durationMs", async () => {
      const t = convexTest(schema, modules);

      const eventId = await t.run(async (ctx) => {
        return ctx.db.insert("executionEvents", {
          slug: "clean-csv",
          timestamp: Date.now(),
          status: "started",
          fingerprint: "fp-abc",
        });
      });

      await t.mutation(internal.execution_events.logFail, {
        eventId,
        durationMs: 500,
      });

      const event = await t.run(async (ctx) => {
        return ctx.db.get(eventId);
      });

      expect(event).not.toBeNull();
      expect(event!.status).toBe("failed");
      expect(event!.durationMs).toBe(500);
    });
  });

  describe("listByFingerprint", () => {
    it("returns events for a given fingerprint, most recent first", async () => {
      const t = convexTest(schema, modules);

      // Insert events with different fingerprints
      await t.run(async (ctx) => {
        await ctx.db.insert("executionEvents", {
          slug: "compress-images",
          timestamp: 1000,
          status: "completed",
          fingerprint: "fp-1",
          durationMs: 200,
        });
        await ctx.db.insert("executionEvents", {
          slug: "resize-images",
          timestamp: 2000,
          status: "started",
          fingerprint: "fp-1",
        });
        await ctx.db.insert("executionEvents", {
          slug: "clean-csv",
          timestamp: 3000,
          status: "completed",
          fingerprint: "fp-other",
          durationMs: 100,
        });
      });

      const events = await t.query(api.execution_events.listByFingerprint, {
        fingerprint: "fp-1",
      });

      expect(events).toHaveLength(2);
      // Ordered desc by creation time
      expect(events[0].slug).toBe("resize-images");
      expect(events[1].slug).toBe("compress-images");
    });

    it("respects the limit parameter", async () => {
      const t = convexTest(schema, modules);

      await t.run(async (ctx) => {
        for (let i = 0; i < 5; i++) {
          await ctx.db.insert("executionEvents", {
            slug: `slug-${i}`,
            timestamp: i * 1000,
            status: "completed",
            fingerprint: "fp-limit",
            durationMs: 100,
          });
        }
      });

      const events = await t.query(api.execution_events.listByFingerprint, {
        fingerprint: "fp-limit",
        limit: 2,
      });

      expect(events).toHaveLength(2);
    });

    it("returns empty array for unknown fingerprint", async () => {
      const t = convexTest(schema, modules);

      const events = await t.query(api.execution_events.listByFingerprint, {
        fingerprint: "nonexistent",
      });

      expect(events).toHaveLength(0);
    });
  });

  describe("countByFingerprint", () => {
    it("counts only events from the current month", async () => {
      const t = convexTest(schema, modules);

      const now = new Date();
      const thisMonth = now.getTime();
      // One year ago — clearly outside current month
      const lastYear = new Date(
        now.getFullYear() - 1,
        now.getMonth(),
        15,
      ).getTime();

      await t.run(async (ctx) => {
        await ctx.db.insert("executionEvents", {
          slug: "compress-images",
          timestamp: thisMonth,
          status: "completed",
          fingerprint: "fp-count",
          durationMs: 100,
        });
        await ctx.db.insert("executionEvents", {
          slug: "resize-images",
          timestamp: thisMonth - 1000,
          status: "completed",
          fingerprint: "fp-count",
          durationMs: 200,
        });
        // Old event — should not be counted
        await ctx.db.insert("executionEvents", {
          slug: "clean-csv",
          timestamp: lastYear,
          status: "completed",
          fingerprint: "fp-count",
          durationMs: 50,
        });
      });

      const count = await t.query(api.execution_events.countByFingerprint, {
        fingerprint: "fp-count",
      });

      expect(count).toBe(2);
    });

    it("returns 0 for fingerprint with no events", async () => {
      const t = convexTest(schema, modules);

      const count = await t.query(api.execution_events.countByFingerprint, {
        fingerprint: "no-events",
      });

      expect(count).toBe(0);
    });
  });
});
