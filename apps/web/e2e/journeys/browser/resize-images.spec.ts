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
} from "../../helpers";

/**
 * Browser execution journey — resize-images
 *
 * Tests the resize-images bnto running 100% client-side via Rust→WASM.
 * Resizes images to a target width while optionally maintaining aspect ratio.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("resize-images — browser execution @browser", () => {
  test("detects browser execution mode", async ({ page }) => {
    await navigateToRecipe(page, "resize-images", "Resize Images Online Free");
    await assertBrowserExecution(page);
  });

  test("single JPEG: resize, download, verify valid image", async ({ page }) => {
    await navigateToRecipe(page, "resize-images", "Resize Images Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "medium.jpg")]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    await downloadAndVerify(page, {
      filenamePattern: /\.jpe?g$/i,
      magicBytes: MAGIC.JPEG,
    });
  });

  test("batch: resize two images with Download All", async ({ page }) => {
    await navigateToRecipe(page, "resize-images", "Resize Images Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
    ]);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(2);
    await expect(page.getByTestId("download-all-button", ":visible")).toBeVisible();
  });
});
