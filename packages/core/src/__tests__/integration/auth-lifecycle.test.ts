/**
 * Auth lifecycle integration tests (S1-S3).
 *
 * Tests anonymous sign-in, password sign-up/sign-in, unauthenticated
 * rejection, and the auth API surface against real Convex dev functions.
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
  createAnonymousClient,
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
    expect(user!.isAnonymous).toBe(false);
    expect(user!.email).toBe(email);
    expect(user!.plan).toBe("free");
    expect(user!.runsUsed).toBeTypeOf("number");
    expect(user!.runLimit).toBeTypeOf("number");
    expect(user!.runsResetAt).toBeTypeOf("number");
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
    const workflows = await unauthClient.query(api.workflows.list);
    expect(workflows).toEqual([]);
  });

  it("client without auth token is rejected from mutations", async () => {
    const unauthClient = createUnauthenticatedClient();
    await expect(
      unauthClient.mutation(api.workflows.save, {
        name: "should-fail",
        definition: { nodes: [] },
      }),
    ).rejects.toThrow("Not authenticated");
  });
});

// ── S3: Auth API surface ────────────────────────────────────────────────

describe("S3: auth API surface", () => {
  let anon: AuthenticatedClient;

  beforeAll(async () => {
    anon = await createAnonymousClient();
  });

  it("anonymous sign-in returns valid token, userId, and refreshToken", () => {
    expect(anon.userId).toBeTruthy();
    expect(anon.token).toBeTruthy();
    expect(anon.refreshToken).toBeTruthy();
    // userId should look like a Convex ID (not empty, not a JWT)
    expect(anon.userId.length).toBeGreaterThan(0);
    expect(anon.userId.length).toBeLessThan(100);
  });

  it("anonymous user has correct default fields", async () => {
    const user = await getCurrentUser(anon.client);
    expect(user).not.toBeNull();
    expect(user!._id).toBe(anon.userId);
    expect(user!.isAnonymous).toBe(true);
    expect(user!.plan).toBe("free");
    expect(user!.runsUsed).toBe(0);
    expect(user!.runLimit).toBeTypeOf("number");
    expect(user!.runLimit).toBeGreaterThan(0);
    expect(user!.runsResetAt).toBeTypeOf("number");
    expect(user!.runsResetAt).toBeGreaterThan(Date.now());
  });

  it("anonymous user can call protected queries (workflows.list)", async () => {
    const workflows = await anon.client.query(api.workflows.list);
    expect(Array.isArray(workflows)).toBe(true);
  });

  it("anonymous user can call protected mutations (workflows.save)", async () => {
    const workflowName = `test-auth-anon-${Date.now()}`;
    const workflowId = await anon.client.mutation(api.workflows.save, {
      name: workflowName,
      definition: { nodes: [] },
    });
    expect(workflowId).toBeTruthy();

    // Verify the workflow is visible in the user's list
    const workflows = await anon.client.query(api.workflows.list);
    const found = workflows.find(
      (w: { name: string }) => w.name === workflowName,
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
    const refreshed = await refreshClientToken(anon);
    expect(refreshed.token).toBeTruthy();
    expect(refreshed.refreshToken).toBeTruthy();

    // Refreshed client still works
    const user = await getCurrentUser(refreshed.client);
    expect(user).not.toBeNull();
    expect(user!._id).toBe(anon.userId);
  });

  it("runs remaining query works for anonymous user", async () => {
    const remaining = await anon.client.query(api.users.getRunsRemaining);
    expect(remaining).not.toBeNull();
    expect(remaining).toBeTypeOf("number");
    expect(remaining).toBeGreaterThanOrEqual(0);
  });

  it("two separate anonymous sessions get distinct userIds", async () => {
    const anon2 = await createAnonymousClient();
    expect(anon2.userId).not.toBe(anon.userId);

    const user1 = await getCurrentUser(anon.client);
    const user2 = await getCurrentUser(anon2.client);
    expect(user1!._id).not.toBe(user2!._id);
  });
});
