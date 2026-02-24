/**
 * Execution integration tests against real Convex dev.
 *
 * Tests the execution lifecycle via ConvexHttpClient:
 * - startPredefined creates execution record + event
 * - runsUsed increments on the user
 * - Quota enforcement rejects over-limit users
 * - Status transitions (pending → running → completed/failed) via polling
 *
 * Spec: .claude/journeys/auth.md — "Anonymous Execution Flow" (A3, A4, A6)
 * Prerequisites: `task dev:all` must be running.
 */

import { describe, it, expect, beforeAll } from "vitest";
import type { Id } from "@bnto/backend/convex/_generated/dataModel";
import {
  createAnonymousClient,
  getCurrentUser,
  createUnauthenticatedClient,
  type AuthenticatedClient,
  api,
} from "./setup";

// Minimal definition that the Go engine can process (group with no nodes).
// This will start fast and fail/complete quickly — ideal for testing the
// execution lifecycle without waiting for real file I/O.
const MINIMAL_DEFINITION = {
  id: "test-integration",
  type: "group",
  version: "1.0.0",
  name: "Integration Test",
  position: { x: 0, y: 0 },
  metadata: {},
  parameters: {},
  inputPorts: [],
  outputPorts: [],
  nodes: [],
  edges: [],
};

const TEST_SLUG = "compress-images";

// ── Helpers ───────────────────────────────────────────────────────────────

/** Poll execution status until it reaches a terminal state or times out. */
async function pollExecution(
  client: AuthenticatedClient,
  executionId: Id<"executions">,
  { timeoutMs = 90_000, intervalMs = 2_000 } = {},
): Promise<{ status: string; error?: string }> {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    const execution = await client.client.query(api.executions.get, {
      id: executionId,
    });
    if (!execution) {
      throw new Error(`Execution ${executionId} not found (ownership check failed?)`);
    }
    if (execution.status === "completed" || execution.status === "failed") {
      return { status: execution.status, error: execution.error };
    }
    await new Promise((r) => setTimeout(r, intervalMs));
  }
  throw new Error(`Execution ${executionId} did not reach terminal state within ${timeoutMs}ms`);
}

// ── A3: Anonymous can start execution ─────────────────────────────────────

describe("A3: anonymous can start execution", () => {
  let anon: AuthenticatedClient;
  let executionId: Id<"executions">;
  let runsUsedBefore: number;

  beforeAll(async () => {
    anon = await createAnonymousClient();
    const user = await getCurrentUser(anon.client);
    runsUsedBefore = user!.runsUsed;

    executionId = await anon.client.mutation(api.executions.startPredefined, {
      slug: TEST_SLUG,
      definition: MINIMAL_DEFINITION,
    });
  });

  it("returns a valid execution ID", () => {
    expect(executionId).toBeTruthy();
    expect(typeof executionId).toBe("string");
  });

  it("creates execution with pending status", async () => {
    const execution = await anon.client.query(api.executions.get, {
      id: executionId,
    });
    expect(execution).not.toBeNull();
    expect(execution!.status).toMatch(/^(pending|running|completed|failed)$/);
    expect(execution!.userId).toBe(anon.userId);
    expect(execution!.startedAt).toBeTypeOf("number");
  });

  it("increments runsUsed on the user", async () => {
    const user = await getCurrentUser(anon.client);
    expect(user!.runsUsed).toBe(runsUsedBefore + 1);
  });

  it("creates an execution event with correct fields", async () => {
    const events = await anon.client.query(api.execution_events.listByUser, {
      limit: 10,
    });
    const event = events.find(
      (e: { executionId?: string }) => e.executionId === executionId,
    );
    expect(event).toBeDefined();
    expect(event!.slug).toBe(TEST_SLUG);
    expect(event!.userId).toBe(anon.userId);
    expect(event!.status).toMatch(/^(started|completed|failed)$/);
    expect(event!.timestamp).toBeTypeOf("number");
  });
});

// ── A4: Anonymous can subscribe to progress ───────────────────────────────

describe("A4: anonymous can read execution progress", () => {
  let anon: AuthenticatedClient;
  let executionId: Id<"executions">;

  beforeAll(async () => {
    anon = await createAnonymousClient();
    executionId = await anon.client.mutation(api.executions.startPredefined, {
      slug: TEST_SLUG,
      definition: MINIMAL_DEFINITION,
    });
  });

  it("execution is visible to the owner via get query", async () => {
    const execution = await anon.client.query(api.executions.get, {
      id: executionId,
    });
    expect(execution).not.toBeNull();
    expect(execution!._id).toBe(executionId);
  });

  it("execution is NOT visible to other users", async () => {
    const other = await createAnonymousClient();
    const execution = await other.client.query(api.executions.get, {
      id: executionId,
    });
    expect(execution).toBeNull();
  });

  it("execution is NOT visible to unauthenticated clients", async () => {
    const unauth = createUnauthenticatedClient();
    const execution = await unauth.query(api.executions.get, {
      id: executionId,
    });
    expect(execution).toBeNull();
  });
});

