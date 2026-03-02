import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  MAGIC,
  navigateToRecipe,
  assertBrowserExecution,
  uploadFiles,
  runAndComplete,
  downloadAndVerify,
  downloadAllAsZip,
} from "../../helpers";

test.use({ reducedMotion: "reduce" });

/**
 * Browser execution journey — compress-images
 *
 * 4-phase E2E tests for the compress-images bnto running 100% client-side
 * via Rust→WASM. No backend required — files never leave the browser.
 *
 * Verified programmatically: magic bytes, file sizes, data attributes.
 */

test.describe("compress-images — browser execution @browser", () => {
  test("detects browser execution mode", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");
    await assertBrowserExecution(page);
  });

  test("single JPEG: full lifecycle", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);

    await runAndComplete(page);

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);
    await expect(
      outputFile.getByRole("button", { name: /download/i }),
    ).toBeVisible();

    // Verify download produces valid compressed JPEG
    const inputSize = fs.statSync(
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ).size;

    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
      maxSize: inputSize,
    });

    expect(buffer.length).toBeGreaterThan(0);
  });

  test("batch: multiple images with Download All as ZIP", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
    ]);

    await runAndComplete(page);

    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(2);

    const { download } = await downloadAllAsZip(page);
    expect(download.suggestedFilename()).toBe("compress-images-results.zip");
  });

  test("back button resets from completed to configure phase", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);

    const runButton = await runAndComplete(page);

    // Back button resets execution — returns to Phase 2 (configure) with files retained
    const backButton = page.locator('[data-testid="bnto-shell"] button').first();
    await backButton.click();

    await expect(page.getByText("1 file selected")).toBeVisible();
    await expect(runButton).toHaveAttribute("data-phase", "idle");
  });

  test("all Tier 1 bntos detect browser execution mode", async ({ page }) => {
    const tier1Slugs = [
      { slug: "resize-images", h1: "Resize Images Online Free" },
      { slug: "convert-image-format", h1: "Convert Image Format Online Free" },
      { slug: "rename-files", h1: "Rename Files Online Free" },
      { slug: "clean-csv", h1: "Clean CSV Online Free" },
      { slug: "rename-csv-columns", h1: "Rename CSV Columns Online Free" },
    ];

    for (const { slug, h1 } of tier1Slugs) {
      await navigateToRecipe(page, slug, h1);
      await assertBrowserExecution(page);
    }
  });
});
