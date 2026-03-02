import path from "path";
import fs from "fs";
import type { Page } from "@playwright/test";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
} from "../../helpers";

test.use({ reducedMotion: "reduce" });

/**
 * Advanced compress-images tests: quality slider and batch processing.
 *
 * Quality slider test verifies that WASM output actually varies with
 * the quality parameter (not just a pass-through). Batch test verifies
 * 5+ files across all three codecs process successfully.
 */

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
  await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

  await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "medium.jpg")]);

  await adjustSlider(page, quality, 80);

  await runAndComplete(page);

  const dlPromise = page.waitForEvent("download");
  await page.locator('[data-testid="output-file"]').getByRole("button", { name: /download/i }).click();
  const dl = await dlPromise;

  const dlPath = await dl.path();
  return fs.readFileSync(dlPath!).length;
}

test.describe("compress-images — configuration @browser", () => {
  test("quality slider: q=50 produces smaller output than q=90", async ({
    page,
  }) => {
    const size90 = await compressAtQuality(page, 90);
    const size50 = await compressAtQuality(page, 50);

    // Lower quality MUST produce smaller output
    expect(size50).toBeLessThan(size90);
  });
});

test.describe("compress-images — batch processing @browser", () => {
  test("5 mixed-codec files: all compress and show Download All", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    const batchFiles = [
      "small.jpg",
      "small.png",
      "small.webp",
      "medium.jpg",
      "medium.png",
    ];

    await uploadFiles(
      page,
      batchFiles.map((f) => path.join(IMAGE_FIXTURES_DIR, f)),
    );

    await runAndComplete(page, { timeout: 60000 });

    const outputFiles = page.locator('[data-testid="output-file"]');
    await expect(outputFiles).toHaveCount(5);

    await expect(
      page.getByRole("button", { name: /download all/i }).last(),
    ).toBeVisible();
  });

  test("multi-file progress is monotonic (never decreases)", async ({ page }) => {
    test.setTimeout(60_000);

    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    // Use large files across codecs — slower to compress = more progress samples
    const batchFiles = [
      "large.jpg",
      "large.png",
      "large.webp",
      "medium.jpg",
      "medium.png",
    ];

    await uploadFiles(
      page,
      batchFiles.map((f) => path.join(IMAGE_FIXTURES_DIR, f)),
    );

    // Start a browser-side observer BEFORE clicking Run
    await page.evaluate(() => {
      (window as any).__progressSamples = [] as number[];
      const observer = new MutationObserver(() => {
        const el = document.querySelector(
          '[data-testid="toolbar-progress"][data-overall-percent]',
        );
        if (el) {
          const val = Number(el.getAttribute("data-overall-percent"));
          if (!isNaN(val)) (window as any).__progressSamples.push(val);
        }
      });
      observer.observe(document.body, {
        subtree: true,
        attributes: true,
        attributeFilter: ["data-overall-percent", "data-status", "data-phase"],
      });
      (window as any).__progressObserver = observer;
    });

    await runAndComplete(page, { timeout: 60_000 });

    // Collect and disconnect
    const samples = await page.evaluate(() => {
      (window as any).__progressObserver?.disconnect();
      return (window as any).__progressSamples as number[];
    });

    expect(
      samples.length,
      `Expected progress samples during 5-file batch but got ${samples.length}`,
    ).toBeGreaterThanOrEqual(2);

    // Assert monotonic: each sample >= previous
    for (let i = 1; i < samples.length; i++) {
      expect(
        samples[i],
        `Progress decreased: ${samples[i - 1]}% → ${samples[i]}% (sample ${i}/${samples.length}, all: [${samples.join(", ")}])`,
      ).toBeGreaterThanOrEqual(samples[i - 1]!);
    }
  });
});
