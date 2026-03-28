/**
 * Editor Save journey tests — SV1, SV3 from editor.md journey matrix.
 *
 * SV1: Save recipe via rename dialog, verify it appears in My Recipes.
 * SV3: Load saved recipe from My Recipes back into the editor.
 *
 * Both require authentication (Convex backend + signed-in user). Tagged @auth.
 */

import { test, expect } from "../../fixtures";
import { navigateToEditor, addNodeFromPalette } from "../../helpers/editor";
import { saveRecipe, navigateToMyRecipes } from "../../helpers/editor-save";
import { testEmail, TEST_PASSWORD, TEST_NAME } from "../../accounts";

test.use({ reducedMotion: "reduce" });

/** Autosave debounce is 1s. Wait 2s to be safe. */
const AUTOSAVE_WAIT = 2_000;

// ---------------------------------------------------------------------------
// Helpers (local to this spec — mirrors auth-lifecycle.spec.ts)
// ---------------------------------------------------------------------------

async function signUp(page: import("@playwright/test").Page, email: string) {
  await page.goto("/signin");
  const authHeading = page.getByTestId("auth-heading");
  await expect(authHeading).toBeVisible();
  const headingText = await authHeading.textContent();
  if (headingText?.includes("Welcome back")) {
    await page.getByTestId("auth-mode-toggle").click();
  }
  await page.getByTestId("auth-name-input").fill(TEST_NAME);
  await page.getByTestId("auth-email-input").fill(email);
  await page.getByTestId("auth-password-input").fill(TEST_PASSWORD);
  await page.getByTestId("auth-submit").click();
  await page.waitForURL("/", { timeout: 15_000 });
}

// ---------------------------------------------------------------------------
// SV1-SV3: Save & Load (require auth) @auth
// ---------------------------------------------------------------------------

test.describe("Save recipe to account @auth", () => {
  test("SV1: save recipe, verify it appears in My Recipes", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    // SETUP: Navigate to editor, build a recipe
    await navigateToEditor(page);
    await addNodeFromPalette(page, "Compress Images");

    // SAVE: Open File > Rename, name it, confirm
    await page.getByTestId("toolbar-file-menu").click();
    await page.getByTestId("menu-rename").click();
    await saveRecipe(page, "Test Recipe SV1");

    // Wait for autosave to persist the renamed recipe
    await page.waitForTimeout(AUTOSAVE_WAIT);

    // VERIFY: Navigate to My Recipes, recipe should appear
    await navigateToMyRecipes(page);
    const recipeCards = page.getByTestId("recipe-card");
    await expect(recipeCards.filter({ hasText: "Test Recipe SV1" })).toBeVisible({
      timeout: 10_000,
    });
  });

  test("SV3: load saved recipe from My Recipes into editor", async ({ page }) => {
    const email = testEmail();
    await signUp(page, email);

    // SETUP: Save a recipe first
    await navigateToEditor(page);
    await addNodeFromPalette(page, "Compress Images");

    await page.getByTestId("toolbar-file-menu").click();
    await page.getByTestId("menu-rename").click();
    await saveRecipe(page, "SV3 Recipe");

    // Wait for autosave to persist the renamed recipe
    await page.waitForTimeout(AUTOSAVE_WAIT);

    // NAVIGATE: Go to My Recipes
    await navigateToMyRecipes(page);
    const recipeCards = page.getByTestId("recipe-card");
    await expect(recipeCards.filter({ hasText: "SV3 Recipe" })).toBeVisible({ timeout: 10_000 });

    // LOAD: Click the recipe card (it has an href to /editor?recipe=<id>)
    await recipeCards.filter({ hasText: "SV3 Recipe" }).click();

    // VERIFY: Editor loads with the saved definition
    await expect(page.getByTestId("recipe-editor")).toBeVisible({
      timeout: 15_000,
    });

    // Should have at least Input + Output + Compress node = 3 node cards
    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards).toHaveCount(3, { timeout: 10_000 });
  });
});
