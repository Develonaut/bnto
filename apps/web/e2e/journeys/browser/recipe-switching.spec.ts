import path from "path";
import { test, expect } from "../../fixtures";
import {
  IMAGE_FIXTURES_DIR,
  CSV_FIXTURES_DIR,
  navigateToRecipe,
  assertBrowserExecution,
  uploadFiles,
  runAndComplete,
} from "../../helpers";

test.use({ reducedMotion: "reduce" });

/**
 * User journey — switching between recipes
 *
 * Validates that navigating between recipe pages doesn't leak execution state.
 * Guards against the singleton store bug where completed status from recipe A
 * would trigger auto-download when mounting recipe B.
 */

test.describe("recipe switching — state isolation @browser", () => {
  test("navigating after completion starts fresh (no stale phase)", async ({
    page,
  }) => {
    // --- Recipe A: compress-images → run to completion ---
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);

    await runAndComplete(page);

    // Confirm recipe A completed with results
    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(1);

    // --- Navigate to Recipe B: clean-csv ---
    await navigateToRecipe(page, "clean-csv", "Clean CSV Online Free");

    // Recipe B must start at Phase 1 (idle) — no stale completed state
    await assertBrowserExecution(page);

    // No output files from recipe A should be visible
    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(0);

    // Run button must not show completed phase if visible
    const runButtonB = page.locator('[data-testid="run-button"]');
    const runButtonCount = await runButtonB.count();
    if (runButtonCount > 0) {
      await expect(runButtonB.first()).not.toHaveAttribute(
        "data-phase",
        "completed",
      );
    }
  });

  test("no spurious download when navigating to a new recipe after completion", async ({
    page,
  }) => {
    // --- Recipe A: compress-images → run to completion ---
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);

    await runAndComplete(page);

    // --- Navigate to Recipe B and watch for downloads ---
    let downloadTriggered = false;
    page.on("download", () => {
      downloadTriggered = true;
    });

    await navigateToRecipe(page, "clean-csv", "Clean CSV Online Free");

    // Wait for network to settle — any useEffect auto-download would fire
    await page.waitForLoadState("networkidle");

    expect(downloadTriggered).toBe(false);
  });

  test("recipe B runs independently after recipe A completes", async ({
    page,
  }) => {
    // --- Recipe A: compress-images → run to completion ---
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);

    await runAndComplete(page);

    // --- Navigate to Recipe B: clean-csv ---
    await navigateToRecipe(page, "clean-csv", "Clean CSV Online Free");

    // --- Run Recipe B with its own files ---
    await uploadFiles(page, [
      path.join(CSV_FIXTURES_DIR, "messy.csv"),
    ]);

    const runButtonB = page.locator('[data-testid="run-button"]:visible');
    await expect(runButtonB).toHaveAttribute("data-phase", "idle");

    await runAndComplete(page);

    // Recipe B has its own results — not recipe A's image results
    await expect(page.locator('[data-testid="output-file"]')).toHaveCount(1);
  });

  test("rapid navigation between recipes doesn't leak state", async ({
    page,
  }) => {
    const recipes = [
      { slug: "compress-images", h1: "Compress Images Online Free" },
      { slug: "clean-csv", h1: "Clean CSV Online Free" },
      { slug: "rename-files", h1: "Rename Files Online Free" },
    ];

    // Run one recipe to completion first
    await navigateToRecipe(page, "compress-images", "Compress Images Online Free");

    await uploadFiles(page, [
      path.join(IMAGE_FIXTURES_DIR, "small.jpg"),
    ]);

    await runAndComplete(page);

    // Now navigate rapidly through all 3 recipes
    for (const recipe of recipes) {
      await navigateToRecipe(page, recipe.slug, recipe.h1);
      await assertBrowserExecution(page);
      await expect(page.locator('[data-testid="output-file"]')).toHaveCount(0);
    }
  });
});
