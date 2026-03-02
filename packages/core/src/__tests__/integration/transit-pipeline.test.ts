/**
 * Full R2 transit pipeline integration tests against real Convex dev + R2.
 *
 * Tests the complete file lifecycle:
 * - Upload → Execute (via Go API) → Download
 * - Download access control (ownership, auth)
 * - Download error cases (pending execution, no output files)
 *
 * This is the highest-priority integration gap — validates the full
 * browser → R2 → Go engine → R2 → browser pipeline.
 *
 * Prerequisites: `task dev:all` must be running (Convex dev + Go API + tunnel + R2 dev bucket).
 */

import { describe, it, expect, beforeAll } from "vitest";
import { readFileSync } from "fs";
import type { Id } from "@bnto/backend/convex/_generated/dataModel";
import {
  createPasswordClient,
  generateTestEmail,
  createUnauthenticatedClient,
  type AuthenticatedClient,
  api,
} from "./setup";
import {
  TEST_IMAGE_PATH,
  COMPRESS_IMAGES_DEFINITION,
  MINIMAL_DEFINITION,
  TEST_SLUG,
  pollExecution,
  uploadAndStartExecution,
} from "./transit-helpers";

// ── Full Transit Pipeline ──────────────────────────────────────────────

describe("full transit pipeline: upload → execute → download", () => {
  let user: AuthenticatedClient;
  let sessionId: string;
  let executionId: Id<"executions">;

  beforeAll(async () => {
    user = await createPasswordClient(generateTestEmail(), "test-transit-123", {
      flow: "signUp",
    });
    const result = await uploadAndStartExecution(user);
    executionId = result.executionId;
    sessionId = result.sessionId;
  }, 30_000);

  it("execution is created with sessionId", async () => {
    const execution = await user.client.query(api.executions.get, {
      id: executionId,
    });
    expect(execution).not.toBeNull();
    expect(execution!.sessionId).toBe(sessionId);
  });

  it("execution completes via Go API transit flow", async () => {
    const result = await pollExecution(user, executionId);
    expect(result.status).toBe("completed");
    expect(result.error).toBeUndefined();
  });

  it("completed execution has output files", async () => {
    await pollExecution(user, executionId);

    const execution = await user.client.query(api.executions.get, {
      id: executionId,
    });
    expect(execution!.outputFiles).toBeDefined();
    expect(execution!.outputFiles!.length).toBeGreaterThan(0);

    const outputFile = execution!.outputFiles![0];
    expect(outputFile.key).toContain("executions/");
    expect(outputFile.key).toContain("/output/");
    expect(outputFile.name).toBeTruthy();
    expect(outputFile.sizeBytes).toBeGreaterThan(0);
    expect(outputFile.contentType).toBeTruthy();
  });

  it("download URLs are valid for completed execution", async () => {
    await pollExecution(user, executionId);

    const downloadResult = await user.client.action(
      api.downloads.generateDownloadUrls,
      { executionId },
    );

    expect(downloadResult.urls.length).toBeGreaterThan(0);
    expect(downloadResult.expiresAt).toBeGreaterThan(Date.now());

    for (const file of downloadResult.urls) {
      expect(file.url).toContain("https://");
      expect(file.name).toBeTruthy();
      expect(file.sizeBytes).toBeGreaterThan(0);
      expect(file.contentType).toBeTruthy();
    }
  });

  it("downloaded file is a valid image", async () => {
    await pollExecution(user, executionId);

    const downloadResult = await user.client.action(
      api.downloads.generateDownloadUrls,
      { executionId },
    );

    const file = downloadResult.urls[0];
    const response = await fetch(file.url);
    expect(response.status).toBe(200);

    const body = await response.arrayBuffer();
    expect(body.byteLength).toBeGreaterThan(0);
    // Output should be same size or smaller than input (compressed)
    const inputSize = new Uint8Array(readFileSync(TEST_IMAGE_PATH)).length;
    expect(body.byteLength).toBeLessThanOrEqual(inputSize * 1.1);
  });
});

// ── Download Access Control ────────────────────────────────────────────

describe("download: access control", () => {
  let owner: AuthenticatedClient;
  let executionId: Id<"executions">;

  beforeAll(async () => {
    owner = await createPasswordClient(generateTestEmail(), "test-dl-own-123", {
      flow: "signUp",
    });
    const result = await uploadAndStartExecution(owner);
    executionId = result.executionId;
    await pollExecution(owner, executionId);
  }, 150_000);

  it("other users cannot download another user's outputs", async () => {
    const other = await createPasswordClient(generateTestEmail(), "other-dl-123", {
      flow: "signUp",
    });
    await expect(
      other.client.action(api.downloads.generateDownloadUrls, {
        executionId,
      }),
    ).rejects.toThrow("Execution not found");
  });

  it("unauthenticated clients cannot download outputs", async () => {
    const unauth = createUnauthenticatedClient();
    await expect(
      unauth.action(api.downloads.generateDownloadUrls, {
        executionId,
      }),
    ).rejects.toThrow();
  });
});

// ── Download Error Cases ───────────────────────────────────────────────

describe("download: error cases", () => {
  let user: AuthenticatedClient;

  beforeAll(async () => {
    user = await createPasswordClient(generateTestEmail(), "test-dl-err-123", {
      flow: "signUp",
    });
  });

  it("rejects download for pending/running execution", async () => {
    const executionId = await user.client.mutation(
      api.executions.startPredefined,
      {
        slug: TEST_SLUG,
        definition: COMPRESS_IMAGES_DEFINITION,
      },
    );

    await expect(
      user.client.action(api.downloads.generateDownloadUrls, {
        executionId,
      }),
    ).rejects.toThrow("Execution is not completed");
  });

  it("returns empty URLs for execution with no output files", async () => {
    const executionId = await user.client.mutation(
      api.executions.startPredefined,
      {
        slug: "test-no-output",
        definition: MINIMAL_DEFINITION,
      },
    );

    await pollExecution(user, executionId, {
      timeoutMs: 90_000,
    });

    const result = await user.client.action(
      api.downloads.generateDownloadUrls,
      { executionId },
    );

    expect(result.urls).toHaveLength(0);
  });
});
