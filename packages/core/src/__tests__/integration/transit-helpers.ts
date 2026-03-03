/**
 * Shared helpers for upload/download integration tests.
 * Provides test fixtures, R2 upload helper, and execution polling.
 */

import { readFileSync } from "fs";
import { resolve } from "path";
import type { Id } from "@bnto/backend/convex/_generated/dataModel";
import { type AuthenticatedClient, api } from "./setup";

// ── Test Fixtures ────────────────────────────────────────────────────────

/** Small PNG from engine test fixtures (~440KB — well under 25MB free limit). */
export const TEST_IMAGE_PATH = resolve(
  __dirname,
  "../../../../../archive/engine-go/tests/fixtures/images/Product_Render.png",
);

/** compress-images recipe definition — reads from INPUT_DIR, outputs to OUTPUT_DIR. */
export const COMPRESS_IMAGES_DEFINITION = JSON.parse(
  readFileSync(
    resolve(
      __dirname,
      "../../../../../archive/engine-go/tests/fixtures/workflows/compress-images.bnto.json",
    ),
    "utf-8",
  ),
);

export const TEST_SLUG = "compress-images";

/** Minimal definition (empty group) — completes fast, produces no output files. */
export const MINIMAL_DEFINITION = {
  id: "test-no-output",
  type: "group",
  version: "1.0.0",
  name: "No Output Test",
  position: { x: 0, y: 0 },
  metadata: {},
  parameters: {},
  inputPorts: [],
  outputPorts: [],
  nodes: [],
  edges: [],
};

// ── Helpers ──────────────────────────────────────────────────────────────

/** Read test image as a Uint8Array. */
export function readTestImage(): Uint8Array {
  return new Uint8Array(readFileSync(TEST_IMAGE_PATH));
}

/** Upload a file to an R2 presigned PUT URL. Returns the HTTP status. */
export async function uploadToPresignedUrl(
  url: string,
  fileBytes: Uint8Array,
  contentType: string,
): Promise<number> {
  // Copy into a fresh ArrayBuffer to satisfy TS strict typing
  const buffer = new ArrayBuffer(fileBytes.length);
  new Uint8Array(buffer).set(fileBytes);
  const blob = new Blob([buffer], { type: contentType });
  const response = await fetch(url, {
    method: "PUT",
    headers: { "Content-Type": contentType },
    body: blob,
  });
  return response.status;
}

/** Poll execution status until terminal state or timeout. */
export async function pollExecution(
  client: AuthenticatedClient,
  executionId: Id<"executions">,
  { timeoutMs = 120_000, intervalMs = 2_000 } = {},
): Promise<{
  status: string;
  error?: string;
  outputFiles?: Array<{
    key: string;
    name: string;
    sizeBytes: number;
    contentType: string;
  }>;
}> {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    const execution = await client.client.query(api.executions.get, {
      id: executionId,
    });
    if (!execution) {
      throw new Error(`Execution ${executionId} not found`);
    }
    if (execution.status === "completed" || execution.status === "failed") {
      return {
        status: execution.status,
        error: execution.error,
        outputFiles: execution.outputFiles,
      };
    }
    await new Promise((r) => setTimeout(r, intervalMs));
  }
  throw new Error(
    `Execution ${executionId} did not complete within ${timeoutMs}ms`,
  );
}

/**
 * Upload a test image and start a predefined execution with sessionId.
 * Returns the executionId and sessionId for downstream assertions.
 */
export async function uploadAndStartExecution(
  client: AuthenticatedClient,
): Promise<{ executionId: Id<"executions">; sessionId: string }> {
  const fileBuffer = readTestImage();

  const uploadResult = await client.client.action(
    api.uploads.generateUploadUrls,
    {
      files: [
        {
          name: "Product_Render.png",
          contentType: "image/png",
          sizeBytes: fileBuffer.length,
        },
      ],
    },
  );

  await uploadToPresignedUrl(
    uploadResult.urls[0].url,
    fileBuffer,
    "image/png",
  );

  const executionId = await client.client.mutation(
    api.executions.startPredefined,
    {
      slug: TEST_SLUG,
      definition: COMPRESS_IMAGES_DEFINITION,
      sessionId: uploadResult.sessionId,
    },
  );

  return { executionId, sessionId: uploadResult.sessionId };
}
