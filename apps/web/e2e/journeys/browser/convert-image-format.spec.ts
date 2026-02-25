import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Browser execution journey — convert-image-format
 *
 * Tests format conversion running 100% client-side via Rust→WASM.
 * Converts between JPEG, PNG, and WebP formats. Magic byte verification
 * confirms the output is a genuinely re-encoded file, not a pass-through.
 *
 * Default config: convert to WebP at quality 80.
 */

const FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../../test-fixtures/images",
);

test.describe("convert-image-format — browser execution", () => {
  test("detects browser execution mode", async ({ page }) => {
    await page.goto("/convert-image-format");

    await expect(
      page.getByRole("heading", {
        name: "Convert Image Format Online Free",
      }),
    ).toBeVisible();

    const shell = page.locator('[data-testid="bnto-shell"]');
    await expect(shell).toHaveAttribute("data-execution-mode", "browser");
  });

  test("JPEG → WebP: convert, download, verify WebP magic bytes", async ({
    page,
  }) => {
    await page.goto("/convert-image-format");

    await expect(
      page.getByRole("heading", {
        name: "Convert Image Format Online Free",
      }),
    ).toBeVisible();

    // Default format is WebP — leave config as-is
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "small.jpg"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("convert-01-jpeg-selected.png", {
      fullPage: true,
    });

    // Convert
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    const results = page.locator(
      '[data-testid="browser-execution-results"]',
    );
    await expect(results).toBeVisible();
    await expect(page.getByText("1 file ready")).toBeVisible();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("convert-02-webp-result.png", {
      fullPage: true,
    });

    // Download and verify WebP magic bytes
    const downloadPromise = page.waitForEvent("download");
    await page.locator('[data-testid="download-button"]').click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/\.webp$/i);

    const downloadPath = await download.path();
    const downloadedFile = fs.readFileSync(downloadPath!);
    expect(downloadedFile.length).toBeGreaterThan(0);

    // WebP header: RIFF....WEBP
    expect(downloadedFile[0]).toBe(0x52); // R
    expect(downloadedFile[1]).toBe(0x49); // I
    expect(downloadedFile[2]).toBe(0x46); // F
    expect(downloadedFile[3]).toBe(0x46); // F
    expect(downloadedFile[8]).toBe(0x57); // W
    expect(downloadedFile[9]).toBe(0x45); // E
    expect(downloadedFile[10]).toBe(0x42); // B
    expect(downloadedFile[11]).toBe(0x50); // P
  });

  test("PNG → JPEG: convert via format selector, verify JPEG magic bytes", async ({
    page,
  }) => {
    await page.goto("/convert-image-format");

    await expect(
      page.getByRole("heading", {
        name: "Convert Image Format Online Free",
      }),
    ).toBeVisible();

    // Change target format to JPEG via the select dropdown
    const formatSelect = page.locator('[data-testid="format-select"]').or(
      page.getByRole("combobox"),
    );
    await formatSelect.click();
    await page.getByRole("option", { name: /jpeg/i }).click();

    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "small.png"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    await expect(page.getByText("1 file ready")).toBeVisible();

    // Download and verify JPEG magic bytes
    const downloadPromise = page.waitForEvent("download");
    await page.locator('[data-testid="download-button"]').click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/\.jpe?g$/i);

    const downloadPath = await download.path();
    const downloadedFile = fs.readFileSync(downloadPath!);
    expect(downloadedFile.length).toBeGreaterThan(0);

    // JPEG magic bytes: FF D8 FF
    expect(downloadedFile[0]).toBe(0xff);
    expect(downloadedFile[1]).toBe(0xd8);
    expect(downloadedFile[2]).toBe(0xff);
  });
});
