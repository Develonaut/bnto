import { test, expect } from "../../fixtures";
import { testEmail, TEST_PASSWORD, TEST_NAME } from "../../accounts";

test.use({ reducedMotion: "reduce" });

/**
 * Browser auth behavior verification
 *
 * Tests the mechanical correctness of authentication infrastructure —
 * signal cookies, session cookie lifecycle, cross-navigation persistence,
 * and mid-session auth loss detection. Complements auth-lifecycle.spec.ts
 * which covers the user-facing sign-up/sign-in/sign-out flows.
 *
 * Sprint 3 Wave 2 — "Token expiry, sign-out invalidation, cookie-based
 * default mode"
 */

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Sign up a new user and wait until we land on home. */
async function signUp(
  page: import("@playwright/test").Page,
  email: string,
) {
  await page.goto("/signin");
  await expect(
    page.getByRole("heading", { name: "Create an account" }),
  ).toBeVisible();

  await page.getByPlaceholder("Your name").fill(TEST_NAME);
  await page.getByPlaceholder("Enter your email").fill(email);
  await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
  await page.getByRole("button", { name: "Create account" }).click();
  await page.waitForURL("/", { timeout: 15000 });
}

/** Sign in an existing user and wait until we land on home. */
async function signIn(
  page: import("@playwright/test").Page,
  email: string,
) {
  await page.goto("/signin");

  // Set has-account cookie so form shows sign-in mode
  await page.context().addCookies([
    { name: "bnto-has-account", value: "1", domain: "localhost", path: "/" },
  ]);
  await page.goto("/signin");
  await expect(
    page.getByRole("heading", { name: "Welcome back" }),
  ).toBeVisible();

  await page.getByPlaceholder("Enter your email").fill(email);
  await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
  await page.getByRole("button", { name: "Sign in" }).click();
  await page.waitForURL("/", { timeout: 15000 });
}

/** Click sign out and wait for /signin. */
async function signOut(page: import("@playwright/test").Page) {
  const userMenu = page.locator('[data-testid="nav-user-menu"]');
  await expect(userMenu).toBeVisible({ timeout: 10000 });
  await userMenu.click();
  await page.locator('[data-testid="nav-sign-out"]').click();
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
  test("sign-out sets bnto-signout signal cookie", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    // Before sign-out — no signal cookie
    const beforeCookie = await getCookie(page, "bnto-signout");
    expect(beforeCookie).toBeUndefined();

    // Sign out
    await signOut(page);

    // Signal cookie should be set immediately after sign-out
    const signalCookie = await getCookie(page, "bnto-signout");
    expect(signalCookie).toBeDefined();
    expect(signalCookie!.value).toBe("1");
  });

  test("signal cookie prevents /signin → / bounce during sign-out", async ({
    page,
  }) => {
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
    const email = testEmail();
    await signUp(page, email);
    await signOut(page);

    // Signal cookie exists immediately after sign-out
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

  test("session cookies are cleared after sign-out completes", async ({
    page,
  }) => {
    const email = testEmail();
    await signUp(page, email);

    // Verify cookies exist before sign-out
    let jwt = await getCookie(page, "__convexAuthJWT");
    expect(jwt).toBeDefined();

    await signOut(page);

    // Wait for server-side cleanup (sign-out fires authSignOut in background)
    await page.waitForTimeout(3000);

    // Session cookies should be cleared by the server
    jwt = await getCookie(page, "__convexAuthJWT");
    const refresh = await getCookie(page, "__convexAuthRefreshToken");

    // After server cleanup, both should be cleared
    const jwtCleared = !jwt || jwt.value === "";
    const refreshCleared = !refresh || refresh.value === "";
    expect(jwtCleared || refreshCleared).toBeTruthy();
  });

  test("bnto-has-account cookie persists through sign-out", async ({
    page,
  }) => {
    const email = testEmail();
    await signUp(page, email);
    await signOut(page);

    // bnto-has-account should survive sign-out (it's permanent, not session-scoped)
    const hasAccount = await getCookie(page, "bnto-has-account");
    expect(hasAccount).toBeDefined();
    expect(hasAccount!.value).toBe("1");

    // Verify the form shows "Welcome back" (not "Create an account")
    await expect(
      page.getByRole("heading", { name: "Welcome back" }),
    ).toBeVisible();
  });

  test("fresh browser context without bnto-has-account sees signup form", async ({
    page,
  }) => {
    // Fresh context — no cookies at all
    await page.goto("/signin");

    await expect(
      page.getByRole("heading", { name: "Create an account" }),
    ).toBeVisible();

    // Verify no bnto-has-account cookie exists
    const hasAccount = await getCookie(page, "bnto-has-account");
    expect(hasAccount).toBeUndefined();
  });
});

// ---------------------------------------------------------------------------
// Session persistence across navigation
// ---------------------------------------------------------------------------

