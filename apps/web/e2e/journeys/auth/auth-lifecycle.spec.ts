import { test, expect } from "../../fixtures";
import { testEmail, TEST_PASSWORD, TEST_NAME } from "../../accounts";

test.use({ reducedMotion: "reduce" });

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

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Sign up a new user and wait until we land on home. */
async function signUp(page: import("@playwright/test").Page, email: string) {
  await page.goto("/signin");

  // Fresh context has no bnto-has-account cookie → starts in signup mode
  await expect(page.getByRole("heading", { name: "Create an account" })).toBeVisible();

  await page.getByPlaceholder("Your name").fill(TEST_NAME);
  await page.getByPlaceholder("Enter your email").fill(email);
  await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
  await page.getByRole("button", { name: "Create account" }).click();
  await page.waitForURL("/", { timeout: 15000 });
}

/** Sign out via the NavUser dropdown and wait for /signin. */
async function signOut(page: import("@playwright/test").Page) {
  const userMenu = page.locator('[data-testid="nav-user-menu"]');
  await expect(userMenu).toBeVisible({ timeout: 10000 });
  await userMenu.click();
  await page.locator('[data-testid="nav-sign-out"]').click();
  await page.waitForURL("/signin", { timeout: 10000 });
}

// ---------------------------------------------------------------------------
// New user journey: first visit → sign up → authenticated home → sign out
// ---------------------------------------------------------------------------

