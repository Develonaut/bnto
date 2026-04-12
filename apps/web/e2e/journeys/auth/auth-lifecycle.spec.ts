import { test, expect } from "../../fixtures";
import { testEmail, TEST_PASSWORD, TEST_NAME } from "../../accounts";

// Auth UI removed in open-source-first positioning.
// Follow-up PR will delete auth routes and these test files.
test.skip();

/**
 * Auth lifecycle E2E journeys
 *
 * Tests the full sign-up, sign-in, sign-out flows in a real browser.
 * Verifies form mode defaults, proxy redirects, session persistence,
 * and that users land on the correct screens at every step.
 *
 * Each test uses a unique email to avoid conflicts with other test runs.
 * Emails use @test.bnto.dev domain — cleaned up by global teardown.
 *
 * Note: NavUser UI was removed in the open-source-first positioning.
 * Sign-out is simulated programmatically (clear cookies + signal cookie).
 */

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Sign up a new user and wait until we land on home. */
async function signUp(page: import("@playwright/test").Page, email: string) {
  await page.goto("/signin");

  // Fresh context has no bnto-has-account cookie → starts in signup mode
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

// ---------------------------------------------------------------------------
// New user journey: first visit → sign up → authenticated home → sign out
// ---------------------------------------------------------------------------

test.describe("New user journey @auth", () => {
  test("fresh visitor sees signup form by default", async ({ page }) => {
    await page.goto("/signin");

    // No bnto-has-account cookie → defaults to signup mode
    const authHeading = page.getByTestId("auth-heading");
    await expect(authHeading).toBeVisible();
    await expect(authHeading).toContainText("Create an account");
    await expect(page.getByTestId("auth-name-input")).toBeVisible();
  });

  test("sign up → lands on home", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);
    await expect(page).toHaveURL("/");
  });

  test("sign up → sign out → stays on /signin (no bounce)", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    // Sign out
    await signOut(page);

    // Should stay on /signin — NOT bounce back to /
    await expect(page).toHaveURL("/signin");
  });
});

// ---------------------------------------------------------------------------
// Returning user journey: has account → sign in → authenticated home
// ---------------------------------------------------------------------------

test.describe("Returning user journey @auth", () => {
  test("returning user sees signin form (persisted auth store)", async ({ page }) => {
    // Seed localStorage via addInitScript so it runs BEFORE page scripts.
    await page.addInitScript(() => {
      const state = {
        state: {
          user: { id: "seed", name: "Test User", email: "test@example.com", image: null },
          hasAccount: true,
        },
        version: 0,
      };
      localStorage.setItem("bnto-auth", JSON.stringify(state));
    });

    await page.goto("/signin");

    // Should show "Welcome back" (signin mode), not "Create an account"
    const authHeading = page.getByTestId("auth-heading");
    await expect(authHeading).toBeVisible();
    await expect(authHeading).toContainText("Welcome back");
    // Name field should NOT be visible in signin mode
    await expect(page.getByTestId("auth-name-input")).not.toBeVisible();
  });

  test("sign in with existing account → lands on home", async ({ page }) => {
    const email = testEmail();

    // Create the account first
    await signUp(page, email);

    // Break the Convex WebSocket connection so it can't re-issue cookies
    await page.goto("about:blank");

    // Clear session cookies (simulates browser restart) but keep localStorage
    await page.context().clearCookies({ name: "__convexAuthJWT" });
    await page.context().clearCookies({ name: "__convexAuthRefreshToken" });
    await page.goto("/signin");

    // bnto-has-account cookie still present → signin mode
    const authHeading = page.getByTestId("auth-heading");
    await expect(authHeading).toBeVisible();
    await expect(authHeading).toContainText("Welcome back");

    // Sign in with existing credentials
    await page.getByTestId("auth-email-input").fill(email);
    await page.getByTestId("auth-password-input").fill(TEST_PASSWORD);
    await page.getByTestId("auth-submit").click();

    // Should redirect to home
    await page.waitForURL("/", { timeout: 15000 });
    await expect(page).toHaveURL("/");
  });
});

