import { test, expect } from "../fixtures";
import { navigateToEditor, getNodeCount } from "./editor-helpers";

test.use({ reducedMotion: "reduce" });

/**
 * Editor entry point tests — EN1, EN2, EN3 from editor.md journey matrix.
 *
 * Verifies that the editor loads correctly from different entry points:
 * blank canvas, predefined recipe, and invalid slug fallback.
 *
 * @browser — no Convex backend needed.
 */

test.describe("editor entry @browser @editor", () => {
  test("EN1: /editor loads blank canvas with Input + Output nodes", async ({ page }) => {
    await navigateToEditor(page);

    // Should have exactly 2 nodes: Input and Output
    // (plus a placeholder node, but those use a different testid)
    const nodeCards = page.locator('[data-testid="node-card"]');
    await expect(nodeCards).toHaveCount(2);

    // Verify Input and Output labels are present
    await expect(nodeCards.filter({ hasText: "Input" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Output" })).toHaveCount(1);

    // Run button should exist and be in idle phase
    const runButton = page.locator('[data-testid="editor-run-button"]');
    await expect(runButton).toBeVisible();
    await expect(runButton).toHaveAttribute("data-phase", "idle");
  });

  test("EN2: /editor?from=compress-images loads predefined recipe", async ({ page }) => {
    await navigateToEditor(page, "compress-images");

    // compress-images has Input + processing node(s) + Output = at least 3 nodes
    const nodeCount = await getNodeCount(page);
    expect(nodeCount).toBeGreaterThanOrEqual(3);

    // Verify Input and Output are present
    const nodeCards = page.locator('[data-testid="node-card"]');
    await expect(nodeCards.filter({ hasText: "Input" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Output" })).toHaveCount(1);

    // Run button should be idle and enabled (has processing nodes)
    const runButton = page.locator('[data-testid="editor-run-button"]');
    await expect(runButton).toHaveAttribute("data-phase", "idle");
    await expect(runButton).toBeEnabled();
  });

  test("EN3: /editor?from=invalid-slug falls back to blank canvas", async ({ page }) => {
    await navigateToEditor(page, "this-recipe-does-not-exist");

    // Should fall back to blank canvas: Input + Output only
    const nodeCards = page.locator('[data-testid="node-card"]');
    await expect(nodeCards).toHaveCount(2);
    await expect(nodeCards.filter({ hasText: "Input" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Output" })).toHaveCount(1);
  });
});
