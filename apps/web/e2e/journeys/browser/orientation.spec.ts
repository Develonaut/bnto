import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";

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
 *
 * After processing, the output should have dimensions 800x1200 (orientation
 * physically applied to pixels) with no EXIF orientation tag needed.
 */

const FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../../test-fixtures/images",
);

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

test.describe("EXIF orientation — all image bntos", () => {
  test("compress: portrait JPEG preserves orientation-corrected dimensions", async ({
    page,
  }) => {
    await page.goto("/compress-images");

    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Upload the EXIF-oriented fixture (1200x800 raw, orientation=6)
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "portrait-rotated.jpg"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    // Compress
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    await expect(page.getByText("1 file ready")).toBeVisible();

    // Download and verify dimensions
    const downloadPromise = page.waitForEvent("download");
    await page.locator('[data-testid="download-button"]').click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();

    const outputFile = fs.readFileSync(downloadPath!);

    // Valid JPEG
    expect(outputFile[0]).toBe(0xff);
    expect(outputFile[1]).toBe(0xd8);

    // Dimensions should be orientation-corrected: 800x1200 (portrait)
    // NOT the raw 1200x800 (landscape)
    const dims = getJpegDimensions(outputFile);
    expect(dims.width).toBe(800);
    expect(dims.height).toBe(1200);
  });

  test("resize: portrait JPEG uses orientation-corrected dimensions", async ({
    page,
  }) => {
    await page.goto("/resize-images");

    await expect(
      page.getByRole("heading", { name: "Resize Images Online Free" }),
    ).toBeVisible();

    // Upload the EXIF-oriented fixture
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "portrait-rotated.jpg"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    // Resize (default settings)
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    await expect(page.getByText("1 file ready")).toBeVisible();

    // Download and verify it's a valid JPEG with portrait aspect ratio
    const downloadPromise = page.waitForEvent("download");
    await page.locator('[data-testid="download-button"]').click();
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
    await page.goto("/convert-image-format");

    await expect(
      page.getByRole("heading", {
        name: "Convert Image Format Online Free",
      }),
    ).toBeVisible();

    // Upload file first — config controls only appear in Phase 2 (after file selection)
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "portrait-rotated.jpg"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    // Change target format to PNG
    const formatSelect = page.locator('[data-testid="format-select"]').or(
      page.getByRole("combobox"),
    );
    await formatSelect.click();
    await page.getByRole("option", { name: /png/i }).click();

    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    // Convert
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    await expect(page.getByText("1 file ready")).toBeVisible();

    // Download and verify PNG with orientation-corrected dimensions
    const downloadPromise = page.waitForEvent("download");
    await page.locator('[data-testid="download-button"]').click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();

    const outputFile = fs.readFileSync(downloadPath!);

    // PNG magic bytes: 89 50 4E 47 (‰PNG)
    expect(outputFile[0]).toBe(0x89);
    expect(outputFile[1]).toBe(0x50);
    expect(outputFile[2]).toBe(0x4e);
    expect(outputFile[3]).toBe(0x47);

    // PNG stores dimensions in IHDR chunk at fixed offset:
    //   bytes 0-7: signature
    //   bytes 8-11: chunk length
    //   bytes 12-15: "IHDR"
    //   bytes 16-19: width (big-endian uint32)
    //   bytes 20-23: height (big-endian uint32)
    const pngWidth = outputFile.readUInt32BE(16);
    const pngHeight = outputFile.readUInt32BE(20);

    // Should be portrait: 800x1200 (orientation applied)
    expect(pngWidth).toBe(800);
    expect(pngHeight).toBe(1200);
  });
});
