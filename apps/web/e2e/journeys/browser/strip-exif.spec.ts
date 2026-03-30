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
} from "../../helpers";

/**
 * Browser execution journey — strip-exif
 *
 * Tests EXIF metadata stripping running 100% client-side via Rust→WASM.
 * Verified programmatically: magic bytes, file sizes, data attributes.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("strip-exif — browser execution @browser", () => {
  test("single JPEG: strip EXIF, download, verify valid image", async ({ page }) => {
    await navigateToRecipe(page, "strip-exif", "Strip EXIF Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);
    await expect(outputFile.getByTestId("download-button")).toBeVisible();

    // Verify download produces valid JPEG (format preserved after EXIF strip)
    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });

    expect(buffer.length).toBeGreaterThan(0);
  });

  test("batch: multiple images with Download All as ZIP", async ({ page }) => {
    await navigateToRecipe(page, "strip-exif", "Strip EXIF Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
    ]);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(2);

    const { download } = await downloadAllAsZip(page);
    expect(download.suggestedFilename()).toBe("strip-exif-results.zip");
  });
});
