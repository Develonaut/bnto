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
  getJpegDimensions,
} from "../../helpers";

/**
 * Browser execution journey — resize-images
 *
 * Tests the resize-images bnto running 100% client-side via Rust→WASM.
 * Resizes images to a target width while optionally maintaining aspect ratio.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("resize-images — browser execution @browser", () => {
  test("single JPEG: resize to default width, verify dimensions", async ({ page }) => {
    await navigateToRecipe(page, "resize-images", "Resize Images Online Free");

    // medium.jpg is 400x400 — default resize width is 200
    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "medium.jpg")]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });

    // Verify output was actually resized to default width (200px)
    const dims = getJpegDimensions(buffer);
    expect(dims.width).toBe(200);
    // Aspect ratio preserved: 400x400 input → 200x200 output
    expect(dims.height).toBe(200);
  });

  test("batch: resize two images auto-download as ZIP", async ({ page }) => {
    await navigateToRecipe(page, "resize-images", "Resize Images Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
    ]);

    const { download } = await runAndCaptureAutoDownload(page);
    expect(download.suggestedFilename()).toBe("resize-images-results.zip");

    await expect(page.getByTestId("output-file")).toHaveCount(2);
  });
});
