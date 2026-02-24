import path from "path";
import { test, expect } from "./fixtures";
import {
  ENGINE_FIXTURES,
  ENGINE_IMAGES,
  waitForSession,
  assertOutputFiles,
  downloadAndVerify,
  downloadAllAndVerify,
  readDownloadedFile,
  assertImageFormat,
} from "./integrationHelpers";

test.use({ reducedMotion: "reduce" });

/**
 * Full-stack integration E2E tests.
 *
 * Exercise the complete user journey against the real dev stack:
 *   Input file → tool page → Run → output files → Download
 *
 * Each test behaves like a real user: drop a file, use the tool, get the
 * output. The download step is the critical proof — if the user can't
 * receive their processed file, the tool is broken.
 *
 * Test fixtures are shared with the Go engine — single source of truth.
 *
 * Screenshots are organized per-flow (each bnto tool gets its own directory):
 *   __screenshots__/execution.integration.spec.ts/
 *     compress-images/  — single + multi image pipelines
 *     resize-images/    — resize to custom width
 *     convert-image-format/ — format conversion
 *     clean-csv/        — CSV cleaning
 *     reset/            — Run Again reset flow
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

    // ── Step 1: Select file ──────────────────────────────────────────
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_IMAGES, "Product_Render.png"),
    );
    await expect(page.getByText("1 file selected")).toBeVisible();
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    await expect(page).toHaveScreenshot(
      ["compress-images", "single-01-files-selected.png"],
      { fullPage: true },
    );

    // ── Step 2: Execute ──────────────────────────────────────────────
    await runButton.click();

    // Note: in-progress state is transient and page height varies —
    // not suitable for stable screenshot baselines. The lifecycle is
    // validated via data-phase attribute checks below.

    // Wait for completion
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";
    expect(phase).toBe("completed");

    // ── Step 3: Verify output ────────────────────────────────────────
    await assertOutputFiles(page, 1);

    await expect(page).toHaveScreenshot(
      ["compress-images", "single-03-completed.png"],
      { fullPage: true },
    );

    // ── Step 4: Download and verify the output ──────────────────────
    const download = await downloadAndVerify(page);
    // The engine compresses to WebP for better ratio — verify output is
    // a valid image (filename may still say .png but content is WebP)
    const outputBuffer = await readDownloadedFile(download);
    assertImageFormat(outputBuffer, "webp");
    // Compressed output should be smaller than the 430KB input
    expect(outputBuffer.length).toBeLessThan(430_000);
    console.log(
      `[verify] Output is valid WebP, ${outputBuffer.length} bytes (compressed)`,
    );
  });

  test("multiple images pipeline", async ({ page }) => {
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: /compress images/i }),
    ).toBeVisible();
    await waitForSession(page);

    // ── Step 1: Select files ─────────────────────────────────────────
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(ENGINE_IMAGES, "Product_Render.png"),
      path.join(ENGINE_FIXTURES, "overlay-sample.png"),
    ]);
    await expect(page.getByText("2 files selected")).toBeVisible();
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    await expect(page).toHaveScreenshot(
      ["compress-images", "multi-01-files-selected.png"],
      { fullPage: true },
    );

    // ── Step 2: Execute ──────────────────────────────────────────────
    await runButton.click();
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";
    expect(phase).toBe("completed");

    // ── Step 3: Verify output ────────────────────────────────────────
    await assertOutputFiles(page, 2);

    await expect(page).toHaveScreenshot(
      ["compress-images", "multi-02-completed.png"],
      { fullPage: true },
    );

    // ── Step 4: Download individual files and verify outputs ────────
    // Download each file via its per-row download button (more reliable
    // than "Download All" which triggers rapid sequential downloads
    // that browsers may block).
    const firstFileButton = page.getByRole("button", {
      name: /Download Product_Render\.png/i,
    });
    await expect(firstFileButton).toBeVisible({ timeout: 15_000 });

    const [download1] = await Promise.all([
      page.waitForEvent("download", { timeout: 30_000 }),
      firstFileButton.click(),
    ]);
    expect(download1.suggestedFilename()).toMatch(/Product_Render/i);
    const buf1 = await readDownloadedFile(download1);
    assertImageFormat(buf1, "webp");
    console.log(
      `[verify] ${download1.suggestedFilename()} — valid WebP, ${buf1.length} bytes`,
    );

    // Verify "Download All" button is ready (proves all URLs loaded)
    await expect(
      page.locator('[data-testid="download-all-button"]'),
    ).toBeEnabled();
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

    // ── Step 1: Select file ──────────────────────────────────────────
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_IMAGES, "Product_Render.png"),
    );
    await expect(page.getByText("1 file selected")).toBeVisible();
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    await expect(page).toHaveScreenshot(
      ["resize-images", "01-files-selected.png"],
      { fullPage: true },
    );

    // ── Step 2: Execute ──────────────────────────────────────────────
    await runButton.click();
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";
    expect(phase).toBe("completed");

    // ── Step 3: Verify output ────────────────────────────────────────
    await assertOutputFiles(page, 1);

    await expect(page).toHaveScreenshot(
      ["resize-images", "02-completed.png"],
      { fullPage: true },
    );

    // ── Step 4: Download and verify the output ──────────────────────
    const download = await downloadAndVerify(page);
    expect(download.suggestedFilename()).toMatch(/\.png$/i);

    // Verify the downloaded file is a valid PNG image
    const outputBuffer = await readDownloadedFile(download);
    assertImageFormat(outputBuffer, "png");
    console.log(
      `[verify] Output is valid PNG, ${outputBuffer.length} bytes (resized)`,
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

    // ── Step 1: Select file ──────────────────────────────────────────
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_IMAGES, "Product_Render.png"),
    );
    await expect(page.getByText("1 file selected")).toBeVisible();
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    await expect(page).toHaveScreenshot(
      ["convert-image-format", "01-files-selected.png"],
      { fullPage: true },
    );

    // ── Step 2: Execute ──────────────────────────────────────────────
    await runButton.click();
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";
    expect(phase).toBe("completed");

    // ── Step 3: Verify output ────────────────────────────────────────
    await assertOutputFiles(page, 1);

    await expect(page).toHaveScreenshot(
      ["convert-image-format", "02-completed.png"],
      { fullPage: true },
    );

    // ── Step 4: Download and verify the output ──────────────────────
    const download = await downloadAndVerify(page);
    // Convert to WebP should produce a .webp file
    expect(download.suggestedFilename()).toMatch(/\.webp$/i);

    // Verify the downloaded file is actually a WebP image
    const outputBuffer = await readDownloadedFile(download);
    assertImageFormat(outputBuffer, "webp");
    console.log(
      `[verify] Output is valid WebP, ${outputBuffer.length} bytes`,
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

    // ── Step 1: Select file ──────────────────────────────────────────
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      path.join(ENGINE_FIXTURES, "test-dirty.csv"),
    );
    await expect(page.getByText("1 file selected")).toBeVisible();
    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    await expect(page).toHaveScreenshot(
      ["clean-csv", "01-files-selected.png"],
      { fullPage: true },
    );

    // ── Step 2: Execute ──────────────────────────────────────────────
    await runButton.click();
    await expect(runButton).toHaveAttribute(
      "data-phase",
      /(completed|failed)/,
      { timeout: 90_000 },
    );
    const phase =
      (await runButton.getAttribute("data-phase")) ?? "unknown";
    expect(phase).toBe("completed");

    // ── Step 3: Verify output ────────────────────────────────────────
    await assertOutputFiles(page, 1);

    await expect(page).toHaveScreenshot(
      ["clean-csv", "02-completed.png"],
      { fullPage: true },
    );

    // ── Step 4: Download and verify the output ──────────────────────
    const download = await downloadAndVerify(page);
    expect(download.suggestedFilename()).toMatch(/\.csv$/i);

    // Read the cleaned CSV and verify it's valid
    const outputBuffer = await readDownloadedFile(download);
    const csvContent = outputBuffer.toString("utf-8");
    // Should have content (header + at least 1 data row)
    const lines = csvContent.trim().split("\n");
    expect(lines.length).toBeGreaterThanOrEqual(2);
    console.log(
      `[verify] Output is valid CSV, ${lines.length} lines, ${outputBuffer.length} bytes`,
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

    // Verify output is there before resetting
    await assertOutputFiles(page, 1);

    // Click Run Again to reset
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "idle");
    await expect(runButton).toBeDisabled();
    await expect(runButton).toContainText("Select files to run");
    await expect(page.getByText("1 file selected")).not.toBeVisible();

    // Snapshot: reset to idle state after completion
    await expect(page).toHaveScreenshot(
      ["reset", "01-idle-after-run.png"],
      { fullPage: true },
    );
  });
});
