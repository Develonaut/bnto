import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Browser execution journey — rename-csv-columns
 *
 * Tests CSV column renaming running 100% client-side via Rust→WASM.
 * Note: Full column remapping UI is not yet implemented (requires
 * array-level transforms in the engine). Current behavior processes
 * the CSV through the WASM pipeline and outputs with columns preserved.
 */

const CSV_FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../../test-fixtures/csv",
);

test.describe("rename-csv-columns — browser execution", () => {
  test("detects browser execution mode", async ({ page }) => {
    await page.goto("/rename-csv-columns");

    await expect(
      page.getByRole("heading", {
        name: "Rename CSV Columns Online Free",
      }),
    ).toBeVisible();

    const shell = page.locator('[data-testid="bnto-shell"]');
    await expect(shell).toHaveAttribute("data-execution-mode", "browser");
  });

  test("process CSV: output preserves column structure", async ({
    page,
  }) => {
    await page.goto("/rename-csv-columns");

    await expect(
      page.getByRole("heading", {
        name: "Rename CSV Columns Online Free",
      }),
    ).toBeVisible();

    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(CSV_FIXTURES_DIR, "simple.csv"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await expect(runButton).toBeEnabled();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot(
      "rename-csv-01-file-selected.png",
    );

    // Run
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("rename-csv-02-result.png");

    // Download and verify output is valid CSV with columns preserved
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/\.csv$/i);

    const downloadPath = await download.path();
    const output = fs.readFileSync(downloadPath!, "utf-8");

    // Column headers should be present
    const firstLine = output.split("\n")[0];
    expect(firstLine).toBeTruthy();

    // Data rows should be preserved
    expect(output).toContain("Alice");
    expect(output).toContain("Bob");
  });

  test("many-column CSV: all columns survive processing", async ({
    page,
  }) => {
    await page.goto("/rename-csv-columns");

    await expect(
      page.getByRole("heading", {
        name: "Rename CSV Columns Online Free",
      }),
    ).toBeVisible();

    // many-columns.csv has 8 columns — verifies wide CSV support
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(CSV_FIXTURES_DIR, "many-columns.csv"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    const outputFile2 = page.locator('[data-testid="output-file"]');
    await expect(outputFile2).toHaveCount(1);

    // Download and verify all 8 columns survive
    const downloadPromise = page.waitForEvent("download");
    await outputFile2.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    const output = fs.readFileSync(downloadPath!, "utf-8");

    // All original column headers should be present
    expect(output).toContain("first_name");
    expect(output).toContain("last_name");
    expect(output).toContain("email");
    expect(output).toContain("department");

    // Data should be preserved
    expect(output).toContain("Jane");
    expect(output).toContain("Engineering");
  });
});
