/**
 * Integration test harness for @bnto/core against real Convex dev.
 *
 * Creates ConvexHttpClient instances authenticated via @convex-dev/auth's
 * signIn action — no React, no browser.
 *
 * Prerequisites:
 *   - `task dev:all` running (Convex dev + Go API + Cloudflare tunnel)
 *   - CONVEX_URL env var set (defaults to dev deployment)
 *
 * Usage:
 *   const { client, userId, token } = await createAnonymousClient();
 *   const user = await client.query(api.users.getMe);
 *
 * Rate limiting: @convex-dev/auth rate-limits sign-in attempts.
 * Create ONE client per describe block, not per test.
 */

import { ConvexHttpClient } from "convex/browser";
import { anyApi } from "convex/server";
import { api } from "@bnto/backend/convex/_generated/api";

// ── Types ────────────────────────────────────────────────────────────────

/** Result of creating an authenticated ConvexHttpClient. */
export interface AuthenticatedClient {
  /** ConvexHttpClient with auth token set. */
  client: ConvexHttpClient;
  /** Convex user ID (same as users table _id). */
  userId: string;
  /** JWT access token. */
  token: string;
  /** Refresh token for extending the session. */
  refreshToken: string;
}

/**
 * Shape returned by @convex-dev/auth v0.0.90 signIn action.
 * The response is a flat `{ tokens }` object — userId is extracted from
 * the JWT `sub` claim (format: "userId|sessionId").
 */
interface SignInResult {
  tokens: { token: string; refreshToken: string } | null;
}

/**
 * Decode a JWT payload without verifying the signature.
 * Only used in tests to extract userId from the `sub` claim.
 */
function decodeJwtPayload(token: string): { sub: string; [key: string]: unknown } {
  const payload = token.split(".")[1];
  return JSON.parse(Buffer.from(payload, "base64url").toString());
}

/**
 * Extract the Convex userId from a JWT token.
 * The `sub` claim has format "userId|sessionId".
 */
function extractUserId(token: string): string {
  const { sub } = decodeJwtPayload(token);
  const userId = sub.split("|")[0];
  if (!userId) throw new Error(`Could not extract userId from JWT sub: ${sub}`);
  return userId;
}

// ── Environment ──────────────────────────────────────────────────────────

const DEFAULT_CONVEX_URL = "https://zealous-canary-422.convex.cloud";

/**
 * Returns the Convex deployment URL for integration tests.
 * Uses CONVEX_URL env var, falling back to the dev deployment.
 */
export function getConvexUrl(): string {
  return process.env.CONVEX_URL ?? DEFAULT_CONVEX_URL;
}

// ── Client Factories ─────────────────────────────────────────────────────

/**
 * Creates a ConvexHttpClient with a fresh anonymous session.
 *
 * Anonymous auth creates a real `users` row with `isAnonymous: true`.
 * The returned client can call any Convex function as this anonymous user.
 *
 * Rate limiting note: call once per describe block, reuse across tests.
 */
export async function createAnonymousClient(): Promise<AuthenticatedClient> {
  const client = new ConvexHttpClient(getConvexUrl());

  // anyApi bypasses generated type constraints — @convex-dev/auth's
  // signIn action types don't fully surface in the generated api.
  const result = (await client.action(anyApi.auth.signIn, {
    provider: "anonymous",
  })) as SignInResult;

  if (!result.tokens) {
    throw new Error(
      `Anonymous sign-in failed: ${JSON.stringify(result)}`,
    );
  }

  const { token, refreshToken } = result.tokens;
  client.setAuth(token);

  return {
    client,
    userId: extractUserId(token),
    token,
    refreshToken,
  };
}

