import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import { IMAGE_FIXTURES_DIR, CSV_FIXTURES_DIR, MAGIC } from "../../helpers";
import {
  navigateToEditor,
  addNodeFromPalette,
  selectNode,
  ensureConfigPanelOpen,
  setSelectParam,
  runEditorWithFiles,
  openRunPanel,
  exportRecipe,
} from "../../helpers/editor";

/**
 * Editor from-scratch journeys — build recipes on a blank canvas.
 *
 * These tests validate the core user story: open a blank editor,
 * add processing nodes, configure them, upload files, run, and
 * verify output. This proves the editor can create new recipes
 * from scratch that produce correct results.
 *
 * Convention: SETUP → BUILD → EXECUTE → VERIFY.
 * @browser — no Convex backend needed.
 */

test.describe("editor from-scratch journeys @browser", () => {
  test("build image compress recipe from scratch", async ({ page }) => {
    // SETUP — blank canvas
    await navigateToEditor(page);

    // BUILD — add compress node
    await addNodeFromPalette(page, "Compress Images");

    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards).toHaveCount(3); // Input + Compress + Output

    // EXECUTE — upload and run
    const inputFile = path.join(IMAGE_FIXTURES_DIR, "small.jpg");
    const inputSize = fs.statSync(inputFile).size;
    await runEditorWithFiles(page, [inputFile]);

    // VERIFY — check output
    await openRunPanel(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByTestId("download-button").click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    const buffer = fs.readFileSync(downloadPath!);

    // Valid JPEG output
    expect(buffer[0]).toBe(MAGIC.JPEG[0]);
    expect(buffer[1]).toBe(MAGIC.JPEG[1]);
    expect(buffer[2]).toBe(MAGIC.JPEG[2]);

    // Compressed
    expect(buffer.length).toBeLessThanOrEqual(inputSize);
    expect(buffer.length).toBeGreaterThan(0);
  });

  test("build image resize recipe from scratch — verify structure", async ({ page }) => {
    await navigateToEditor(page);

    // BUILD — add resize node
    await addNodeFromPalette(page, "Resize Images");

    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards).toHaveCount(3); // Input + Resize + Output

    // VERIFY — export and check structure (execution requires width/height
    // configuration, which is tested via PR4 predefined recipe)
    const { buffer, filename } = await exportRecipe(page);
    expect(filename).toMatch(/\.bnto\.json$/);

    const json = JSON.parse(buffer.toString("utf-8"));
    const types = json.nodes.map((n: { type: string }) => n.type);
    expect(types).toContain("input");
    expect(types).toContain("output");
    expect(types).toContain("image-resize");
  });

  test("build CSV clean recipe from scratch", async ({ page }) => {
    await navigateToEditor(page);

    // BUILD — add clean node
    await addNodeFromPalette(page, "Clean CSV");

    // EXECUTE
    await runEditorWithFiles(page, [path.join(CSV_FIXTURES_DIR, "messy.csv")]);

    // VERIFY
    await openRunPanel(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByTestId("download-button").click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    const buffer = fs.readFileSync(downloadPath!);
    expect(buffer.length).toBeGreaterThan(0);

    // Valid CSV text
    const text = buffer.toString("utf-8");
    expect(text).toContain(",");
  });

  test("build file rename recipe from scratch — verify structure", async ({ page }) => {
    await navigateToEditor(page);

    // BUILD — add rename node
    await addNodeFromPalette(page, "Rename Files");

    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards).toHaveCount(3); // Input + Rename + Output

    // VERIFY — export and check structure (execution requires rename config,
    // which is tested via PR3 predefined recipe)
    const { buffer, filename } = await exportRecipe(page);
    expect(filename).toMatch(/\.bnto\.json$/);

    const json = JSON.parse(buffer.toString("utf-8"));
    const types = json.nodes.map((n: { type: string }) => n.type);
    expect(types).toContain("input");
    expect(types).toContain("output");
    expect(types).toContain("file-rename");
  });

  test("build convert-to-webp recipe from scratch — verify structure", async ({ page }) => {
    await navigateToEditor(page);

    // BUILD — add convert node
    await addNodeFromPalette(page, "Convert Image Format");

    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards).toHaveCount(3); // Input + Convert + Output

    // CONFIGURE — format is required before export is enabled
    await selectNode(page, "Convert");
    await ensureConfigPanelOpen(page);
    await setSelectParam(page, "format", "webp");

    // VERIFY — export and check structure
    const { buffer, filename } = await exportRecipe(page);
    expect(filename).toMatch(/\.bnto\.json$/);

    const json = JSON.parse(buffer.toString("utf-8"));
    const types = json.nodes.map((n: { type: string }) => n.type);
    expect(types).toContain("input");
    expect(types).toContain("output");
    expect(types).toContain("image-convert");
  });

  test("build recipe from scratch and export as JSON", async ({ page }) => {
    await navigateToEditor(page);

    // BUILD — add a compress node
    await addNodeFromPalette(page, "Compress Images");

    // VERIFY export
    const { buffer, filename } = await exportRecipe(page);
    expect(filename).toMatch(/\.bnto\.json$/);

    const json = JSON.parse(buffer.toString("utf-8"));
    expect(json.nodes).toBeDefined();
    expect(Array.isArray(json.nodes)).toBe(true);

    // Should have Input + Compress + Output
    expect(json.nodes.length).toBe(3);

    // Verify node types
    const types = json.nodes.map((n: { type: string }) => n.type);
    expect(types).toContain("input");
    expect(types).toContain("output");
    expect(types).toContain("image-compress");
  });

  test("run again after completion reuses files", async ({ page }) => {
    await navigateToEditor(page);
    await addNodeFromPalette(page, "Compress Images");

    // First run
    await runEditorWithFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    const runButton = page.getByTestId("run-button");
    await expect(runButton).toHaveAttribute("data-phase", "completed");

    // Run again (should reuse the same files)
    await runButton.click();
    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30_000,
    });

    // Still shows results
    await openRunPanel(page);
    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);
  });
});
