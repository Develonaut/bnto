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
  assertWebPBytes,
} from "../../helpers";

test.use({ reducedMotion: "reduce" });

/**
 * Codec-specific compress-images tests.
 *
 * Each test selects a file, compresses it via WASM, downloads the output,
 * and verifies the file header (magic bytes) matches the expected codec.
 */

test.describe("compress-images — codec coverage @browser", () => {
  test("JPEG: select, compress, download, verify magic bytes", async ({
    page,
  }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "large.jpg")]);

    await runAndComplete(page);

    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(1);

    const inputSize = fs.statSync(
      path.join(IMAGE_FIXTURES_DIR, "large.jpg"),
    ).size;

    await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
      maxSize: inputSize,
    });
  });

  test("PNG: select, compress with progress, download, verify magic bytes", async ({
    page,
  }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "large.png")]);

    // Click run manually — we want to observe progress
    const runButton = page.locator('[data-testid="run-button"]:visible');
    await runButton.click();

    // 1MB PNG should take long enough to observe progress
    const progressEl = page.getByRole("progressbar");
    await expect(progressEl).toBeVisible({ timeout: 10000 });

    // Wait for completion
    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 60000,
    });

    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(1);

    const inputSize = fs.statSync(
      path.join(IMAGE_FIXTURES_DIR, "large.png"),
    ).size;

    await downloadAndVerify(page, {
      filenamePattern: /\.png$/i,
      magicBytes: MAGIC.PNG,
      maxSize: inputSize,
    });
  });

  test("WebP: select, compress, download, verify RIFF/WEBP header", async ({
    page,
  }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "large.webp")]);

    await runAndComplete(page);

    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(1);

    // WebP encoding is lossless-only in the Rust image crate.
    // Re-encoding a lossy WebP as lossless can produce a LARGER file.
    // We only verify the output is valid WebP, not that it's smaller.
    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.webp$/i,
    });

    assertWebPBytes(buffer);
  });
});
