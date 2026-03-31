import path from "path";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  MAGIC,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  downloadAndVerify,
  assertWebPBytes,
  openConfigDialog,
  closeConfigDialog,
} from "../../helpers";

/**
 * Browser execution journey — convert-image-format
 *
 * Tests format conversion running 100% client-side via Rust→WASM.
 * Converts between JPEG, PNG, and WebP formats. Magic byte verification
 * confirms the output is a genuinely re-encoded file, not a pass-through.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("convert-image-format — browser execution @browser", () => {
  test("JPEG → WebP: convert, download, verify WebP magic bytes", async ({ page }) => {
    await navigateToRecipe(page, "convert-image-format", "Convert Image Format Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(1);

    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.webp$/i,
    });

    assertWebPBytes(buffer);
  });

  test("PNG → JPEG: convert via config dialog format selector, verify JPEG magic bytes", async ({
    page,
  }) => {
    await navigateToRecipe(page, "convert-image-format", "Convert Image Format Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.png")]);

    // Open config dialog and change target format to JPEG
    await openConfigDialog(page);
    await page.getByTestId("control-select-param-format").click();
    await page.getByTestId("select-option-jpeg").click();
    await closeConfigDialog(page);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(1);

    await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });
  });
});
