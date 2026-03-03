/**
 * Auth lifecycle integration tests (S1-S3).
 *
 * Tests password sign-up/sign-in, unauthenticated rejection, and the
 * auth API surface against real Convex dev functions.
 *
 * These complement the convex-test unit tests (which test logic in-memory)
 * by catching: wrong env vars, missing indexes, auth provider misconfiguration,
 * schema migration issues, and real @convex-dev/auth signIn behavior.
 *
 * Spec: .claude/journeys/auth.md — "Standard Auth Lifecycle" (S1-S3)
 * Prerequisites: `task dev:all` must be running.
 */

import { describe, it, expect, beforeAll } from "vitest";
import {
  createPasswordClient,
  createUnauthenticatedClient,
  generateTestEmail,
  getCurrentUser,
  refreshClientToken,
  type AuthenticatedClient,
  api,
} from "./setup";

// ── S1: Email sign-in works ─────────────────────────────────────────────

describe("S1: email sign-in works", () => {
  const email = generateTestEmail();
  const password = "test-password-123";
  let signedUp: AuthenticatedClient;
  let signedIn: AuthenticatedClient;

  beforeAll(async () => {
    signedUp = await createPasswordClient(email, password, { flow: "signUp" });
    signedIn = await createPasswordClient(email, password, { flow: "signIn" });
  });

  it("sign-up creates a user with valid token and userId", () => {
    expect(signedUp.userId).toBeTruthy();
    expect(signedUp.token).toBeTruthy();
    expect(signedUp.refreshToken).toBeTruthy();
  });

  it("signed-up user has correct fields", async () => {
    const user = await getCurrentUser(signedUp.client);
    expect(user).not.toBeNull();
    expect(user!._id).toBe(signedUp.userId);
    expect(user!.email).toBe(email);
    expect(user!.plan).toBe("free");
  });

  it("sign-in returns the same userId as sign-up", () => {
    expect(signedIn.userId).toBe(signedUp.userId);
  });

  it("signed-in user can query their own data", async () => {
    const user = await getCurrentUser(signedIn.client);
    expect(user).not.toBeNull();
    expect(user!._id).toBe(signedIn.userId);
    expect(user!.email).toBe(email);
  });
});

// ── S2: Sign-out behavior ───────────────────────────────────────────────

describe("S2: sign-out behavior", () => {
  // Sign-out at the ConvexHttpClient level is limited — the JWT may still
  // validate until expiry since it's stateless. Full sign-out verification
  // (cookie clearing, proxy redirect, session invalidation in browser)
  // is tested in Playwright E2E.
  //
  // What we CAN test: a client with no auth token behaves as unauthenticated.

  it("client without auth token gets null from protected queries", async () => {
    const unauthClient = createUnauthenticatedClient();
    const user = await unauthClient.query(api.users.getMe);
    expect(user).toBeNull();
  });

  it("client without auth token gets empty from list queries", async () => {
    const unauthClient = createUnauthenticatedClient();
    const recipes = await unauthClient.query(api.recipes.list);
    expect(recipes).toEqual([]);
  });

  it("client without auth token is rejected from mutations", async () => {
    const unauthClient = createUnauthenticatedClient();
    await expect(
      unauthClient.mutation(api.recipes.save, {
        name: "should-fail",
        definition: { nodes: [] },
      }),
    ).rejects.toThrow("Not authenticated");
  });
});

// ── S3: Auth API surface ────────────────────────────────────────────────

describe("S3: auth API surface", () => {
  let passwordUser: AuthenticatedClient;
  const email = generateTestEmail();
  const password = "test-surface-123";

  beforeAll(async () => {
    passwordUser = await createPasswordClient(email, password, { flow: "signUp" });
  });

  it("password user can call protected queries (recipes.list)", async () => {
    const recipes = await passwordUser.client.query(api.recipes.list);
    expect(Array.isArray(recipes)).toBe(true);
  });

  it("password user can call protected mutations (recipes.save)", async () => {
    const recipeName = `test-auth-${Date.now()}`;
    const recipeId = await passwordUser.client.mutation(api.recipes.save, {
      name: recipeName,
      definition: { nodes: [] },
    });
    expect(recipeId).toBeTruthy();

    const recipes = await passwordUser.client.query(api.recipes.list);
    const found = recipes.find(
      (r: { name: string }) => r.name === recipeName,
    );
    expect(found).toBeDefined();
  });

  it("unauthenticated client is rejected from startPredefined", async () => {
    const unauthClient = createUnauthenticatedClient();
    await expect(
      unauthClient.mutation(api.executions.startPredefined, {
        slug: "compress-images",
        definition: { type: "group", nodes: [] },
      }),
    ).rejects.toThrow("Not authenticated");
  });

  it("invalid password sign-in fails gracefully", async () => {
    await expect(
      createPasswordClient("nonexistent@test.bnto.dev", "wrong-password", {
        flow: "signIn",
      }),
    ).rejects.toThrow();
  });

  it("token refresh returns new valid tokens", async () => {
    const refreshed = await refreshClientToken(passwordUser);
    expect(refreshed.token).toBeTruthy();
    expect(refreshed.refreshToken).toBeTruthy();

    const user = await getCurrentUser(refreshed.client);
    expect(user).not.toBeNull();
    expect(user!._id).toBe(passwordUser.userId);
  });

  it("two separate password accounts get distinct userIds", async () => {
    const user2 = await createPasswordClient(generateTestEmail(), "other-pass-123", {
      flow: "signUp",
    });
    expect(user2.userId).not.toBe(passwordUser.userId);

    const me1 = await getCurrentUser(passwordUser.client);
    const me2 = await getCurrentUser(user2.client);
    expect(me1!._id).not.toBe(me2!._id);
  });
});
