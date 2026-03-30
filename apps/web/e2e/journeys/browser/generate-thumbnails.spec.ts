import path from "path";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  MAGIC,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  downloadAndVerify,
  downloadAllAsZip,
  assertWebPBytes,
} from "../../helpers";

/**
 * Browser execution journey — generate-thumbnails
 *
 * Multi-node pipeline: resize (150px) → convert (WebP) → rename (thumb_ prefix).
 * Verified programmatically: WebP magic bytes, thumb_ filename prefix, data attributes.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("generate-thumbnails — browser execution @browser", () => {
  test("single JPEG: resize + convert + rename lifecycle", async ({ page }) => {
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
    expect(buffer.length).toBeGreaterThan(0);
  });

  test("batch: multiple images with Download All as ZIP", async ({ page }) => {
    await navigateToRecipe(page, "generate-thumbnails", "Generate Thumbnails Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
    ]);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(2);

    const { download } = await downloadAllAsZip(page);
    expect(download.suggestedFilename()).toBe("generate-thumbnails-results.zip");
  });

  test("back button resets from completed to configure phase", async ({ page }) => {
    await navigateToRecipe(page, "generate-thumbnails", "Generate Thumbnails Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    const runButton = await runAndComplete(page);

    const backButton = page.getByTestId("back-button");
    await backButton.click();

    await expect(page.getByTestId("run-button")).toBeVisible();
    await expect(runButton).toHaveAttribute("data-phase", "idle");
  });
});
