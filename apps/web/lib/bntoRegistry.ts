/**
 * Bnto slug registry — thin wrapper over @bnto/nodes recipes.
 *
 * Drives generateStaticParams, generateMetadata, sitemap, gallery,
 * llms.txt, and middleware slug validation from one place.
 *
 * All recipe data lives in @bnto/nodes (the single source of truth).
 * This module maps Recipe → BntoEntry to preserve the existing consumer API.
 */

import { RECIPES, getRecipeBySlug, type Recipe } from "@bnto/nodes";

export interface BntoEntry {
  slug: string;
  title: string;
  description: string;
  h1: string;
  fixture: string;
  features: string[];
}

function toBntoEntry(r: Recipe): BntoEntry {
  return {
    slug: r.slug,
    title: r.seo.title,
    description: r.description,
    h1: r.seo.h1,
    fixture: `${r.slug}.bnto.json`,
    features: r.features,
  };
}

/**
 * All registered Tier 1 bntos. Each entry maps to a public URL at /{slug}.
 */
export const BNTO_REGISTRY: readonly BntoEntry[] = RECIPES.map(toBntoEntry);

/** Returns true if the slug maps to a registered bnto. */
export function isValidBntoSlug(slug: string): boolean {
  return getRecipeBySlug(slug) !== undefined;
}

/** Returns the registry entry for a slug, or undefined if not found. */
export function getBntoBySlug(slug: string): BntoEntry | undefined {
  const recipe = getRecipeBySlug(slug);
  return recipe ? toBntoEntry(recipe) : undefined;
}

/**
 * Paths reserved for app routes — never use as bnto slugs.
 *
 * Validated at test time to prevent collisions between bnto slugs
 * and current or anticipated app routes.
 */
export const RESERVED_PATHS = [
  "signin",
  "signout",
  "signup",
  "waitlist",
  "workflows",
  "executions",
  "settings",
  "pricing",
  "about",
  "blog",
  "docs",
  "changelog",
  "api",
  "admin",
  "dashboard",
  "motorway",
] as const;
