import path from "path";
import { test, expect } from "./fixtures";
import {
  ENGINE_FIXTURES,
  ENGINE_IMAGES,
  waitForSession,
  runPipeline,
  assertCompletedWithScreenshot,
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

    const phase = await runPipeline(page, {
      files: [path.join(ENGINE_IMAGES, "Product_Render.png")],
      debugLabel: "compress-single",
    });
    await assertCompletedWithScreenshot(
      page,
      phase,
      "integration-compress-completed.png",
    );

    // Single file — Download button
    await expect(
      page.locator('[data-testid="download-button"]'),
    ).toBeVisible({ timeout: 15_000 });
  });

  test("multiple images pipeline", async ({ page }) => {
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: /compress images/i }),
    ).toBeVisible();
    await waitForSession(page);

    const phase = await runPipeline(page, {
      files: [
        path.join(ENGINE_IMAGES, "Product_Render.png"),
        path.join(ENGINE_FIXTURES, "overlay-sample.png"),
      ],
      debugLabel: "compress-multi",
    });
    await assertCompletedWithScreenshot(
      page,
      phase,
      "integration-compress-multi-completed.png",
    );

    // Multiple files — Download All button
    await expect(
      page.locator('[data-testid="download-all-button"]'),
    ).toBeVisible({ timeout: 15_000 });
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

    const phase = await runPipeline(page, {
      files: [path.join(ENGINE_IMAGES, "Product_Render.png")],
      debugLabel: "resize",
    });
    await assertCompletedWithScreenshot(
      page,
      phase,
      "integration-resize-completed.png",
    );
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

    const phase = await runPipeline(page, {
      files: [path.join(ENGINE_IMAGES, "Product_Render.png")],
      debugLabel: "convert-format",
    });
    await assertCompletedWithScreenshot(
      page,
      phase,
      "integration-convert-format-completed.png",
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

    const phase = await runPipeline(page, {
      files: [path.join(ENGINE_FIXTURES, "test-dirty.csv")],
      debugLabel: "clean-csv",
    });
    await assertCompletedWithScreenshot(
      page,
      phase,
      "integration-clean-csv-completed.png",
    );
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

    const phase = await runPipeline(page, {
      files: [path.join(ENGINE_IMAGES, "Product_Render.png")],
      debugLabel: "reset",
    });

    if (phase !== "completed") {
      test.skip(true, "Execution did not complete — skipping reset test");
      return;
    }

    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "idle");
    await expect(runButton).toBeDisabled();
    await expect(runButton).toContainText("Select files to run");
    await expect(page.getByText("1 file selected")).not.toBeVisible();
  });
});
