/**
 * Bnto slug registry — thin wrapper over @bnto/registry recipes.
 *
 * Drives generateStaticParams, generateMetadata, sitemap, gallery,
 * llms.txt, and middleware slug validation from one place.
 *
 * All recipe data flows through @bnto/registry (the curation layer).
 * This module maps Recipe → BntoEntry to preserve the existing consumer API.
 */

import { getAllRecipes, getRecipeBySlug } from "@bnto/registry";
import type { Recipe } from "@bnto/core";

export interface BntoEntry {
  slug: string;
  title: string;
  description: string;
  h1: string;
  fixture: string;
  features: string[];
  category: string;
}

export function toBntoEntry(r: Recipe): BntoEntry {
  return {
    slug: r.slug,
    title: `${r.name} Online Free -- bnto`,
    description: r.description,
    h1: `${r.name} Online Free`,
    fixture: `${r.slug}.bnto.json`,
    features: r.features,
    category: r.category,
  };
}

/**
 * All registered bntos. Each entry maps to a public URL at /{slug}.
 */
export const BNTO_REGISTRY: readonly BntoEntry[] = getAllRecipes().map(toBntoEntry);

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
  "ingest",
  "explore",
] as const;
