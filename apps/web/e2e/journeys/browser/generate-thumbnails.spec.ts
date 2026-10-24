import fs from "fs";
import path from "path";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  MAGIC,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  downloadAndVerify,
  runAndCaptureAutoDownload,
  assertWebPBytes,
  getWebPDimensions,
} from "../../helpers";

/**
 * Browser execution journey — generate-thumbnails
 *
 * Multi-node pipeline: resize (150px) → convert (WebP) → rename (thumb_ prefix).
 * Verified programmatically: WebP magic bytes, thumb_ filename prefix, data attributes.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

/** Bytes as the results list formats them (decimal, k=1000, one decimal). */
function formatBytes(bytes: number): string {
  const units = ["B", "KB", "MB", "GB"];
  const i = bytes === 0 ? 0 : Math.floor(Math.log(bytes) / Math.log(1000));
  return `${parseFloat((bytes / Math.pow(1000, i)).toFixed(1))} ${units[i]}`;
}

test.describe("generate-thumbnails — browser execution @browser", () => {
  test("single JPEG: resize + convert + rename lifecycle, verify dimensions", async ({ page }) => {
    await navigateToRecipe(page, "generate-thumbnails", "Generate Thumbnails Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);
    await expect(outputFile.getByTestId("download-button")).toBeVisible();

    // Output should be WebP with thumb_ prefix
    const buffer = await downloadAndVerify(page, {
      filenamePattern: /^thumb_.*\.webp$/i,
      magicBytes: MAGIC.WEBP_RIFF,
    });

    assertWebPBytes(buffer);

    // "Original" is the uploaded photo, not the resized image convert saw.
    const uploadBytes = fs.statSync(path.join(IMAGE_FIXTURES_DIR, "small.jpg")).size;
    await expect(outputFile.getByTestId("original-size")).toHaveText(formatBytes(uploadBytes));

    // Verify thumbnail dimensions — default resize is 150px width, without enlarging.
    // small.jpg is 100x100, smaller than the target, so it keeps its size.
    expect(getWebPDimensions(buffer)).toEqual({ width: 100, height: 100 });
  });

  test("batch: multiple images auto-download as ZIP on completion", async ({ page }) => {
    await navigateToRecipe(page, "generate-thumbnails", "Generate Thumbnails Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
    ]);

    const { download } = await runAndCaptureAutoDownload(page);
    expect(download.suggestedFilename()).toBe("generate-thumbnails-results.zip");

    await expect(page.getByTestId("output-file")).toHaveCount(2);
  });

  test("back button resets from completed to configure step", async ({ page }) => {
    await navigateToRecipe(page, "generate-thumbnails", "Generate Thumbnails Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    const runButton = await runAndComplete(page);

    const backButton = page.getByTestId("back-button");
    await backButton.click();

    await expect(page.getByTestId("run-button")).toBeVisible();
    await expect(runButton).toHaveAttribute("data-step", "idle");
  });
});
