import { describe, expect, it, vi, beforeEach } from "vitest";
import { NextRequest } from "next/server";

/**
 * Middleware tests verify canonical URL normalization:
 *   - Lowercase path segments
 *   - Convert underscores to hyphens
 *   - Strip trailing slashes
 *
 * Everything else passes through (bnto slugs, unknown paths -> 404 at page level).
 */

const BASE_URL = "http://localhost:3000";

// Mock convexAuthNextjsMiddleware to extract and call our handler directly
let capturedHandler:
  | ((request: NextRequest) => Promise<Response | void>)
  | null = null;

vi.mock("@convex-dev/auth/nextjs/server", () => ({
  convexAuthNextjsMiddleware: (handler: typeof capturedHandler) => {
    capturedHandler = handler;
    return async (request: NextRequest) => {
      const result = await handler!(request);
      return result ?? new Response(null, { status: 200 });
    };
  },
}));

// Import proxy after mock setup
const { default: proxy } = await import("../proxy");

function createRequest(pathname: string) {
  const url = `${BASE_URL}${pathname}`;
  return new NextRequest(url);
}

/** Call proxy with a mock NextFetchEvent to satisfy the two-arg middleware signature. */
async function callProxy(request: NextRequest) {
  const response = await proxy(request, {} as Parameters<typeof proxy>[1]);
  return response!;
}

describe("proxy", () => {
  beforeEach(() => {
    expect(capturedHandler).not.toBeNull();
  });

  describe("URL normalization", () => {
    it("passes through on /", async () => {
      const response = await callProxy(createRequest("/"));
      expect(response.status).toBe(200);
    });

    it("passes through on unknown routes (404 at page level)", async () => {
      const response = await callProxy(createRequest("/admin"));
      expect(response.status).toBe(200);
    });

    it("passes through on tool paths", async () => {
      const response = await callProxy(createRequest("/compress-images"));
      expect(response.status).toBe(200);
    });

    it("redirects uppercase to lowercase (301)", async () => {
      const response = await callProxy(createRequest("/Compress-Images"));
      expect(response.status).toBe(301);
      const location = new URL(response.headers.get("location")!);
      expect(location.pathname).toBe("/compress-images");
    });

    it("redirects underscores to hyphens (301)", async () => {
      const response = await callProxy(createRequest("/compress_images"));
      expect(response.status).toBe(301);
      const location = new URL(response.headers.get("location")!);
      expect(location.pathname).toBe("/compress-images");
    });

    it("normalizes combined case + underscore (301)", async () => {
      const response = await callProxy(createRequest("/Compress_Images"));
      expect(response.status).toBe(301);
      const location = new URL(response.headers.get("location")!);
      expect(location.pathname).toBe("/compress-images");
    });
  });
});
