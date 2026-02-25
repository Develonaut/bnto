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

    const runButton = page.locator('[data-testid="run-button"]');
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

    const results = page.locator(
      '[data-testid="browser-execution-results"]',
    );
    await expect(results).toBeVisible();
    await expect(page.getByText("1 file ready")).toBeVisible();
    await expect(
      page.locator('[data-testid="download-button"]'),
    ).toBeVisible();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("00-single-jpg-compressed-result.png", {
      fullPage: true,
    });

    // --- VERIFY: download produces valid compressed JPEG ---
    const inputPath = path.join(FIXTURES_DIR, "small.jpg");
    const inputSize = fs.statSync(inputPath).size;

    const downloadPromise = page.waitForEvent("download");
    await page.locator('[data-testid="download-button"]').click();
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

  test("batch: multiple images with Download All", async ({ page }) => {
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
    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();

    // --- FINISH: all results listed ---
    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    await expect(page.getByText("2 files ready")).toBeVisible();
    await expect(
      page.locator('[data-testid="download-all-button"]'),
    ).toBeVisible();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("00-batch-2-files-compressed.png", {
      fullPage: true,
    });
  });

  test("Run Again resets to idle state", async ({ page }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Process a file
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "small.jpg"),
    ]);

    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });
    await expect(runButton).toContainText("Run Again");

    // Click "Run Again" to reset
    await runButton.click();

    // Should reset to idle
    await expect(runButton).toHaveAttribute("data-phase", "idle");
    await expect(runButton).toBeDisabled();
    await expect(runButton).toContainText("Select files to run");

    // Files should be cleared
    await expect(page.getByText("1 file selected")).not.toBeVisible();
  });

  test("non-browser slugs still use cloud execution mode", async ({
    page,
  }) => {
    // resize-images is not yet browser-capable
    await page.goto("/resize-images");

    await expect(
      page.getByRole("heading", { name: "Resize Images Online Free" }),
    ).toBeVisible();

    const shell = page.locator('[data-testid="bnto-shell"]');
    await expect(shell).toHaveAttribute("data-execution-mode", "cloud");
  });
});
