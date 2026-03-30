import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  MAGIC,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  openConfigDialog,
  closeConfigDialog,
  getJpegDimensions,
} from "../../helpers";

/**
 * EXIF orientation preservation — all image bntos
 *
 * Verifies that compress, resize, and convert all correctly apply EXIF
 * orientation metadata during processing. The most common real-world
 * case: a phone camera held in portrait mode saves a landscape-oriented
 * sensor image with EXIF orientation=6 (rotate 90° CW).
 *
 * Test fixture: portrait-rotated.jpg
 *   - Raw pixel dimensions: 1200x800 (landscape — how the sensor captured it)
 *   - EXIF orientation: 6 (rotate 90° CW)
 *   - After correction: 800x1200 (portrait — how the user saw it)
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("EXIF orientation — all image bntos @browser", () => {
  test("compress: portrait JPEG preserves orientation-corrected dimensions", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "portrait-rotated.jpg")]);

    await runAndComplete(page);

    const outputFileCard = page.getByTestId("output-file");
    await expect(outputFileCard).toHaveCount(1);

    // Download and verify dimensions
    const downloadPromise = page.waitForEvent("download");
    await outputFileCard.getByTestId("download-button").click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();

    const outputFile = fs.readFileSync(downloadPath!);

    // Valid JPEG
    expect(outputFile[0]).toBe(0xff);
    expect(outputFile[1]).toBe(0xd8);

    // Dimensions should be orientation-corrected: 800x1200 (portrait)
    const dims = getJpegDimensions(outputFile);
    expect(dims.width).toBe(800);
    expect(dims.height).toBe(1200);
  });

  test("resize: portrait JPEG uses orientation-corrected dimensions", async ({ page }) => {
    await navigateToRecipe(page, "resize-images", "Resize Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "portrait-rotated.jpg")]);

    await runAndComplete(page);

    const outputFileCard = page.getByTestId("output-file");
    await expect(outputFileCard).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFileCard.getByTestId("download-button").click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();

    const outputFile = fs.readFileSync(downloadPath!);

    expect(outputFile[0]).toBe(0xff);
    expect(outputFile[1]).toBe(0xd8);

    // Output should be portrait (height > width) after orientation correction
    const dims = getJpegDimensions(outputFile);
    expect(dims.height).toBeGreaterThan(dims.width);
  });

  test("convert: portrait JPEG → PNG preserves orientation", async ({ page }) => {
    await navigateToRecipe(page, "convert-image-format", "Convert Image Format Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "portrait-rotated.jpg")]);

    // Open config dialog and change target format to PNG
    await openConfigDialog(page);
    await page.getByTestId("control-select-param-format").click();
    await page.getByTestId("select-option-png").click();
    await closeConfigDialog(page);

    await runAndComplete(page);

    const outputFileCard = page.getByTestId("output-file");
    await expect(outputFileCard).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFileCard.getByTestId("download-button").click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();

    const outputFile = fs.readFileSync(downloadPath!);

    // PNG magic bytes
    for (let i = 0; i < MAGIC.PNG.length; i++) {
      expect(outputFile[i]).toBe(MAGIC.PNG[i]);
    }

    // PNG dimensions from IHDR chunk
    const pngWidth = outputFile.readUInt32BE(16);
    const pngHeight = outputFile.readUInt32BE(20);

    expect(pngWidth).toBe(800);
    expect(pngHeight).toBe(1200);
  });
});