// ---------------------------------------------------------------------------
// Form behavior
// ---------------------------------------------------------------------------

test.describe("Auth form behavior @auth", () => {
  test("mode toggle switches between signin and signup", async ({ page }) => {
    await page.goto("/signin");

    // Default: signup mode (fresh context, no cookie)
    const authHeading = page.getByTestId("auth-heading");
    await expect(authHeading).toBeVisible();
    await expect(authHeading).toContainText("Create an account");

    // Toggle to signin
    await page.getByTestId("auth-mode-toggle").click();
    await expect(authHeading).toContainText("Welcome back");
    await expect(page.getByTestId("auth-name-input")).not.toBeVisible();

    // Toggle back to signup
    await page.getByTestId("auth-mode-toggle").click();
    await expect(authHeading).toContainText("Create an account");
    await expect(page.getByTestId("auth-name-input")).toBeVisible();
  });

  test("invalid credentials show error message", async ({ page }) => {
    await page.goto("/signin");

    // Switch to signin mode
    await page.getByTestId("auth-mode-toggle").click();

    await page.getByTestId("auth-email-input").fill("nonexistent@test.bnto.dev");
    await page.getByTestId("auth-password-input").fill("wrongpassword1");
    await page.getByTestId("auth-submit").click();

    const error = page.getByTestId("auth-error");
    await expect(error).toBeVisible({ timeout: 10000 });
    await expect(error).toContainText("Invalid email or password");
  });

  test("signup with existing email signs in instead of erroring", async ({ page }) => {
    const email = testEmail();

    // Create the account first
    await signUp(page, email);

    // Clear persisted auth store (must be on app origin before navigating away)
    await page.evaluate(() => localStorage.removeItem("bnto-auth"));

    // Break the Convex WebSocket connection so it can't re-issue cookies
    await page.goto("about:blank");

    // Clear all cookies (safe now — no active WebSocket to re-issue them)
    await page.context().clearCookies();
    await page.goto("/signin");

    // Should show signup form (no persisted auth → fresh visitor)
    const authHeading = page.getByTestId("auth-heading");
    await expect(authHeading).toBeVisible();
    await expect(authHeading).toContainText("Create an account");

    // Try to sign up again with the same email — @convex-dev/auth silently
    // signs in the existing user rather than throwing a duplicate error
    await page.getByTestId("auth-name-input").fill("Another User");
    await page.getByTestId("auth-email-input").fill(email);
    await page.getByTestId("auth-password-input").fill(TEST_PASSWORD);
    await page.getByTestId("auth-submit").click();

    // Should redirect to home (signed in successfully)
    await page.waitForURL("/", { timeout: 15000 });
    await expect(page).toHaveURL("/");
  });
});

// ---------------------------------------------------------------------------
// Proxy route protection
// ---------------------------------------------------------------------------

test.describe("Proxy route protection @auth", () => {
  test("unauthenticated user redirected from protected route to /signin with returnTo", async ({
    page,
  }) => {
    await page.goto("/executions");
    await page.waitForURL("**/signin?returnTo=**", { timeout: 10000 });

    // Verify returnTo param preserves original path
    const url = new URL(page.url());
    expect(url.searchParams.get("returnTo")).toBe("/executions");

    // Fresh context → signup mode
    const authHeading = page.getByTestId("auth-heading");
    await expect(authHeading).toBeVisible();
    await expect(authHeading).toContainText("Create an account");
  });

  test("authenticated user redirected from /signin to / (client-side)", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    // Try to visit /signin while authenticated — proxy redirects to /
    await page.goto("/signin");
    await page.waitForURL("/", { timeout: 10000 });
  });

  test("sign-out invalidates access to protected routes", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    // Sign out (programmatic)
    await signOut(page);

    // Wait for session cookie to clear server-side
    await page.waitForTimeout(2000);

    // Protected route should now redirect to /signin with returnTo
    await page.goto("/executions");
    await page.waitForURL("**/signin?returnTo=**", { timeout: 10000 });
  });
});
