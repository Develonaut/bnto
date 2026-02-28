import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Browser execution journey — compress-images
 *
 * 4-phase E2E tests for the compress-images bnto running 100% client-side
 * via Rust→WASM. No backend required — files never leave the browser.
 *
 * Uses real image files from test-fixtures/ so the WASM compressor
 * can actually decode and process them.
 *
 * Phases: BEFORE → PROGRESS → FINISH → VERIFY
 */

const FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../../test-fixtures/images",
);

test.describe("compress-images — browser execution", () => {
  test("detects browser execution mode", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    const shell = page.locator('[data-testid="bnto-shell"]');
    await expect(shell).toHaveAttribute("data-execution-mode", "browser");
  });

  test("single JPEG: full lifecycle", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // --- BEFORE: file selected, ready to run ---
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "small.jpg"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await expect(runButton).toBeEnabled();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("00-single-jpg-file-selected.png", {
      fullPage: true,
    });

    // --- Click Run ---
    await runButton.click();

    // --- FINISH: execution complete, results displayed ---
    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    // FileCard transitions to output state with download button
    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);
    await expect(
      outputFile.getByRole("button", { name: /download/i }),
    ).toBeVisible();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("00-single-jpg-compressed-result.png", {
      fullPage: true,
    });

    // --- VERIFY: download produces valid compressed JPEG ---
    const inputPath = path.join(FIXTURES_DIR, "small.jpg");
    const inputSize = fs.statSync(inputPath).size;

    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toContain("compressed");
    expect(download.suggestedFilename()).toMatch(/\.jpe?g$/i);

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();

    const downloadedFile = fs.readFileSync(downloadPath!);
    expect(downloadedFile.length).toBeGreaterThan(0);
    expect(downloadedFile.length).toBeLessThanOrEqual(inputSize);

    // Verify JPEG magic bytes (SOI marker: FF D8 FF)
    expect(downloadedFile[0]).toBe(0xff);
    expect(downloadedFile[1]).toBe(0xd8);
    expect(downloadedFile[2]).toBe(0xff);
  });

  test("batch: multiple images with Download All as ZIP", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // --- BEFORE: multiple files selected ---
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "small.jpg"),
      path.join(FIXTURES_DIR, "small.png"),
    ]);

    await expect(page.getByText("2 files selected")).toBeVisible();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("00-batch-2-files-selected.png", {
      fullPage: true,
    });

    // --- Click Run ---
    const runButton = page.locator('[data-testid="run-button"]:visible');
    await runButton.click();

    // --- FINISH: all results listed ---
    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(2);
    const downloadAllBtn = page.getByRole("button", { name: /download all/i }).last();
    await expect(downloadAllBtn).toBeVisible();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("00-batch-2-files-compressed.png", {
      fullPage: true,
    });

    // --- VERIFY: Download All produces a valid ZIP file ---
    const downloadPromise = page.waitForEvent("download");
    await downloadAllBtn.click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toBe("compress-images-results.zip");

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();

    const downloadedFile = fs.readFileSync(downloadPath!);
    // ZIP magic bytes: PK (0x50 0x4B 0x03 0x04)
    expect(downloadedFile[0]).toBe(0x50);
    expect(downloadedFile[1]).toBe(0x4b);
    expect(downloadedFile[2]).toBe(0x03);
    expect(downloadedFile[3]).toBe(0x04);
    expect(downloadedFile.length).toBeGreaterThan(100);
  });

  test("back button resets from completed to configure phase", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Process a file
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "small.jpg"),
    ]);

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    // Back button resets execution — returns to Phase 2 (configure) with files retained
    const backButton = page.locator('[data-testid="bnto-shell"] button').first();
    await backButton.click();

    // Should be back in Phase 2 — files still selected, run button idle
    await expect(page.getByText("1 file selected")).toBeVisible();
    await expect(runButton).toHaveAttribute("data-phase", "idle");
  });

  test("all Tier 1 bntos detect browser execution mode", async ({
    page,
  }) => {
    // All 6 Tier 1 slugs are browser-capable via Rust→WASM
    const tier1Slugs = [
      { slug: "resize-images", h1: "Resize Images Online Free" },
      { slug: "convert-image-format", h1: "Convert Image Format Online Free" },
      { slug: "rename-files", h1: "Rename Files Online Free" },
      { slug: "clean-csv", h1: "Clean CSV Online Free" },
      { slug: "rename-csv-columns", h1: "Rename CSV Columns Online Free" },
    ];

    for (const { slug, h1 } of tier1Slugs) {
      await page.goto(`/${slug}`);
      await expect(
        page.getByRole("heading", { name: h1 }),
      ).toBeVisible();

      const shell = page.locator('[data-testid="bnto-shell"]');
      await expect(shell).toHaveAttribute("data-execution-mode", "browser");
    }
  });
});
