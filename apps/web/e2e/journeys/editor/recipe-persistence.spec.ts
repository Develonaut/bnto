/**
 * Recipe persistence journey tests — local save/load/sync lifecycle.
 *
 * Validates the two-layer auto-save system:
 *   Layer 1: Zustand recipesStore → localStorage ("bnto-recipes")
 *   Layer 2: Cloud sync to Convex (auth-gated, tested as @auth)
 *
 * Tests 1-2, 5: Unauthed, browser-only (@browser). No Convex needed.
 * Tests 3-4:    Auth-gated (@auth). Skipped until auth infrastructure is ready.
 */

import { test, expect } from "../../fixtures";
import {
  navigateToEditor,
  addNodeFromPalette,
  selectNode,
  ensureConfigPanelOpen,
  setNumberParam,
  dismissBetaDialog,
} from "../../helpers/editor";
import { navigateToMyRecipes } from "../../helpers/editor-save";

/** Autosave debounce is 1s. Wait 2s to be safe. */
const AUTOSAVE_WAIT = 2_000;

/** localStorage key used by Zustand persist middleware. */
const STORAGE_KEY = "bnto-recipes";

// ---------------------------------------------------------------------------
// Helpers (local to this spec)
// ---------------------------------------------------------------------------

/**
 * Read stored recipes from localStorage.
 * Returns the recipes Record or null if nothing stored.
 * Zustand persist format: { state: { recipes: { ... } }, version: N }
 */
async function getStoredRecipes(page: import("@playwright/test").Page) {
  return page.evaluate((key) => {
    const raw = localStorage.getItem(key);
    if (!raw) return null;
    try {
      const parsed = JSON.parse(raw);
      return (parsed?.state?.recipes as Record<string, unknown>) ?? null;
    } catch {
      return null;
    }
  }, STORAGE_KEY);
}

/**
 * Wait for the editor to stabilize after a client-side navigation
 * (e.g., clicking a recipe card in My Recipes). Similar to navigateToEditor
 * but without the page.goto() call.
 */
async function waitForEditorReady(
  page: import("@playwright/test").Page,
  expectedNodeCount: number,
) {
  await dismissBetaDialog(page);

  const editor = page.getByTestId("recipe-editor");
  const nodeCards = page.getByTestId("node-card");

  await expect(async () => {
    await expect(editor).toBeVisible();
    const count = await nodeCards.count();
    expect(count).toBeGreaterThanOrEqual(expectedNodeCount);
  }).toPass({ timeout: 15_000, intervals: [500, 500, 1_000, 1_000, 2_000] });
}

// ---------------------------------------------------------------------------
// 1. Unauthed: local persistence round-trip @browser
// ---------------------------------------------------------------------------

test.describe("recipe persistence — local round-trip @browser", () => {
  test("create recipe, navigate to My Recipes, verify listed, reopen in editor", async ({
    page,
  }) => {
    // SETUP: Navigate to blank editor (fresh context = empty localStorage)
    await navigateToEditor(page);

    // BUILD: Add a compress node (Input + Output + Compress = 3 nodes)
    await addNodeFromPalette(page, "Compress Images");

    // WAIT: Let autosave debounce complete
    await page.waitForTimeout(AUTOSAVE_WAIT);

    // VERIFY: localStorage has the recipe
    const recipes = await getStoredRecipes(page);
    expect(recipes).not.toBeNull();
    const recipeIds = Object.keys(recipes!);
    expect(recipeIds.length).toBeGreaterThanOrEqual(1);

    // Verify recipe has expected shape
    const firstRecipe = recipes![recipeIds[0]] as Record<string, unknown>;
    expect(firstRecipe).toHaveProperty("id");
    expect(firstRecipe).toHaveProperty("definition");
    expect(firstRecipe).toHaveProperty("savedAt");
    expect(firstRecipe.syncedAt).toBeNull(); // unauthed = never synced

    // NAVIGATE: Go to My Recipes
    await navigateToMyRecipes(page);

    // VERIFY: Recipe appears in the list
    const recipeCard = page.getByTestId("recipe-card");
    await expect(recipeCard.first()).toBeVisible({ timeout: 5_000 });

    // REOPEN: Click the first recipe card to open in editor
    await recipeCard.first().click();

    // VERIFY: Editor loads with the saved definition
    // Input + Output + Compress = 3 node cards
    await waitForEditorReady(page, 3);
    const nodeCards = page.getByTestId("node-card");
    await expect(nodeCards).toHaveCount(3, { timeout: 10_000 });
  });
});

// ---------------------------------------------------------------------------
// 2. Unauthed: reload persistence @browser
// ---------------------------------------------------------------------------

