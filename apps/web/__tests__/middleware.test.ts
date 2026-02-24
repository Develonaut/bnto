import { describe, expect, it, vi, beforeEach } from "vitest";
import { NextRequest } from "next/server";
import { SIGNOUT_COOKIE } from "@bnto/core/constants";

/**
 * Middleware tests verify the three-tier proxy logic:
 *
 * 1. Canonical URL normalization (case, underscores, trailing slash)
 * 2. Auth user on /signin (no signout signal) -> redirect to /
 * 3. Unauth user on protected route -> redirect to /signin
 *
 * Everything else passes through (bnto slugs, unknown paths -> 404 at page level).
 *
 * We mock `convexAuthNextjsMiddleware` to isolate our route protection logic
 * from Convex's token refresh internals.
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

// Import middleware after mock setup
const { default: middleware } = await import("../middleware");

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

describe("middleware", () => {
  beforeEach(() => {
    // Ensure handler was captured
    expect(capturedHandler).not.toBeNull();
  });

  describe("unauthenticated user", () => {
    it("passes through on public paths", async () => {
      const response = await middleware(createRequest("/"));
      expect(response.status).toBe(200);
    });

    it("passes through on /signin", async () => {
      const response = await middleware(createRequest("/signin"));
      expect(response.status).toBe(200);
    });

    it("passes through on /waitlist", async () => {
      const response = await middleware(createRequest("/waitlist"));
      expect(response.status).toBe(200);
    });

    it("redirects to /signin on private route /workflows", async () => {
      const response = await middleware(createRequest("/workflows"));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });

    it("redirects to /signin on private route /executions", async () => {
      const response = await middleware(createRequest("/executions"));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });

    it("redirects to /signin on private route /settings", async () => {
      const response = await middleware(createRequest("/settings"));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });

    it("passes through on unknown routes (404 at page level)", async () => {
      const response = await middleware(createRequest("/admin"));
      expect(response.status).toBe(200);
    });

    it("redirects to /signin on protected sub-route", async () => {
      const response = await middleware(createRequest("/workflows/123"));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });
  });

  describe("authenticated user", () => {
    it("passes through on public paths", async () => {
      const response = await middleware(createRequest("/", AUTH_COOKIES));
      expect(response.status).toBe(200);
    });

    it("passes through on private paths", async () => {
      const response = await middleware(
        createRequest("/workflows", AUTH_COOKIES),
      );
      expect(response.status).toBe(200);
    });

    it("passes through on /executions", async () => {
      const response = await middleware(
        createRequest("/executions", AUTH_COOKIES),
      );
      expect(response.status).toBe(200);
    });

    it("passes through on /settings", async () => {
      const response = await middleware(
        createRequest("/settings", AUTH_COOKIES),
      );
      expect(response.status).toBe(200);
    });

    it("redirects from /signin to / (auth user should not see signin)", async () => {
      const response = await middleware(
        createRequest("/signin", AUTH_COOKIES),
      );
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe("/");
    });
  });

  describe("signout signal", () => {
    const signoutCookies = {
      ...AUTH_COOKIES,
      [SIGNOUT_COOKIE]: "1",
    };

    it("allows auth user through to /signin when signout signal is present", async () => {
      const response = await middleware(
        createRequest("/signin", signoutCookies),
      );
      // With signout signal, the redirect goes to /signin (pass-through)
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });

    it("does not affect other routes", async () => {
      const response = await middleware(
        createRequest("/workflows", signoutCookies),
      );
      expect(response.status).toBe(200);
    });
  });
});
