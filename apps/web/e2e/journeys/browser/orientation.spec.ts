import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  MAGIC,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
} from "../../helpers";

test.use({ reducedMotion: "reduce" });

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

/**
 * Extract image dimensions from a JPEG file by parsing SOF markers.
 *
 * JPEG stores dimensions in Start Of Frame (SOF) markers:
 *   FF C0 (baseline), FF C1 (extended), FF C2 (progressive)
 *
 * SOF marker structure:
 *   FF Cx            — marker (2 bytes)
 *   LL LL            — segment length (2 bytes)
 *   PP               — precision (1 byte)
 *   HH HH            — height (2 bytes, big-endian)
 *   WW WW            — width (2 bytes, big-endian)
 */
function getJpegDimensions(data: Buffer): { width: number; height: number } {
  for (let i = 0; i < data.length - 9; i++) {
    if (
      data[i] === 0xff &&
      (data[i + 1] === 0xc0 ||
        data[i + 1] === 0xc1 ||
        data[i + 1] === 0xc2)
    ) {
      const height = data.readUInt16BE(i + 5);
      const width = data.readUInt16BE(i + 7);
      return { width, height };
    }
  }
  throw new Error("No SOF marker found in JPEG data");
}

test.describe("EXIF orientation — all image bntos @browser", () => {
  test("compress: portrait JPEG preserves orientation-corrected dimensions", async ({
    page,
  }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "portrait-rotated.jpg"),
    ]);

    await runAndComplete(page);

    const outputFileCard = page.locator('[data-testid="output-file"]');
    await expect(outputFileCard).toHaveCount(1);

    // Download and verify dimensions
    const downloadPromise = page.waitForEvent("download");
    await outputFileCard.getByRole("button", { name: /download/i }).click();
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

  test("resize: portrait JPEG uses orientation-corrected dimensions", async ({
    page,
  }) => {
    await navigateToRecipe(page, "resize-images", "Resize Images Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "portrait-rotated.jpg"),
    ]);

    await runAndComplete(page);

    const outputFileCard = page.locator('[data-testid="output-file"]');
    await expect(outputFileCard).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFileCard.getByRole("button", { name: /download/i }).click();
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

  test("convert: portrait JPEG → PNG preserves orientation", async ({
    page,
  }) => {
    await navigateToRecipe(page, "convert-image-format", "Convert Image Format Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "portrait-rotated.jpg"),
    ]);

    // Change target format to PNG
    const formatSelect = page.locator('[data-testid="format-select"]').or(
      page.getByRole("combobox"),
    );
    await formatSelect.click();
    await page.getByRole("option", { name: /png/i }).click();

    await runAndComplete(page);

    const outputFileCard = page.locator('[data-testid="output-file"]');
    await expect(outputFileCard).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFileCard.getByRole("button", { name: /download/i }).click();
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
