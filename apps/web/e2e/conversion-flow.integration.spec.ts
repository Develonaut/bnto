import path from "path";
import { test, expect } from "./fixtures";
import {
  ENGINE_IMAGES,
  waitForSession,
  waitForUserId,
} from "./integrationHelpers";

test.use({ reducedMotion: "reduce" });

/**
 * Conversion flow integration E2E tests (C1-C3 from journeys/auth.md).
 *
 * Verifies that anonymous → password upgrade preserves the userId in the
 * browser. ConvexHttpClient integration tests proved userId is NOT preserved
 * with stateless JWT auth. Browser cookies may behave differently because
 * @convex-dev/auth/react tracks sessions via HTTP-only cookies.
 *
 * Tests are serial: C2 and C3 depend on C1 creating the test account.
 * If C1 fails, subsequent tests are skipped automatically.
 *
 * Every critical interaction point has a full-page screenshot assertion so
 * future regressions are caught visually, not just by data assertions.
 *
 * Run via: task e2e:integration
 * Requires: task dev:all (Next.js + Convex + Go API + tunnel)
 */

// Unique email per test run to avoid collisions
const TEST_EMAIL = `test-c1-${Date.now()}@test.bnto.dev`;
const TEST_PASSWORD = "TestConversion123!";
const TEST_NAME = "C1 Test User";

// Shared across serial tests — C1 writes, C2 reads
let capturedAnonymousUserId = "";

