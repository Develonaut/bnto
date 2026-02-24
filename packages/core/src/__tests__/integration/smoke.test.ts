/**
 * Smoke test — validates the integration test harness works.
 *
 * Creates an anonymous client, queries the user, and verifies
 * the response shape. If this passes, the harness is wired correctly
 * and subsequent test files (auth, execution, upload/download) can
 * rely on the setup utilities.
 *
 * Prerequisites: `task dev:all` must be running.
 */

import { describe, it, expect, beforeAll } from "vitest";
import {
  createAnonymousClient,
  getCurrentUser,
  createUnauthenticatedClient,
  type AuthenticatedClient,
  api,
} from "./setup";

describe("integration test harness", () => {
  let anon: AuthenticatedClient;

  beforeAll(async () => {
    anon = await createAnonymousClient();
  });

  it("creates an anonymous client with valid userId", () => {
    expect(anon.userId).toBeTruthy();
    expect(anon.token).toBeTruthy();
    expect(anon.refreshToken).toBeTruthy();
  });

  it("anonymous client can query users.getMe", async () => {
    const user = await getCurrentUser(anon.client);
    expect(user).not.toBeNull();
    expect(user!._id).toBe(anon.userId);
    expect(user!.isAnonymous).toBe(true);
    expect(user!.plan).toBe("free");
    expect(user!.runsUsed).toBeTypeOf("number");
    expect(user!.runLimit).toBeTypeOf("number");
  });

  it("unauthenticated client gets null from users.getMe", async () => {
    const unauthClient = createUnauthenticatedClient();
    const user = await unauthClient.query(api.users.getMe);
    expect(user).toBeNull();
  });
});
