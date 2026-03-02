import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";

const modules = import.meta.glob("./**/*.ts");

describe("executionEvents schema integrity", () => {
  it("stores all expected fields on an execution event", async () => {
    const t = convexTest(schema, modules);

    const eventId = await t.run(async (ctx) => {
      return ctx.db.insert("executionEvents", {
        userId: undefined,
        slug: "compress-images",
        timestamp: 1000,
        durationMs: 500,
        status: "completed",
        executionId: undefined,
      });
    });

    const event = await t.run(async (ctx) => {
      return ctx.db.get(eventId);
    });

    expect(event).not.toBeNull();
    expect(event!.slug).toBe("compress-images");
    expect(event!.timestamp).toBe(1000);
    expect(event!.durationMs).toBe(500);
    expect(event!.status).toBe("completed");
  });

  it("allows events with userId", async () => {
    const t = convexTest(schema, modules);

    const userId = await t.run(async (ctx) => {
      return ctx.db.insert("users", {
        email: "test@example.com",
        plan: "free",
        totalRuns: 0,
      });
    });

    const eventId = await t.run(async (ctx) => {
      return ctx.db.insert("executionEvents", {
        userId,
        slug: "resize-images",
        timestamp: Date.now(),
        status: "started",
      });
    });

    const event = await t.run(async (ctx) => {
      return ctx.db.get(eventId);
    });

    expect(event).not.toBeNull();
    expect(event!.userId).toBe(userId);
  });

  it("allows events without userId (unauthenticated)", async () => {
    const t = convexTest(schema, modules);

    const eventId = await t.run(async (ctx) => {
      return ctx.db.insert("executionEvents", {
        slug: "clean-csv",
        timestamp: Date.now(),
        status: "started",
      });
    });

    const event = await t.run(async (ctx) => {
      return ctx.db.get(eventId);
    });

    expect(event).not.toBeNull();
    expect(event!.userId).toBeUndefined();
  });
});