/**
 * Creates a ConvexHttpClient authenticated with email/password.
 *
 * @param email - Test email (use generateTestEmail() for unique values)
 * @param password - Password (min 6 chars for @convex-dev/auth)
 * @param options.flow - "signUp" to create a new account, "signIn" to log in
 *
 * For conversion tests (anonymous -> real account), sign up with the same
 * client that was previously authenticated anonymously — @convex-dev/auth
 * will upgrade the existing user rather than creating a new one.
 */
export async function createPasswordClient(
  email: string,
  password: string,
  options: { flow: "signUp" | "signIn" } = { flow: "signUp" },
): Promise<AuthenticatedClient> {
  const client = new ConvexHttpClient(getConvexUrl());

  const result = (await client.action(anyApi.auth.signIn, {
    provider: "password",
    params: { email, password, flow: options.flow },
  })) as SignInResult;

  if (!result.tokens) {
    throw new Error(
      `Password sign-in failed (${options.flow}): ${JSON.stringify(result)}`,
    );
  }

  const { token, refreshToken } = result.tokens;
  client.setAuth(token);

  return {
    client,
    userId: extractUserId(token),
    token,
    refreshToken,
  };
}

/**
 * Upgrades an existing anonymous client to a password account.
 *
 * Uses the SAME ConvexHttpClient so @convex-dev/auth recognizes the existing
 * session and patches the user rather than creating a new one.
 * The userId should be preserved across the upgrade.
 *
 * @param anonClient - The authenticated anonymous client to upgrade
 * @param email - Email for the new account
 * @param password - Password (min 6 chars)
 * @returns Updated AuthenticatedClient with new tokens but same userId
 */
export async function upgradeAnonymousToPassword(
  anonClient: AuthenticatedClient,
  email: string,
  password: string,
): Promise<AuthenticatedClient> {
  const result = (await anonClient.client.action(anyApi.auth.signIn, {
    provider: "password",
    params: { email, password, flow: "signUp" },
  })) as SignInResult;

  if (!result.tokens) {
    throw new Error(
      `Anonymous upgrade failed: ${JSON.stringify(result)}`,
    );
  }

  const { token, refreshToken } = result.tokens;
  anonClient.client.setAuth(token);

  return {
    client: anonClient.client,
    userId: extractUserId(token),
    token,
    refreshToken,
  };
}

/**
 * Refreshes the auth token for a client with an expiring JWT.
 * Useful for long-running test suites (JWT expires in 1 hour).
 */
export async function refreshClientToken(
  authClient: AuthenticatedClient,
): Promise<AuthenticatedClient> {
  const result = (await authClient.client.action(anyApi.auth.signIn, {
    refreshToken: authClient.refreshToken,
  })) as SignInResult;

  if (!result.tokens) {
    throw new Error(
      `Token refresh failed: ${JSON.stringify(result)}`,
    );
  }

  authClient.client.setAuth(result.tokens.token);

  return {
    ...authClient,
    token: result.tokens.token,
    refreshToken: result.tokens.refreshToken,
  };
}

// ── Test Helpers ─────────────────────────────────────────────────────────

/**
 * Generates a unique test email address.
 * Uses timestamp + random suffix to avoid collisions across test runs.
 *
 * Convention: all test emails use the @test.bnto.dev domain
 * so they're easily identifiable in the dev database.
 */
export function generateTestEmail(): string {
  const ts = Date.now();
  const rand = Math.random().toString(36).slice(2, 8);
  return `test-${ts}-${rand}@test.bnto.dev`;
}

/**
 * Creates an unauthenticated ConvexHttpClient (no auth token set).
 * Useful for testing that unauthenticated requests are properly rejected.
 */
export function createUnauthenticatedClient(): ConvexHttpClient {
  return new ConvexHttpClient(getConvexUrl());
}

/**
 * Queries the current user via the authenticated client.
 * Returns null if not authenticated.
 */
export async function getCurrentUser(client: ConvexHttpClient) {
  return client.query(api.users.getMe);
}

// ── Re-exports ───────────────────────────────────────────────────────────

export { api };
export { ConvexHttpClient };
