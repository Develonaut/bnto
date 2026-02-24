import path from "path";
import { test, expect } from "./fixtures";
import {
  ENGINE_FIXTURES,
  ENGINE_IMAGES,
  waitForSession,
} from "./integrationHelpers";

test.use({ reducedMotion: "reduce" });

/**
 * Full-stack integration E2E tests.
 *
 * Exercise the complete execution pipeline against the real dev stack
 * (Next.js + Convex + Go API via Cloudflare tunnel + R2).
 *
 * Test fixtures are shared with the Go engine — single source of truth.
 *
 * Each test captures full-page snapshots at critical interaction points:
 *   - Session ready + files selected (pre-execution baseline)
 *   - Execution in progress (when capturable)
 *   - Execution completed with results
 *
 * Run via: task e2e:integration
 * Requires: task dev:all (Next.js + Convex + Go API + tunnel)
 */

// ---------------------------------------------------------------------------
// compress-images
// ---------------------------------------------------------------------------

test.describe("Integration — compress-images", () => {
  test("single image pipeline", async ({ page }) => {
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: /compress images/i }),
    ).toBeVisible();
    await waitForSession(page);

    // Select file
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_IMAGES, "Product_Render.png"),
    );
    await expect(page.getByText("1 file selected")).toBeVisible();
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    // Snapshot: session ready, file selected, ready to run
    await expect(page).toHaveScreenshot(
      "compress-single-01-files-selected.png",
      { fullPage: true },
    );

    // Execute
    await runButton.click();

    // Try to capture processing state (may be fleeting)
    const midPhase = await runButton.getAttribute("data-phase");
    if (midPhase === "uploading" || midPhase === "running") {
      await expect(page).toHaveScreenshot(
        "compress-single-02-in-progress.png",
        { fullPage: true },
      );
    }

    // Wait for completion
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";
    expect(phase).toBe("completed");

    // Wait for results
    const results = page.locator('[data-testid="execution-results"]');
    await expect(results).toBeVisible({ timeout: 10_000 });
    await expect(
      page.locator('[data-testid="download-button"]'),
    ).toBeVisible({ timeout: 15_000 });

    // Snapshot: execution completed with download button
    await expect(page).toHaveScreenshot(
      "compress-single-03-completed.png",
      { fullPage: true },
    );
  });

  test("multiple images pipeline", async ({ page }) => {
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: /compress images/i }),
    ).toBeVisible();
    await waitForSession(page);

    // Select multiple files
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(ENGINE_IMAGES, "Product_Render.png"),
      path.join(ENGINE_FIXTURES, "overlay-sample.png"),
    ]);
    await expect(page.getByText("2 files selected")).toBeVisible();
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    // Snapshot: multiple files selected
    await expect(page).toHaveScreenshot(
      "compress-multi-01-files-selected.png",
      { fullPage: true },
    );

    // Execute
    await runButton.click();
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";
    expect(phase).toBe("completed");

    // Wait for results
    const results = page.locator('[data-testid="execution-results"]');
    await expect(results).toBeVisible({ timeout: 10_000 });
    await expect(
      page.locator('[data-testid="download-all-button"]'),
    ).toBeVisible({ timeout: 15_000 });

    // Snapshot: multi-file results with Download All
    await expect(page).toHaveScreenshot(
      "compress-multi-02-completed.png",
      { fullPage: true },
    );
  });
});

// ---------------------------------------------------------------------------
// resize-images
// ---------------------------------------------------------------------------

test.describe("Integration — resize-images", () => {
  test("resize to custom width", async ({ page }) => {
    await page.goto("/resize-images");
    await expect(
      page.getByRole("heading", { name: /resize images/i }),
    ).toBeVisible();
    await waitForSession(page);

    // Select file
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_IMAGES, "Product_Render.png"),
    );
    await expect(page.getByText("1 file selected")).toBeVisible();
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    // Snapshot: resize config + file selected
    await expect(page).toHaveScreenshot("resize-01-files-selected.png", {
      fullPage: true,
    });

    // Execute
    await runButton.click();
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";
    expect(phase).toBe("completed");

    const results = page.locator('[data-testid="execution-results"]');
    await expect(results).toBeVisible({ timeout: 10_000 });

    // Snapshot: resize completed
    await expect(page).toHaveScreenshot("resize-02-completed.png", {
      fullPage: true,
    });
  });
});

// ---------------------------------------------------------------------------
// convert-image-format
// ---------------------------------------------------------------------------

test.describe("Integration — convert-image-format", () => {
  test("convert to WebP", async ({ page }) => {
    await page.goto("/convert-image-format");
    await expect(
      page.getByRole("heading", { name: /convert image/i }),
    ).toBeVisible();
    await waitForSession(page);

    // Select file
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_IMAGES, "Product_Render.png"),
    );
    await expect(page.getByText("1 file selected")).toBeVisible();
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    // Snapshot: format config + file selected
    await expect(page).toHaveScreenshot(
      "convert-format-01-files-selected.png",
      { fullPage: true },
    );

    // Execute
    await runButton.click();
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";
    expect(phase).toBe("completed");

    const results = page.locator('[data-testid="execution-results"]');
    await expect(results).toBeVisible({ timeout: 10_000 });

    // Snapshot: format conversion completed
    await expect(page).toHaveScreenshot(
      "convert-format-02-completed.png",
      { fullPage: true },
    );
  });
});

// ---------------------------------------------------------------------------
// clean-csv
// ---------------------------------------------------------------------------

test.describe("Integration — clean-csv", () => {
  test("clean dirty CSV", async ({ page }) => {
    await page.goto("/clean-csv");
    await expect(
      page.getByRole("heading", { name: /clean csv/i }),
    ).toBeVisible({ timeout: 15_000 });
    await waitForSession(page);

    // Select file
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_FIXTURES, "test-dirty.csv"),
    );
    await expect(page.getByText("1 file selected")).toBeVisible();
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    // Snapshot: CSV config + file selected
    await expect(page).toHaveScreenshot("clean-csv-01-files-selected.png", {
      fullPage: true,
    });

    // Execute
    await runButton.click();
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";
    expect(phase).toBe("completed");

    const results = page.locator('[data-testid="execution-results"]');
    await expect(results).toBeVisible({ timeout: 10_000 });

    // Snapshot: CSV cleaning completed
    await expect(page).toHaveScreenshot("clean-csv-02-completed.png", {
      fullPage: true,
    });
  });
});

// ---------------------------------------------------------------------------
// Reset after completion
// ---------------------------------------------------------------------------

test.describe("Integration — reset after completion", () => {
  test("Run Again resets to idle", async ({ page }) => {
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: /compress images/i }),
    ).toBeVisible();
    await waitForSession(page);

    // Run a pipeline first
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_IMAGES, "Product_Render.png"),
    );
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();
    await runButton.click();
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";

    if (phase !== "completed") {
      test.skip(true, "Execution did not complete — skipping reset test");
      return;
    }

    // Click Run Again to reset
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "idle");
    await expect(runButton).toBeDisabled();
    await expect(runButton).toContainText("Select files to run");
    await expect(page.getByText("1 file selected")).not.toBeVisible();

    // Snapshot: reset to idle state after completion
    await expect(page).toHaveScreenshot("reset-01-idle-after-run.png", {
      fullPage: true,
    });
  });
});
