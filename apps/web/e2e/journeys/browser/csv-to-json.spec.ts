import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";
import { CSV_FIXTURES_DIR, navigateToRecipe, uploadFiles, runAndComplete } from "../../helpers";

/**
 * Browser execution journey — csv-to-json
 *
 * Tests CSV-to-JSON conversion running 100% client-side via Rust→WASM.
 * Default config: comma delimiter, compact (non-pretty) output.
 */

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

test.describe("csv-to-json — browser execution @browser", () => {
  test("simple CSV: convert to JSON array, download, verify valid JSON", async ({ page }) => {
    await navigateToRecipe(page, "csv-to-json", "CSV to JSON Online Free");

    await uploadFiles(page, [path.join(CSV_FIXTURES_DIR, "simple.csv")]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    // Download and verify output is valid JSON
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByTestId("download-button").click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/\.json$/i);

    const downloadPath = await download.path();
    const downloadedFile = fs.readFileSync(downloadPath!, "utf-8");

    // Must be valid JSON
    const parsed = JSON.parse(downloadedFile);
    expect(Array.isArray(parsed)).toBe(true);

    // Should have 5 data rows (simple.csv has 5 rows + header)
    expect(parsed).toHaveLength(5);

    // Each row should be an object with CSV header keys
    const first = parsed[0];
    expect(first).toHaveProperty("name");
    expect(first).toHaveProperty("age");
    expect(first).toHaveProperty("city");

    // Values are strings (by design — no type coercion)
    expect(first.name).toBe("Alice");
    expect(typeof first.age).toBe("string");
  });

  test("CSV with empty cells: missing values become empty strings", async ({ page }) => {
    await navigateToRecipe(page, "csv-to-json", "CSV to JSON Online Free");

    await uploadFiles(page, [path.join(CSV_FIXTURES_DIR, "empty-cells.csv")]);

    await runAndComplete(page);

    const outputFile = page.getByTestId("output-file");
    await expect(outputFile).toHaveCount(1);

    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByTestId("download-button").click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    const downloadedFile = fs.readFileSync(downloadPath!, "utf-8");

    const parsed = JSON.parse(downloadedFile);
    expect(Array.isArray(parsed)).toBe(true);
    expect(parsed.length).toBeGreaterThan(0);

    // Empty cells should be empty strings, not null/undefined
    const bob = parsed.find((r: Record<string, string>) => r.name === "Bob");
    expect(bob).toBeDefined();
    expect(bob.age).toBe("");
  });
});
