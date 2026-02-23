/**
 * Menu reader — typed access to the engine-generated recipe catalog.
 *
 * The engine (Go) is the single source of truth for predefined bntos.
 * This module reads the generated JSON and exposes typed helpers.
 *
 * To regenerate: `task menu:generate`
 */

import menuData from "./generated/menu.json";

export interface AcceptSpec {
  mimeTypes: string[];
  extensions: string[];
  label: string;
  mimePrefix?: string;
}

export interface SEOSpec {
  title: string;
  h1: string;
}

export interface Recipe {
  slug: string;
  name: string;
  description: string;
  category: string;
  accept: AcceptSpec;
  features: string[];
  seo: SEOSpec;
  definition: Record<string, unknown>;
}

const entries = menuData as Recipe[];
const bySlug = new Map(entries.map((e) => [e.slug, e]));

/** All predefined recipes from the engine menu. */
export const MENU: readonly Recipe[] = entries;

/** Returns true if the slug maps to a recipe in the menu. */
export function isValidMenuSlug(slug: string): boolean {
  return bySlug.has(slug);
}

/** Returns the recipe for a slug, or undefined if not found. */
export function getRecipe(slug: string): Recipe | undefined {
  return bySlug.get(slug);
}
