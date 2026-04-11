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
} from "../../helpers";

/**
 * Browser execution journey — svg-to-jpeg
 *
 * Tests SVG-to-JPEG rasterization running 100% client-side via Rust→WASM.
 * Uses resvg for rendering, image crate for JPEG encoding.
 * Verifies JPEG magic bytes in output.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("svg-to-jpeg — browser execution @browser", () => {
  test("single SVG: rasterize to JPEG, verify magic bytes", async ({ page }) => {
    await navigateToRecipe(page, "svg-to-jpeg");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.svg")]);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(1);

    await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });
  });

  test("batch: multiple SVGs auto-download as ZIP", async ({ page }) => {
    await navigateToRecipe(page, "svg-to-jpeg");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.svg"),
      path.join(IMAGE_FIXTURES_DIR, "mascot-sushi-friends.svg"),
    ]);

    const { download } = await runAndCaptureAutoDownload(page);
    expect(download.suggestedFilename()).toBe("svg-to-jpeg-results.zip");

    await expect(page.getByTestId("output-file")).toHaveCount(2);
  });
});
