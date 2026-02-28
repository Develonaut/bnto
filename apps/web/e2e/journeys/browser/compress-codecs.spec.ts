import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Codec-specific compress-images tests.
 *
 * Each test selects a file, compresses it via WASM, downloads the output,
 * and verifies the file header (magic bytes) matches the expected codec.
 */

const FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../../test-fixtures/images",
);

test.describe("compress-images — codec coverage", () => {
  test("JPEG: select, compress, download, verify magic bytes", async ({
    page,
  }) => {
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Select large JPEG
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([path.join(FIXTURES_DIR, "large.jpg")]);
    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await expect(runButton).toBeEnabled();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("01-jpeg-file-selected.png", {
      fullPage: true,
    });

    // Compress
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("02-jpeg-compressed-result.png", {
      fullPage: true,
    });

    // Download and verify
    const inputSize = fs.statSync(
      path.join(FIXTURES_DIR, "large.jpg"),
    ).size;

    const dlPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const dl = await dlPromise;

    expect(dl.suggestedFilename()).toMatch(/\.jpe?g$/i);

    const dlPath = await dl.path();
    const dlFile = fs.readFileSync(dlPath!);
    expect(dlFile.length).toBeGreaterThan(0);
    expect(dlFile.length).toBeLessThanOrEqual(inputSize);

    // JPEG magic bytes: FF D8 FF (SOI marker)
    expect(dlFile[0]).toBe(0xff);
    expect(dlFile[1]).toBe(0xd8);
    expect(dlFile[2]).toBe(0xff);
  });

  test("PNG: select, compress with progress, download, verify magic bytes", async ({
    page,
  }) => {
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Select large PNG (1MB — slow enough for progress capture)
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([path.join(FIXTURES_DIR, "large.png")]);
    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await expect(runButton).toBeEnabled();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("03-png-file-selected.png", {
      fullPage: true,
    });

    // Compress
    await runButton.click();

    // 1MB PNG should take long enough to observe progress
    const progressEl = page.getByRole("progressbar");
    await expect(progressEl).toBeVisible({ timeout: 10000 });

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("04-png-compressing.png", {
      fullPage: true,
    });

    // Wait for completion
    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 60000,
    });

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("05-png-compressed-result.png", {
      fullPage: true,
    });

    // Download and verify
    const inputSize = fs.statSync(
      path.join(FIXTURES_DIR, "large.png"),
    ).size;

    const dlPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const dl = await dlPromise;

    expect(dl.suggestedFilename()).toMatch(/\.png$/i);

    const dlPath = await dl.path();
    const dlFile = fs.readFileSync(dlPath!);
    expect(dlFile.length).toBeGreaterThan(0);
    expect(dlFile.length).toBeLessThanOrEqual(inputSize);

    // PNG magic bytes: 89 50 4E 47 (‰PNG)
    expect(dlFile[0]).toBe(0x89);
    expect(dlFile[1]).toBe(0x50);
    expect(dlFile[2]).toBe(0x4e);
    expect(dlFile[3]).toBe(0x47);
  });

  test("WebP: select, compress, download, verify RIFF/WEBP header", async ({
    page,
  }) => {
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    // Select large WebP
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([path.join(FIXTURES_DIR, "large.webp")]);
    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await expect(runButton).toBeEnabled();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("06-webp-file-selected.png", {
      fullPage: true,
    });

    // Compress
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("07-webp-compressed-result.png", {
      fullPage: true,
    });

    // Download and verify
    // Note: WebP encoding is lossless-only in the Rust image crate.
    // Re-encoding a lossy WebP as lossless can produce a LARGER file.
    // We only verify the output is valid WebP, not that it's smaller.
    const dlPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const dl = await dlPromise;

    expect(dl.suggestedFilename()).toMatch(/\.webp$/i);

    const dlPath = await dl.path();
    const dlFile = fs.readFileSync(dlPath!);
    expect(dlFile.length).toBeGreaterThan(0);

    // WebP header: RIFF....WEBP
    // Bytes 0-3: 52 49 46 46 (RIFF)
    expect(dlFile[0]).toBe(0x52); // R
    expect(dlFile[1]).toBe(0x49); // I
    expect(dlFile[2]).toBe(0x46); // F
    expect(dlFile[3]).toBe(0x46); // F
    // Bytes 8-11: 57 45 42 50 (WEBP)
    expect(dlFile[8]).toBe(0x57); // W
    expect(dlFile[9]).toBe(0x45); // E
    expect(dlFile[10]).toBe(0x42); // B
    expect(dlFile[11]).toBe(0x50); // P
  });
});
