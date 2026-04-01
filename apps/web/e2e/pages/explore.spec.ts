import { test, expect } from "../fixtures";
import { getAllRecipes } from "@bnto/registry";

/**
 * Explore page — verifies recipe discovery, category filtering,
 * and navigation from explore cards to tool pages.
 */

const ALL_RECIPES = getAllRecipes();
const FIRST_RECIPE = ALL_RECIPES[0];

test.use({ expectedErrors: ["CONVEX_UNAUTH"] });

/* ── Desktop ─────────────────────────────────────────────────── */

test.describe("Explore page — desktop @browser", () => {
  test("renders heading and recipe grid", async ({ page }) => {
    await page.goto("/explore");

    await expect(page.getByTestId("explore-heading")).toBeVisible();
    await expect(page.getByTestId("explore-recipe-grid")).toBeVisible();

    // All recipes should appear
    for (const recipe of ALL_RECIPES) {
      await expect(page.getByTestId(`explore-recipe-${recipe.slug}`)).toBeVisible();
    }
  });

  test("category filters are visible", async ({ page }) => {
    await page.goto("/explore");

    // "All" category is always present
    await expect(page.getByTestId("explore-category-all")).toBeVisible();

    // At least image and data categories exist
    await expect(page.getByTestId("explore-category-image")).toBeVisible();
    await expect(page.getByTestId("explore-category-data")).toBeVisible();
  });

  test("clicking category filter updates URL and filters grid", async ({ page }) => {
    await page.goto("/explore");

    // Click Image category
    await page.getByTestId("explore-category-image").click();
    await expect(page).toHaveURL(/[?&]category=image/);

    // Non-image recipes should be hidden
    const csvRecipe = page.getByTestId("explore-recipe-clean-csv");
    await expect(csvRecipe).not.toBeVisible();

    // Image recipes should still be visible
    await expect(page.getByTestId("explore-recipe-compress-images")).toBeVisible();

    // Click All to reset
    await page.getByTestId("explore-category-all").click();
    await expect(csvRecipe).toBeVisible();
  });

  test("clicking recipe card navigates to tool page", async ({ page }) => {
    await page.goto("/explore");

    const recipeCard = page.getByTestId(`explore-recipe-${FIRST_RECIPE.slug}`);
    await expect(recipeCard).toBeVisible();

    // Click the card (it's inside a link)
    await recipeCard.click();
    await expect(page).toHaveURL(`/${FIRST_RECIPE.slug}`);
    await expect(page.getByTestId("recipe-heading")).toBeVisible();
  });
});

/* ── Mobile ──────────────────────────────────────────────────── */

test.describe("Explore page — mobile @browser", () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test("renders responsively on mobile", async ({ page }) => {
    await page.goto("/explore");

    await expect(page.getByTestId("explore-heading")).toBeVisible();
    await expect(page.getByTestId("explore-recipe-grid")).toBeVisible();
  });
});
