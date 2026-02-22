import { describe, expect, it } from "vitest";
import {
  AUTH_PATHS,
  isAuthPath,
  isPublicPath,
  PUBLIC_PATHS,
  ROUTES,
} from "../routes";

describe("ROUTES", () => {
  it("contains all expected route paths", () => {
    expect(ROUTES.home).toBe("/");
    expect(ROUTES.signin).toBe("/signin");
    expect(ROUTES.waitlist).toBe("/waitlist");
    expect(ROUTES.workflows).toBe("/workflows");
    expect(ROUTES.executions).toBe("/executions");
    expect(ROUTES.settings).toBe("/settings");
  });
});

describe("PUBLIC_PATHS", () => {
  it("includes home, signin, and waitlist", () => {
    expect(PUBLIC_PATHS).toContain("/");
    expect(PUBLIC_PATHS).toContain("/signin");
    expect(PUBLIC_PATHS).toContain("/waitlist");
  });

  it("does not include private routes", () => {
    const publicSet = new Set<string>(PUBLIC_PATHS);
    expect(publicSet.has("/workflows")).toBe(false);
    expect(publicSet.has("/executions")).toBe(false);
    expect(publicSet.has("/settings")).toBe(false);
  });
});

describe("AUTH_PATHS", () => {
  it("includes signin", () => {
    expect(AUTH_PATHS).toContain("/signin");
  });

  it("does not include home or waitlist", () => {
    const authSet = new Set<string>(AUTH_PATHS);
    expect(authSet.has("/")).toBe(false);
    expect(authSet.has("/waitlist")).toBe(false);
  });
});

describe("isPublicPath", () => {
  it("returns true for public paths", () => {
    expect(isPublicPath("/")).toBe(true);
    expect(isPublicPath("/signin")).toBe(true);
    expect(isPublicPath("/waitlist")).toBe(true);
  });

  it("returns false for private paths", () => {
    expect(isPublicPath("/workflows")).toBe(false);
    expect(isPublicPath("/executions")).toBe(false);
    expect(isPublicPath("/settings")).toBe(false);
  });

  it("returns false for unknown paths", () => {
    expect(isPublicPath("/admin")).toBe(false);
    expect(isPublicPath("/foo/bar")).toBe(false);
    expect(isPublicPath("")).toBe(false);
  });

  it("does not match path prefixes", () => {
    expect(isPublicPath("/signin/callback")).toBe(false);
    expect(isPublicPath("/waitlist/confirm")).toBe(false);
  });
});

describe("isAuthPath", () => {
  it("returns true for auth-only paths", () => {
    expect(isAuthPath("/signin")).toBe(true);
  });

  it("returns false for non-auth paths", () => {
    expect(isAuthPath("/")).toBe(false);
    expect(isAuthPath("/waitlist")).toBe(false);
    expect(isAuthPath("/workflows")).toBe(false);
    expect(isAuthPath("/settings")).toBe(false);
  });

  it("does not match path prefixes", () => {
    expect(isAuthPath("/signin/callback")).toBe(false);
  });
});
