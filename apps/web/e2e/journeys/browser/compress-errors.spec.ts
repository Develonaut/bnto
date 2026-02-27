import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Error handling tests for compress-images browser execution.
 *
 * Validates that unsupported and corrupt files produce clear error
 * messages without crashing, and that the "Try Again" reset works.
 */

test.describe("compress-images — error handling", () => {
  test("unsupported file: shows error, no crash", async ({ page }) => {
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Set a file with image/jpeg MIME (bypasses accept filter) but
    // non-image content. WASM will fail to decode this.
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles({
      name: "document.jpg",
      mimeType: "image/jpeg",
      buffer: Buffer.from("This is plain text, not a JPEG image."),
    });

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();

    // Should transition to failed phase
    await expect(runButton).toHaveAttribute("data-phase", "failed", {
      timeout: 30000,
    });

    // Error card should be visible
    const errorCard = page.locator('[data-testid="client-error"]');
    await expect(errorCard).toBeVisible();
    await expect(errorCard).toContainText("Something went wrong");

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("08-error-unsupported-file.png", {
      fullPage: true,
    });

    // Page should still be functional — "Try Again" resets to Phase 1
    await expect(runButton).toContainText("Try Again");
    await runButton.click();
    await expect(page.getByText("Drag & drop files here")).toBeVisible();
    await expect(
      page.locator('[data-testid="run-button"]'),
    ).not.toBeVisible();
  });

  test("corrupt image: error card with Try Again", async ({ page }) => {
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // File with JPEG extension but garbage bytes.
    // ImageFormat::detect() succeeds (extension fallback) but
    // ImageReader::decode() fails → ProcessingFailed error.
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles({
      name: "corrupted-photo.jpg",
      mimeType: "image/jpeg",
      buffer: Buffer.from([
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
        0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
      ]),
    });

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();

    // Should show error state
    await expect(runButton).toHaveAttribute("data-phase", "failed", {
      timeout: 30000,
    });

    const errorCard = page.locator('[data-testid="client-error"]');
    await expect(errorCard).toBeVisible();
    await expect(errorCard).toContainText("Something went wrong");

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("09-error-corrupt-image.png", {
      fullPage: true,
    });

    // "Try Again" resets to Phase 1 (dropzone), ready for new files
    await expect(runButton).toContainText("Try Again");
    await runButton.click();

    await expect(page.getByText("Drag & drop files here")).toBeVisible();
    await expect(
      page.locator('[data-testid="run-button"]'),
    ).not.toBeVisible();
  });
});
