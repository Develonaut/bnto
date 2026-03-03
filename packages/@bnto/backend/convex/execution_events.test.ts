import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { internal } from "./_generated/api";

const modules = import.meta.glob("./**/*.ts");

/** Create a user for test events. */
async function seedUser(t: ReturnType<typeof convexTest>) {
  return t.run(async (ctx) => {
    return ctx.db.insert("users", {
      email: "test@example.com",
      plan: "free",
      totalRuns: 0,
    });
  });
}

describe("executionEvents", () => {
  describe("logComplete", () => {
    it("sets status to completed and records durationMs", async () => {
      const t = convexTest(schema, modules);
      const userId = await seedUser(t);

      // Insert a test event directly
      const eventId = await t.run(async (ctx) => {
        return ctx.db.insert("executionEvents", {
          userId,
          slug: "compress-images",
          timestamp: Date.now(),
          status: "started",
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
      const userId = await seedUser(t);

      // Insert and delete to get a valid-format ID that doesn't exist
      const eventId = await t.run(async (ctx) => {
        const id = await ctx.db.insert("executionEvents", {
          userId,
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
      const userId = await seedUser(t);

      const eventId = await t.run(async (ctx) => {
        return ctx.db.insert("executionEvents", {
          userId,
          slug: "clean-csv",
          timestamp: Date.now(),
          status: "started",
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
});
