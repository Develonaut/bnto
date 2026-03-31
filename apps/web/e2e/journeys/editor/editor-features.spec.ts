import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import { IMAGE_FIXTURES_DIR } from "../../helpers";
import { navigateToEditor, runEditorWithFiles, exportRecipe } from "../../helpers/editor";

/**
 * Editor feature tests — file input, auto-download, export.
 *
 * Non-recipe-specific editor behaviors verified programmatically.
 *
 * Convention: SETUP → EXECUTE → VERIFY.
 * @browser — no Convex backend needed.
 */

test.describe("editor features @browser", () => {
  test("FA1: file input accept attribute reflects Input node extensions", async ({ page }) => {
    // Load compress-images — Input node has image MIME types + extensions
    await navigateToEditor(page, "compress-images");

    const fileInput = page.getByTestId("run-file-input");
    const accept = await fileInput.getAttribute("accept");

    // Should contain image MIME types or file extensions — not be empty/null
    expect(accept).toBeTruthy();
    expect(accept).toMatch(/image\/|\.jpg|\.jpeg|\.png|\.webp/);
  });

  test("FA2: CSV recipe file input accepts CSV extensions", async ({ page }) => {
    // Load clean-csv — Input node has CSV MIME types + extensions
    await navigateToEditor(page, "clean-csv");

    const fileInput = page.getByTestId("run-file-input");
    const accept = await fileInput.getAttribute("accept");

    expect(accept).toBeTruthy();
    expect(accept).toMatch(/\.csv|text\/csv/);
  });

  test("AD1: auto-download fires on completion", async ({ page }) => {
    await navigateToEditor(page, "compress-images");

    const inputFile = path.join(IMAGE_FIXTURES_DIR, "small.jpg");

    // Set up download listener BEFORE running — auto-download fires on completion
    const downloadPromise = page.waitForEvent("download", { timeout: 30_000 });
    await runEditorWithFiles(page, [inputFile]);
    const download = await downloadPromise;

    // Verify a real file was downloaded automatically (no manual click)
    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();
    const buffer = fs.readFileSync(downloadPath!);
    expect(buffer.length).toBeGreaterThan(0);
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
