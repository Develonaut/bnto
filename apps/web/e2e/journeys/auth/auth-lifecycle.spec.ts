import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Auth lifecycle E2E journeys (S1-S3 from journeys/auth.md)
 *
 * Tests the full sign-up, sign-in, sign-out flows in a real browser.
 * Verifies the NavUser component, proxy route protection, client-side
 * auth redirects (SignInForm), and session persistence.
 *
 * Each test uses a unique email to avoid conflicts with other test runs.
 * Emails use @test.bnto.dev domain for easy identification.
 */

function testEmail() {
  const id = `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
  return `e2e-${id}@test.bnto.dev`;
}

const TEST_PASSWORD = "TestPassword123!";
const TEST_NAME = "E2E Test User";

test.describe("Auth lifecycle — sign up, sign in, sign out @auth", () => {
  test("S1: sign up creates account and redirects to home", async ({
    page,
  }) => {
    const email = testEmail();

    // Navigate to sign-in page
    await page.goto("/signin");
    await expect(
      page.getByRole("heading", { name: "Welcome back" }),
    ).toBeVisible();

    // Switch to sign-up mode
    await page.getByRole("button", { name: "Sign up" }).click();
    await expect(
      page.getByRole("heading", { name: "Create an account" }),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("00-signup-form.png");

    // Fill form
    await page.getByPlaceholder("Your name").fill(TEST_NAME);
    await page.getByPlaceholder("Enter your email").fill(email);
    await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);

    // Submit
    await page.getByRole("button", { name: "Create account" }).click();

    // Should redirect to home after successful sign-up
    await page.waitForURL("/", { timeout: 15000 });
    await expect(page).toHaveURL("/");

    // NavUser should show user menu icon (not "Sign In" button)
    const userMenu = page.locator('[data-testid="nav-user-menu"]');
    await expect(userMenu).toBeVisible({ timeout: 10000 });

    await expect(page).toHaveScreenshot("01-signed-in-home.png");

    // Open user menu — should show email (name storage depends on auth provider)
    await userMenu.click();
    await expect(page.getByText(email)).toBeVisible();

    await expect(page).toHaveScreenshot("02-user-menu-open.png");
  });

  test("S1: sign in with existing account works", async ({ page }) => {
    const email = testEmail();

    // First, create the account
    await page.goto("/signin");
    await page.getByRole("button", { name: "Sign up" }).click();
    await page.getByPlaceholder("Your name").fill(TEST_NAME);
    await page.getByPlaceholder("Enter your email").fill(email);
    await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
    await page.getByRole("button", { name: "Create account" }).click();
    await page.waitForURL("/", { timeout: 15000 });

    // Clear cookies to simulate a fresh browser visit.
    // The account still exists in Convex — only the local session is cleared.
    await page.context().clearCookies();
    await page.goto("/signin");

    // Now sign back in
    await page.getByPlaceholder("Enter your email").fill(email);
    await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
    await page.getByRole("button", { name: "Sign in" }).click();

    // Should redirect to home
    await page.waitForURL("/", { timeout: 15000 });
    await expect(page).toHaveURL("/");

    // NavUser should show user menu
    await expect(
      page.locator('[data-testid="nav-user-menu"]'),
    ).toBeVisible({ timeout: 10000 });
  });

  test("S1: invalid credentials show error", async ({ page }) => {
    await page.goto("/signin");

    await page.getByPlaceholder("Enter your email").fill("nonexistent@test.bnto.dev");
    await page.getByPlaceholder("Enter your password").fill("wrongpassword1");
    await page.getByRole("button", { name: "Sign in" }).click();

    // Error message should appear (use p[role=alert] to avoid matching Next.js route announcer)
    const error = page.locator('p[role="alert"]');
    await expect(error).toBeVisible({ timeout: 10000 });
    await expect(error).toContainText("Invalid email or password");

    await expect(page).toHaveScreenshot("03-signin-error.png");
  });

  test("S2: sign out clears session and redirects to signin", async ({
    page,
  }) => {
    const email = testEmail();

    // Create account and land on home
    await page.goto("/signin");
    await page.getByRole("button", { name: "Sign up" }).click();
    await page.getByPlaceholder("Your name").fill(TEST_NAME);
    await page.getByPlaceholder("Enter your email").fill(email);
    await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
    await page.getByRole("button", { name: "Create account" }).click();
    await page.waitForURL("/", { timeout: 15000 });

    // Confirm authenticated state
    const userMenu = page.locator('[data-testid="nav-user-menu"]');
    await expect(userMenu).toBeVisible({ timeout: 10000 });

    // Sign out via user menu
    await userMenu.click();
    await page.locator('[data-testid="nav-sign-out"]').click();

    // Should land on /signin (navbar hidden on auth screens)
    await page.waitForURL("/signin", { timeout: 10000 });
    await expect(
      page.getByRole("heading", { name: "Welcome back" }),
    ).toBeVisible();

    await expect(page).toHaveScreenshot("04-signed-out.png");

    // Navigate to home — Sign In option should be in user menu
    await page.goto("/");
    const userMenu2 = page.locator('[data-testid="nav-user-menu"]');
    await expect(userMenu2).toBeVisible({ timeout: 10000 });
    await userMenu2.click();
    await expect(
      page.locator('[data-testid="nav-sign-in"]'),
    ).toBeVisible();
  });

  test("S3: navbar Sign In button navigates to /signin", async ({ page }) => {
    await page.goto("/");

    // Wait for NavUser to settle from loading state, then open menu
    const userMenu = page.locator('[data-testid="nav-user-menu"]');
    await expect(userMenu).toBeVisible({ timeout: 10000 });
    await userMenu.click();

    // Sign In is inside the user menu dropdown
    const signInButton = page.locator('[data-testid="nav-sign-in"]');
    await expect(signInButton).toBeVisible();

    await signInButton.click();
    await page.waitForURL("/signin", { timeout: 10000 });
    await expect(
      page.getByRole("heading", { name: "Welcome back" }),
    ).toBeVisible();
  });
});

test.describe("Auth — proxy route protection @auth", () => {
  test("unauthenticated user redirected from protected route", async ({
    page,
  }) => {
    // Try to visit a protected route without auth
    await page.goto("/executions");

    // Should be redirected to /signin
    await page.waitForURL("/signin", { timeout: 10000 });
    await expect(
      page.getByRole("heading", { name: "Welcome back" }),
    ).toBeVisible();
  });

  test("authenticated user redirected from /signin to / (client-side)", async ({
    page,
  }) => {
    const email = testEmail();

    // Create account
    await page.goto("/signin");
    await page.getByRole("button", { name: "Sign up" }).click();
    await page.getByPlaceholder("Your name").fill(TEST_NAME);
    await page.getByPlaceholder("Enter your email").fill(email);
    await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
    await page.getByRole("button", { name: "Create account" }).click();
    await page.waitForURL("/", { timeout: 15000 });

    // Now try to visit /signin — should be redirected to / by SignInForm's
    // useEffect (client-side). The proxy does NOT redirect auth users from
    // /signin because Convex anonymous sessions make isAuthenticated unreliable.
    // SignInForm checks for a real account (hasAccount = isAuthenticated && !user?.isAnonymous)
    // and calls router.replace("/") when detected.
    await page.goto("/signin");
    await page.waitForURL("/", { timeout: 10000 });
  });
});
