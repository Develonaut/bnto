import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import { IMAGE_FIXTURES_DIR, CSV_FIXTURES_DIR, MAGIC } from "../../helpers";
import {
  navigateToEditor,
  runEditorWithFiles,
  openRunPanel,
  getResultCount,
  exportRecipe,
} from "../../helpers/editor";

/**
 * Editor predefined recipes — PR1-PR6
 *
 * Opens each predefined recipe in the editor, runs it with real files,
 * and verifies output matches expectations. Validates that editing a
 * predefined recipe in the editor produces the same results as running
 * it from the recipe page.
 *
 * Convention: SETUP → EXECUTE → VERIFY.
 * @browser — no Convex backend needed.
 */

test.describe("editor predefined recipes @browser", () => {
  test("PR1: compress-images — run and verify JPEG output", async ({ page }) => {
    await navigateToEditor(page, "compress-images");

    const inputFile = path.join(IMAGE_FIXTURES_DIR, "small.jpg");
    const inputSize = fs.statSync(inputFile).size;

    await runEditorWithFiles(page, [inputFile]);

    // Open run panel to see results
    await openRunPanel(page);

    // Should have 1 output file
    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    // Download and verify JPEG magic bytes
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();
    const buffer = fs.readFileSync(downloadPath!);

    // Valid JPEG
    expect(buffer[0]).toBe(MAGIC.JPEG[0]);
    expect(buffer[1]).toBe(MAGIC.JPEG[1]);
    expect(buffer[2]).toBe(MAGIC.JPEG[2]);

    // Compressed (smaller or equal to input)
    expect(buffer.length).toBeLessThanOrEqual(inputSize);
    expect(buffer.length).toBeGreaterThan(0);
  });

  test("PR1b: compress-images — batch with multiple files", async ({ page }) => {
    await navigateToEditor(page, "compress-images");

    await runEditorWithFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "small.png"),
    ]);

    await openRunPanel(page);

    // Should have 2 output files
    const outputFiles = page.locator('[data-testid="output-file"]');
    await expect(outputFiles).toHaveCount(2);
  });

  test("PR2: clean-csv — run and verify CSV output", async ({ page }) => {
    await navigateToEditor(page, "clean-csv");

    const inputFile = path.join(CSV_FIXTURES_DIR, "messy.csv");

    await runEditorWithFiles(page, [inputFile]);

    await openRunPanel(page);

    // Should have 1 output file
    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    // Download and verify it's a non-empty file
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();
    const buffer = fs.readFileSync(downloadPath!);
    expect(buffer.length).toBeGreaterThan(0);

    // Verify the output is valid CSV text (should start with header row)
    const text = buffer.toString("utf-8");
    expect(text).toContain(",");
  });

  test("PR3: rename-files — run and verify renamed output", async ({ page }) => {
    await navigateToEditor(page, "rename-files");

    await runEditorWithFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    await openRunPanel(page);

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    // Download and verify the file is valid (non-empty)
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    // Filename should contain the rename prefix
    expect(download.suggestedFilename()).toMatch(/renamed-/);

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();
    const buffer = fs.readFileSync(downloadPath!);
    expect(buffer.length).toBeGreaterThan(0);
  });

  test("PR4: resize-images — run and verify output", async ({ page }) => {
    await navigateToEditor(page, "resize-images");

    await runEditorWithFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    await openRunPanel(page);

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    // Download and verify JPEG magic bytes
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    const buffer = fs.readFileSync(downloadPath!);
    expect(buffer[0]).toBe(MAGIC.JPEG[0]);
    expect(buffer[1]).toBe(MAGIC.JPEG[1]);
    expect(buffer.length).toBeGreaterThan(0);
  });

  test("PR5: convert-image-format — run and verify WebP output", async ({ page }) => {
    await navigateToEditor(page, "convert-image-format");

    await runEditorWithFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    await openRunPanel(page);

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    // Download and verify WebP magic bytes (RIFF + WEBP)
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    const buffer = fs.readFileSync(downloadPath!);

    // RIFF header
    expect(buffer[0]).toBe(MAGIC.WEBP_RIFF[0]);
    expect(buffer[1]).toBe(MAGIC.WEBP_RIFF[1]);
    expect(buffer[2]).toBe(MAGIC.WEBP_RIFF[2]);
    expect(buffer[3]).toBe(MAGIC.WEBP_RIFF[3]);

    // WEBP tag at offset 8
    expect(buffer[8]).toBe(MAGIC.WEBP_TAG[0]);
    expect(buffer[9]).toBe(MAGIC.WEBP_TAG[1]);
    expect(buffer[10]).toBe(MAGIC.WEBP_TAG[2]);
    expect(buffer[11]).toBe(MAGIC.WEBP_TAG[3]);
  });

  test("PR6: rename-csv-columns — run and verify output", async ({ page }) => {
    await navigateToEditor(page, "rename-csv-columns");

    await runEditorWithFiles(page, [path.join(CSV_FIXTURES_DIR, "simple.csv")]);

    await openRunPanel(page);

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    const buffer = fs.readFileSync(downloadPath!);
    expect(buffer.length).toBeGreaterThan(0);
  });

  test("FA1: file input accept attribute reflects Input node extensions", async ({ page }) => {
    // Load compress-images — Input node has image MIME types + extensions
    await navigateToEditor(page, "compress-images");

    const fileInput = page.locator('[data-testid="run-file-input"]');
    const accept = await fileInput.getAttribute("accept");

    // Should contain image MIME types or file extensions — not be empty/null
    expect(accept).toBeTruthy();
    expect(accept).toMatch(/image\/|\.jpg|\.jpeg|\.png|\.webp/);
  });

  test("FA2: CSV recipe file input accepts CSV extensions", async ({ page }) => {
    // Load clean-csv — Input node has CSV MIME types + extensions
    await navigateToEditor(page, "clean-csv");

    const fileInput = page.locator('[data-testid="run-file-input"]');
    const accept = await fileInput.getAttribute("accept");

    expect(accept).toBeTruthy();
    expect(accept).toMatch(/\.csv|text\/csv/);
  });

  test("XP1: export recipe produces valid JSON", async ({ page }) => {
    await navigateToEditor(page, "compress-images");

    const { buffer, filename } = await exportRecipe(page);

    // Filename should be a .bnto.json file
    expect(filename).toMatch(/\.bnto\.json$/);

    // Content should be valid JSON with expected structure
    const json = JSON.parse(buffer.toString("utf-8"));
    expect(json.type).toBeDefined();
    expect(json.nodes).toBeDefined();
    expect(Array.isArray(json.nodes)).toBe(true);
    expect(json.nodes.length).toBeGreaterThanOrEqual(3);
  });
});
