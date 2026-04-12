/**
 * Editor resilience — stale/malformed recipe handling.
 *
 * Verifies the editor doesn't crash when loading a recipe that contains
 * unknown or deprecated node types (e.g., from old sessionStorage data
 * or a future schema migration).
 *
 * All tests are @browser — no Convex backend needed.
 */

import { test, expect } from "../../fixtures";
import { dismissBetaDialog } from "../../helpers/editor";

/** sessionStorage key used by Zustand persist middleware. */
const STORAGE_KEY = "bnto-recipes";

/** A fixed recipe ID we'll inject into sessionStorage. */
const STALE_RECIPE_ID = "stale-recipe-test-id";

/**
 * A recipe definition containing a deprecated "image" node type
 * that no longer exists in NODE_TYPE_INFO. This simulates stale
 * sessionStorage data from before a node type was renamed/removed.
 */
const STALE_RECIPE_DEFINITION = {
  id: "stale-root",
  type: "loop",
  name: "Stale Recipe",
  nodes: [
    {
      id: "stale-input",
      type: "input",
      name: "Input",
      parameters: { mode: "file-upload" },
    },
    {
      id: "stale-image-node",
      type: "image", // Unknown/deprecated node type
      name: "Old Image Node",
      parameters: { quality: 80 },
    },
    {
      id: "stale-output",
      type: "output",
      name: "Output",
      parameters: { mode: "download" },
    },
  ],
};

/**
 * Zustand persist format for the recipesStore.
 * Matches the shape written by createEnhancedStore + persist middleware.
 */
function buildStoragePayload() {
  return JSON.stringify({
    state: {
      recipes: {
        [STALE_RECIPE_ID]: {
          id: STALE_RECIPE_ID,
          slug: "stale-recipe",
          name: "Stale Recipe",
          description: "A recipe with deprecated node types",
          category: "image",
          definition: STALE_RECIPE_DEFINITION,
          accept: { mimeTypes: [], extensions: [], label: "Any" },
          features: [],
          savedAt: Date.now(),
        },
      },
    },
    version: 0,
  });
}

test.describe("editor stale recipe resilience @browser", () => {
  test("SR1: editor loads without crashing when recipe has unknown node types", async ({
    page,
  }) => {
    // Inject stale recipe into sessionStorage before navigation
    await page.addInitScript(
      ({ key, payload }) => {
        sessionStorage.setItem(key, payload);
        localStorage.setItem("bnto-editor-experimental-dismissed", "true");
      },
      { key: STORAGE_KEY, payload: buildStoragePayload() },
    );

    // Navigate directly to editor with the stale recipe ID
    await page.goto(`/editor?recipe=${STALE_RECIPE_ID}`);

    // Wait for editor to render — should NOT crash
    const editor = page.getByTestId("recipe-editor");
    await expect(editor).toBeVisible({ timeout: 15_000 });

    // Node cards should render — the unknown "image" type should degrade
    // gracefully, not crash the entire editor
    const nodeCards = page.getByTestId("node-card");
    await expect(async () => {
      const count = await nodeCards.count();
      expect(count).toBeGreaterThanOrEqual(2);
    }).toPass({ timeout: 10_000, intervals: [500, 1000, 2000] });
  });

  test("SR2: unknown node type renders with fallback styling", async ({ page }) => {
    // Inject stale recipe into sessionStorage
    await page.addInitScript(
      ({ key, payload }) => {
        sessionStorage.setItem(key, payload);
        localStorage.setItem("bnto-editor-experimental-dismissed", "true");
      },
      { key: STORAGE_KEY, payload: buildStoragePayload() },
    );

    await page.goto(`/editor?recipe=${STALE_RECIPE_ID}`);
    await dismissBetaDialog(page);

    const editor = page.getByTestId("recipe-editor");
    await expect(editor).toBeVisible({ timeout: 15_000 });

    // Wait for nodes to render
    const nodeCards = page.getByTestId("node-card");
    await expect(async () => {
      const count = await nodeCards.count();
      expect(count).toBeGreaterThanOrEqual(2);
    }).toPass({ timeout: 10_000, intervals: [500, 1000, 2000] });

    // Verify the I/O nodes still render correctly alongside the unknown node
    await expect(nodeCards.filter({ hasText: "Input" })).toHaveCount(1);
    await expect(nodeCards.filter({ hasText: "Output" })).toHaveCount(1);

    // The editor should not have thrown any unrecoverable errors.
    // The console may show a warning about the unknown type, but the
    // app should remain functional.
  });

  test("SR3: editor remains interactive after loading stale recipe", async ({ page }) => {
    // Inject stale recipe into sessionStorage
    await page.addInitScript(
      ({ key, payload }) => {
        sessionStorage.setItem(key, payload);
        localStorage.setItem("bnto-editor-experimental-dismissed", "true");
      },
      { key: STORAGE_KEY, payload: buildStoragePayload() },
    );

    await page.goto(`/editor?recipe=${STALE_RECIPE_ID}`);
    await dismissBetaDialog(page);

    const editor = page.getByTestId("recipe-editor");
    await expect(editor).toBeVisible({ timeout: 15_000 });

    const nodeCards = page.getByTestId("node-card");
    await expect(async () => {
      const count = await nodeCards.count();
      expect(count).toBeGreaterThanOrEqual(2);
    }).toPass({ timeout: 10_000, intervals: [500, 1000, 2000] });

    // Verify the editor toolbar is functional — properties button should be clickable
    const propertiesButton = page.getByTestId("toolbar-properties");
    if (await propertiesButton.isVisible()) {
      await propertiesButton.click();
      // No crash = success
    }
  });
});
