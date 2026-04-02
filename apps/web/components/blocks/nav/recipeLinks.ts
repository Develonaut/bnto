/**
 * Recipe navigation data derived from @bnto/registry.
 *
 * Short nav descriptions override the longer SEO copy.
 * Both Navbar (desktop) and MobileNavMenu (mobile) consume these.
 */

import { getAllRecipes } from "@bnto/registry";

export interface RecipeLink {
  label: string;
  description: string;
  url: string;
}

export interface RecipeCategory {
  title: string;
  links: RecipeLink[];
}

/** Display names for menu categories in the nav dropdown. */
const CATEGORY_TITLES: Record<string, string> = {
  image: "Image",
  spreadsheet: "Data",
  file: "File",
};

/** Ordered list of categories — controls the section order in the nav. */
const CATEGORY_ORDER = ["image", "spreadsheet", "file"];

/** Short nav descriptions that override the longer SEO-oriented menu copy. */
const NAV_DESCRIPTIONS: Record<string, string> = {
  "compress-images": "Shrink PNG, JPEG, and WebP without losing quality",
  "resize-images": "Scale images to exact dimensions or percentages",
  "convert-image-format": "Switch between PNG, JPEG, WebP, and GIF",
  "strip-exif": "Remove EXIF metadata from images",
  "watermark-images": "Add a logo or watermark to images",
  "clean-csv": "Remove empty rows, trim whitespace, deduplicate",
  "rename-csv-columns": "Rename column headers in bulk",
  "csv-to-json": "Convert CSV files to JSON format",
  "merge-csv": "Combine multiple CSVs into one",
  "rename-files": "Batch rename files with patterns",
};

/** Recipes grouped by category, derived from the engine menu. */
export const RECIPES: RecipeCategory[] = buildRecipeCategories();

function buildRecipeCategories(): RecipeCategory[] {
  const grouped = new Map<string, RecipeLink[]>();

  for (const recipe of getAllRecipes()) {
    const links = grouped.get(recipe.category) ?? [];
    links.push({
      label: recipe.name,
      description: NAV_DESCRIPTIONS[recipe.slug] ?? recipe.description,
      url: `/${recipe.slug}`,
    });
    grouped.set(recipe.category, links);
  }

  return CATEGORY_ORDER.filter((cat) => grouped.has(cat)).map((cat) => ({
    title: CATEGORY_TITLES[cat] ?? cat,
    links: grouped.get(cat)!,
  }));
}
