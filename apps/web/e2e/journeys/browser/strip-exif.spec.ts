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
  runAndCaptureAutoDownload,
  assertExifStripped,
  getJpegDimensions,
} from "../../helpers";

/**
 * Browser execution journey — strip-exif
 *
 * Tests EXIF metadata stripping running 100% client-side via Rust→WASM.
 * Verified programmatically: magic bytes, EXIF absence, dimension preservation.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("strip-exif — browser execution @browser", () => {
  test("single JPEG: strip EXIF, verify metadata removed and dimensions preserved", async ({
    page,
  }) => {
    // portrait-rotated.jpg has substantial EXIF data (orientation, camera metadata)
    const inputPath = path.join(IMAGE_FIXTURES_DIR, "portrait-rotated.jpg");
    const inputSize = fs.statSync(inputPath).size;

    await navigateToRecipe(page, "strip-exif", "Strip EXIF Online Free");

    await uploadFiles(page, [inputPath]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);
    await expect(outputFile.getByTestId("download-button")).toBeVisible();

    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });

    // EXIF APP1 markers must be absent — proves the strip actually ran
    assertExifStripped(buffer);

    // Dimensions preserved — image wasn't corrupted by stripping
    // portrait-rotated.jpg: after EXIF orientation correction = 800x1200
    const dims = getJpegDimensions(buffer);
    expect(dims.width).toBe(800);
    expect(dims.height).toBe(1200);

    // Output should be smaller than input (EXIF data removed)
    expect(buffer.length).toBeLessThan(inputSize);
  });

  test("batch: multiple images auto-download as ZIP on completion", async ({ page }) => {
    await navigateToRecipe(page, "strip-exif", "Strip EXIF Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
    ]);

    const { download } = await runAndCaptureAutoDownload(page);
    expect(download.suggestedFilename()).toBe("strip-exif-results.zip");

    await expect(page.getByTestId("output-file")).toHaveCount(2);
  });
});
