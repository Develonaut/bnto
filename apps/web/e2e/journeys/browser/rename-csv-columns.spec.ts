import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import {
  CSV_FIXTURES_DIR,
  navigateToRecipe,
  assertBrowserExecution,
  uploadFiles,
  runAndComplete,
} from "../../helpers";

test.use({ reducedMotion: "reduce" });

/**
 * Browser execution journey — rename-csv-columns
 *
 * Tests CSV column renaming running 100% client-side via Rust→WASM.
 */

test.describe("rename-csv-columns — browser execution @browser", () => {
  test("detects browser execution mode", async ({ page }) => {
    await navigateToRecipe(page, "rename-csv-columns", "Rename CSV Columns Online Free");
    await assertBrowserExecution(page);
  });

  test("process CSV: output preserves column structure", async ({
    page,
  }) => {
    await navigateToRecipe(page, "rename-csv-columns", "Rename CSV Columns Online Free");

    await uploadFiles(page, [
      path.join(CSV_FIXTURES_DIR, "simple.csv"),
    ]);

    await runAndComplete(page);

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    // Download and verify output is valid CSV with columns preserved
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/\.csv$/i);

    const downloadPath = await download.path();
    const output = fs.readFileSync(downloadPath!, "utf-8");

    const firstLine = output.split("\n")[0];
    expect(firstLine).toBeTruthy();

    expect(output).toContain("Alice");
    expect(output).toContain("Bob");
  });

  test("many-column CSV: all columns survive processing", async ({
    page,
  }) => {
    await navigateToRecipe(page, "rename-csv-columns", "Rename CSV Columns Online Free");

    await uploadFiles(page, [
      path.join(CSV_FIXTURES_DIR, "many-columns.csv"),
    ]);

    await runAndComplete(page);

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    const output = fs.readFileSync(downloadPath!, "utf-8");

    expect(output).toContain("first_name");
    expect(output).toContain("last_name");
    expect(output).toContain("email");
    expect(output).toContain("department");

    expect(output).toContain("Jane");
    expect(output).toContain("Engineering");
  });
});
