/**
 * Recipe navigation data derived from @bnto/registry.
 *
 * Short nav descriptions override the longer SEO copy.
 * Both Navbar (desktop) and MobileNavMenu (mobile) consume these.
 */

import { getBrowserRecipes } from "@bnto/registry";

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
  vector: "Vector",
};

/** Ordered list of categories — controls the section order in the nav. */
const CATEGORY_ORDER = ["image", "spreadsheet", "file", "vector"];

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
  "svg-to-png": "Convert SVG vector files to PNG images",
  "svg-to-jpeg": "Convert SVG vector files to JPEG images",
};

/** Slugs shown in the nav dropdown. The rest are discoverable via /explore. */
const FEATURED_SLUGS = new Set([
  "compress-images",
  "resize-images",
  "convert-image-format",
  "optimize-images-for-web",
  "watermark-images",
  "clean-csv",
  "csv-to-json",
  "merge-csv",
  "rename-files",
  "svg-to-png",
  "svg-to-jpeg",
]);

/** All recipes grouped by category, derived from the engine registry. */
export const RECIPES: RecipeCategory[] = buildRecipeCategories();

/** Featured subset for the nav dropdown — curated for breadth, not volume. */
export const FEATURED_RECIPES: RecipeCategory[] = buildRecipeCategories(FEATURED_SLUGS);

function buildRecipeCategories(slugFilter?: Set<string>): RecipeCategory[] {
  const grouped = new Map<string, RecipeLink[]>();

  for (const recipe of getBrowserRecipes()) {
    if (slugFilter && !slugFilter.has(recipe.slug)) continue;
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
