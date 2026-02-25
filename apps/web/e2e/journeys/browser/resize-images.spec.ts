import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Browser execution journey — resize-images
 *
 * Tests the resize-images bnto running 100% client-side via Rust→WASM.
 * Resizes images to a target width while optionally maintaining aspect ratio.
 * No backend required — files never leave the browser.
 */

const FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../../test-fixtures/images",
);

test.describe("resize-images — browser execution", () => {
  test("detects browser execution mode", async ({ page }) => {
    await page.goto("/resize-images");

    await expect(
      page.getByRole("heading", { name: "Resize Images Online Free" }),
    ).toBeVisible();

    const shell = page.locator('[data-testid="bnto-shell"]');
    await expect(shell).toHaveAttribute("data-execution-mode", "browser");
  });

  test("single JPEG: resize, download, verify valid image", async ({
    page,
  }) => {
    await page.goto("/resize-images");

    await expect(
      page.getByRole("heading", { name: "Resize Images Online Free" }),
    ).toBeVisible();

    // --- BEFORE: file selected, ready to run ---
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "medium.jpg"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("resize-01-file-selected.png", {
      fullPage: true,
    });

    // --- Click Run ---
    await runButton.click();

    // --- FINISH: execution complete ---
    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    const results = page.locator(
      '[data-testid="browser-execution-results"]',
    );
    await expect(results).toBeVisible();
    await expect(page.getByText("1 file ready")).toBeVisible();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("resize-02-result.png", {
      fullPage: true,
    });

    // --- VERIFY: download produces valid JPEG ---
    const downloadPromise = page.waitForEvent("download");
    await page.locator('[data-testid="download-button"]').click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/\.jpe?g$/i);

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();

    const downloadedFile = fs.readFileSync(downloadPath!);
    expect(downloadedFile.length).toBeGreaterThan(0);

    // JPEG magic bytes: FF D8 FF (SOI marker)
    expect(downloadedFile[0]).toBe(0xff);
    expect(downloadedFile[1]).toBe(0xd8);
    expect(downloadedFile[2]).toBe(0xff);
  });

  test("batch: resize two images with Download All", async ({ page }) => {
    await page.goto("/resize-images");

    await expect(
      page.getByRole("heading", { name: "Resize Images Online Free" }),
    ).toBeVisible();

    // Select two images of different codecs
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "small.jpg"),
      path.join(FIXTURES_DIR, "small.png"),
    ]);

    await expect(page.getByText("2 files selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    await expect(page.getByText("2 files ready")).toBeVisible();
    await expect(
      page.locator('[data-testid="download-all-button"]'),
    ).toBeVisible();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("resize-03-batch-result.png", {
      fullPage: true,
    });
  });
});
