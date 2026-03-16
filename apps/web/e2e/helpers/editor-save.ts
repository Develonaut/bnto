/**
 * Editor save helpers — shared utilities for save/load journey tests.
 *
 * These helpers require authentication (Convex backend running).
 */

import type { Page } from "@playwright/test";
import { expect } from "../fixtures";

/**
 * Fill in the save-recipe dialog and confirm.
 *
 * Note: Save is now automatic (debounced auto-save). This helper is only
 * used by auth-gated tests (SV1-SV3) that trigger the explicit save dialog.
 */
export async function saveRecipe(page: Page, name: string) {
  // Fill in recipe name
  const nameInput = page.getByTestId("save-recipe-name");
  await nameInput.waitFor({ timeout: 3_000 });
  await nameInput.fill(name);

  // Click Save button in dialog
  await page.getByTestId("save-recipe-confirm").click();

  // Wait for dialog to close (indicates success)
  await expect(nameInput).not.toBeVisible({ timeout: 10_000 });
}

/**
 * Navigate to My Recipes page and wait for the grid to load.
 */
export async function navigateToMyRecipes(page: Page) {
  await page.goto("/my-recipes");
  // Wait for the page heading
  await expect(page.getByTestId("my-recipes-heading")).toBeVisible({
    timeout: 10_000,
  });
}
