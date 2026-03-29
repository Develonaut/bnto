import path from "path";
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

/**
 * Browser execution journey — watermark-images
 *
 * Tests watermark overlay running 100% client-side via Rust→WASM.
 * Uploads source images AND a watermark file (via config panel),
 * then verifies the output is a valid image with magic bytes.
 *
 * The watermark recipe requires two file inputs:
 *   1. Source images — uploaded via the main file dropzone
 *   2. Watermark image — uploaded via the config panel's file picker
 */

const WATERMARK_FIXTURE = path.resolve(__dirname, "../../../../../test-fixtures/watermark.png");

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("watermark-images — browser execution @browser", () => {
  test("detects browser execution mode", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");
    await assertBrowserExecution(page);
  });

  test("single JPEG: apply watermark at default position, verify output", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");

    // Upload source image
    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // Upload watermark image via config panel
    const watermarkInput = page.getByTestId("watermark-file-input");
    await watermarkInput.setInputFiles(WATERMARK_FIXTURE);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(1);

    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });

    // Output should be larger than the tiny watermark (100x34) alone
    expect(buffer.length).toBeGreaterThan(100);
  });

  test("landscape source: watermark applied, valid JPEG output", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "large.jpg")]);

    const watermarkInput = page.getByTestId("watermark-file-input");
    await watermarkInput.setInputFiles(WATERMARK_FIXTURE);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(1);

    await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });
  });

  test("position change: top-left via grid, verify valid output", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // Upload watermark
    const watermarkInput = page.getByTestId("watermark-file-input");
    await watermarkInput.setInputFiles(WATERMARK_FIXTURE);

    // Change position to top-left
    const topLeft = page.getByTestId("position-grid").getByRole("radio", { name: "Top Left" });
    await topLeft.click();

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(1);

    await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });
  });

  test("batch: multiple images with watermark, Download All as ZIP", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "large.jpg"),
    ]);

    // Upload watermark
    const watermarkInput = page.getByTestId("watermark-file-input");
    await watermarkInput.setInputFiles(WATERMARK_FIXTURE);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(2);

    const { download } = await downloadAllAsZip(page);
    expect(download.suggestedFilename()).toBe("watermark-images-results.zip");
  });

  test("PNG source: watermark applied, valid PNG output", async ({ page }) => {
    await navigateToRecipe(page, "watermark-images");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.png")]);

    const watermarkInput = page.getByTestId("watermark-file-input");
    await watermarkInput.setInputFiles(WATERMARK_FIXTURE);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(1);

    await downloadAndVerify(page, {
      filenamePattern: /\.png$/i,
      magicBytes: MAGIC.PNG,
    });
  });
});
