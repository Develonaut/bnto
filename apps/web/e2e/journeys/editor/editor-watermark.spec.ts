import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import { IMAGE_FIXTURES_DIR, MAGIC } from "../../helpers";
import { navigateToEditor, runEditorWithFiles, openRunPanel, selectNode } from "../../helpers/editor";

/**
 * Editor watermark recipe journey.
 *
 * Opens the watermark-images predefined recipe in the editor,
 * configures the watermark image via the config panel's file control,
 * runs with source images, and verifies output.
 *
 * Convention: SETUP → CONFIGURE → EXECUTE → VERIFY.
 * @browser — no Convex backend needed.
 */

const WATERMARK_FIXTURE = path.resolve(__dirname, "../../../../../test-fixtures/watermark.png");

test.describe("editor watermark recipe @browser", () => {
  test("PR-WM1: open watermark recipe, upload watermark, run, verify output", async ({ page }) => {
    await navigateToEditor(page, "watermark-images");

    // Select the Watermark node to open its config panel
    await selectNode(page, "Watermark");

    // Upload watermark image via the file control in the config panel.
    // SchemaField prefixes field names with "param-", so the testid is
    // "control-file-param-watermark" (not "control-file-watermark").
    const fileInput = page.locator('[data-testid="control-file-param-watermark"] input[type="file"]');
    await fileInput.setInputFiles(WATERMARK_FIXTURE);

    // Run with a source image
    await runEditorWithFiles(page, [path.join(IMAGE_FIXTURES_DIR, "small.jpg")]);
    await openRunPanel(page);

    // Should have 1 output file
    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    // Download and verify JPEG magic bytes
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByTestId("download-button").click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    expect(downloadPath).toBeTruthy();
    const buffer = fs.readFileSync(downloadPath!);

    expect(buffer[0]).toBe(MAGIC.JPEG[0]);
    expect(buffer[1]).toBe(MAGIC.JPEG[1]);
    expect(buffer[2]).toBe(MAGIC.JPEG[2]);
    expect(buffer.length).toBeGreaterThan(0);
  });

  test("PR-WM2: batch with landscape image, verify output count", async ({ page }) => {
    await navigateToEditor(page, "watermark-images");

    // Select and configure watermark node
    await selectNode(page, "Watermark");
    const fileInput = page.locator('[data-testid="control-file-param-watermark"] input[type="file"]');
    await fileInput.setInputFiles(WATERMARK_FIXTURE);

    // Run with multiple files
    await runEditorWithFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
      path.join(IMAGE_FIXTURES_DIR, "large.jpg"),
    ]);
    await openRunPanel(page);

    // Should have 2 output files
    await expect(page.getByTestId("output-file")).toHaveCount(2);
  });
});
