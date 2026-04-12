/**
 * Shared URL collection for search engine notifications.
 *
 * Reuses the RECIPES registry (same pattern as generate-lighthouserc.ts)
 * to build the full list of indexable URLs on bnto.io.
 */

import { RECIPES } from "../../packages/@bnto/registry/src/recipesCatalog";

const SITE_URL = "https://bnto.io";
const STATIC_PAGES = ["/", "/explore", "/pricing", "/faq", "/privacy"];

export function collectUrls(): string[] {
  const staticUrls = STATIC_PAGES.map((p) => `${SITE_URL}${p}`);
  const recipeUrls = RECIPES.map((r) => `${SITE_URL}/${r.slug}`);
  return [...staticUrls, ...recipeUrls];
}
