import { describe, expect, it } from "vitest";
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
 */

const BASE_URL = "http://localhost:3000";
const SESSION_COOKIE = "better-auth.session_token";

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

// Import middleware after setup -- it imports route helpers
const { middleware } = await import("../middleware");

describe("middleware", () => {
  describe("unauthenticated user", () => {
    it("passes through on public paths", () => {
      const response = middleware(createRequest("/"));
      expect(response.status).toBe(200);
    });

    it("passes through on /signin", () => {
      const response = middleware(createRequest("/signin"));
      expect(response.status).toBe(200);
    });

    it("passes through on /waitlist", () => {
      const response = middleware(createRequest("/waitlist"));
      expect(response.status).toBe(200);
    });

    it("redirects to /signin on private route /workflows", () => {
      const response = middleware(createRequest("/workflows"));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });

    it("redirects to /signin on private route /executions", () => {
      const response = middleware(createRequest("/executions"));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });

    it("redirects to /signin on private route /settings", () => {
      const response = middleware(createRequest("/settings"));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });

    it("passes through on unknown routes (404 at page level)", () => {
      const response = middleware(createRequest("/admin"));
      expect(response.status).toBe(200);
    });

    it("redirects to /signin on protected sub-route", () => {
      const response = middleware(createRequest("/workflows/123"));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe(
        "/signin",
      );
    });
  });

  describe("authenticated user", () => {
    const authCookies = { [SESSION_COOKIE]: "valid-session-token" };

    it("passes through on public paths", () => {
      const response = middleware(createRequest("/", authCookies));
      expect(response.status).toBe(200);
    });

    it("passes through on private paths", () => {
      const response = middleware(createRequest("/workflows", authCookies));
      expect(response.status).toBe(200);
    });

    it("passes through on /executions", () => {
      const response = middleware(createRequest("/executions", authCookies));
      expect(response.status).toBe(200);
    });

    it("passes through on /settings", () => {
      const response = middleware(createRequest("/settings", authCookies));
      expect(response.status).toBe(200);
    });

    it("redirects from /signin to / (auth user should not see signin)", () => {
      const response = middleware(createRequest("/signin", authCookies));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe("/");
    });
  });

  describe("signout signal", () => {
    const signoutCookies = {
      [SESSION_COOKIE]: "valid-session-token",
      [SIGNOUT_COOKIE]: "1",
    };

    it("allows auth user through to /signin when signout signal is present", () => {
      const response = middleware(createRequest("/signin", signoutCookies));
      expect(response.status).toBe(200);
    });

    it("does not affect other routes", () => {
      const response = middleware(createRequest("/workflows", signoutCookies));
      expect(response.status).toBe(200);
    });
  });

  describe("secure cookie variant", () => {
    const secureCookies = {
      [`__Secure-${SESSION_COOKIE}`]: "valid-session-token",
    };

    it("recognizes __Secure- prefixed session cookie", () => {
      const response = middleware(createRequest("/workflows", secureCookies));
      expect(response.status).toBe(200);
    });

    it("redirects from /signin with __Secure- cookie", () => {
      const response = middleware(createRequest("/signin", secureCookies));
      expect(response.status).toBe(307);
      expect(new URL(response.headers.get("location")!).pathname).toBe("/");
    });
  });
});
