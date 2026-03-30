import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  runAndCaptureAutoDownload,
  assertContentPreserved,
} from "../../helpers";

/**
 * Browser execution journey — rename-files
 *
 * Tests file renaming running 100% client-side via Rust→WASM.
 * Accepts any file type. The pattern config (default: "renamed-{{name}}")
 * determines the output filename. File contents are preserved as-is.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("rename-files — browser execution @browser", () => {
  test("single file: rename with default pattern, verify content preserved", async ({ page }) => {
    const inputPath = path.join(IMAGE_FIXTURES_DIR, "small.jpg");

    await navigateToRecipe(page, "rename-files", "Rename Files Online Free");

    await uploadFiles(page, [inputPath]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    // Verify download filename includes "renamed"
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByTestId("download-button").click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toContain("renamed");

    // File content must be byte-identical to input — rename is content-preserving
    const downloadPath = await download.path();
    const outputBuffer = fs.readFileSync(downloadPath!);
    assertContentPreserved(inputPath, outputBuffer);
  });

  test("batch: rename multiple files auto-download as ZIP", async ({ page }) => {
    await navigateToRecipe(page, "rename-files", "Rename Files Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
      path.join(IMAGE_FIXTURES_DIR, "small.webp"),
    ]);

    const { download } = await runAndCaptureAutoDownload(page);
    expect(download.suggestedFilename()).toBe("rename-files-results.zip");

    await expect(page.getByTestId("output-file")).toHaveCount(3);
  });
});
