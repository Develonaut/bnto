import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Anonymous → password conversion E2E journeys (C1-C3 from journeys/auth.md)
 *
 * Tests the anonymous session lifecycle and the conversion to a real account.
 *
 * KNOWN LIMITATION: @convex-dev/auth v0.0.90 does NOT preserve the anonymous
 * userId during password sign-up. The library creates a new user instead of
 * patching the anonymous one. This means userId changes on conversion —
 * the C1-C2 userId preservation test is marked fixme until this is fixed
 * (either via a custom auth callback or an upstream @convex-dev/auth fix).
 *
 * Impact: anonymous user's work (run history, saved state) is lost on sign-up.
 * This blocks the monetization funnel and must be fixed before Stripe (M5).
 */

function testEmail() {
  const id = `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
  return `e2e-conversion-${id}@test.bnto.dev`;
}

const TEST_PASSWORD = "ConvertMe123!";
const TEST_NAME = "Conversion Test User";

test.describe("Anonymous → password conversion", () => {
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

    // Capture the anonymous userId
    const userId = await shell.getAttribute("data-user-id");
    expect(userId).toBeTruthy();
    expect(userId!.length).toBeGreaterThan(0);

    await expect(page).toHaveScreenshot("00-anonymous-session-active.png");
  });

  // FIXME: @convex-dev/auth v0.0.90 creates a new user on password sign-up
  // instead of upgrading the anonymous one. The userId changes, breaking the
  // conversion funnel. This needs either:
  // - A custom createOrUpdateUser that links by email/device fingerprint
  // - An upstream fix in @convex-dev/auth for anonymous→password upgrade
  // - A post-signup migration that merges anonymous user data to the new user
  test.fixme(
    "C1-C2: userId preserved when anonymous user signs up",
    async ({ page }) => {
      const email = testEmail();

      // Step 1: Visit a tool page to get an anonymous session
      await page.goto("/compress-images");

      const shell = page.locator('[data-testid="bnto-shell"]');
      await expect(shell).toHaveAttribute("data-session", "ready", {
        timeout: 15000,
      });

      // Capture anonymous userId
      const anonymousUserId = await shell.getAttribute("data-user-id");
      expect(anonymousUserId).toBeTruthy();

      // Step 2: Navigate to sign-in and create a password account.
      // Anonymous users are "authenticated" in Convex, so the proxy redirects
      // them away from /signin. Set the signout signal cookie to bypass this.
      await page.context().addCookies([
        {
          name: "bnto-signout",
          value: "1",
          path: "/",
          domain: "localhost",
        },
      ]);
      await page.goto("/signin");
      await expect(
        page.getByRole("heading", { name: "Welcome back" }),
      ).toBeVisible();

      // Switch to sign-up mode
      await page.getByRole("button", { name: "Sign up" }).click();
      await expect(
        page.getByRole("heading", { name: "Create an account" }),
      ).toBeVisible();

      // Fill sign-up form
      await page.getByPlaceholder("Your name").fill(TEST_NAME);
      await page.getByPlaceholder("Enter your email").fill(email);
      await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);

      // Submit
      await page.getByRole("button", { name: "Create account" }).click();

      // Wait for redirect to home
      await page.waitForURL("/", { timeout: 15000 });

      // Step 3: Navigate back to a tool page to check the userId
      await page.goto("/compress-images");
      await expect(shell).toHaveAttribute("data-session", "ready", {
        timeout: 15000,
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

  // FIXME: Same root cause as C1-C2. After anonymous→password sign-up, the
  // Convex client session doesn't transition cleanly to the new password user.
  // NavUser shows "Sign In" because the auth token still references the
  // anonymous session. A clean "sign up as fresh user" flow works fine
  // (covered by auth-lifecycle.spec.ts S1 tests).
  test.fixme(
    "C3: signed-up user has correct profile after anonymous conversion",
    async ({ page }) => {
      const email = testEmail();

      // Create anonymous session on a tool page
      await page.goto("/compress-images");
      const shell = page.locator('[data-testid="bnto-shell"]');
      await expect(shell).toHaveAttribute("data-session", "ready", {
        timeout: 15000,
      });

      // Sign up — set signout signal to bypass proxy redirect for anonymous users
      await page.context().addCookies([
        {
          name: "bnto-signout",
          value: "1",
          path: "/",
          domain: "localhost",
        },
      ]);
      await page.goto("/signin");
      await page.getByRole("button", { name: "Sign up" }).click();
      await page.getByPlaceholder("Your name").fill(TEST_NAME);
      await page.getByPlaceholder("Enter your email").fill(email);
      await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
      await page.getByRole("button", { name: "Create account" }).click();
      await page.waitForURL("/", { timeout: 15000 });

      // After anonymous→password upgrade, the Convex client should transition
      // to the new session. Reload forces the server to re-read auth cookies.
      await page.reload();

      // Open user menu — should show email from the new account
      const userMenu = page.locator('[data-testid="nav-user-menu"]');
      await expect(userMenu).toBeVisible({ timeout: 10000 });
      await userMenu.click();

      await expect(page.getByText(email)).toBeVisible();

      // Sign-out button should be available
      await expect(
        page.locator('[data-testid="nav-sign-out"]'),
      ).toBeVisible();

      await expect(page).toHaveScreenshot("02-upgraded-user-menu.png");
    },
  );
});
