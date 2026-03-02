/**
 * Shared navigation data for desktop and mobile navbars.
 *
 * Recipe links are derived from the engine-generated menu — no duplicate
 * data to maintain. Short nav descriptions override the longer SEO copy.
 * Both Navbar (desktop) and MobileNavMenu (mobile) consume these.
 */

import { MENU } from "@/lib/menu";

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
  "clean-csv": "Remove empty rows, trim whitespace, deduplicate",
  "rename-csv-columns": "Rename column headers in bulk",
  "rename-files": "Batch rename files with patterns",
};

/** Recipes grouped by category, derived from the engine menu. */
export const RECIPES: RecipeCategory[] = buildRecipeCategories();

function buildRecipeCategories(): RecipeCategory[] {
  const grouped = new Map<string, RecipeLink[]>();

  for (const recipe of MENU) {
    const links = grouped.get(recipe.category) ?? [];
    links.push({
      label: recipe.name,
      description: NAV_DESCRIPTIONS[recipe.slug] ?? recipe.description,
      url: `/${recipe.slug}`,
    });
    grouped.set(recipe.category, links);
  }

  return CATEGORY_ORDER
    .filter((cat) => grouped.has(cat))
    .map((cat) => ({
      title: CATEGORY_TITLES[cat] ?? cat,
      links: grouped.get(cat)!,
    }));
}

export interface PageLink {
  label: string;
  href: string;
}

export const PAGE_LINKS: PageLink[] = [
  { label: "My Recipes", href: "/workflows" },
  { label: "Pricing", href: "/pricing" },
  { label: "FAQ", href: "/faq" },
];
