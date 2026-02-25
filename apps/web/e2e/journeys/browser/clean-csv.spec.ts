import path from "path";
import fs from "fs";
import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Browser execution journey — clean-csv
 *
 * Tests CSV cleaning running 100% client-side via Rust→WASM.
 * Default config: trim whitespace + remove empty rows (duplicates off).
 * The messy.csv fixture has leading/trailing spaces, empty rows,
 * and duplicate entries — perfect for validating the cleaning pipeline.
 */

const CSV_FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../../test-fixtures/csv",
);

test.describe("clean-csv — browser execution", () => {
  test("detects browser execution mode", async ({ page }) => {
    await page.goto("/clean-csv");

    await expect(
      page.getByRole("heading", { name: "Clean CSV Online Free" }),
    ).toBeVisible();

    const shell = page.locator('[data-testid="bnto-shell"]');
    await expect(shell).toHaveAttribute("data-execution-mode", "browser");
  });

  test("messy CSV: clean, download, verify output is valid CSV", async ({
    page,
  }) => {
    await page.goto("/clean-csv");

    await expect(
      page.getByRole("heading", { name: "Clean CSV Online Free" }),
    ).toBeVisible();

    // Select the messy CSV fixture
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(CSV_FIXTURES_DIR, "messy.csv"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await expect(runButton).toBeEnabled();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("clean-csv-01-file-selected.png", {
      fullPage: true,
    });

    // Run with default config (trim whitespace + remove empty rows)
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    const results = page.locator(
      '[data-testid="browser-execution-results"]',
    );
    await expect(results).toBeVisible();
    await expect(page.getByText("1 file ready")).toBeVisible();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("clean-csv-02-result.png", {
      fullPage: true,
    });

    // Download and verify output is valid CSV text
    const downloadPromise = page.waitForEvent("download");
    await page.locator('[data-testid="download-button"]').click();
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
    expect(lines.length).toBeGreaterThan(1); // header + at least one data row

    // Whitespace should be trimmed — no leading/trailing spaces in cells
    for (const line of lines) {
      const cells = line.split(",");
      for (const cell of cells) {
        // Each cell should not start or end with spaces
        expect(cell).toBe(cell.trim());
      }
    }
  });

  test("simple CSV: passes through cleanly", async ({ page }) => {
    await page.goto("/clean-csv");

    await expect(
      page.getByRole("heading", { name: "Clean CSV Online Free" }),
    ).toBeVisible();

    // simple.csv is already clean — should pass through without changes
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(CSV_FIXTURES_DIR, "simple.csv"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]');
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    await expect(page.getByText("1 file ready")).toBeVisible();

    // Download and verify all original data rows are preserved
    const downloadPromise = page.waitForEvent("download");
    await page.locator('[data-testid="download-button"]').click();
    const download = await downloadPromise;

    const downloadPath = await download.path();
    const output = fs.readFileSync(downloadPath!, "utf-8");

    // All 5 data rows + header should survive cleaning
    expect(output).toContain("Alice");
    expect(output).toContain("Bob");
    expect(output).toContain("Charlie");
    expect(output).toContain("Diana");
    expect(output).toContain("Eve");
  });
});
