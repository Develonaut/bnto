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
  assertWebPBytes,
} from "../../helpers";

/**
 * Browser execution journey — optimize-images-for-web
 *
 * Multi-node pipeline: resize → convert (WebP) → compress.
 * First multi-node predefined recipe. Verified programmatically:
 * WebP magic bytes (proves convert step ran), file sizes, data attributes.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("optimize-images-for-web — browser execution @browser", () => {
  test("detects browser execution mode", async ({ page }) => {
    await navigateToRecipe(page, "optimize-images-for-web", "Optimize Images for Web Online Free");
    await assertBrowserExecution(page);
  });

  test("single JPEG: resize + convert + compress lifecycle", async ({ page }) => {
    await navigateToRecipe(page, "optimize-images-for-web", "Optimize Images for Web Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);
    await expect(outputFile.getByTestId("download-button")).toBeVisible();

    // Output should be WebP (convert step) — verify RIFF+WEBP magic bytes
    const buffer = await downloadAndVerify(page, {
      filenamePattern: /\.webp$/i,
      magicBytes: MAGIC.WEBP_RIFF,
    });

    assertWebPBytes(buffer);
    expect(buffer.length).toBeGreaterThan(0);
  });

  test("batch: multiple images with Download All as ZIP", async ({ page }) => {
    await navigateToRecipe(page, "optimize-images-for-web", "Optimize Images for Web Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
    ]);

    await runAndComplete(page);

    await expect(page.getByTestId("output-file")).toHaveCount(2);

    const { download } = await downloadAllAsZip(page);
    expect(download.suggestedFilename()).toBe("optimize-images-for-web-results.zip");
  });

  test("back button resets from completed to configure phase", async ({ page }) => {
    await navigateToRecipe(page, "optimize-images-for-web", "Optimize Images for Web Online Free");

    await uploadFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    const runButton = await runAndComplete(page);

    const backButton = page.getByTestId("back-button");
    await backButton.click();

    await expect(page.getByTestId("file-count")).toBeVisible();
    await expect(runButton).toHaveAttribute("data-phase", "idle");
  });
});
