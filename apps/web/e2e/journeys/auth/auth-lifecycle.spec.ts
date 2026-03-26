import { test, expect } from "../../fixtures";
import { testEmail, TEST_PASSWORD, TEST_NAME } from "../../accounts";

/**
 * Auth lifecycle E2E journeys
 *
 * Tests the full sign-up, sign-in, sign-out flows in a real browser.
 * Verifies form mode defaults, proxy redirects, session persistence,
 * and that users land on the correct screens at every step.
 *
 * Each test uses a unique email to avoid conflicts with other test runs.
 * Emails use @test.bnto.dev domain — cleaned up by global teardown.
 */

test.use({ reducedMotion: "reduce" });

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

/** Sign out via the NavUser dropdown and wait for /signin. */
async function signOut(page: import("@playwright/test").Page) {
  const userMenu = page.getByTestId("nav-user-menu");
  await expect(userMenu).toBeVisible({ timeout: 10000 });
  await userMenu.click();
  await page.getByTestId("nav-sign-out").click();
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

  test("sign up → lands on home → sees user menu", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    await expect(page).toHaveURL("/");

    // NavUser shows authenticated state (user menu, not "Sign In")
    const userMenu = page.getByTestId("nav-user-menu");
    await expect(userMenu).toBeVisible({ timeout: 10000 });

    // Open menu — should display the user's email
    await userMenu.click();
    await expect(page.getByTestId("nav-user-email")).toBeVisible();
    await expect(page.getByTestId("nav-user-email")).toContainText(email);
  });

  test("sign up → sign out → stays on /signin (no bounce)", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    // Sign out
    await signOut(page);

    // Should stay on /signin — NOT bounce back to /
    await expect(page).toHaveURL("/signin");

    // After sign-out, hasAccount persists in store → shows signin mode
    const authHeading = page.getByTestId("auth-heading");
    await expect(authHeading).toBeVisible();
    await expect(authHeading).toContainText("Welcome back");

    // Wait briefly for session cleanup, then confirm the user is truly signed out
    await page.waitForTimeout(2000);
    await page.goto("/");
    const userMenu = page.getByTestId("nav-user-menu");
    await expect(userMenu).toBeVisible({ timeout: 10000 });
    await userMenu.click();
    await expect(page.getByTestId("nav-sign-in")).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Returning user journey: has account → sign in → authenticated home
// ---------------------------------------------------------------------------

test.describe("Returning user journey @auth", () => {
  test("returning user sees signin form (persisted auth store)", async ({ page }) => {
    // Seed localStorage via addInitScript so it runs BEFORE page scripts.
    // This avoids race conditions with Zustand persist: if we seed after
    // page load, async state changes (e.g. SessionProvider) trigger persist
    // writes that can overwrite the seeded value before the next navigation.
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

    // Clear session cookies (simulates browser restart) but keep bnto-has-account
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

    // NavUser should show authenticated state
    await expect(page.getByTestId("nav-user-menu")).toBeVisible({ timeout: 10000 });
  });

  test("sign out → sign back in → full round-trip", async ({ page }) => {
    const email = testEmail();

    // 1. Sign up (creates account + sets bnto-has-account cookie)
    await signUp(page, email);
    await expect(page).toHaveURL("/");

    // 2. Sign out
    await signOut(page);
    await expect(page).toHaveURL("/signin");

    // 3. Sign back in — bnto-has-account cookie present → signin mode shown
    const authHeading = page.getByTestId("auth-heading");
    await expect(authHeading).toBeVisible();
    await expect(authHeading).toContainText("Welcome back");

    await page.getByTestId("auth-email-input").fill(email);
    await page.getByTestId("auth-password-input").fill(TEST_PASSWORD);
    await page.getByTestId("auth-submit").click();
    await page.waitForURL("/", { timeout: 15000 });

    // 4. Confirm authenticated — wait for auth to fully resolve, then check email
    const signOutItem = page.getByTestId("nav-sign-out");
    // Poll: open menu, check for sign-out item (proves auth resolved)
    await expect(async () => {
      const userMenu = page.getByTestId("nav-user-menu");
      await userMenu.click();
      await expect(signOutItem).toBeVisible({ timeout: 1000 });
    }).toPass({ timeout: 15000 });
    await expect(page.getByTestId("nav-user-email")).toBeVisible();
    await expect(page.getByTestId("nav-user-email")).toContainText(email);
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

  test("switching to signup clears pre-filled email", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);
    await signOut(page);

    // After sign-out, persisted user email should pre-fill the signin form
    const emailInput = page.getByTestId("auth-email-input");
    await expect(emailInput).toHaveValue(email);

    // Toggle to signup — email should clear (new account = new email)
    await page.getByTestId("auth-mode-toggle").click();
    await expect(emailInput).toHaveValue("");

    // Toggle back to signin — email should re-fill from persisted user
    await page.getByTestId("auth-mode-toggle").click();
    await expect(emailInput).toHaveValue(email);
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

    // Clear session and persisted auth store, then go back to signin
    await page.context().clearCookies();
    await page.evaluate(() => localStorage.removeItem("bnto-auth"));
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
  test("unauthenticated user redirected from protected route to /signin", async ({ page }) => {
    await page.goto("/executions");
    await page.waitForURL("/signin", { timeout: 10000 });

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

    // Sign out from home (protected pages like /settings don't have a
    // rendered page yet — sign out from a page that works reliably)
    await signOut(page);

    // Wait for session cookie to clear server-side
    await page.waitForTimeout(2000);

    // Protected route should now redirect to /signin
    await page.goto("/executions");
    await page.waitForURL("/signin", { timeout: 10000 });
  });

  test("navbar Sign In navigates to /signin", async ({ page }) => {
    await page.goto("/");

    const userMenu = page.getByTestId("nav-user-menu");
    await expect(userMenu).toBeVisible({ timeout: 10000 });
    await userMenu.click();

    const signInButton = page.getByTestId("nav-sign-in");
    await expect(signInButton).toBeVisible({ timeout: 10000 });

    await signInButton.click();
    await page.waitForURL("/signin", { timeout: 10000 });
  });
});
