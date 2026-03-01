import path from "path";
import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * Browser execution journey — rename-files
 *
 * Tests file renaming running 100% client-side via Rust→WASM.
 * Accepts any file type. The pattern config (default: "renamed-{{name}}")
 * determines the output filename. File contents are preserved as-is.
 */

const FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../../test-fixtures/images",
);

test.describe("rename-files — browser execution", () => {
  test("detects browser execution mode", async ({ page }) => {
    await page.goto("/rename-files");

    await expect(
      page.getByRole("heading", { name: "Rename Files Online Free" }),
    ).toBeVisible();

    const shell = page.locator('[data-testid="bnto-shell"]');
    await expect(shell).toHaveAttribute("data-execution-mode", "browser");
  });

  test("single file: rename with default pattern, download", async ({
    page,
  }) => {
    await page.goto("/rename-files");

    await expect(
      page.getByRole("heading", { name: "Rename Files Online Free" }),
    ).toBeVisible();

    // Default pattern is "renamed-{{name}}" — output should be "renamed-small.jpg"
    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "small.jpg"),
    ]);

    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await expect(runButton).toBeEnabled();

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("rename-01-file-selected.png");

    // Run
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("rename-02-result.png");

    // Verify download filename includes "renamed-"
    const downloadPromise = page.waitForEvent("download");
    await outputFile.getByRole("button", { name: /download/i }).click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toContain("renamed");
  });

  test("batch: rename multiple files with Download All", async ({
    page,
  }) => {
    await page.goto("/rename-files");

    await expect(
      page.getByRole("heading", { name: "Rename Files Online Free" }),
    ).toBeVisible();

    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(FIXTURES_DIR, "small.jpg"),
      path.join(FIXTURES_DIR, "small.png"),
      path.join(FIXTURES_DIR, "small.webp"),
    ]);

    await expect(page.getByText("3 files selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(3);
    await expect(
      page.getByRole("button", { name: /download all/i }).last(),
    ).toBeVisible();

    // All output files should be listed
    const outputFiles = page.locator('[data-testid="output-file"]');
    await expect(outputFiles).toHaveCount(3);

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot("rename-03-batch-result.png");
  });
});
