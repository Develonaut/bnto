import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import {
  CSV_FIXTURES_DIR,
  navigateToRecipe,
  uploadFiles,
  runAndComplete,
  parseCsvOutput,
} from "../../helpers";

/**
 * Browser execution journey — merge-csv
 *
 * Tests CSV merge running 100% client-side via Rust WASM.
 * Batch processor: receives all files at once, outputs a single merged CSV.
 *
 * simple.csv: 5 data rows (name,age,city,email,active)
 * messy.csv: data rows with same headers (name,age,city)
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("merge-csv — browser execution @browser", () => {
  test("merge two CSVs with matching headers: combined rows", async ({ page }) => {
    await navigateToRecipe(page, "merge-csv", "Merge CSV Online Free");

    // Use CSVs with overlapping headers for meaningful merge
    await uploadFiles(page, [
      path.join(CSV_FIXTURES_DIR, "simple.csv"),
      path.join(CSV_FIXTURES_DIR, "messy.csv"),
    ]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByTestId("download-button").click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/\.csv$/i);

    const downloadPath = await download.path();
    const content = fs.readFileSync(downloadPath!, "utf-8");

    const { headers, rows } = parseCsvOutput(content);

    // Headers present
    expect(headers).toContain("name");

    // Combined rows from both files — simple.csv has 5 rows, messy.csv has data rows
    // Total should be more than either file alone
    expect(rows.length).toBeGreaterThan(5);

    // Data from simple.csv
    const allValues = rows.flat().join(",");
    expect(allValues).toContain("Alice");
    expect(allValues).toContain("Eve");

    // Data from messy.csv — Diana and Charlie are in messy.csv
    expect(allValues).toContain("Diana");
    expect(allValues).toContain("Charlie");
  });
});
