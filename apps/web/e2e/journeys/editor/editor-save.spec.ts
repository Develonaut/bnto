/**
 * Editor Save journey tests — SV1-SV4 from editor.md journey matrix.
 *
 * SV1-SV3: Require authentication (Convex backend + signed-in user).
 *          Tagged @auth — skipped unless auth infrastructure is available.
 * SV4:     Client-side only (beforeunload) — tagged @editor.
 */

import { test, expect } from "../../fixtures";
import { navigateToEditor, addNodeFromPalette } from "../../helpers/editor";

// ---------------------------------------------------------------------------
// SV4: Unsaved changes warning on navigation @editor
// ---------------------------------------------------------------------------

test.describe("Unsaved changes warning @editor", () => {
  test("SV4: beforeunload fires when editor has unsaved changes", async ({ page }) => {
    // SETUP: Navigate to blank editor
    await navigateToEditor(page);

    // BUILD: Add a node to make the editor dirty
    await addNodeFromPalette(page, "Compress Images");

    // VERIFY: The beforeunload handler is registered.
    // Playwright detects beforeunload dialogs via page.on("dialog").
    // We trigger a reload and verify the dialog fires.
    let dialogFired = false;
    page.on("dialog", async (dialog) => {
      dialogFired = true;
      await dialog.dismiss();
    });

    try {
      await page.reload({ timeout: 5_000 });
    } catch {
      // Reload may fail if dialog is dismissed — that's expected
    }

    // The beforeunload dialog should have fired
    expect(dialogFired).toBe(true);
  });

  test("SV4: no warning when editor is clean (no changes)", async ({ page }) => {
    // SETUP: Navigate to predefined recipe (clean state)
    await navigateToEditor(page, "compress-images");

    // VERIFY: Reload without any changes — no dialog should fire
    let dialogFired = false;
    page.on("dialog", async (dialog) => {
      dialogFired = true;
      await dialog.dismiss();
    });

    await page.reload();
    // Wait a tick to ensure no dialog fires
    await page.waitForTimeout(500);

    expect(dialogFired).toBe(false);
  });
});

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
    await page
      .locator('[data-testid="editor-toolbar"]')
      .getByRole("button", { name: /file menu/i })
      .click();
    // MenuItem renders as <button> — scope to menu dialog
    const menuContent = page.getByRole("dialog").filter({ hasText: "Save" });
    await menuContent.getByRole("button", { name: /^Save$/i }).click();

    const nameInput = page.locator('[data-testid="save-recipe-name"]');
    await nameInput.fill("Test Recipe SV1");
    await page.locator('[data-testid="save-recipe-confirm"]').click();

    // Wait for dialog to close
    await expect(nameInput).not.toBeVisible({ timeout: 10_000 });

    // VERIFY: Navigate to My Recipes, recipe should appear
    await page.goto("/my-recipes");
    await expect(page.getByText("Test Recipe SV1")).toBeVisible({
      timeout: 10_000,
    });
  });

  test("SV3: load saved recipe from My Recipes into editor", async ({ page }) => {
    // SETUP: Save a recipe first (prerequisite)
    await navigateToEditor(page);
    await addNodeFromPalette(page, "Compress Images");

    await page
      .locator('[data-testid="editor-toolbar"]')
      .getByRole("button", { name: /file menu/i })
      .click();
    // MenuItem renders as <button> — scope to menu dialog
    const menuContent = page.getByRole("dialog").filter({ hasText: "Save" });
    await menuContent.getByRole("button", { name: /^Save$/i }).click();
    await page.locator('[data-testid="save-recipe-name"]').fill("SV3 Recipe");
    await page.locator('[data-testid="save-recipe-confirm"]').click();

    // NAVIGATE: Go to My Recipes
    await page.goto("/my-recipes");
    await expect(page.getByText("SV3 Recipe")).toBeVisible({ timeout: 10_000 });

    // LOAD: Click the recipe card (it has an href to /editor?recipe=<id>)
    await page.getByText("SV3 Recipe").click();

    // VERIFY: Editor loads with the saved definition
    await expect(page.locator('[data-testid="recipe-editor"]')).toBeVisible({
      timeout: 15_000,
    });

    // Should have at least Input + Output + Compress node = 3 node cards
    const nodeCards = page.locator('[data-testid="node-card"]');
    await expect(nodeCards).toHaveCount(3, { timeout: 10_000 });
  });
});