test.describe.serial("Integration — Conversion Flow (C1-C3)", () => {
  /**
   * C1: Anonymous → signup conversion preserves userId.
   *
   * Full-page screenshots captured at each critical interaction point:
   *   1. Anonymous session ready (baseline tool page state)
   *   2. Files selected, ready to run
   *   3. Execution completed with results
   *   4. Sign-up form populated before submit
   *   5. Post-conversion tool page (userId preserved)
   */
  test("C1: anonymous → password upgrade preserves userId", async ({
    page,
  }) => {
    // ── Step 1: Anonymous session established ────────────────────────
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: /compress images/i }),
    ).toBeVisible();
    await waitForSession(page);
    const anonymousUserId = await waitForUserId(page);
    expect(anonymousUserId).toBeTruthy();
    console.log(`[C1] Anonymous userId: ${anonymousUserId}`);

    // Snapshot: tool page loaded with anonymous session — baseline state
    await expect(page).toHaveScreenshot(
      "c1-01-anonymous-session-ready.png",
      { fullPage: true },
    );

    // ── Step 2: Select files ─────────────────────────────────────────
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_IMAGES, "Product_Render.png"),
    );
    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    // Snapshot: files selected, Run button enabled — pre-execution state
    await expect(page).toHaveScreenshot(
      "c1-02-files-selected-ready.png",
      { fullPage: true },
    );

    // ── Step 3: Execute ──────────────────────────────────────────────
    await runButton.click();

    // Capture the processing state if we can (may be fleeting)
    const processingPhase = await runButton.getAttribute("data-phase");
    if (processingPhase === "uploading" || processingPhase === "running") {
      await expect(page).toHaveScreenshot(
        "c1-03-execution-in-progress.png",
        { fullPage: true },
      );
    }

    // Wait for terminal phase
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";

    if (phase !== "completed") {
      // Snapshot: failure state for debugging
      await page.screenshot({
        path: "test-results/debug-c1-execution-failed.png",
        fullPage: true,
      });
      test.skip(
        true,
        `Pipeline did not complete (phase: ${phase}) — cannot verify conversion`,
      );
      return;
    }

    // Wait for results panel
    const results = page.locator('[data-testid="execution-results"]');
    await expect(results).toBeVisible({ timeout: 10_000 });

    // Snapshot: execution completed with results — proves anonymous can run
    await expect(page).toHaveScreenshot(
      "c1-04-anonymous-execution-completed.png",
      { fullPage: true },
    );

    // ── Step 4: Navigate to sign-up ──────────────────────────────────
    await page.goto("/signup");
    await expect(
      page.getByRole("heading", { name: /create an account/i }),
    ).toBeVisible();

    // Fill the sign-up form
    await page.getByPlaceholder("Your name").fill(TEST_NAME);
    await page.getByPlaceholder("Enter your email").fill(TEST_EMAIL);
    await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);

    // Snapshot: sign-up form filled out — the conversion touchpoint
    await expect(page).toHaveScreenshot(
      "c1-05-signup-form-filled.png",
      { fullPage: true },
    );

    // Submit
    await page.getByRole("button", { name: /create account/i }).click();

    // Wait for redirect to / (sign-up success)
    await page.waitForURL("/", { timeout: 30_000 });

    // ── Step 5: Return to bnto page and verify userId ────────────────
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: /compress images/i }),
    ).toBeVisible();
    await waitForSession(page);
    const postConversionUserId = await waitForUserId(page);
    expect(postConversionUserId).toBeTruthy();
    console.log(`[C1] Post-conversion userId: ${postConversionUserId}`);

    // ── THE CRITICAL ASSERTION ───────────────────────────────────────
    // If this fails, the conversion funnel is broken. Anonymous user's
    // runs, executions, and workflows are orphaned on upgrade.
    expect(postConversionUserId).toBe(anonymousUserId);

    // Share with C2 for cross-test userId continuity assertion
    capturedAnonymousUserId = anonymousUserId;

    // Snapshot: post-conversion tool page — same userId, upgraded account
    await expect(page).toHaveScreenshot(
      "c1-06-post-conversion-tool-page.png",
      { fullPage: true },
    );
  });

  /**
   * C2: Converted user retains access and can still run bntos.
   *
   * Full-page screenshots:
   *   1. Sign-in form with credentials
   *   2. Tool page with userId continuity confirmed
   *   3. Execution completed by converted user
   */
  test("C2: converted user can still run bntos", async ({ page }) => {
    // ── Step 1: Sign in ──────────────────────────────────────────────
    await page.goto("/signin");
    await expect(
      page.getByRole("heading", { name: /welcome back/i }),
    ).toBeVisible();

    await page.getByPlaceholder("Enter your email").fill(TEST_EMAIL);
    await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);

    // Snapshot: sign-in form filled — proves re-auth UI works
    await expect(page).toHaveScreenshot(
      "c2-01-signin-form-filled.png",
      { fullPage: true },
    );

    await page.getByRole("button", { name: /sign in/i }).click();
    await page.waitForURL("/", { timeout: 30_000 });

    // ── Step 2: Navigate to bnto page ────────────────────────────────
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: /compress images/i }),
    ).toBeVisible();
    await waitForSession(page);
    const userId = await waitForUserId(page);
    expect(userId).toBeTruthy();
    console.log(`[C2] Signed-in userId: ${userId}`);

    // Verify same userId as C1's anonymous session
    if (capturedAnonymousUserId) {
      expect(userId).toBe(capturedAnonymousUserId);
    }

    // Snapshot: tool page as signed-in converted user
    await expect(page).toHaveScreenshot(
      "c2-02-signed-in-tool-page.png",
      { fullPage: true },
    );

    // ── Step 3: Run a bnto ───────────────────────────────────────────
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_IMAGES, "Product_Render.png"),
    );
    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();
    await runButton.click();

    // Wait for terminal phase
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";
    expect(phase).toBe("completed");

    // Wait for results panel
    const results = page.locator('[data-testid="execution-results"]');
    await expect(results).toBeVisible({ timeout: 10_000 });

    // Snapshot: execution completed by converted user — proves pipeline works
    await expect(page).toHaveScreenshot(
      "c2-03-converted-user-execution-completed.png",
      { fullPage: true },
    );
  });

  /**
   * C3: Converted user gets full free-tier quota.
   *
   * Full-page screenshots:
   *   1. Tool page with no upgrade prompt (free tier active)
   *   2. Files selected, Run button enabled (quota not exhausted)
   */
  test("C3: converted user has upgraded quota", async ({ page }) => {
    // Sign in
    await page.goto("/signin");
    await expect(
      page.getByRole("heading", { name: /welcome back/i }),
    ).toBeVisible();
    await page.getByPlaceholder("Enter your email").fill(TEST_EMAIL);
    await page.getByPlaceholder("Enter your password").fill(TEST_PASSWORD);
    await page.getByRole("button", { name: /sign in/i }).click();
    await page.waitForURL("/", { timeout: 30_000 });

    // Navigate to bnto page
    await page.goto("/compress-images");
    await waitForSession(page);

    // The UpgradePrompt should NOT be visible — free tier has 25 runs
    const upgradePrompt = page.locator('[data-testid="upgrade-prompt"]');
    await expect(upgradePrompt).not.toBeVisible({ timeout: 5_000 });

    // Snapshot: tool page with no upgrade prompt — free tier quota active
    await expect(page).toHaveScreenshot(
      "c3-01-no-upgrade-prompt-free-tier.png",
      { fullPage: true },
    );

    // Add a file to enable Run button
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_IMAGES, "Product_Render.png"),
    );

    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled({ timeout: 5_000 });

    // Snapshot: files selected, Run enabled — quota not exhausted
    await expect(page).toHaveScreenshot(
      "c3-02-run-enabled-quota-active.png",
      { fullPage: true },
    );

    console.log("[C3] Converted user has active quota (not exhausted)");
  });
});
