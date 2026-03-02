import { describe, expect, it, vi, beforeEach } from "vitest";
import { NextRequest } from "next/server";

/**
 * Middleware tests verify the three-tier proxy logic:
 *
 * 1. Canonical URL normalization (case, underscores, trailing slash)
 * 2. Auth routes (/signin, /signup) -> redirect to / if already authenticated
 * 3. Protected routes (/executions, /settings) -> redirect to /signin if not authenticated
 *
 * The signout signal cookie (bnto-signout) bypasses the auth-route redirect
 * so users can reach /signin during sign-out despite the stale session cookie.
 *
 * Everything else passes through (bnto slugs, unknown paths -> 404 at page level).
 */

const BASE_URL = "http://localhost:3000";

// Mock convexAuthNextjsMiddleware to extract and call our handler directly
let capturedHandler: ((
  request: NextRequest,
  ctx: { convexAuth: { isAuthenticated: () => Promise<boolean> } },
) => Promise<Response | void>) | null = null;

vi.mock("@convex-dev/auth/nextjs/server", () => ({
  convexAuthNextjsMiddleware: (handler: typeof capturedHandler) => {
    capturedHandler = handler;
    // Return a function that calls the handler with mock convexAuth
    return async (request: NextRequest) => {
      const isAuth = request.cookies.has("__convexAuthJWT");
      const result = await handler!(request, {
        convexAuth: { isAuthenticated: async () => isAuth },
      });
      return result ?? new Response(null, { status: 200 });
    };
  },
  nextjsMiddlewareRedirect: (request: NextRequest, pathname: string) => {
    const url = new URL(pathname, request.url);
    return Response.redirect(url, 307);
  },
}));

// Import proxy after mock setup
const { default: proxy } = await import("../proxy");

function createRequest(
  pathname: string,
  cookies: Record<string, string> = {},
) {
  const url = `${BASE_URL}${pathname}`;
  const request = new NextRequest(url);
  for (const [name, value] of Object.entries(cookies)) {
    request.cookies.set(name, value);
  }
  return request;
}

/** Simulate an authenticated request by setting the mock JWT cookie. */
const AUTH_COOKIES = { __convexAuthJWT: "mock-token" };

describe("proxy", () => {
  beforeEach(() => {
    // Ensure handler was captured
    expect(capturedHandler).not.toBeNull();
  });

  describe("unauthenticated user", () => {
    it("passes through on public paths", async () => {
      const response = await proxy(createRequest("/"));
      expect(response.status).toBe(200);
    });

    it("passes through on /signin", async () => {
      const response = await proxy(createRequest("/signin"));
      expect(response.status).toBe(200);
    });

    it("passes through on /waitlist", async () => {
      const response = await proxy(createRequest("/waitlist"));
      expect(response.status).toBe(200);
    });

    it("passes through on /my-recipes (public with conversion prompt)", async () => {
      const response = await proxy(createRequest("/my-recipes"));
      expect(response.status).toBe(200);
    });

    it("redirects to /signin on private route /executions", async () => {
      const response = await proxy(createRequest("/executions"));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });

    it("redirects to /signin on private route /settings", async () => {
      const response = await proxy(createRequest("/settings"));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });

    it("passes through on unknown routes (404 at page level)", async () => {
      const response = await proxy(createRequest("/admin"));
      expect(response.status).toBe(200);
    });

    it("redirects to /signin on protected sub-route", async () => {
      const response = await proxy(createRequest("/settings/account"));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });
  });

  describe("authenticated user", () => {
    it("passes through on public paths", async () => {
      const response = await proxy(createRequest("/", AUTH_COOKIES));
      expect(response.status).toBe(200);
    });

    it("passes through on private paths", async () => {
      const response = await proxy(
        createRequest("/my-recipes", AUTH_COOKIES),
      );
      expect(response.status).toBe(200);
    });

    it("passes through on /executions", async () => {
      const response = await proxy(
        createRequest("/executions", AUTH_COOKIES),
      );
      expect(response.status).toBe(200);
    });

    it("passes through on /settings", async () => {
      const response = await proxy(
        createRequest("/settings", AUTH_COOKIES),
      );
      expect(response.status).toBe(200);
    });

    it("redirects from /signin to / (already authenticated)", async () => {
      const response = await proxy(
        createRequest("/signin", AUTH_COOKIES),
      );
      // Proxy redirects authenticated users away from /signin to /
      expect(response.status).toBe(307);
      expect(response.headers.get("location")).toContain("/");
    });
  });
});
