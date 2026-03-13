import { test, expect } from "../../fixtures";
import { enableEditorFlag, navigateToEditor } from "../../helpers/editor";

test.use({ reducedMotion: "reduce" });

/**
 * Editor entry & navigation — EN1-EN3
 *
 * Verifies the editor loads correctly for blank canvas, predefined recipes,
 * and invalid slugs. All tests are @browser — no Convex backend needed.
 *
 * Convention: SETUP → VERIFY (no execution step for entry tests).
 */

test.describe("editor entry & navigation @browser", () => {
  test("EN1: blank canvas loads with I/O nodes", async ({ page }) => {
    await enableEditorFlag(page);
    await navigateToEditor(page);

    // Blank editor should show input and output I/O nodes
    const nodeCards = page.locator('[data-testid="node-card"]');
    await expect(nodeCards).toHaveCount(2);

    // Verify I/O node labels are present
    await expect(nodeCards.filter({ hasText: "Input" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Output" })).toHaveCount(1);

    // Placeholder or divider should be present for adding nodes
    const addButton = page.getByRole("button", { name: "Add node" });
    await expect(addButton.first()).toBeVisible();
  });

  test("EN2: predefined recipe loads correct nodes", async ({ page }) => {
    await enableEditorFlag(page);
    await navigateToEditor(page, "compress-images");

    // compress-images has: Input, For Each (loop containing Compress), Output
    // Top-level nodes visible on canvas: Input, For Each, Output
    const nodeCards = page.locator('[data-testid="node-card"]');
    await expect(nodeCards.filter({ hasText: "Input" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Output" })).toHaveCount(1);

    // The loop container node should be visible
    await expect(nodeCards.filter({ hasText: /For Each/i })).toHaveCount(1);
  });

  test("EN2b: clean-csv recipe loads correct nodes", async ({ page }) => {
    await enableEditorFlag(page);
    await navigateToEditor(page, "clean-csv");

    // clean-csv has: Input, Clean, Output (no loop)
    const nodeCards = page.locator('[data-testid="node-card"]');
    await expect(nodeCards.filter({ hasText: "Input" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Output" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Clean" })).toHaveCount(1);
  });

  test("EN3: invalid slug falls back to blank canvas", async ({ page }) => {
    await enableEditorFlag(page);
    await navigateToEditor(page, "nonexistent-recipe");

    // Should fall back to blank editor with just I/O nodes
    const nodeCards = page.locator('[data-testid="node-card"]');
    await expect(nodeCards).toHaveCount(2);
    await expect(nodeCards.filter({ hasText: "Input" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Output" })).toHaveCount(1);
  });

  // EN4 covered by individual recipe load tests above + predefined recipe
  // execution tests in editor-predefined.spec.ts. Looping page.goto in a
  // single test is flaky due to Next.js hydration timing with
  // useSyncExternalStore feature flags. Each recipe is individually verified
  // by the execution tests instead.
});