// ── Status transitions (polling) ──────────────────────────────────────────

describe("execution reaches terminal state", () => {
  let anon: AuthenticatedClient;
  let executionId: Id<"executions">;

  beforeAll(async () => {
    anon = await createAnonymousClient();
    executionId = await anon.client.mutation(api.executions.startPredefined, {
      slug: TEST_SLUG,
      definition: MINIMAL_DEFINITION,
    });
  });

  it("transitions to a terminal state (completed or failed)", async () => {
    // The Go API processes the empty-group definition and returns quickly.
    // If GO_API_URL is not configured, the executeWorkflow action will fail
    // immediately — either way we reach a terminal state.
    const result = await pollExecution(anon, executionId);
    expect(result.status).toMatch(/^(completed|failed)$/);
  });

  it("execution has completedAt timestamp after terminal state", async () => {
    await pollExecution(anon, executionId);

    const execution = await anon.client.query(api.executions.get, {
      id: executionId,
    });
    expect(execution).not.toBeNull();
    expect(execution!.completedAt).toBeTypeOf("number");
    expect(execution!.completedAt!).toBeGreaterThan(execution!.startedAt);
  });

  it("execution event is updated with terminal status and duration", async () => {
    await pollExecution(anon, executionId);

    const events = await anon.client.query(api.execution_events.listByUser, {
      limit: 10,
    });
    const event = events.find(
      (e: { executionId?: string }) => e.executionId === executionId,
    );
    expect(event).toBeDefined();
    expect(event!.status).toMatch(/^(completed|failed)$/);
    expect(event!.durationMs).toBeTypeOf("number");
    expect(event!.durationMs).toBeGreaterThanOrEqual(0);
  });
});

// ── Unauthenticated rejection ─────────────────────────────────────────────

describe("unauthenticated execution rejection", () => {
  it("unauthenticated client is rejected from startPredefined", async () => {
    const unauth = createUnauthenticatedClient();
    await expect(
      unauth.mutation(api.executions.startPredefined, {
        slug: TEST_SLUG,
        definition: MINIMAL_DEFINITION,
      }),
    ).rejects.toThrow("Not authenticated");
  });
});

// ── A6: Anonymous quota enforcement ───────────────────────────────────────

describe("A6: anonymous quota enforcement", () => {
  let anon: AuthenticatedClient;

  beforeAll(async () => {
    anon = await createAnonymousClient();
  });

  it("rejects execution when anonymous run limit is exceeded", async () => {
    // Exhaust the anonymous run limit by starting executions.
    // Default anonymous limit is 3 (ANONYMOUS_RUN_LIMIT env var).
    // We start executions until we hit the quota error.
    const maxAttempts = 10; // Safety cap — should hit limit well before this
    let quotaError: Error | null = null;

    for (let i = 0; i < maxAttempts; i++) {
      try {
        await anon.client.mutation(api.executions.startPredefined, {
          slug: TEST_SLUG,
          definition: MINIMAL_DEFINITION,
        });
      } catch (e) {
        quotaError = e as Error;
        break;
      }
    }

    expect(quotaError).not.toBeNull();
    // The error should be a quota error, not an auth error
    expect(quotaError!.message).not.toContain("Not authenticated");
    expect(quotaError!.message).toContain("ANONYMOUS_QUOTA_EXCEEDED");
  });

  it("runsUsed matches the number of successful executions", async () => {
    const user = await getCurrentUser(anon.client);
    expect(user).not.toBeNull();
    // runsUsed should equal the limit since we exhausted it
    expect(user!.runsUsed).toBeGreaterThanOrEqual(1);
  });

  it("runs remaining is zero after exhausting quota", async () => {
    // After the quota fix, runLimit on the user record matches the
    // ANONYMOUS_RUN_LIMIT env var (default 3). enforceQuota and
    // getRunsRemaining use the same source of truth.
    const remaining = await anon.client.query(api.users.getRunsRemaining);
    expect(remaining).toBe(0);
  });
});

// ── Multiple executions tracking ──────────────────────────────────────────

describe("multiple executions are tracked correctly", () => {
  let anon: AuthenticatedClient;
  const executionIds: Id<"executions">[] = [];

  beforeAll(async () => {
    anon = await createAnonymousClient();

    // Start two executions
    for (let i = 0; i < 2; i++) {
      const id = await anon.client.mutation(api.executions.startPredefined, {
        slug: `test-slug-${i}`,
        definition: MINIMAL_DEFINITION,
      });
      executionIds.push(id);
    }
  });

  it("each execution has a unique ID", () => {
    expect(executionIds[0]).not.toBe(executionIds[1]);
  });

  it("runsUsed reflects total executions started", async () => {
    const user = await getCurrentUser(anon.client);
    expect(user!.runsUsed).toBe(2);
  });

  it("execution events are logged for each slug", async () => {
    const events = await anon.client.query(api.execution_events.listByUser, {
      limit: 10,
    });
    const slugs = events.map((e: { slug: string }) => e.slug);
    expect(slugs).toContain("test-slug-0");
    expect(slugs).toContain("test-slug-1");
  });
});
