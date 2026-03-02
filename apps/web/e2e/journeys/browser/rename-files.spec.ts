import path from "path";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  navigateToRecipe,
  assertBrowserExecution,
  uploadFiles,
  runAndComplete,
} from "../../helpers";

test.use({ reducedMotion: "reduce" });

/**
 * Browser execution journey — rename-files
 *
 * Tests file renaming running 100% client-side via Rust→WASM.
 * Accepts any file type. The pattern config (default: "renamed-{{name}}")
 * determines the output filename. File contents are preserved as-is.
 */

test.describe("rename-files — browser execution @browser", () => {
  test("detects browser execution mode", async ({ page }) => {
    await navigateToRecipe(page, "rename-files", "Rename Files Online Free");
    await assertBrowserExecution(page);
  });

  test("single file: rename with default pattern, download", async ({
    page,
  }) => {
    await navigateToRecipe(page, "rename-files", "Rename Files Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);

    await runAndComplete(page);

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    // Verify download filename includes "renamed-"
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toContain("renamed");
  });

  test("batch: rename multiple files with Download All", async ({
    page,
  }) => {
    await navigateToRecipe(page, "rename-files", "Rename Files Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
      path.join(IMAGE_FIXTURES_DIR, "small.webp"),
    ]);

    await runAndComplete(page);

    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(3);
    await expect(
      page.getByRole("button", { name: /download all/i }).last(),
    ).toBeVisible();
  });
});
