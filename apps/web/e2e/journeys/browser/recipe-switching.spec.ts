import path from "path";
import { test, expect } from "../../fixtures";

test.use({ reducedMotion: "reduce" });

/**
 * User journey — switching between recipes
 *
 * Validates that navigating between recipe pages doesn't leak execution state.
 * The core scenario: complete a recipe, navigate to a different one, and confirm
 * the new page starts fresh (idle phase, no stale results, no spurious downloads).
 *
 * This guards against the singleton store bug where completed status from recipe A
 * would trigger auto-download when mounting recipe B.
 */

const IMAGE_FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../../test-fixtures/images",
);
const CSV_FIXTURES_DIR = path.resolve(
  __dirname,
  "../../../../../test-fixtures/csv",
);

test.describe("recipe switching — state isolation", () => {
  test("navigating after completion starts fresh (no stale phase)", async ({
    page,
  }) => {
    // --- Recipe A: compress-images → run to completion ---
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);
    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await runButton.click();

    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    // Confirm recipe A completed with results
    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    // --- Navigate to Recipe B: clean-csv ---
    await page.goto("/clean-csv");
    await expect(
      page.getByRole("heading", { name: "Clean CSV Online Free" }),
    ).toBeVisible();

    // Recipe B must start at Phase 1 (idle) — no stale completed state
    const shell = page.locator('[data-testid="bnto-shell"]');
    await expect(shell).toHaveAttribute("data-execution-mode", "browser");

    // No output files from recipe A should be visible
    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(0);

    // No run button should show completed phase
    const runButtonB = page.locator('[data-testid="run-button"]');
    // Run button may not be visible yet (Phase 1 = dropzone, no run button)
    // but if visible, it must be idle
    const runButtonCount = await runButtonB.count();
    if (runButtonCount > 0) {
      await expect(runButtonB.first()).not.toHaveAttribute(
        "data-phase",
        "completed",
      );
    }

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot(
      "00-clean-csv-fresh-after-compress.png",
      { fullPage: true },
    );
  });

  test("no spurious download when navigating to a new recipe after completion", async ({
    page,
  }) => {
    // --- Recipe A: compress-images → run to completion ---
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await runButton.click();
    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    // --- Navigate to Recipe B and watch for downloads ---
    // Set up download listener BEFORE navigation so we catch anything
    // triggered during page mount
    let downloadTriggered = false;
    page.on("download", () => {
      downloadTriggered = true;
    });

    await page.goto("/clean-csv");
    await expect(
      page.getByRole("heading", { name: "Clean CSV Online Free" }),
    ).toBeVisible();

    // Wait a beat for any useEffect auto-download to fire (it would be
    // near-instant if the completed state leaked)
    await page.waitForTimeout(1000);

    expect(downloadTriggered).toBe(false);
  });

  test("recipe B runs independently after recipe A completes", async ({
    page,
  }) => {
    // --- Recipe A: compress-images → run to completion ---
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    const fileInputA = page.locator('input[type="file"]');
    await fileInputA.setInputFiles([
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);

    const runButtonA = page.locator('[data-testid="run-button"]:visible');
    await runButtonA.click();
    await expect(runButtonA).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    // --- Navigate to Recipe B: clean-csv ---
    await page.goto("/clean-csv");
    await expect(
      page.getByRole("heading", { name: "Clean CSV Online Free" }),
    ).toBeVisible();

    // --- Run Recipe B with its own files ---
    const fileInputB = page.locator('input[type="file"]');
    await fileInputB.setInputFiles([
      path.join(CSV_FIXTURES_DIR, "messy.csv"),
    ]);
    await expect(page.getByText("1 file selected")).toBeVisible();

    const runButtonB = page.locator('[data-testid="run-button"]:visible');
    await expect(runButtonB).toHaveAttribute("data-phase", "idle");
    await runButtonB.click();

    await expect(runButtonB).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    // Recipe B has its own results — not recipe A's image results
    const outputFile = page.locator('[data-testid="output-file"]');
    await expect(outputFile).toHaveCount(1);

    await page.evaluate(() => window.scrollTo(0, 0));
    await expect(page).toHaveScreenshot(
      "00-clean-csv-independent-result.png",
      { fullPage: true },
    );
  });

  test("rapid navigation between recipes doesn't leak state", async ({
    page,
  }) => {
    // Navigate through 3 recipes quickly — none should show stale state
    const recipes = [
      { slug: "compress-images", h1: "Compress Images Online Free" },
      { slug: "clean-csv", h1: "Clean CSV Online Free" },
      { slug: "rename-files", h1: "Rename Files Online Free" },
    ];

    // Run one recipe to completion first
    await page.goto("/compress-images");
    await expect(
      page.getByRole("heading", { name: "Compress Images Online Free" }),
    ).toBeVisible();

    const fileInput = page.locator('input[type="file"]');
    await fileInput.setInputFiles([
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);

    const runButton = page.locator('[data-testid="run-button"]:visible');
    await runButton.click();
    await expect(runButton).toHaveAttribute("data-phase", "completed", {
      timeout: 30000,
    });

    // Now navigate rapidly through all 3 recipes
    for (const recipe of recipes) {
      await page.goto(`/${recipe.slug}`);
      await expect(
        page.getByRole("heading", { name: recipe.h1 }),
      ).toBeVisible();

      // Each recipe must be in Phase 1 (idle dropzone)
      const shell = page.locator('[data-testid="bnto-shell"]');
      await expect(shell).toHaveAttribute("data-execution-mode", "browser");

      // No stale output files
      await expect(page.locator('[data-testid="output-file"]')).toHaveCount(0);
    }
  });
});