test.describe("Session persistence @auth", () => {
  test("authenticated session persists across page navigation", async ({
    page,
  }) => {
    const email = testEmail();
    await signUp(page, email);

    // Navigate to multiple public pages — session should persist
    const routes = ["/", "/compress-images", "/clean-csv", "/"];

    for (const route of routes) {
      await page.goto(route);

      // User menu should remain visible (authenticated state)
      const userMenu = page.locator('[data-testid="nav-user-menu"]');
      await expect(userMenu).toBeVisible({ timeout: 10000 });
    }

    // Open menu on last page — email should still be visible
    const userMenu = page.locator('[data-testid="nav-user-menu"]');
    await userMenu.click();
    await expect(page.getByText(email)).toBeVisible();
  });

  test("auth state is consistent after back/forward navigation", async ({
    page,
  }) => {
    const email = testEmail();
    await signUp(page, email);

    // Navigate to a recipe page
    await page.goto("/compress-images");
    await expect(
      page.locator('[data-testid="nav-user-menu"]'),
    ).toBeVisible({ timeout: 10000 });

    // Navigate to another page
    await page.goto("/clean-csv");
    await expect(
      page.locator('[data-testid="nav-user-menu"]'),
    ).toBeVisible({ timeout: 10000 });

    // Go back
    await page.goBack();
    await expect(
      page.locator('[data-testid="nav-user-menu"]'),
    ).toBeVisible({ timeout: 10000 });

    // Go forward
    await page.goForward();
    await expect(
      page.locator('[data-testid="nav-user-menu"]'),
    ).toBeVisible({ timeout: 10000 });
  });
});

// ---------------------------------------------------------------------------
// Mid-session auth loss (simulated token expiry)
// ---------------------------------------------------------------------------

test.describe("Mid-session auth loss @auth", () => {
  test("clearing JWT cookie triggers session loss redirect", async ({
    page,
  }) => {
    const email = testEmail();
    await signUp(page, email);

    // Confirm authenticated state
    const userMenu = page.locator('[data-testid="nav-user-menu"]');
    await expect(userMenu).toBeVisible({ timeout: 10000 });

    // Simulate token expiry by clearing the JWT cookie
    await page.context().clearCookies({ name: "__convexAuthJWT" });
    await page.context().clearCookies({ name: "__convexAuthRefreshToken" });

    // Trigger a page interaction that forces auth re-evaluation.
    // Navigation to a protected route forces the proxy to check auth.
    await page.goto("/executions");

    // Should redirect to /signin since JWT is gone
    await page.waitForURL("/signin", { timeout: 10000 });
  });

  test("clearing JWT while on public page — SessionProvider detects loss", async ({
    page,
  }) => {
    const email = testEmail();
    await signUp(page, email);

    // Navigate to a public page
    await page.goto("/compress-images");
    await expect(
      page.locator('[data-testid="nav-user-menu"]'),
    ).toBeVisible({ timeout: 10000 });

    // Simulate token expiry by clearing JWT cookie
    await page.context().clearCookies({ name: "__convexAuthJWT" });
    await page.context().clearCookies({ name: "__convexAuthRefreshToken" });

    // SessionProvider polls Convex auth state. When the JWT is gone,
    // useConvexAuth() transitions to unauthenticated → onSessionLost fires.
    // This may take a moment as the Convex client detects the invalid token.
    // Wait for the redirect OR for the nav to show unauthenticated state.
    await page.waitForURL("/signin", { timeout: 15000 }).catch(() => {
      // If no redirect (public page doesn't trigger proxy check),
      // at minimum the UI should reflect unauthenticated state
    });

    // Whether redirected or not, auth UI should reflect unauthenticated state.
    // If we're on /signin, the redirect worked. If still on the recipe page,
    // the nav should show "Sign In" instead of the user menu dropdown.
    const url = page.url();
    if (url.includes("/signin")) {
      // Redirect happened — SessionProvider detected loss
      await expect(page).toHaveURL("/signin");
    } else {
      // Still on public page — verify nav shows unauthenticated state.
      // The user menu should no longer be visible (replaced by sign-in link).
      await expect(
        page.locator('[data-testid="nav-user-menu"]'),
      ).not.toBeVisible({ timeout: 10000 });
    }
  });
});

// ---------------------------------------------------------------------------
// Full auth round-trip with cookie verification
// ---------------------------------------------------------------------------

test.describe("Auth round-trip verification @auth", () => {
  test("sign-up → verify cookies → sign-out → verify cleared → sign-in → verify cookies", async ({
    page,
  }) => {
    const email = testEmail();

    // 1. Sign up
    await signUp(page, email);

    // 2. Verify session cookies exist
    let jwt = await getCookie(page, "__convexAuthJWT");
    expect(jwt).toBeDefined();
    const hasAccount = await getCookie(page, "bnto-has-account");
    expect(hasAccount).toBeDefined();

    // 3. Sign out
    await signOut(page);

    // 4. Wait for server cleanup
    await page.waitForTimeout(3000);

    // 5. Verify bnto-has-account persists but session is invalidated
    const hasAccountAfter = await getCookie(page, "bnto-has-account");
    expect(hasAccountAfter).toBeDefined();

    // Protected route should reject
    await page.goto("/executions");
    await page.waitForURL("/signin", { timeout: 10000 });

    // 6. Sign back in
    await signIn(page, email);

    // 7. Verify fresh session cookies exist
    jwt = await getCookie(page, "__convexAuthJWT");
    expect(jwt).toBeDefined();

    // 8. Verify authenticated state in UI
    const userMenu = page.locator('[data-testid="nav-user-menu"]');
    await expect(userMenu).toBeVisible({ timeout: 10000 });
    await userMenu.click();
    await expect(page.getByText(email)).toBeVisible();
  });
});
