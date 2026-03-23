/**
 * Editor Save journey tests — SV1-SV3 from editor.md journey matrix.
 *
 * SV1-SV3: Require authentication (Convex backend + signed-in user).
 *          Tagged @auth — skipped until auth infrastructure is available.
 *
 * SV4 (beforeunload warning) was removed: the auto-save system persists
 * changes within 1 second, so "unsaved changes" no longer applies.
 * Reload survival is covered by recipe-persistence.spec.ts.
 */

import { test, expect } from "../../fixtures";
import { navigateToEditor, addNodeFromPalette } from "../../helpers/editor";

// ---------------------------------------------------------------------------
// SV1-SV3: Save & Load (require auth) @auth
// ---------------------------------------------------------------------------

test.describe("Save recipe to account @auth", () => {
  // These tests require a signed-in user with Convex running.
  // They will be enabled once auth test infrastructure is in place.

  test.skip(true, "Requires auth infrastructure — enable when sign-in helpers are available");

  test("SV1: save recipe, verify it appears in My Recipes", async ({ page }) => {
    // SETUP: Navigate to editor, build a recipe
    await navigateToEditor(page);
    await addNodeFromPalette(page, "Compress Images");

    // SAVE: Open File > Save, name it, confirm
    await page.getByTestId("panel-file-menu").click();
    // TODO: Save menu item removed from toolbar — add testid when Save is re-introduced
    // await page.getByTestId("toolbar-save-item").click();

    const nameInput = page.getByTestId("save-recipe-name");
    await nameInput.fill("Test Recipe SV1");
    await page.getByTestId("save-recipe-confirm").click();

    // Wait for dialog to close
    await expect(nameInput).not.toBeVisible({ timeout: 10_000 });

    // VERIFY: Navigate to My Recipes, recipe should appear
    await page.goto("/my-recipes");
    await expect(page.getByTestId("recipe-card-Test Recipe SV1")).toBeVisible({
      timeout: 10_000,
    });
  });

  test("SV3: load saved recipe from My Recipes into editor", async ({ page }) => {
    // SETUP: Save a recipe first (prerequisite)
    await navigateToEditor(page);
    await addNodeFromPalette(page, "Compress Images");

    await page.getByTestId("panel-file-menu").click();
    // TODO: Save menu item removed from toolbar — add testid when Save is re-introduced
    // await page.getByTestId("toolbar-save-item").click();
    await page.getByTestId("save-recipe-name").fill("SV3 Recipe");
    await page.getByTestId("save-recipe-confirm").click();

    // NAVIGATE: Go to My Recipes
    await page.goto("/my-recipes");
    await expect(page.getByTestId("recipe-card-SV3 Recipe")).toBeVisible({ timeout: 10_000 });

    // LOAD: Click the recipe card (it has an href to /editor?recipe=<id>)
    await page.getByTestId("recipe-card-SV3 Recipe").click();

    // VERIFY: Editor loads with the saved definition
    await expect(page.getByTestId("recipe-editor")).toBeVisible({
      timeout: 15_000,
    });

    // Should have at least Input + Output + Compress node = 3 node cards
    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards).toHaveCount(3, { timeout: 10_000 });
  });
});
