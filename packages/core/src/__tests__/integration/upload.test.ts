/**
 * Upload integration tests against real Convex dev + R2.
 *
 * Tests presigned URL generation, validation enforcement, and actual
 * file upload to the R2 dev bucket via presigned PUT URLs.
 *
 * Prerequisites: `task dev:all` must be running.
 */

import { describe, it, expect, beforeAll } from "vitest";
import {
  createAnonymousClient,
  createUnauthenticatedClient,
  type AuthenticatedClient,
  api,
} from "./setup";
import { readTestImage, uploadToPresignedUrl } from "./transit-helpers";

// ── Presigned URL Generation ─────────────────────────────────────────────

describe("upload: presigned URL generation", () => {
  let anon: AuthenticatedClient;

  beforeAll(async () => {
    anon = await createAnonymousClient();
  });

  it("returns valid presigned URLs for a single file", async () => {
    const result = await anon.client.action(api.uploads.generateUploadUrls, {
      files: [
        { name: "test.png", contentType: "image/png", sizeBytes: 1024 },
      ],
    });

    expect(result.sessionId).toBeTruthy();
    expect(typeof result.sessionId).toBe("string");
    expect(result.urls).toHaveLength(1);
    expect(result.urls[0].name).toBe("test.png");
    expect(result.urls[0].key).toContain(`uploads/${result.sessionId}/`);
    expect(result.urls[0].url).toContain("https://");
    expect(result.expiresAt).toBeGreaterThan(Date.now());
  });

  it("returns multiple presigned URLs for a batch", async () => {
    const result = await anon.client.action(api.uploads.generateUploadUrls, {
      files: [
        { name: "a.png", contentType: "image/png", sizeBytes: 1024 },
        { name: "b.csv", contentType: "text/csv", sizeBytes: 512 },
        { name: "c.json", contentType: "application/json", sizeBytes: 256 },
      ],
    });

    expect(result.urls).toHaveLength(3);
    const names = result.urls.map((u: { name: string }) => u.name);
    expect(names).toEqual(["a.png", "b.csv", "c.json"]);
    for (const u of result.urls) {
      expect(u.key).toContain(result.sessionId);
    }
  });

  it("each call generates a unique sessionId", async () => {
    const r1 = await anon.client.action(api.uploads.generateUploadUrls, {
      files: [
        { name: "x.png", contentType: "image/png", sizeBytes: 1024 },
      ],
    });
    const r2 = await anon.client.action(api.uploads.generateUploadUrls, {
      files: [
        { name: "y.png", contentType: "image/png", sizeBytes: 1024 },
      ],
    });

    expect(r1.sessionId).not.toBe(r2.sessionId);
  });
});

// ── Validation Enforcement ───────────────────────────────────────────────

describe("upload: validation enforcement", () => {
  let anon: AuthenticatedClient;

  beforeAll(async () => {
    anon = await createAnonymousClient();
  });

  it("rejects unsupported MIME type", async () => {
    await expect(
      anon.client.action(api.uploads.generateUploadUrls, {
        files: [
          {
            name: "script.exe",
            contentType: "application/x-msdownload",
            sizeBytes: 1024,
          },
        ],
      }),
    ).rejects.toThrow();
  });

  it("rejects empty batch", async () => {
    await expect(
      anon.client.action(api.uploads.generateUploadUrls, {
        files: [],
      }),
    ).rejects.toThrow();
  });

  it("rejects file exceeding free plan size limit (25MB)", async () => {
    const overLimit = 26 * 1024 * 1024;
    await expect(
      anon.client.action(api.uploads.generateUploadUrls, {
        files: [
          {
            name: "huge.png",
            contentType: "image/png",
            sizeBytes: overLimit,
          },
        ],
      }),
    ).rejects.toThrow();
  });

  it("rejects unauthenticated upload requests", async () => {
    const unauth = createUnauthenticatedClient();
    await expect(
      unauth.action(api.uploads.generateUploadUrls, {
        files: [
          { name: "test.png", contentType: "image/png", sizeBytes: 1024 },
        ],
      }),
    ).rejects.toThrow();
  });
});

// ── Actual File Upload to R2 ─────────────────────────────────────────────

describe("upload: actual file upload to R2", () => {
  let anon: AuthenticatedClient;

  beforeAll(async () => {
    anon = await createAnonymousClient();
  });

  it("uploads a real PNG file via presigned URL", async () => {
    const fileBuffer = readTestImage();

    const result = await anon.client.action(api.uploads.generateUploadUrls, {
      files: [
        {
          name: "Product_Render.png",
          contentType: "image/png",
          sizeBytes: fileBuffer.length,
        },
      ],
    });

    expect(result.urls).toHaveLength(1);

    const status = await uploadToPresignedUrl(
      result.urls[0].url,
      fileBuffer,
      "image/png",
    );

    expect(status).toBe(200);
  });
});
