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
 * Browser execution journey — clean-csv
 *
 * Tests CSV cleaning running 100% client-side via Rust→WASM.
 * Default config: trim whitespace + remove empty rows (duplicates off).
 */

test.describe("clean-csv — browser execution @browser", () => {
  test("detects browser execution mode", async ({ page }) => {
    await navigateToRecipe(page, "clean-csv", "Clean CSV Online Free");
    await assertBrowserExecution(page);
  });

  test("messy CSV: clean, download, verify output is valid CSV", async ({
    page,
  }) => {
    await navigateToRecipe(page, "clean-csv", "Clean CSV Online Free");

    await uploadFiles(page, [
      path.join(CSV_FIXTURES_DIR, "messy.csv"),
    ]);

    await runAndComplete(page);

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    // Download and verify output is valid CSV text
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/\.csv$/i);

    const downloadPath = await download.path();
    const downloadedFile = fs.readFileSync(downloadPath!, "utf-8");

    // Output should contain CSV content — at minimum a header row
    expect(downloadedFile).toContain("name");
    expect(downloadedFile).toContain("age");
    expect(downloadedFile).toContain("city");

    // Should not contain empty lines (empty rows removed)
    const lines = downloadedFile.split("\n").filter((l) => l.trim() !== "");
    expect(lines.length).toBeGreaterThan(1);

    // Whitespace should be trimmed — no leading/trailing spaces in cells
    for (const line of lines) {
      const cells = line.split(",");
      for (const cell of cells) {
        expect(cell).toBe(cell.trim());
      }
    }
  });

  test("simple CSV: passes through cleanly", async ({ page }) => {
    await navigateToRecipe(page, "clean-csv", "Clean CSV Online Free");

    await uploadFiles(page, [
      path.join(CSV_FIXTURES_DIR, "simple.csv"),
    ]);

    await runAndComplete(page);

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    // Download and verify all original data rows are preserved
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    const output = fs.readFileSync(downloadPath!, "utf-8");

    expect(output).toContain("Alice");
    expect(output).toContain("Bob");
    expect(output).toContain("Charlie");
    expect(output).toContain("Diana");
    expect(output).toContain("Eve");
  });
});
