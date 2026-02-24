/**
 * Bnto slug registry — thin wrapper over the engine-generated menu.
 *
 * Drives generateStaticParams, generateMetadata, sitemap, gallery,
 * llms.txt, and middleware slug validation from one place.
 *
 * All data originates in engine/pkg/menu/recipes/*.json. This module
 * maps Recipe → BntoEntry to preserve the existing consumer API.
 *
 * To regenerate: `task menu:generate`
 */

import { MENU, getRecipe, isValidMenuSlug, type Recipe } from "./menu";

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
export const BNTO_REGISTRY: readonly BntoEntry[] = MENU.map(toBntoEntry);

/** Returns true if the slug maps to a registered bnto. */
export function isValidBntoSlug(slug: string): boolean {
  return isValidMenuSlug(slug);
}

/** Returns the registry entry for a slug, or undefined if not found. */
export function getBntoBySlug(slug: string): BntoEntry | undefined {
  const recipe = getRecipe(slug);
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
] as const;
