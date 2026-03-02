import { describe, expect, it } from "vitest";
import {
  AUTH_PATHS,
  isAuthPath,
  isProtectedPath,
  PROTECTED_PATHS,
  ROUTES,
} from "../routes";

describe("ROUTES", () => {
  it("contains all expected route paths", () => {
    expect(ROUTES.home).toBe("/");
    expect(ROUTES.signin).toBe("/signin");
    expect(ROUTES.signup).toBe("/signup");
    expect(ROUTES.waitlist).toBe("/waitlist");
    expect(ROUTES.myRecipes).toBe("/my-recipes");
    expect(ROUTES.executions).toBe("/executions");
    expect(ROUTES.settings).toBe("/settings");
  });
});

describe("AUTH_PATHS", () => {
  it("includes signin and signup", () => {
    expect(AUTH_PATHS).toContain("/signin");
    expect(AUTH_PATHS).toContain("/signup");
  });

  it("does not include home or waitlist", () => {
    const authSet = new Set<string>(AUTH_PATHS);
    expect(authSet.has("/")).toBe(false);
    expect(authSet.has("/waitlist")).toBe(false);
  });
});

describe("PROTECTED_PATHS", () => {
  it("includes executions and settings", () => {
    expect(PROTECTED_PATHS).toContain("/executions");
    expect(PROTECTED_PATHS).toContain("/settings");
  });

  it("does not include public routes", () => {
    const protectedSet = new Set<string>(PROTECTED_PATHS);
    expect(protectedSet.has("/")).toBe(false);
    expect(protectedSet.has("/signin")).toBe(false);
    expect(protectedSet.has("/waitlist")).toBe(false);
    expect(protectedSet.has("/my-recipes")).toBe(false);
  });
});

describe("isAuthPath", () => {
  it("returns true for auth-only paths", () => {
    expect(isAuthPath("/signin")).toBe(true);
    expect(isAuthPath("/signup")).toBe(true);
  });

  it("returns false for non-auth paths", () => {
    expect(isAuthPath("/")).toBe(false);
    expect(isAuthPath("/waitlist")).toBe(false);
    expect(isAuthPath("/my-recipes")).toBe(false);
    expect(isAuthPath("/settings")).toBe(false);
  });

  it("does not match path prefixes", () => {
    expect(isAuthPath("/signin/callback")).toBe(false);
  });
});

describe("isProtectedPath", () => {
  it("returns true for protected paths", () => {
    expect(isProtectedPath("/executions")).toBe(true);
    expect(isProtectedPath("/settings")).toBe(true);
  });

  it("matches sub-paths of protected routes", () => {
    expect(isProtectedPath("/settings/account")).toBe(true);
    expect(isProtectedPath("/executions/123")).toBe(true);
  });

  it("returns false for public paths", () => {
    expect(isProtectedPath("/")).toBe(false);
    expect(isProtectedPath("/signin")).toBe(false);
    expect(isProtectedPath("/waitlist")).toBe(false);
    expect(isProtectedPath("/my-recipes")).toBe(false);
    expect(isProtectedPath("/my-recipes/123")).toBe(false);
  });

  it("returns false for bnto slugs and unknown paths", () => {
    expect(isProtectedPath("/compress-images")).toBe(false);
    expect(isProtectedPath("/clean-csv")).toBe(false);
    expect(isProtectedPath("/some-random-page")).toBe(false);
  });
});
