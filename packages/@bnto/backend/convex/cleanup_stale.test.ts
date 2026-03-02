import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { internal } from "./_generated/api";

const modules = import.meta.glob("./**/*.ts");

const THREE_HOURS_MS = 3 * 60 * 60 * 1000;

async function seedUser(t: ReturnType<typeof convexTest>) {
  return t.run(async (ctx) => {
    return ctx.db.insert("users", {
      email: "test@example.com",
      plan: "free",
      totalRuns: 0,
    });
  });
}

async function insertExecution(
  t: ReturnType<typeof convexTest>,
  userId: Awaited<ReturnType<typeof seedUser>>,
  overrides: {
    status: "pending" | "running" | "completed" | "failed";
    startedAt: number;
    sessionId?: string;
    completedAt?: number;
    error?: string;
  },
) {
  return t.run(async (ctx) => {
    return ctx.db.insert("executions", {
      userId,
      status: overrides.status,
      progress: [],
      startedAt: overrides.startedAt,
      sessionId: overrides.sessionId,
      completedAt: overrides.completedAt,
      error: overrides.error,
    });
  });
}

describe("markStaleAsFailed", () => {
  it("does not touch fresh pending executions", async () => {
    const t = convexTest(schema, modules);
    const userId = await seedUser(t);
    const execId = await insertExecution(t, userId, {
      status: "pending",
      startedAt: Date.now(),
    });

    const result = await t.mutation(
      internal.cleanup_stale.markStaleAsFailed,
      {},
    );

    expect(result).toEqual({ cleaned: [] });

    const execution = await t.run(async (ctx) => ctx.db.get(execId));
    expect(execution!.status).toBe("pending");
    expect(execution!.error).toBeUndefined();
  });

  it("marks old pending execution as failed", async () => {
    const t = convexTest(schema, modules);
    const userId = await seedUser(t);
    const execId = await insertExecution(t, userId, {
      status: "pending",
      startedAt: Date.now() - THREE_HOURS_MS,
      sessionId: "session-123",
    });

    const result = await t.mutation(
      internal.cleanup_stale.markStaleAsFailed,
      {},
    );

    expect(result.cleaned).toHaveLength(1);
    expect(result.cleaned[0]!.sessionId).toBe("session-123");

    const execution = await t.run(async (ctx) => ctx.db.get(execId));
    expect(execution!.status).toBe("failed");
    expect(execution!.error).toBe("Execution timed out (stale cleanup)");
    expect(execution!.completedAt).toBeDefined();
  });

  it("marks old running execution as failed", async () => {
    const t = convexTest(schema, modules);
    const userId = await seedUser(t);
    const execId = await insertExecution(t, userId, {
      status: "running",
      startedAt: Date.now() - THREE_HOURS_MS,
    });

    const result = await t.mutation(
      internal.cleanup_stale.markStaleAsFailed,
      {},
    );

    expect(result.cleaned).toHaveLength(1);

    const execution = await t.run(async (ctx) => ctx.db.get(execId));
    expect(execution!.status).toBe("failed");
    expect(execution!.error).toBe("Execution timed out (stale cleanup)");
  });

  it("does not touch completed executions regardless of age", async () => {
    const t = convexTest(schema, modules);
    const userId = await seedUser(t);
    await insertExecution(t, userId, {
      status: "completed",
      startedAt: Date.now() - THREE_HOURS_MS,
      completedAt: Date.now() - THREE_HOURS_MS + 5000,
    });

    const result = await t.mutation(
      internal.cleanup_stale.markStaleAsFailed,
      {},
    );

    expect(result).toEqual({ cleaned: [] });
  });

  it("does not touch failed executions regardless of age", async () => {
    const t = convexTest(schema, modules);
    const userId = await seedUser(t);
    const execId = await insertExecution(t, userId, {
      status: "failed",
      startedAt: Date.now() - THREE_HOURS_MS,
      completedAt: Date.now() - THREE_HOURS_MS + 5000,
      error: "Previous error",
    });

    const result = await t.mutation(
      internal.cleanup_stale.markStaleAsFailed,
      {},
    );

    expect(result).toEqual({ cleaned: [] });

    const execution = await t.run(async (ctx) => ctx.db.get(execId));
    expect(execution!.error).toBe("Previous error");
  });

  it("respects 100-per-run cap", async () => {
    const t = convexTest(schema, modules);
    const userId = await seedUser(t);

    const staleTime = Date.now() - THREE_HOURS_MS;
    for (let i = 0; i < 105; i++) {
      await insertExecution(t, userId, {
        status: "pending",
        startedAt: staleTime,
      });
    }

    const result = await t.mutation(
      internal.cleanup_stale.markStaleAsFailed,
      {},
    );

    expect(result.cleaned).toHaveLength(100);

    const remaining = await t.run(async (ctx) => {
      return ctx.db
        .query("executions")
        .filter((q) => q.eq(q.field("status"), "pending"))
        .collect();
    });
    expect(remaining).toHaveLength(5);
  });

  it("handles mix of fresh and stale executions", async () => {
    const t = convexTest(schema, modules);
    const userId = await seedUser(t);

    const freshId = await insertExecution(t, userId, {
      status: "running",
      startedAt: Date.now() - 60_000,
    });
    const staleId = await insertExecution(t, userId, {
      status: "running",
      startedAt: Date.now() - THREE_HOURS_MS,
    });
    const completedId = await insertExecution(t, userId, {
      status: "completed",
      startedAt: Date.now() - THREE_HOURS_MS,
      completedAt: Date.now() - THREE_HOURS_MS + 5000,
    });

    const result = await t.mutation(
      internal.cleanup_stale.markStaleAsFailed,
      {},
    );

    expect(result.cleaned).toHaveLength(1);

    const fresh = await t.run(async (ctx) => ctx.db.get(freshId));
    expect(fresh!.status).toBe("running");

    const stale = await t.run(async (ctx) => ctx.db.get(staleId));
    expect(stale!.status).toBe("failed");

    const completed = await t.run(async (ctx) => ctx.db.get(completedId));
    expect(completed!.status).toBe("completed");
  });

  // NOTE: R2 cleanup orchestration (cleanupStaleExecutions action calling
  // deleteByPrefix) cannot be tested in convex-test because deleteByPrefix
  // is a "use node" action requiring external R2 access. The action is a thin
  // orchestrator — correctness verified by code review + the mutation tests above.
});
