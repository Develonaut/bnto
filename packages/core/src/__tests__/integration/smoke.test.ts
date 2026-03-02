/**
 * Smoke test — validates the integration test harness works.
 *
 * Creates a password client, queries the user, and verifies
 * the response shape. If this passes, the harness is wired correctly
 * and subsequent test files (auth, execution, upload/download) can
 * rely on the setup utilities.
 *
 * Prerequisites: `task dev:all` must be running.
 */

import { describe, it, expect, beforeAll } from "vitest";
import {
  createPasswordClient,
  getCurrentUser,
  createUnauthenticatedClient,
  generateTestEmail,
  type AuthenticatedClient,
  api,
} from "./setup";

describe("integration test harness", () => {
  let user: AuthenticatedClient;

  beforeAll(async () => {
    user = await createPasswordClient(generateTestEmail(), "smoke-test-123", {
      flow: "signUp",
    });
  });

  it("creates a password client with valid userId", () => {
    expect(user.userId).toBeTruthy();
    expect(user.token).toBeTruthy();
    expect(user.refreshToken).toBeTruthy();
  });

  it("password client can query users.getMe", async () => {
    const me = await getCurrentUser(user.client);
    expect(me).not.toBeNull();
    expect(me!._id).toBe(user.userId);
    expect(me!.plan).toBe("free");
  });

  it("unauthenticated client gets null from users.getMe", async () => {
    const unauthClient = createUnauthenticatedClient();
    const me = await unauthClient.query(api.users.getMe);
    expect(me).toBeNull();
  });
});
