import path from "path";
import { test, expect } from "../../fixtures";
import { IMAGE_FIXTURES_DIR, CSV_FIXTURES_DIR } from "../../helpers";
import {
  navigateToEditor,
  addNodeFromPalette,
  selectNode,
  ensureConfigPanelOpen,
  setNumberParam,
  setTextParam,
  setSelectParam,
  runEditorWithFiles,
  openRunPanel,
  exportRecipe,
} from "../../helpers/editor";
import { assertDefinitionMatchesFixture } from "../../helpers/definitions";
import { downloadFirstOutput, assertWebP, assertJpeg } from "../../helpers/assertions";

/**
 * Custom recipe journeys — multi-node pipelines built from scratch.
 *
 * Convention: SETUP → BUILD → CONFIGURE → EXECUTE → VERIFY → DRIFT CHECK.
 *
 * The DRIFT CHECK phase exports the recipe and compares its structure against
 * authoritative recipe definitions in engine/recipes/ (engine-owned .bnto.json files).
 *
 * @browser — no Convex backend needed.
 */

test.describe("editor custom recipes @browser", () => {
  test("CR1: web-ready image pipeline — resize → convert → compress", async ({ page }) => {
    await navigateToEditor(page);
    const nodeCards = page.getByTestId("node-card");

    // BUILD — add 3 image processing nodes
    await addNodeFromPalette(page, "Resize Images");
    await addNodeFromPalette(page, "Convert Image Format");
    await addNodeFromPalette(page, "Compress Images");
    await expect(nodeCards).toHaveCount(5); // Input + 3 ops + Output

    // CONFIGURE — set resize width (matches optimize-images-for-web predefined recipe)
    await selectNode(page, "Resize");
    await ensureConfigPanelOpen(page);
    await setNumberParam(page, "width", 800);

    // CONFIGURE — set convert format to webp
    await selectNode(page, "Convert");
    await ensureConfigPanelOpen(page);
    await setSelectParam(page, "format", "webp");

    // EXECUTE
    const inputFile = path.join(IMAGE_FIXTURES_DIR, "small.jpg");
    await runEditorWithFiles(page, [inputFile]);

    // VERIFY — output should be WebP (from convert step)
    await openRunPanel(page);
    const { download, buffer } = await downloadFirstOutput(page);
    assertWebP(buffer);
    expect(buffer.length).toBeGreaterThan(0);
    expect(download.suggestedFilename()).toMatch(/\.webp$/i);

    // DRIFT CHECK
    const { buffer: exportBuffer } = await exportRecipe(page);
    const exported = JSON.parse(exportBuffer.toString("utf-8"));
    assertDefinitionMatchesFixture(exported, "optimize-images-for-web.bnto.json");
  });

  test("CR2: compress + organize — compress → rename with suffix", async ({ page }) => {
    await navigateToEditor(page);
    const nodeCards = page.getByTestId("node-card");

    // BUILD
    await addNodeFromPalette(page, "Compress Images");
    await addNodeFromPalette(page, "Rename Files");
    await expect(nodeCards).toHaveCount(4); // Input + 2 ops + Output

    // CONFIGURE — set rename suffix
    await selectNode(page, "Rename");
    await ensureConfigPanelOpen(page);
    await setTextParam(page, "suffix", "-min");

    // EXECUTE
    await runEditorWithFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // VERIFY
    await openRunPanel(page);
    const { download, buffer } = await downloadFirstOutput(page);
    assertJpeg(buffer);
    expect(buffer.length).toBeGreaterThan(0);
    expect(download.suggestedFilename()).toContain("-min");

    // DRIFT CHECK
    const { buffer: exportBuffer } = await exportRecipe(page);
    const exported = JSON.parse(exportBuffer.toString("utf-8"));
    assertDefinitionMatchesFixture(exported, "compress-and-rename.bnto.json");
  });

  test("CR3: clean & standardize CSV — clean → rename columns", async ({ page }) => {
    await navigateToEditor(page);
    const nodeCards = page.getByTestId("node-card");

    // BUILD — add both CSV operations
    await addNodeFromPalette(page, "Clean CSV");
    await addNodeFromPalette(page, "Rename CSV Columns");
    await expect(nodeCards).toHaveCount(4); // Input + 2 ops + Output

    // EXECUTE
    await runEditorWithFiles(page, [path.join(CSV_FIXTURES_DIR, "messy.csv")]);

    // VERIFY
    await openRunPanel(page);
    const { buffer } = await downloadFirstOutput(page);
    const text = buffer.toString("utf-8");
    expect(text).toContain(",");

    // Proves CLEAN step ran: no empty rows
    const rows = text.trim().split("\n");
    for (const row of rows) {
      const cells = row.split(",");
      const allEmpty = cells.every((c) => c.trim() === "");
      expect(allEmpty).toBe(false);
    }

    // Proves CLEAN step ran: trimmed whitespace
    expect(text).not.toMatch(/\s{2,}[A-Z]/);

    // Proves CLEAN step ran: removed duplicates
    const dataRows = rows.slice(1);
    expect(dataRows.length).toBeLessThan(8);

    // Proves RENAME COLUMNS step ran: headers pass through
    expect(rows[0]).toContain("name");
    expect(rows[0]).toContain("age");
    expect(rows[0]).toContain("city");

    // DRIFT CHECK
    const { buffer: exportBuffer } = await exportRecipe(page);
    const exported = JSON.parse(exportBuffer.toString("utf-8"));
    assertDefinitionMatchesFixture(exported, "standardize-csv.bnto.json");
  });

  test("CR4: thumbnail generator — resize → convert → rename", async ({ page }) => {
    await navigateToEditor(page);
    const nodeCards = page.getByTestId("node-card");

    // BUILD — 3 nodes spanning image + file types
    await addNodeFromPalette(page, "Resize Images");
    await addNodeFromPalette(page, "Convert Image Format");
    await addNodeFromPalette(page, "Rename Files");
    await expect(nodeCards).toHaveCount(5); // Input + 3 ops + Output

    // CONFIGURE — resize to thumbnail size (matches generate-thumbnails predefined recipe)
    await selectNode(page, "Resize");
    await ensureConfigPanelOpen(page);
    await setNumberParam(page, "width", 150);

    // CONFIGURE — convert to WebP
    await selectNode(page, "Convert");
    await ensureConfigPanelOpen(page);
    await setSelectParam(page, "format", "webp");

    // CONFIGURE — add thumbnail prefix
    await selectNode(page, "Rename");
    await ensureConfigPanelOpen(page);
    await setTextParam(page, "prefix", "thumb_");

    // EXECUTE
    await runEditorWithFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);

    // VERIFY
    await openRunPanel(page);
    const { download, buffer } = await downloadFirstOutput(page);
    expect(download.suggestedFilename()).toMatch(/^thumb_/);
    assertWebP(buffer);
    expect(buffer.length).toBeGreaterThan(0);
    expect(download.suggestedFilename()).toMatch(/\.webp$/i);

    // DRIFT CHECK
    const { buffer: exportBuffer } = await exportRecipe(page);
    const exported = JSON.parse(exportBuffer.toString("utf-8"));
    assertDefinitionMatchesFixture(exported, "generate-thumbnails.bnto.json");
  });

  test("CR5: all 6 operations — add each from palette, verify structure", async ({ page }) => {
    await navigateToEditor(page);
    const nodeCards = page.getByTestId("node-card");

    // BUILD — add all 6 browser operations
    await addNodeFromPalette(page, "Compress Images");
    await addNodeFromPalette(page, "Resize Images");
    await addNodeFromPalette(page, "Convert Image Format");
    await addNodeFromPalette(page, "Rename Files");
    await addNodeFromPalette(page, "Clean CSV");
    await addNodeFromPalette(page, "Rename CSV Columns");
    await expect(nodeCards).toHaveCount(8); // 2 I/O + 6 processing

    // CONFIGURE — image-convert requires format before export is enabled
    await selectNode(page, "Convert");
    await ensureConfigPanelOpen(page);
    await setSelectParam(page, "format", "webp");

    // VERIFY — export and check all node types present
    const { buffer, filename } = await exportRecipe(page);
    expect(filename).toMatch(/\.bnto\.json$/);

    const json = JSON.parse(buffer.toString("utf-8"));
    expect(json.nodes).toBeDefined();
    expect(json.nodes.length).toBe(8);

    // No drift check — this is a custom all-operations composition with
    // no predefined recipe equivalent. Structure is verified above.
  });
});
