import { test, expect } from "../../fixtures";
import { testEmail, TEST_PASSWORD, TEST_NAME } from "../../accounts";

// Auth UI removed in open-source-first positioning.
// Follow-up PR will delete auth routes and these test files.
test.skip();

/**
 * Browser auth behavior verification
 *
 * Tests the mechanical correctness of authentication infrastructure —
 * signal cookies, session cookie lifecycle, cross-navigation persistence,
 * and mid-session auth loss detection. Complements auth-lifecycle.spec.ts
 * which covers the user-facing sign-up/sign-in/sign-out flows.
 *
 * Note: NavUser UI was removed in the open-source-first positioning.
 * Sign-out is simulated programmatically (clear cookies + signal cookie).
 * Tests that verified NavUser visibility have been removed.
 */

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Sign up a new user and wait until we land on home. */
async function signUp(page: import("@playwright/test").Page, email: string) {
  await page.goto("/signin");
  const authHeading = page.getByTestId("auth-heading");
  await expect(authHeading).toBeVisible();
  await expect(authHeading).toContainText("Create an account");

  await page.getByTestId("auth-name-input").fill(TEST_NAME);
  await page.getByTestId("auth-email-input").fill(email);
  await page.getByTestId("auth-password-input").fill(TEST_PASSWORD);
  await page.getByTestId("auth-submit").click();
  await page.waitForURL("/", { timeout: 15000 });
}

/** Programmatic sign-out — clears session cookies + sets signal cookie.
 *  NavUser UI was removed; this simulates the cookie-level effects. */
async function signOut(page: import("@playwright/test").Page) {
  await page.evaluate(() => {
    document.cookie = "bnto-signout=1; path=/; max-age=10; samesite=lax";
  });
  await page.context().clearCookies({ name: "__convexAuthJWT" });
  await page.context().clearCookies({ name: "__convexAuthRefreshToken" });
  await page.goto("/signin");
  await page.waitForURL("/signin", { timeout: 10000 });
}

/** Get all cookies for the current page context. */
async function getCookies(page: import("@playwright/test").Page) {
  return page.context().cookies();
}

/** Find a specific cookie by name. */
async function getCookie(page: import("@playwright/test").Page, name: string) {
  const cookies = await getCookies(page);
  return cookies.find((c) => c.name === name);
}

// ---------------------------------------------------------------------------
// Sign-out signal cookie mechanics
// ---------------------------------------------------------------------------

test.describe("Sign-out signal cookie @auth", () => {
  test("signal cookie prevents /signin → / bounce during sign-out", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    // Sign out — should land on /signin without bouncing
    await signOut(page);
    await expect(page).toHaveURL("/signin");

    // Navigate to /signin again while signal cookie is still active —
    // should NOT redirect to / despite stale session cookie
    await page.goto("/signin");
    await expect(page).toHaveURL("/signin");
  });

  test("signal cookie expires after ~10 seconds", async ({ page }) => {
    // Set signal cookie directly to test TTL
    await page.goto("/signin");
    await page.evaluate(() => {
      document.cookie = "bnto-signout=1; path=/; max-age=10; samesite=lax";
    });

    // Signal cookie exists immediately
    let signalCookie = await getCookie(page, "bnto-signout");
    expect(signalCookie).toBeDefined();

    // Wait for the 10-second TTL to expire
    await page.waitForTimeout(11000);

    // Signal cookie should be gone
    signalCookie = await getCookie(page, "bnto-signout");
    expect(signalCookie).toBeUndefined();
  });
});

// ---------------------------------------------------------------------------
// Session cookie lifecycle (sign-out invalidation)
// ---------------------------------------------------------------------------

test.describe("Session cookie lifecycle @auth", () => {
  test("session cookies exist after sign-up", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    const jwt = await getCookie(page, "__convexAuthJWT");
    const refresh = await getCookie(page, "__convexAuthRefreshToken");

    expect(jwt).toBeDefined();
    expect(refresh).toBeDefined();
  });

  test("hasAccount persists through sign-out (localStorage store)", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);
    await signOut(page);

    // hasAccount in the persisted auth store should survive sign-out
    const storeData = await page.evaluate(() => localStorage.getItem("bnto-auth"));
    expect(storeData).not.toBeNull();
    const parsed = JSON.parse(storeData!);
    expect(parsed.state.hasAccount).toBe(true);

    // Verify the form shows "Welcome back" (not "Create an account")
    const authHeading = page.getByTestId("auth-heading");
    await expect(authHeading).toBeVisible();
    await expect(authHeading).toContainText("Welcome back");
  });

  test("fresh browser context without auth store sees signup form", async ({ page }) => {
    // Fresh context — no localStorage at all
    await page.goto("/signin");

    const authHeading = page.getByTestId("auth-heading");
    await expect(authHeading).toBeVisible();
    await expect(authHeading).toContainText("Create an account");

    // Verify auth store has default state (Zustand persist writes defaults on init)
    const storeData = await page.evaluate(() => localStorage.getItem("bnto-auth"));
    expect(storeData).not.toBeNull();
    const parsed = JSON.parse(storeData!);
    expect(parsed.state.hasAccount).toBe(false);
    expect(parsed.state.user).toBeNull();
  });
});

// ---------------------------------------------------------------------------
// Mid-session auth loss (simulated token expiry)
// ---------------------------------------------------------------------------

test.describe("Mid-session auth loss @auth", () => {
  test("clearing JWT cookie triggers session loss redirect", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    // Simulate token expiry by clearing the JWT cookie
    await page.context().clearCookies({ name: "__convexAuthJWT" });
    await page.context().clearCookies({ name: "__convexAuthRefreshToken" });

    // Break the Convex WebSocket connection so it can't re-issue cookies
    await page.goto("about:blank");

    // Navigation to a protected route forces the proxy to check auth.
    await page.goto("/executions");

    // Should redirect to /signin with returnTo since JWT is gone
    await page.waitForURL("**/signin?returnTo=**", { timeout: 10000 });
  });

  test("clearing JWT while on public page — proxy catches on next navigation", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    // Navigate to a public page
    await page.goto("/compress-images");

    // Simulate token expiry by clearing JWT cookie
    await page.context().clearCookies({ name: "__convexAuthJWT" });
    await page.context().clearCookies({ name: "__convexAuthRefreshToken" });

    // Break the Convex WebSocket connection so it can't re-issue cookies
    await page.goto("about:blank");

    // Navigate to a protected route — proxy should catch the missing JWT
    await page.goto("/executions");
    await page.waitForURL("**/signin?returnTo=**", { timeout: 10000 });
  });
});
