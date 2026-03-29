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

/**
 * Browser execution journey — merge-csv
 *
 * Tests CSV merge running 100% client-side via Rust WASM.
 * Batch processor: receives all files at once, outputs a single merged CSV.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("merge-csv — browser execution @browser", () => {
  test("detects browser execution mode", async ({ page }) => {
    await navigateToRecipe(page, "merge-csv", "Merge CSV Online Free");
    await assertBrowserExecution(page);
  });

  test("merge two CSVs: combined rows in output", async ({ page }) => {
    await navigateToRecipe(page, "merge-csv", "Merge CSV Online Free");

    await uploadFiles(page, [
      path.join(CSV_FIXTURES_DIR, "simple.csv"),
      path.join(CSV_FIXTURES_DIR, "numeric-values.csv"),
    ]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    // Download and verify output is valid CSV with rows from both files
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByTestId("download-button").click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/\.csv$/i);

    const downloadPath = await download.path();
    const content = fs.readFileSync(downloadPath!, "utf-8");

    // Should contain data from simple.csv (Alice) and numeric-values.csv
    expect(content).toContain("Alice");
    // Output should have a header row + data rows from both files
    const lines = content.trim().split("\n");
    expect(lines.length).toBeGreaterThan(2);
  });
});
