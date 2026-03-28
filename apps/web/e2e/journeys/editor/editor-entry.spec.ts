import { test, expect } from "../../fixtures";
import { navigateToEditor } from "../../helpers/editor";

/**
 * Editor entry & navigation — EN1-EN3 + beta banner
 *
 * Verifies the editor loads correctly for blank canvas, predefined recipes,
 * and invalid slugs. Also tests the dismissible beta banner.
 * All tests are @browser — no Convex backend needed.
 *
 * Convention: SETUP → VERIFY (no execution step for entry tests).
 */

test.describe("editor entry & navigation @browser", () => {
  test("EN1: blank canvas loads with I/O nodes", async ({ page }) => {
    await navigateToEditor(page);

    // Blank editor should show input and output I/O nodes
    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards).toHaveCount(2);

    // Verify I/O node labels are present
    await expect(nodeCards.filter({ hasText: "Input" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Output" })).toHaveCount(1);

    // Placeholder or divider should be present for adding nodes
    const addButton = page.getByTestId("placeholder-node");
    await expect(addButton.first()).toBeVisible();
  });

  test("EN2: predefined recipe loads correct nodes", async ({ page }) => {
    await navigateToEditor(page, "compress-images");

    // compress-images has: Input, Compress, Output (auto-iteration group)
    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards.filter({ hasText: "Input" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Output" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: /Compress/i })).toHaveCount(1);
  });

  test("EN2b: clean-csv recipe loads correct nodes", async ({ page }) => {
    await navigateToEditor(page, "clean-csv");

    // clean-csv has: Input, Clean, Output (no loop)
    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards.filter({ hasText: "Input" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Output" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Clean" })).toHaveCount(1);
  });

  test("EN3: invalid slug shows 404 on tool page", async ({ page }) => {
    // Invalid slugs never reach the editor — the tool page returns 404.
    // The editor only accepts ?recipe={id} (from personal store or cloud).
    const response = await page.goto("/nonexistent-recipe");
    expect(response?.status()).toBe(404);
  });

  test("BN1: beta dialog renders and can be dismissed", async ({ page }) => {
    await page.goto("/editor");

    // Dialog should be visible on first visit
    const dialog = page.getByTestId("editor-beta-dialog");
    await expect(dialog).toBeVisible();
    await expect(dialog).toContainText("recipe editor is in beta");

    // Dismiss via "Get started" button
    await dialog.getByTestId("beta-get-started").click();
    await expect(dialog).not.toBeVisible();

    // Reload — dialog should stay dismissed (localStorage)
    await page.reload();
    await expect(dialog).not.toBeVisible();
  });

  test("BN2: beta dialog does not render when previously dismissed", async ({ page }) => {
    // Pre-set the dismissal flag before navigation
    await page.addInitScript(() => {
      localStorage.setItem("bnto-editor-beta-dismissed", "true");
    });
    await page.goto("/editor");

    const dialog = page.getByTestId("editor-beta-dialog");
    await expect(dialog).not.toBeVisible();
  });
});