test.describe("recipe persistence — reload survival @browser", () => {
  test("recipe survives hard page reload via localStorage", async ({ page }) => {
    // SETUP: Navigate to blank editor
    await navigateToEditor(page);

    // BUILD: Add a compress node
    await addNodeFromPalette(page, "Compress Images");

    // WAIT: Let autosave complete
    await page.waitForTimeout(AUTOSAVE_WAIT);

    // VERIFY: localStorage has recipe data before reload
    const recipesBefore = await getStoredRecipes(page);
    expect(recipesBefore).not.toBeNull();
    const countBefore = Object.keys(recipesBefore!).length;
    expect(countBefore).toBeGreaterThanOrEqual(1);

    // RELOAD: Accept the beforeunload dialog and reload
    page.on("dialog", async (dialog) => {
      await dialog.accept();
    });
    await page.reload();

    // VERIFY: localStorage survived the reload
    const recipesAfter = await getStoredRecipes(page);
    expect(recipesAfter).not.toBeNull();
    const countAfter = Object.keys(recipesAfter!).length;
    expect(countAfter).toBe(countBefore);

    // VERIFY: Can still access the recipe from My Recipes
    await navigateToMyRecipes(page);
    const recipeCard = page.getByTestId("recipe-card");
    await expect(recipeCard.first()).toBeVisible({ timeout: 5_000 });
  });
});

// ---------------------------------------------------------------------------
// 3. Authed: local + cloud sync @auth
// ---------------------------------------------------------------------------

test.describe("recipe persistence — cloud sync @auth", () => {
  test.skip(true, "Requires auth infrastructure — enable when sign-in helpers are available");

  test("save syncs to Convex, syncedAt becomes non-null", async ({ page }) => {
    // TODO: Sign in → open editor → add node → wait for save →
    // verify syncedAt is non-null in localStorage
    await navigateToEditor(page);
    await addNodeFromPalette(page, "Compress Images");
    await page.waitForTimeout(AUTOSAVE_WAIT);

    const recipes = await getStoredRecipes(page);
    expect(recipes).not.toBeNull();
    const first = Object.values(recipes!)[0] as Record<string, unknown>;
    // After cloud sync, syncedAt should be a number (not null)
    expect(first.syncedAt).not.toBeNull();
  });
});

// ---------------------------------------------------------------------------
// 4. Authed: cloud-only hydration @auth
// ---------------------------------------------------------------------------

test.describe("recipe persistence — cloud hydration @auth", () => {
  test.skip(true, "Requires auth infrastructure — enable when sign-in helpers are available");

  test("hydrateFromCloud populates empty local store", async ({ page }) => {
    // TODO: Clear localStorage → sign in → verify hydrateFromCloud
    // pulls recipes from Convex into the local store →
    // verify they appear in /my-recipes
    await page.evaluate((key) => localStorage.removeItem(key), STORAGE_KEY);
    await navigateToMyRecipes(page);
    const recipeCard = page.getByTestId("recipe-card");
    await expect(recipeCard.first()).toBeVisible({ timeout: 10_000 });
  });
});

// ---------------------------------------------------------------------------
// 5. Edit → close → reopen config fidelity @browser
// ---------------------------------------------------------------------------

test.describe("recipe persistence — config fidelity @browser", () => {
  test("specific parameter values survive close and reopen", async ({ page }) => {
    // SETUP: Navigate to blank editor
    await navigateToEditor(page);

    // BUILD: Add a resize node and configure width
    await addNodeFromPalette(page, "Resize Images");
    await selectNode(page, "Resize");
    await ensureConfigPanelOpen(page);
    await setNumberParam(page, "width", 200);

    // WAIT: Let autosave debounce complete
    await page.waitForTimeout(AUTOSAVE_WAIT);

    // VERIFY: localStorage has the recipe with the configured param
    const recipes = await getStoredRecipes(page);
    expect(recipes).not.toBeNull();

    // NAVIGATE: Close editor, go to My Recipes
    await navigateToMyRecipes(page);

    // VERIFY: Recipe appears
    const recipeCard = page.getByTestId("recipe-card");
    await expect(recipeCard.first()).toBeVisible({ timeout: 5_000 });

    // REOPEN: Click recipe to reopen in editor
    await recipeCard.first().click();

    // VERIFY: Editor loads with correct node count
    // Input + Output + Resize = 3 node cards
    await waitForEditorReady(page, 3);

    // VERIFY: Select the resize node, check width value is preserved
    await selectNode(page, "Resize");
    await ensureConfigPanelOpen(page);

    const widthInput = page.getByTestId("control-number-param-width");
    await widthInput.waitFor({ timeout: 5_000 });
    const value = await widthInput.inputValue();
    expect(value).toBe("200");
  });
});
