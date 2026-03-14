/**
 * Editor save helpers — shared utilities for save/load journey tests.
 *
 * These helpers require authentication (Convex backend running).
 */

import type { Page } from "@playwright/test";
import { expect } from "../fixtures";

/**
 * Open the File menu and click Save to open the save dialog.
 * Then fill in the recipe name and confirm.
 */
export async function saveRecipe(page: Page, name: string) {
  // Open file menu
  await page
    .locator('[data-testid="editor-toolbar"]')
    .getByRole("button", { name: /file menu/i })
    .click();

  // Click Save menu item (MenuItem renders as <button>)
  const menuContent = page.getByRole("dialog").filter({ hasText: "Save" });
  await menuContent.getByRole("button", { name: /^Save$/i }).click();

  // Fill in recipe name
  const nameInput = page.locator('[data-testid="save-recipe-name"]');
  await nameInput.waitFor({ timeout: 3_000 });
  await nameInput.fill(name);

  // Click Save button in dialog
  await page.locator('[data-testid="save-recipe-confirm"]').click();

  // Wait for dialog to close (indicates success)
  await expect(nameInput).not.toBeVisible({ timeout: 10_000 });
}

/**
 * Navigate to My Recipes page and wait for the grid to load.
 */
export async function navigateToMyRecipes(page: Page) {
  await page.goto("/my-recipes");
  // Wait for the page heading
  await expect(page.getByRole("heading", { name: "My Recipes" })).toBeVisible({
    timeout: 10_000,
  });
}
