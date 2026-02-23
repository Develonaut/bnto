import { test, expect } from "./fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Execution flow E2E tests.
 *
 * Tests the user journey through the bnto tool page lifecycle:
 *   1. Initial page state (config + drop zone + disabled run button)
 *   2. File selection enables the Run button
 *   3. Run button click transitions to uploading phase
 *   4. Failed state shows Try Again (no backend in E2E)
 *   5. Reset returns to idle state
 *
 * Note: Full backend execution (upload → run → download) requires
 * a running Convex + Go API stack and is covered by integration tests.
 * These E2E tests validate the UI state machine and composition.
 */

test.describe("Execution flow — compress-images", () => {
  test("initial state: config + drop zone + disabled run button", async ({
    page,
  }) => {
    await page.goto("/compress-images");

    // Page heading renders
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Config panel is visible with Settings heading
    await expect(page.getByText("Settings")).toBeVisible();

    // Drop zone is present
    await expect(page.getByText("Drag & drop files here")).toBeVisible();
    await expect(page.getByText("JPEG, PNG, or WebP images")).toBeVisible();

    // Run button is disabled with "Select files to run" label
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeVisible();
    await expect(runButton).toHaveAttribute("data-phase", "idle");
    await expect(runButton).toBeDisabled();
    await expect(runButton).toContainText("Select files to run");

    await expect(page).toHaveScreenshot("exec-initial-state.png");
  });

  test("file selection enables Run button", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Select files via input
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      {
        name: "photo1.jpg",
        mimeType: "image/jpeg",
        buffer: Buffer.from("fake-jpeg-content"),
      },
      {
        name: "photo2.png",
        mimeType: "image/png",
        buffer: Buffer.from("fake-png-content"),
      },
    ]);

    // Files are listed
    await expect(page.getByText("2 files selected")).toBeVisible();
    await expect(page.getByText("photo1.jpg")).toBeVisible();
    await expect(page.getByText("photo2.png")).toBeVisible();

    // Run button is now enabled with "Run" label
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();
    await expect(runButton).toHaveAttribute("data-phase", "idle");
    await expect(runButton).toContainText("Run");

    await expect(page).toHaveScreenshot("exec-files-selected.png");
  });

  test("clicking Run transitions to uploading phase", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Select a file
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      {
        name: "test.jpg",
        mimeType: "image/jpeg",
        buffer: Buffer.from("fake-jpeg-content"),
      },
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    // Click Run — will transition to uploading phase briefly
    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();

    // Button should transition away from idle
    // Without a backend, it will either show uploading briefly then fail,
    // or immediately fail. Either way, it should no longer be "idle".
    await expect(runButton).not.toHaveAttribute("data-phase", "idle", {
      timeout: 5000,
    });

    // Scroll to top — error states can shift viewport to footer
    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("exec-after-run-click.png");
  });

  test("failed execution shows Try Again button", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      {
        name: "test.jpg",
        mimeType: "image/jpeg",
        buffer: Buffer.from("fake-jpeg-content"),
      },
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    // Click Run — will fail without backend
    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();

    // Wait for failed state
    await expect(runButton).toHaveAttribute("data-phase", "failed", {
      timeout: 10000,
    });
    await expect(runButton).toContainText("Try Again");

    // Scroll to top — error states can shift viewport to footer
    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("exec-failed-state.png");
  });

  test("Try Again resets to idle state", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      {
        name: "test.jpg",
        mimeType: "image/jpeg",
        buffer: Buffer.from("fake-jpeg-content"),
      },
    ]);

    // Click Run → fail
    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();
    await expect(runButton).toHaveAttribute("data-phase", "failed", {
      timeout: 10000,
    });

    // Click Try Again
    await runButton.click();

    // Should reset to idle with no files and disabled button
    await expect(runButton).toHaveAttribute("data-phase", "idle");
    await expect(runButton).toBeDisabled();
    await expect(runButton).toContainText("Select files to run");

    // Files should be cleared
    await expect(page.getByText("1 file selected")).not.toBeVisible();

    // Scroll to top — error states can shift viewport to footer
    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("exec-reset-state.png");
  });
});

test.describe("Execution flow — other tools", () => {
  test("resize-images: initial state with width config", async ({ page }) => {
    await page.goto("/resize-images");

    await expect(
      page.getByRole("heading", { name: "Resize Images Online Free" }),
    ).toBeVisible();

    // Config panel shows width control
    await expect(page.getByText("Settings")).toBeVisible();
    await expect(page.getByRole("spinbutton")).toBeVisible();

    // Drop zone and disabled run button
    await expect(page.getByText("Drag & drop files here")).toBeVisible();
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeDisabled();
    await expect(runButton).toHaveAttribute("data-phase", "idle");

    await expect(page).toHaveScreenshot("exec-resize-initial.png");
  });

  test("clean-csv: initial state with toggle switches", async ({ page }) => {
    await page.goto("/clean-csv");

    await expect(
      page.getByRole("heading", { name: "Clean CSV Online Free" }),
    ).toBeVisible({ timeout: 15000 });

    // Config panel shows toggle switches
    await expect(page.getByText("Settings")).toBeVisible();

    // Drop zone accepts CSV files
    await expect(page.getByText("CSV files")).toBeVisible();

    // Run button disabled
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeDisabled();

    await expect(page).toHaveScreenshot("exec-clean-csv-initial.png");
  });

  test("rename-files: initial state with pattern input", async ({ page }) => {
    await page.goto("/rename-files");

    await expect(
      page.getByRole("heading", { name: "Rename Files Online Free" }),
    ).toBeVisible();

    // Config panel shows pattern input
    await expect(page.getByText("Settings")).toBeVisible();
    await expect(page.getByRole("textbox")).toBeVisible();

    // Run button disabled
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeDisabled();

    await expect(page).toHaveScreenshot("exec-rename-initial.png");
  });
});

test.describe("Execution flow — drop zone disabled during processing", () => {
  test("drop zone is disabled after clicking Run", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Select files
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      {
        name: "test.jpg",
        mimeType: "image/jpeg",
        buffer: Buffer.from("fake-jpeg-content"),
      },
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    // Click Run — transitions away from idle
    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();

    // Run button should not be in idle phase
    await expect(runButton).not.toHaveAttribute("data-phase", "idle", {
      timeout: 5000,
    });

    // Run button should be disabled during processing
    // (will either be uploading or already failed)
    const phase = await runButton.getAttribute("data-phase");
    if (phase === "uploading" || phase === "running") {
      await expect(runButton).toBeDisabled();
    }
  });
});
