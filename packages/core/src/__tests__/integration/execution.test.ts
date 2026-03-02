/**
 * Execution integration tests against real Convex dev.
 *
 * Tests the execution lifecycle via ConvexHttpClient:
 * - startPredefined creates execution record + event
 * - Status transitions (pending → running → completed/failed) via polling
 *
 * Prerequisites: `task dev:all` must be running.
 */

import { describe, it, expect, beforeAll } from "vitest";
import type { Id } from "@bnto/backend/convex/_generated/dataModel";
import {
  createPasswordClient,
  generateTestEmail,
  createUnauthenticatedClient,
  type AuthenticatedClient,
  api,
} from "./setup";
import { pollExecution } from "./transit-helpers";

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

// ── Execution start ─────────────────────────────────────────────────────

describe("authenticated user can start execution", () => {
  let user: AuthenticatedClient;
  let executionId: Id<"executions">;

  beforeAll(async () => {
    user = await createPasswordClient(generateTestEmail(), "test-exec-123", {
      flow: "signUp",
    });

    executionId = await user.client.mutation(api.executions.startPredefined, {
      slug: TEST_SLUG,
      definition: MINIMAL_DEFINITION,
    });
  });

  it("returns a valid execution ID", () => {
    expect(executionId).toBeTruthy();
    expect(typeof executionId).toBe("string");
  });

  it("creates execution with pending status", async () => {
    const execution = await user.client.query(api.executions.get, {
      id: executionId,
    });
    expect(execution).not.toBeNull();
    expect(execution!.status).toMatch(/^(pending|running|completed|failed)$/);
    expect(execution!.userId).toBe(user.userId);
    expect(execution!.startedAt).toBeTypeOf("number");
  });

  it("creates an execution event with correct fields", async () => {
    const events = await user.client.query(api.execution_events.listByUser, {
      limit: 10,
    });
    const event = events.find(
      (e: { executionId?: string }) => e.executionId === executionId,
    );
    expect(event).toBeDefined();
    expect(event!.slug).toBe(TEST_SLUG);
    expect(event!.userId).toBe(user.userId);
    expect(event!.status).toMatch(/^(started|completed|failed)$/);
    expect(event!.timestamp).toBeTypeOf("number");
  });
});

// ── Execution visibility ────────────────────────────────────────────────

describe("execution visibility and access control", () => {
  let user: AuthenticatedClient;
  let executionId: Id<"executions">;

  beforeAll(async () => {
    user = await createPasswordClient(generateTestEmail(), "test-vis-123", {
      flow: "signUp",
    });
    executionId = await user.client.mutation(api.executions.startPredefined, {
      slug: TEST_SLUG,
      definition: MINIMAL_DEFINITION,
    });
  });

  it("execution is visible to the owner via get query", async () => {
    const execution = await user.client.query(api.executions.get, {
      id: executionId,
    });
    expect(execution).not.toBeNull();
    expect(execution!._id).toBe(executionId);
  });

  it("execution is NOT visible to other users", async () => {
    const other = await createPasswordClient(generateTestEmail(), "other-123", {
      flow: "signUp",
    });
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

// ── Status transitions (polling) ────────────────────────────────────────

describe("execution reaches terminal state", () => {
  let user: AuthenticatedClient;
  let executionId: Id<"executions">;

  beforeAll(async () => {
    user = await createPasswordClient(generateTestEmail(), "test-poll-123", {
      flow: "signUp",
    });
    executionId = await user.client.mutation(api.executions.startPredefined, {
      slug: TEST_SLUG,
      definition: MINIMAL_DEFINITION,
    });
  });

  it("transitions to a terminal state (completed or failed)", async () => {
    const result = await pollExecution(user, executionId);
    expect(result.status).toMatch(/^(completed|failed)$/);
  });

  it("execution has completedAt timestamp after terminal state", async () => {
    await pollExecution(user, executionId);

    const execution = await user.client.query(api.executions.get, {
      id: executionId,
    });
    expect(execution).not.toBeNull();
    expect(execution!.completedAt).toBeTypeOf("number");
    expect(execution!.completedAt!).toBeGreaterThan(execution!.startedAt);
  });

  it("execution event is updated with terminal status and duration", async () => {
    await pollExecution(user, executionId);

    const events = await user.client.query(api.execution_events.listByUser, {
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

// ── Unauthenticated rejection ───────────────────────────────────────────

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

// ── Multiple executions tracking ────────────────────────────────────────

describe("multiple executions are tracked correctly", () => {
  let user: AuthenticatedClient;
  const executionIds: Id<"executions">[] = [];

  beforeAll(async () => {
    user = await createPasswordClient(generateTestEmail(), "test-multi-123", {
      flow: "signUp",
    });

    // Start two executions
    for (let i = 0; i < 2; i++) {
      const id = await user.client.mutation(api.executions.startPredefined, {
        slug: `test-slug-${i}`,
        definition: MINIMAL_DEFINITION,
      });
      executionIds.push(id);
    }
  });

  it("each execution has a unique ID", () => {
    expect(executionIds[0]).not.toBe(executionIds[1]);
  });

  it("execution events are logged for each slug", async () => {
    const events = await user.client.query(api.execution_events.listByUser, {
      limit: 10,
    });
    const slugs = events.map((e: { slug: string }) => e.slug);
    expect(slugs).toContain("test-slug-0");
    expect(slugs).toContain("test-slug-1");
  });
});
