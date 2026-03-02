import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Anonymous → password conversion E2E journeys (C1-C3 from journeys/auth.md)
 *
 * Tests the anonymous session lifecycle and the conversion to a real account.
 * The PasswordWithAnonymousUpgrade wrapper in auth.ts preserves the same
 * userId when an anonymous user signs up with a password, so all existing
 * data (executions, run counts, events) stays associated with their account.
 */

function testEmail() {
  const id = `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
  return `e2e-conversion-${id}@test.bnto.dev`;
}

const TEST_PASSWORD = "ConvertMe123!";
const TEST_NAME = "Conversion Test User";

test.describe("Anonymous → password conversion @auth", () => {
  test("C1: anonymous session is created on tool page visit", async ({
    page,
  }) => {
    await page.goto("/compress-images");

    // Wait for the anonymous session to be established
    // The bnto-shell should get a data-user-id attribute once the session is active
    const shell = page.locator('[data-testid="bnto-shell"]');
    await expect(shell).toBeVisible({ timeout: 10000 });

    // Wait for session — data-session attribute indicates auth state
    await expect(shell).toHaveAttribute("data-session", "ready", {
      timeout: 15000,
    });

    // Wait for user query to load — data-user-id populates after the
    // session is ready because it depends on a separate React Query fetch
    await expect(shell).not.toHaveAttribute("data-user-id", "", {
      timeout: 10000,
    });

    // Capture the anonymous userId
    const userId = await shell.getAttribute("data-user-id");
    expect(userId).toBeTruthy();
    expect(userId!.length).toBeGreaterThan(0);

    await expect(page).toHaveScreenshot("00-anonymous-session-active.png");
  });

  test(
    "C1-C2: userId preserved when anonymous user signs up",
    async ({ page }) => {
      const email = testEmail();

      // Step 1: Visit a tool page to get an anonymous session
      await page.goto("/compress-images");

      const shell = page.locator('[data-testid="bnto-shell"]');
      await expect(shell).toHaveAttribute("data-session", "ready", {
        timeout: 15000,
      });

      // Wait for user query to load
      await expect(shell).not.toHaveAttribute("data-user-id", "", {
        timeout: 10000,
      });

      // Capture anonymous userId
      const anonymousUserId = await shell.getAttribute("data-user-id");
      expect(anonymousUserId).toBeTruthy();

      // Step 2: Navigate to sign-in and create a password account.
      // The proxy does NOT redirect anonymous users from /signin — auth pages
      // are public. SignInForm only redirects users with a real account (email),
      // so anonymous sessions pass through to the sign-up form.
      await page.goto("/signin");
      await expect(
        page.getByRole("heading", { name: "Welcome back" }),
      ).toBeVisible();

      // Switch to sign-up mode
      await page.getByRole("button", { name: "Sign up" }).click();
      await expect(
        page.getByRole("heading", { name: "Create an account" }),
      ).toBeVisible();

      // Fill sign-up form — can fill immediately, no need to wait.
      await page.getByPlaceholder("Your name").fill(TEST_NAME);
      await page.getByPlaceholder("Enter your email").fill(email);
      await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);

      // Wait for useSignUp to capture the anonymous userId before submitting.
      // The form renders data-anon-uid="" until the Convex session re-establishes
      // and the user query delivers the doc. Without this guard, the form can
      // submit before anonymousUserId is populated, sending undefined to the
      // backend which creates a fresh user instead of upgrading the anonymous one.
      const signUpForm = page.locator("form[data-anon-uid]");
      await expect(signUpForm).not.toHaveAttribute("data-anon-uid", "", {
        timeout: 15000,
      });

      // Submit — the anonymous userId is now captured and will be sent to the backend.
      await page.getByRole("button", { name: "Create account" }).click();

      // Wait for redirect to home — extra time since the form may wait
      // for the anonymous session to resolve before calling the API.
      await page.waitForURL("/", { timeout: 30000 });

      // Step 3: Navigate back to a tool page to check the userId
      await page.goto("/compress-images");
      await expect(shell).toHaveAttribute("data-session", "ready", {
        timeout: 15000,
      });

      // Wait for user query to load
      await expect(shell).not.toHaveAttribute("data-user-id", "", {
        timeout: 10000,
      });

      // Capture the upgraded userId
      const upgradedUserId = await shell.getAttribute("data-user-id");
      expect(upgradedUserId).toBeTruthy();

      // THE CRITICAL ASSERTION:
      // If this passes → browser cookies correctly preserve userId during upgrade
      // If this fails → monetization funnel is broken (userId changes, user loses work)
      expect(upgradedUserId).toBe(anonymousUserId);

      await expect(page).toHaveScreenshot("01-upgraded-user-preserved.png");
    },
  );

  test(
    "C3: signed-up user has correct profile after anonymous conversion",
    async ({ page }) => {
      const email = testEmail();

      // Create anonymous session on a tool page
      await page.goto("/compress-images");
      const shell = page.locator('[data-testid="bnto-shell"]');
      await expect(shell).toHaveAttribute("data-session", "ready", {
        timeout: 15000,
      });

      // Sign up — auth pages are public, no proxy redirect to bypass.
      // Anonymous sessions don't have an email, so SignInForm won't redirect them.
      await page.goto("/signin");
      await page.getByRole("button", { name: "Sign up" }).click();
      await page.getByPlaceholder("Your name").fill(TEST_NAME);
      await page.getByPlaceholder("Enter your email").fill(email);
      await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);

      // Wait for useSignUp to capture the anonymous userId before submitting
      const signUpForm = page.locator("form[data-anon-uid]");
      await expect(signUpForm).not.toHaveAttribute("data-anon-uid", "", {
        timeout: 15000,
      });

      await page.getByRole("button", { name: "Create account" }).click();

      // The form submission waits for the anonymous session to re-establish
      // before calling the signUp API (shows "Creating account..." spinner).
      // Give it extra time since the session may take a few seconds.
      await page.waitForURL("/", { timeout: 30000 });

      // After anonymous→password upgrade, the Convex client transitions to the
      // new session reactively — no reload needed. The signIn() call returns new
      // tokens which are stored client-side, and Convex subscriptions update the
      // user data. A page.reload() here would race the middleware's server-side
      // token refresh against the client-side auth state, causing intermittent
      // "Invalid refresh token" errors.

      // Open user menu — should show email from the new account.
      // Use a generous timeout because the Convex subscription needs to
      // re-establish and getMe needs to return the upgraded user.
      const userMenu = page.locator('[data-testid="nav-user-menu"]');
      await expect(userMenu).toBeVisible({ timeout: 15000 });
      await userMenu.click();

      await expect(page.getByText(email)).toBeVisible({ timeout: 10000 });

      // Sign-out button should be available
      await expect(
        page.locator('[data-testid="nav-sign-out"]'),
      ).toBeVisible();

      await expect(page).toHaveScreenshot("02-upgraded-user-menu.png");
    },
  );
});
