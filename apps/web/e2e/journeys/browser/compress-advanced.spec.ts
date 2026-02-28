import path from "path";
import fs from "fs";
import type { Page } from "@playwright/test";
import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Advanced compress-images tests: quality slider and batch processing.
 *
 * Quality slider test verifies that WASM output actually varies with
 * the quality parameter (not just a pass-through). Batch test verifies
 * 5+ files across all three codecs process successfully.
 */

const FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../../test-fixtures/images",
);

/**
 * Adjust a Radix Slider from its default value to a target value.
 * Uses PageUp/PageDown (±10) and Arrow keys (±1).
 */
async function adjustSlider(page: Page, target: number, defaultValue: number) {
  const slider = page.getByRole("slider");
  await slider.focus();

  const diff = target - defaultValue;
  const abs = Math.abs(diff);
  const bigKey = diff > 0 ? "PageUp" : "PageDown";
  const smallKey = diff > 0 ? "ArrowRight" : "ArrowLeft";

  for (let i = 0; i < Math.floor(abs / 10); i++)
    await page.keyboard.press(bigKey);
  for (let i = 0; i < abs % 10; i++)
    await page.keyboard.press(smallKey);

  await expect(slider).toHaveAttribute("aria-valuenow", String(target));
}

/**
 * Compress medium.jpg at a given quality, return output size in bytes.
 * Navigates to a fresh page each time (clean slider state).
 */
async function compressAtQuality(page: Page, quality: number): Promise<number> {
  await page.goto("/compress-images");
  await expect(
    page.getByRole("heading", { name: "Compress Images Online Free" }),
  ).toBeVisible();

  // Upload file first — config controls only appear in Phase 2 (after file selection)
  const fileInput = page.locator('input[type="file"]');
  await fileInput.setInputFiles([path.join(FIXTURES_DIR, "medium.jpg")]);
  await expect(page.getByText("1 file selected")).toBeVisible();

  await adjustSlider(page, quality, 80);

  const runButton = page.locator('[data-testid="run-button"]:visible');
  await runButton.click();
  await expect(runButton).toHaveAttribute("data-phase", "completed", {
    timeout: 30000,
  });

  const dlPromise = page.waitForEvent("download");
  await page.locator('[data-testid="output-file"]').getByRole("button", { name: /download/i }).click();
  const dl = await dlPromise;

  const dlPath = await dl.path();
  return fs.readFileSync(dlPath!).length;
}

test.describe("compress-images — configuration", () => {
  test("quality slider: q=50 produces smaller output than q=90", async ({
    page,
  }) => {
    const size90 = await compressAtQuality(page, 90);
    const size50 = await compressAtQuality(page, 50);

    // Lower quality MUST produce smaller output
    expect(size50).toBeLessThan(size90);
  });
});

test.describe("compress-images — batch processing", () => {
  test("5 mixed-codec files: all compress and show Download All", async ({ page }) => {
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Select 5 files across all three codecs
    const batchFiles = [
      "small.jpg",
      "small.png",
      "small.webp",
      "medium.jpg",
      "medium.png",
    ];
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles(
      batchFiles.map((f) => path.join(FIXTURES_DIR, f)),
    );

    await expect(page.getByText("5 files selected")).toBeVisible();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("10-batch-5-files-selected.png", {
      fullPage: true,
    });

    // Run
    const runButton = page.locator('[data-testid="run-button"]:visible');
    await runButton.click();

    // Wait for all 5 to complete
    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 60000,
    });

    // Verify all 5 outputs
    const outputFiles = page.locator('[data-testid="output-file"]');
    await expect(outputFiles).toHaveCount(5);

    // Download All button should be visible for multi-file results
    await expect(
      page.getByRole("button", { name: /download all/i }).last(),
    ).toBeVisible();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("11-batch-5-files-compressed.png", {
      fullPage: true,
    });
  });
});
