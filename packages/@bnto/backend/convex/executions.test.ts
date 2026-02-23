import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { internal } from "./_generated/api";

const modules = import.meta.glob("./**/*.ts");

/** Create a minimal user + workflow for execution tests. */
async function seedUserAndWorkflow(t: ReturnType<typeof convexTest>) {
  return t.run(async (ctx) => {
    const userId = await ctx.db.insert("users", {
      userId: "auth-user-1",
      email: "test@example.com",
      isAnonymous: false,
      plan: "free",
      runsUsed: 0,
      runLimit: 25,
      runsResetAt: Date.now() + 30 * 24 * 60 * 60 * 1000,
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
});
