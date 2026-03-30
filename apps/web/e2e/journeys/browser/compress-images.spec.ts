import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  MAGIC,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  downloadAndVerify,
  downloadAllAsZip,
} from "../../helpers";

/**
 * Browser execution journey — compress-images
 *
 * 4-phase E2E tests for the compress-images bnto running 100% client-side
 * via Rust→WASM. No backend required — files never leave the browser.
 *
 * Verified programmatically: magic bytes, file sizes, data attributes.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("compress-images — browser execution @browser", () => {
  test("single JPEG: full lifecycle", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);
    await expect(outputFile.getByTestId("download-button")).toBeVisible();

    // Verify download produces valid compressed JPEG
    const inputSize = fs.statSync(path.join(IMAGE_FIXTURES_DIR, "small.jpg")).size;

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

    await expect(page.getByTestId("output-file")).toHaveCount(2);

    const { download } = await downloadAllAsZip(page);
    expect(download.suggestedFilename()).toBe("compress-images-results.zip");
  });

  test("back button resets from completed to configure phase", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    const runButton = await runAndComplete(page);

    // Back button resets execution — returns to Phase 2 (configure) with files retained
    const backButton = page.getByTestId("back-button");
    await backButton.click();

    await expect(page.getByTestId("run-button")).toBeVisible();
    await expect(runButton).toHaveAttribute("data-phase", "idle");
  });
});
