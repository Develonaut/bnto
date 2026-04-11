/**
 * All predefined recipes — the bnto catalog.
 *
 * Engine owns recipe definitions AND metadata (descriptions, tags).
 * This file derives `Recipe` objects from engine-generated data,
 * adding accept specs derived from the definition's input node.
 */

import { GENERATED_RECIPES } from "@bnto/nodes";
import type { Recipe } from "./recipe";
import { deriveAcceptSpec } from "./deriveAcceptSpec";

/**
 * All predefined recipes that map to public URLs.
 *
 * Derived from engine-generated recipes. Order matches the engine's
 * `RECIPE_DEFINITIONS` array in `recipes.rs`.
 */
export const RECIPES: readonly Recipe[] = GENERATED_RECIPES.map((generated) => ({
  id: generated.slug,
  slug: generated.slug,
  name: generated.name,
  description: generated.description,
  category: generated.category,
  definition: generated.definition,
  accept: deriveAcceptSpec(generated.definition) ?? {
    mimeTypes: [],
    extensions: [],
    label: "Files",
  },
  features: [...generated.tags],
}));
