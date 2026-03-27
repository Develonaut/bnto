import { test, expect } from "../../fixtures";
import { navigateToRecipe } from "../../helpers";

/**
 * Error handling tests for compress-images browser execution.
 *
 * Validates that unsupported and corrupt files produce clear error
 * messages without crashing, and that the "Try Again" reset works.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("compress-images — error handling @browser", () => {
  test("unsupported file: shows error, no crash", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    // Set a file with image/jpeg MIME (bypasses accept filter) but
    // non-image content. WASM will fail to decode this.
    const fileInput = page.getByTestId("file-input");
    await fileInput.setInputFiles({
      name: "document.jpg",
      mimeType: "image/jpeg",
      buffer: Buffer.from("This is plain text, not a JPEG image."),
    });

    await expect(page.getByTestId("file-count")).toBeVisible();

    const runButton = page.getByTestId("run-button", ":visible");
    await runButton.click();

    // Should transition to failed phase
    await expect(runButton).toHaveAttribute("data-phase", "failed", {
      timeout: 30000,
    });

    // Failed banner should be visible in toolbar
    const toolbarProgress = page.getByTestId("toolbar-progress");
    await expect(toolbarProgress).toHaveAttribute("data-status", "failed");

    // Page should still be functional — back button resets to configure phase
    await expect(runButton).toHaveAttribute("aria-label", "Try again");
    const backButton = page.getByTestId("back-button");
    await backButton.click();
    await expect(page.getByTestId("file-count")).toBeVisible();
    await expect(runButton).toHaveAttribute("data-phase", "idle");
  });

  test("corrupt image: error card with Try Again", async ({ page }) => {
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    // File with JPEG extension but garbage bytes.
    const fileInput = page.getByTestId("file-input");
    await fileInput.setInputFiles({
      name: "corrupted-photo.jpg",
      mimeType: "image/jpeg",
      buffer: Buffer.from([
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f,
      ]),
    });

    await expect(page.getByTestId("file-count")).toBeVisible();

    const runButton = page.getByTestId("run-button", ":visible");
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "failed", {
      timeout: 30000,
    });

    // Failed banner should be visible in toolbar
    const toolbarProgress = page.getByTestId("toolbar-progress");
    await expect(toolbarProgress).toHaveAttribute("data-status", "failed");

    // Back button resets to configure phase, ready to try different files
    await expect(runButton).toHaveAttribute("aria-label", "Try again");
    const backButton = page.getByTestId("back-button");
    await backButton.click();
    await expect(page.getByTestId("file-count")).toBeVisible();
    await expect(runButton).toHaveAttribute("data-phase", "idle");
  });
});