test.describe("New user journey @auth", () => {
  test("fresh visitor sees signup form by default", async ({ page }) => {
    await page.goto("/signin");

    // No bnto-has-account cookie → defaults to signup mode
    await expect(page.getByRole("heading", { name: "Create an account" })).toBeVisible();
    await expect(page.getByPlaceholder("Your name")).toBeVisible();

    await expect(page).toHaveScreenshot("00-signup-form-default.png");
  });

  test("sign up → lands on home → sees user menu", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    await expect(page).toHaveURL("/");

    // NavUser shows authenticated state (user menu, not "Sign In")
    const userMenu = page.locator('[data-testid="nav-user-menu"]');
    await expect(userMenu).toBeVisible({ timeout: 10000 });

    await expect(page).toHaveScreenshot("01-signed-in-home.png");

    // Open menu — should display the user's email
    await userMenu.click();
    await expect(page.getByText(email)).toBeVisible();

    await expect(page).toHaveScreenshot("02-user-menu-open.png");
  });

  test("sign up → sign out → stays on /signin (no bounce)", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    // Sign out
    await signOut(page);

    // Should stay on /signin — NOT bounce back to /
    await expect(page).toHaveURL("/signin");

    // After sign-out, hasAccount persists in store → shows signin mode
    await expect(page.getByRole("heading", { name: "Welcome back" })).toBeVisible();

    await expect(page).toHaveScreenshot("03-signed-out-welcome-back.png");

    // Wait briefly for session cleanup, then confirm the user is truly signed out
    await page.waitForTimeout(2000);
    await page.goto("/");
    const userMenu = page.locator('[data-testid="nav-user-menu"]');
    await expect(userMenu).toBeVisible({ timeout: 10000 });
    await userMenu.click();
    await expect(page.locator('[data-testid="nav-sign-in"]')).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Returning user journey: has account → sign in → authenticated home
// ---------------------------------------------------------------------------

test.describe("Returning user journey @auth", () => {
  test("returning user sees signin form (persisted auth store)", async ({ page }) => {
    // Simulate a returning user by seeding the persisted auth store
    await page.goto("/signin");
    await page.evaluate(() => {
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
    await expect(page.getByRole("heading", { name: "Welcome back" })).toBeVisible();
    // Name field should NOT be visible in signin mode
    await expect(page.getByPlaceholder("Your name")).not.toBeVisible();
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
    await expect(page.getByRole("heading", { name: "Welcome back" })).toBeVisible();

    // Sign in with existing credentials
    await page.getByPlaceholder("Enter your email").fill(email);
    await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
    await page.getByRole("button", { name: "Sign in" }).click();

    // Should redirect to home
    await page.waitForURL("/", { timeout: 15000 });
    await expect(page).toHaveURL("/");

    // NavUser should show authenticated state
    await expect(page.locator('[data-testid="nav-user-menu"]')).toBeVisible({ timeout: 10000 });
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
    await expect(page.getByRole("heading", { name: "Welcome back" })).toBeVisible();

    await page.getByPlaceholder("Enter your email").fill(email);
    await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
    await page.getByRole("button", { name: "Sign in" }).click();
    await page.waitForURL("/", { timeout: 15000 });

    // 4. Confirm authenticated — wait for auth to fully resolve, then check email
    const signOutItem = page.locator('[data-testid="nav-sign-out"]');
    // Poll: open menu, check for sign-out item (proves auth resolved)
    await expect(async () => {
      const userMenu = page.locator('[data-testid="nav-user-menu"]');
      await userMenu.click();
      await expect(signOutItem).toBeVisible({ timeout: 1000 });
    }).toPass({ timeout: 15000 });
    await expect(page.getByText(email)).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Form behavior
// ---------------------------------------------------------------------------

test.describe("Auth form behavior @auth", () => {
  test("mode toggle switches between signin and signup", async ({ page }) => {
    await page.goto("/signin");

    // Default: signup mode (fresh context, no cookie)
    await expect(page.getByRole("heading", { name: "Create an account" })).toBeVisible();

    // Toggle to signin
    await page.getByRole("button", { name: "Sign in" }).click();
    await expect(page.getByRole("heading", { name: "Welcome back" })).toBeVisible();
    await expect(page.getByPlaceholder("Your name")).not.toBeVisible();

    // Toggle back to signup
    await page.getByRole("button", { name: "Sign up" }).click();
    await expect(page.getByRole("heading", { name: "Create an account" })).toBeVisible();
    await expect(page.getByPlaceholder("Your name")).toBeVisible();
  });

  test("switching to signup clears pre-filled email", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);
    await signOut(page);

    // After sign-out, persisted user email should pre-fill the signin form
    const emailInput = page.getByPlaceholder("Enter your email");
    await expect(emailInput).toHaveValue(email);

    // Toggle to signup — email should clear (new account = new email)
    await page.getByRole("button", { name: "Sign up" }).click();
    await expect(emailInput).toHaveValue("");

    // Toggle back to signin — email should re-fill from persisted user
    await page.getByRole("button", { name: "Sign in" }).click();
    await expect(emailInput).toHaveValue(email);
  });

  test("invalid credentials show error message", async ({ page }) => {
    await page.goto("/signin");

    // Switch to signin mode
    await page.getByRole("button", { name: "Sign in" }).click();

    await page.getByPlaceholder("Enter your email").fill("nonexistent@test.bnto.dev");
    await page.getByPlaceholder("Enter your password").fill("wrongpassword1");
    await page.getByRole("button", { name: "Sign in" }).click();

    const error = page.locator('p[role="alert"]');
    await expect(error).toBeVisible({ timeout: 10000 });
    await expect(error).toContainText("Invalid email or password");

    await expect(page).toHaveScreenshot("04-signin-error.png");
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
    await expect(page.getByRole("heading", { name: "Create an account" })).toBeVisible();

    // Try to sign up again with the same email — @convex-dev/auth silently
    // signs in the existing user rather than throwing a duplicate error
    await page.getByPlaceholder("Your name").fill("Another User");
    await page.getByPlaceholder("Enter your email").fill(email);
    await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
    await page.getByRole("button", { name: "Create account" }).click();

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
    await expect(page.getByRole("heading", { name: "Create an account" })).toBeVisible();
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

    const userMenu = page.locator('[data-testid="nav-user-menu"]');
    await expect(userMenu).toBeVisible({ timeout: 10000 });
    await userMenu.click();

    const signInButton = page.locator('[data-testid="nav-sign-in"]');
    await expect(signInButton).toBeVisible({ timeout: 10000 });

    await signInButton.click();
    await page.waitForURL("/signin", { timeout: 10000 });
  });
});
