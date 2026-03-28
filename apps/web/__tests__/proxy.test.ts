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
let capturedHandler:
  | ((
      request: NextRequest,
      ctx: { convexAuth: { isAuthenticated: () => Promise<boolean> } },
    ) => Promise<Response | void>)
  | null = null;

vi.mock("@convex-dev/auth/nextjs/server", () => ({
  convexAuthNextjsMiddleware: (handler: typeof capturedHandler) => {
    capturedHandler = handler;
    // Return a function that calls the handler with mock convexAuth.
    // Accepts optional second arg to match the real NextMiddleware signature.
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

function createRequest(pathname: string, cookies: Record<string, string> = {}) {
  const url = `${BASE_URL}${pathname}`;
  const request = new NextRequest(url);
  for (const [name, value] of Object.entries(cookies)) {
    request.cookies.set(name, value);
  }
  return request;
}

/** Call proxy with a mock NextFetchEvent to satisfy the two-arg middleware signature. */
async function callProxy(request: NextRequest) {
  const response = await proxy(request, {} as Parameters<typeof proxy>[1]);
  return response!;
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
      const response = await callProxy(createRequest("/"));
      expect(response.status).toBe(200);
    });

    it("passes through on /signin", async () => {
      const response = await callProxy(createRequest("/signin"));
      expect(response.status).toBe(200);
    });

    it("passes through on /waitlist", async () => {
      const response = await callProxy(createRequest("/waitlist"));
      expect(response.status).toBe(200);
    });

    it("passes through on /my-recipes (public for local recipes)", async () => {
      const response = await callProxy(createRequest("/my-recipes"));
      expect(response.status).toBe(200);
    });

    it("redirects to /signin with returnTo on private route /executions", async () => {
      const response = await callProxy(createRequest("/executions"));
      expect(response.status).toBe(307);
      const location = new URL(response.headers.get("location")!);
      expect(location.pathname).toBe("/signin");
      expect(location.searchParams.get("returnTo")).toBe("/executions");
    });

    it("redirects to /signin with returnTo on private route /settings", async () => {
      const response = await callProxy(createRequest("/settings"));
      expect(response.status).toBe(307);
      const location = new URL(response.headers.get("location")!);
      expect(location.pathname).toBe("/signin");
      expect(location.searchParams.get("returnTo")).toBe("/settings");
    });

    it("passes through on unknown routes (404 at page level)", async () => {
      const response = await callProxy(createRequest("/admin"));
      expect(response.status).toBe(200);
    });

    it("redirects to /signin with returnTo on protected sub-route", async () => {
      const response = await callProxy(createRequest("/settings/account"));
      expect(response.status).toBe(307);
      const location = new URL(response.headers.get("location")!);
      expect(location.pathname).toBe("/signin");
      expect(location.searchParams.get("returnTo")).toBe("/settings/account");
    });
  });

  describe("authenticated user", () => {
    it("passes through on public paths", async () => {
      const response = await callProxy(createRequest("/", AUTH_COOKIES));
      expect(response.status).toBe(200);
    });

    it("passes through on private paths", async () => {
      const response = await callProxy(createRequest("/my-recipes", AUTH_COOKIES));
      expect(response.status).toBe(200);
    });

    it("passes through on /executions", async () => {
      const response = await callProxy(createRequest("/executions", AUTH_COOKIES));
      expect(response.status).toBe(200);
    });

    it("passes through on /settings", async () => {
      const response = await callProxy(createRequest("/settings", AUTH_COOKIES));
      expect(response.status).toBe(200);
    });

    it("redirects from /signin to / (already authenticated)", async () => {
      const response = await callProxy(createRequest("/signin", AUTH_COOKIES));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe("/");
    });

    it("redirects from /signin to returnTo destination when authenticated", async () => {
      const response = await callProxy(createRequest("/signin?returnTo=%2Feditor", AUTH_COOKIES));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe("/editor");
    });
  });
});
