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
  assertWebPBytes,
} from "../../helpers";

test.use({ reducedMotion: "reduce" });

/**
 * Browser execution journey — convert-image-format
 *
 * Tests format conversion running 100% client-side via Rust→WASM.
 * Converts between JPEG, PNG, and WebP formats. Magic byte verification
 * confirms the output is a genuinely re-encoded file, not a pass-through.
 */

test.describe("convert-image-format — browser execution @browser", () => {
  test("detects browser execution mode", async ({ page }) => {
    await navigateToRecipe(page, "convert-image-format", "Convert Image Format Online Free");
    await assertBrowserExecution(page);
  });

  test("JPEG → WebP: convert, download, verify WebP magic bytes", async ({
    page,
  }) => {
    await navigateToRecipe(page, "convert-image-format", "Convert Image Format Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);

    await runAndComplete(page);

    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(1);

    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.webp$/i,
    });

    assertWebPBytes(buffer);
  });

  test("PNG → JPEG: convert via format selector, verify JPEG magic bytes", async ({
    page,
  }) => {
    await navigateToRecipe(page, "convert-image-format", "Convert Image Format Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
    ]);

    // Change target format to JPEG via the select dropdown
    const formatSelect = page.locator('[data-testid="format-select"]').or(
      page.getByRole("combobox"),
    );
    await formatSelect.click();
    await page.getByRole("option", { name: /jpeg/i }).click();

    await runAndComplete(page);

    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(1);

    await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });
  });
});
