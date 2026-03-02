import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { api, internal } from "./_generated/api";

const modules = import.meta.glob("./**/*.ts");

/** Create a minimal user + workflow for execution tests. */
async function seedUserAndWorkflow(t: ReturnType<typeof convexTest>) {
  return t.run(async (ctx) => {
    const userId = await ctx.db.insert("users", {
      email: "test@example.com",
      plan: "free",
      totalRuns: 0,
    });
    const workflowId = await ctx.db.insert("workflows", {
      userId,
      name: "compress-images",
      definition: { type: "group", nodes: [] },
      version: 1,
      isPublic: false,
      createdAt: Date.now(),
      updatedAt: Date.now(),
    });
    return { userId, workflowId };
  });
}

describe("executions", () => {
  describe("complete", () => {
    it("stores outputFiles when provided", async () => {
      const t = convexTest(schema, modules);
      const { userId, workflowId } = await seedUserAndWorkflow(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          workflowId,
          status: "running",
          progress: [],
          startedAt: Date.now(),
        });
      });

      const outputFiles = [
        {
          key: "executions/abc/output/compressed.png",
          name: "compressed.png",
          sizeBytes: 12345,
          contentType: "image/png",
        },
      ];

      await t.mutation(internal.executions.complete, {
        executionId,
        result: { success: true },
        outputFiles,
        startTime: Date.now() - 1000,
      });

      const execution = await t.run(async (ctx) => {
        return ctx.db.get(executionId);
      });

      expect(execution).not.toBeNull();
      expect(execution!.status).toBe("completed");
      expect(execution!.outputFiles).toEqual(outputFiles);
      expect(execution!.completedAt).toBeDefined();
    });

    it("completes without outputFiles", async () => {
      const t = convexTest(schema, modules);
      const { userId, workflowId } = await seedUserAndWorkflow(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          workflowId,
          status: "running",
          progress: [],
          startedAt: Date.now(),
        });
      });

      await t.mutation(internal.executions.complete, {
        executionId,
        result: { success: true },
      });

      const execution = await t.run(async (ctx) => {
        return ctx.db.get(executionId);
      });

      expect(execution!.status).toBe("completed");
      expect(execution!.outputFiles).toBeUndefined();
    });

    it("updates linked execution event with duration", async () => {
      const t = convexTest(schema, modules);
      const { userId, workflowId } = await seedUserAndWorkflow(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          workflowId,
          status: "running",
          progress: [],
          startedAt: Date.now(),
        });
      });

      const eventId = await t.run(async (ctx) => {
        return ctx.db.insert("executionEvents", {
          userId,
          slug: "compress-images",
          timestamp: Date.now(),
          status: "started",
          executionId,
        });
      });

      const startTime = Date.now() - 2000;
      await t.mutation(internal.executions.complete, {
        executionId,
        result: null,
        eventId,
        startTime,
      });

      const event = await t.run(async (ctx) => {
        return ctx.db.get(eventId);
      });

      expect(event!.status).toBe("completed");
      expect(event!.durationMs).toBeGreaterThanOrEqual(0);
    });
  });

  describe("fail", () => {
    it("stores error message and updates event", async () => {
      const t = convexTest(schema, modules);
      const { userId, workflowId } = await seedUserAndWorkflow(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          workflowId,
          status: "running",
          progress: [],
          startedAt: Date.now(),
        });
      });

      const eventId = await t.run(async (ctx) => {
        return ctx.db.insert("executionEvents", {
          userId,
          slug: "compress-images",
          timestamp: Date.now(),
          status: "started",
          executionId,
        });
      });

      await t.mutation(internal.executions.fail, {
        executionId,
        error: "Node image-resize failed: out of memory",
        eventId,
        startTime: Date.now() - 500,
      });

      const execution = await t.run(async (ctx) => {
        return ctx.db.get(executionId);
      });

      expect(execution!.status).toBe("failed");
      expect(execution!.error).toBe(
        "Node image-resize failed: out of memory",
      );
      expect(execution!.completedAt).toBeDefined();

      const event = await t.run(async (ctx) => {
        return ctx.db.get(eventId);
      });
      expect(event!.status).toBe("failed");
    });
  });

  describe("updateProgress", () => {
    it("updates status and progress array", async () => {
      const t = convexTest(schema, modules);
      const { userId, workflowId } = await seedUserAndWorkflow(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          workflowId,
          status: "pending",
          progress: [],
          startedAt: Date.now(),
        });
      });

      const progress = [
        { nodeId: "resize-1", status: "completed" },
        { nodeId: "compress-1", status: "running" },
      ];

      await t.mutation(internal.executions.updateProgress, {
        executionId,
        status: "running",
        progress,
        goExecutionId: "go-exec-123",
      });

      const execution = await t.run(async (ctx) => {
        return ctx.db.get(executionId);
      });

      expect(execution!.status).toBe("running");
      expect(execution!.progress).toEqual(progress);
      expect(execution!.goExecutionId).toBe("go-exec-123");
    });
  });

  describe("sessionId support", () => {
    it("stores sessionId on execution when provided via complete flow", async () => {
      const t = convexTest(schema, modules);
      const { userId, workflowId } = await seedUserAndWorkflow(t);

      // Simulate what start mutation does with sessionId
      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          workflowId,
          status: "pending",
          progress: [],
          sessionId: "session-abc-123",
          startedAt: Date.now(),
        });
      });

      const execution = await t.run(async (ctx) => {
        return ctx.db.get(executionId);
      });

      expect(execution!.sessionId).toBe("session-abc-123");
    });

    it("allows execution without sessionId", async () => {
      const t = convexTest(schema, modules);
      const { userId, workflowId } = await seedUserAndWorkflow(t);

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          workflowId,
          status: "pending",
          progress: [],
          startedAt: Date.now(),
        });
      });

      const execution = await t.run(async (ctx) => {
        return ctx.db.get(executionId);
      });

      expect(execution!.sessionId).toBeUndefined();
    });

    it("outputFiles schema accepts valid file metadata", async () => {
      const t = convexTest(schema, modules);
      const { userId, workflowId } = await seedUserAndWorkflow(t);

      const outputFiles = [
        {
          key: "executions/exec-1/output/result.png",
          name: "result.png",
          sizeBytes: 54321,
          contentType: "image/png",
        },
        {
          key: "executions/exec-1/output/result-2x.png",
          name: "result-2x.png",
          sizeBytes: 98765,
          contentType: "image/png",
        },
      ];

      const executionId = await t.run(async (ctx) => {
        return ctx.db.insert("executions", {
          userId,
          workflowId,
          status: "completed",
          progress: [],
          outputFiles,
          startedAt: Date.now(),
          completedAt: Date.now(),
        });
      });

      const execution = await t.run(async (ctx) => {
        return ctx.db.get(executionId);
      });

      expect(execution!.outputFiles).toEqual(outputFiles);
      expect(execution!.outputFiles).toHaveLength(2);
    });
  });

  // NOTE: R2 cleanup scheduling (via ctx.scheduler.runAfter → cleanup.deleteByPrefix)
  // cannot be tested in convex-test because deleteByPrefix is a "use node" action
  // that requires external R2 access. Scheduling correctness is verified by:
  // 1. Code review of scheduleR2Cleanup helper
  // 2. Go-side cleanup tests (archive/api-go/internal/r2/cleanup_test.go)
  // 3. Pre-existing complete/fail tests pass without sessionId (no scheduling triggered)

  describe("listByUser", () => {
    it("returns empty result for unauthenticated user", async () => {
      const t = convexTest(schema, modules);

      const result = await t.query(api.executions.listByUser, {
        paginationOpts: { numItems: 10, cursor: null },
      });

      expect(result.page).toEqual([]);
      expect(result.isDone).toBe(true);
    });

    it("returns executions for the authenticated user in desc order", async () => {
      const t = convexTest(schema, modules);
      const { userId, workflowId } = await seedUserAndWorkflow(t);

      // Insert 3 executions in chronological order.
      // Convex orders by _creationTime (insertion order), so desc = newest first.
      await t.run(async (ctx) => {
        await ctx.db.insert("executions", {
          userId, workflowId, status: "completed", progress: [], startedAt: 1000, completedAt: 1500,
        });
        await ctx.db.insert("executions", {
          userId, workflowId, status: "failed", progress: [], startedAt: 2000, completedAt: 2200,
        });
        await ctx.db.insert("executions", {
          userId, workflowId, status: "completed", progress: [], startedAt: 3000, completedAt: 3800,
        });
      });

      const result = await t.withIdentity({ subject: userId }).query(
        api.executions.listByUser,
        { paginationOpts: { numItems: 10, cursor: null } },
      );

      expect(result.page).toHaveLength(3);
      // Desc order by _creationTime: most recently inserted first
      expect(result.page[0].startedAt).toBe(3000);
      expect(result.page[1].startedAt).toBe(2000);
      expect(result.page[2].startedAt).toBe(1000);
    });

    it("excludes executions from other users", async () => {
      const t = convexTest(schema, modules);
      const { userId, workflowId } = await seedUserAndWorkflow(t);

      const otherUserId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          email: "other@example.com",
          plan: "free",
          totalRuns: 0,
        });
      });

      await t.run(async (ctx) => {
        await ctx.db.insert("executions", {
          userId, workflowId, status: "completed", progress: [], startedAt: 1000,
        });
        await ctx.db.insert("executions", {
          userId: otherUserId, workflowId, status: "completed", progress: [], startedAt: 2000,
        });
      });

      const result = await t.withIdentity({ subject: userId }).query(
        api.executions.listByUser,
        { paginationOpts: { numItems: 10, cursor: null } },
      );

      expect(result.page).toHaveLength(1);
      expect(result.page[0].userId).toEqual(userId);
    });

    it("paginates results correctly", async () => {
      const t = convexTest(schema, modules);
      const { userId, workflowId } = await seedUserAndWorkflow(t);

      // Insert 5 executions
      await t.run(async (ctx) => {
        for (let i = 1; i <= 5; i++) {
          await ctx.db.insert("executions", {
            userId, workflowId, status: "completed", progress: [], startedAt: i * 1000,
          });
        }
      });

      // First page: 2 items
      const page1 = await t.withIdentity({ subject: userId }).query(
        api.executions.listByUser,
        { paginationOpts: { numItems: 2, cursor: null } },
      );

      expect(page1.page).toHaveLength(2);
      expect(page1.isDone).toBe(false);

      // Second page: next 2 items
      const page2 = await t.withIdentity({ subject: userId }).query(
        api.executions.listByUser,
        { paginationOpts: { numItems: 2, cursor: page1.continueCursor } },
      );

      expect(page2.page).toHaveLength(2);
      expect(page2.isDone).toBe(false);

      // Third page: last 1 item
      const page3 = await t.withIdentity({ subject: userId }).query(
        api.executions.listByUser,
        { paginationOpts: { numItems: 2, cursor: page2.continueCursor } },
      );

      expect(page3.page).toHaveLength(1);
      expect(page3.isDone).toBe(true);
    });
  });
});
