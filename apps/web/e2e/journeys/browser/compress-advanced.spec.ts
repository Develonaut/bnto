import path from "path";
import fs from "fs";
import type { Page } from "@playwright/test";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  openConfigDialog,
  closeConfigDialog,
  runAndCaptureAutoDownload,
} from "../../helpers";

/**
 * Advanced compress-images tests: compression presets and batch processing.
 *
 * Compression preset test verifies that WASM output actually varies with
 * the compression parameter (not just a pass-through). Batch test verifies
 * 5+ files across all three codecs process successfully.
 */

/** Preset names by index. */
const PRESET_LABELS = ["Draft", "Balanced", "Maximum"] as const;

/**
 * Select a compression preset by opening the config dialog, clicking
 * the preset label, and closing the dialog.
 * Presets: 0=Draft(60), 1=Balanced(80), 2=Maximum(100).
 */
async function selectPreset(page: Page, presetIndex: number) {
  await openConfigDialog(page);
  const label = PRESET_LABELS[presetIndex];
  await page.getByTestId("slider-preset", `[data-preset-label="${label}"]`).click();
  await closeConfigDialog(page);
}

/**
 * Compress large.jpg at a given preset index, return output size in bytes.
 * Presets: 0=Draft(60), 1=Balanced(80), 2=Maximum(100).
 */
async function compressAtPreset(page: Page, presetIndex: number): Promise<number> {
  await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

  await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "large.jpg")]);

  await selectPreset(page, presetIndex);

  // Capture the auto-download that fires on completion
  const dlPromise = page.waitForEvent("download");
  await runAndComplete(page);
  const dl = await dlPromise;

  const dlPath = await dl.path();
  return fs.readFileSync(dlPath!).length;
}

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("compress-images — configuration @browser", () => {
  test("compression presets: Draft produces smaller output than Maximum", async ({ page }) => {
    const sizeDraft = await compressAtPreset(page, 0); // Draft (quality=60)
    const sizeMax = await compressAtPreset(page, 2); // Maximum (quality=100)

    // Lower quality (Draft) MUST produce smaller output than higher quality (Maximum)
    expect(sizeDraft).toBeLessThan(sizeMax);
  });
});

test.describe("compress-images — batch processing @browser", () => {
  test("5 mixed-codec files: all compress and auto-download as ZIP", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    const batchFiles = ["small.jpg", "small.png", "small.webp", "medium.jpg", "medium.png"];

    await uploadFiles(
      page,
      batchFiles.map((f) => path.join(IMAGE_FIXTURES_DIR, f)),
    );

    const { download } = await runAndCaptureAutoDownload(page, { timeout: 60000 });
    expect(download.suggestedFilename()).toBe("compress-images-results.zip");

    const outputFiles = page.getByTestId("output-file");
    await expect(outputFiles).toHaveCount(5);
  });

  test("multi-file progress is monotonic (never decreases)", async ({ page }) => {
    test.setTimeout(60_000);

    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    // Use large files across codecs — slower to compress = more progress samples
    const batchFiles = ["large.jpg", "large.png", "large.webp", "medium.jpg", "medium.png"];

    await uploadFiles(
      page,
      batchFiles.map((f) => path.join(IMAGE_FIXTURES_DIR, f)),
    );

    // Start a browser-side observer BEFORE clicking Run
    await page.evaluate(() => {
      (window as any).__progressSamples = [] as number[];
      const observer = new MutationObserver(() => {
        const el = document.querySelector('[data-testid="toolbar-progress"][data-overall-percent]');
        if (el) {
          const val = Number(el.getAttribute("data-overall-percent"));
          if (!isNaN(val)) (window as any).__progressSamples.push(val);
        }
      });
      observer.observe(document.body, {
        subtree: true,
        attributes: true,
        attributeFilter: ["data-overall-percent", "data-status", "data-step"],
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
