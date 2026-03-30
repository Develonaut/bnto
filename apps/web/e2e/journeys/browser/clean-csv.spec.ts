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
 * Browser execution journey — clean-csv
 *
 * Tests CSV cleaning running 100% client-side via Rust→WASM.
 * Default config: trim whitespace + remove empty rows (duplicates off).
 *
 * messy.csv has:
 *   - 1 header row
 *   - 5 data rows (Alice, Bob, Charlie, Alice duplicate, Bob duplicate, Diana)
 *   - 2 empty rows (lines 4 and 8)
 *   - Whitespace padding on Alice, Charlie, Bob (second)
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("clean-csv — browser execution @browser", () => {
  test("messy CSV: verify empty rows removed and whitespace trimmed", async ({ page }) => {
    await navigateToRecipe(page, "clean-csv", "Clean CSV Online Free");

    await uploadFiles(page, [path.join(CSV_FIXTURES_DIR, "messy.csv")]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByTestId("download-button").click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/\.csv$/i);

    const downloadPath = await download.path();
    const downloadedFile = fs.readFileSync(downloadPath!, "utf-8");

    const { headers, rows } = parseCsvOutput(downloadedFile);

    // Headers preserved
    expect(headers).toEqual(["name", "age", "city"]);

    // messy.csv has 2 empty rows + 2 duplicate rows (Alice, Bob appear twice)
    // Default config: trim + remove empty + remove duplicates = 4 unique data rows
    expect(rows.length).toBe(4);

    // No empty rows in output
    for (const row of rows) {
      const allEmpty = row.every((cell) => cell === "");
      expect(allEmpty).toBe(false);
    }

    // Whitespace trimmed — no leading/trailing spaces
    for (const row of rows) {
      for (const cell of row) {
        expect(cell).toBe(cell.trim());
      }
    }

    // Core data present
    const names = rows.map((r) => r[0]);
    expect(names).toContain("Alice");
    expect(names).toContain("Bob");
    expect(names).toContain("Diana");
  });

  test("simple CSV: passes through cleanly, all 5 rows preserved", async ({ page }) => {
    await navigateToRecipe(page, "clean-csv", "Clean CSV Online Free");

    await uploadFiles(page, [path.join(CSV_FIXTURES_DIR, "simple.csv")]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByTestId("download-button").click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    const output = fs.readFileSync(downloadPath!, "utf-8");

    const { headers, rows } = parseCsvOutput(output);

    // simple.csv has 5 data rows — all should survive cleaning
    expect(headers).toContain("name");
    expect(rows).toHaveLength(5);

    const names = rows.map((r) => r[0]);
    expect(names).toEqual(["Alice", "Bob", "Charlie", "Diana", "Eve"]);
  });
});
