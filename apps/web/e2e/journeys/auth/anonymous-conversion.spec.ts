import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Anonymous → password conversion E2E journeys (C1-C3 from journeys/auth.md)
 *
 * Tests the anonymous session lifecycle and the conversion to a real account.
 * The createOrUpdateUser callback in auth.ts preserves the same userId when
 * an anonymous user signs up with a password, so all existing data
 * (executions, run counts, events) stays associated with their account.
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
      // Anonymous users are "authenticated" in Convex, so the proxy redirects
      // them away from /signin. Set the signout signal cookie to bypass this.
      // Derive domain from the current page URL so this works on both
      // localhost and Vercel preview deployments.
      const { hostname } = new URL(page.url());
      await page.context().addCookies([
        {
          name: "bnto-signout",
          value: "1",
          path: "/",
          domain: hostname,
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

      // Wait for anonymous session to reconnect on /signin.
      // After full page navigation, ConvexProvider re-establishes the session
      // from cookies. The form's data-anon-uid attribute reflects when useAuth()
      // has the anonymous user data. Without this wait, Playwright fills and
      // submits faster than the session reconnects, so anonymousUserId is never
      // sent to the backend.
      const form = page.locator("form");
      await expect(form).not.toHaveAttribute("data-anon-uid", "", {
        timeout: 10000,
      });

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

      // Sign up — set signout signal to bypass proxy redirect for anonymous users.
      // Derive domain from the current page URL (localhost vs Vercel preview).
      const { hostname } = new URL(page.url());
      await page.context().addCookies([
        {
          name: "bnto-signout",
          value: "1",
          path: "/",
          domain: hostname,
        },
      ]);
      await page.goto("/signin");
      await page.getByRole("button", { name: "Sign up" }).click();

      // Wait for anonymous session to reconnect before filling form
      const form = page.locator("form");
      await expect(form).not.toHaveAttribute("data-anon-uid", "", {
        timeout: 10000,
      });

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
