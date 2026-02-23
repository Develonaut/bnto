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
        fingerprint: "fp-full",
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
    expect(event!.fingerprint).toBe("fp-full");
  });

  it("allows events with only userId (no fingerprint)", async () => {
    const t = convexTest(schema, modules);

    const userId = await t.run(async (ctx) => {
      return ctx.db.insert("users", {
        userId: "auth-user-123",
        email: "test@example.com",
        isAnonymous: false,
        plan: "free",
        runsUsed: 0,
        runLimit: 5,
        runsResetAt: Date.now() + 30 * 24 * 60 * 60 * 1000,
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
    expect(event!.fingerprint).toBeUndefined();
  });

  it("allows events with only fingerprint (no userId)", async () => {
    const t = convexTest(schema, modules);

    const eventId = await t.run(async (ctx) => {
      return ctx.db.insert("executionEvents", {
        fingerprint: "anon-fp",
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
    expect(event!.fingerprint).toBe("anon-fp");
  });
});
