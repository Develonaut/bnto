/**
 * resolveInitialState — resolves a slug (or blank) into initial editor state.
 *
 * Pure function. Looks up the slug in the recipe registry, converts
 * the definition to bento nodes/configs, and extracts metadata.
 * Falls back to a blank canvas when the slug is missing or unknown.
 */

import {
  createBlankDefinition,
  getRecipeBySlug,
} from "@bnto/nodes";
import { definitionToBento } from "../adapters/definitionToBento";
import type { RecipeMetadata } from "./types";

// ---------------------------------------------------------------------------
// Metadata helpers
// ---------------------------------------------------------------------------

function metadataFromBlank(): RecipeMetadata {
  const def = createBlankDefinition();
  return { id: def.id, name: def.name, type: def.type, version: def.version };
}

function metadataFromDefinition(def: { id: string; name: string; type: string; version: string }): RecipeMetadata {
  return { id: def.id, name: def.name, type: def.type, version: def.version };
}

// ---------------------------------------------------------------------------
// Resolve initial state from slug or blank
// ---------------------------------------------------------------------------

function resolveInitialState(slug?: string) {
  if (slug) {
    const recipe = getRecipeBySlug(slug);
    if (recipe) {
      const { nodes, configs } = definitionToBento(recipe.definition);
      return {
        slug,
        metadata: metadataFromDefinition(recipe.definition),
        nodes,
        configs,
      };
    }
  }
  const blank = definitionToBento(createBlankDefinition());
  return {
    slug: slug ?? null,
    metadata: metadataFromBlank(),
    nodes: blank.nodes,
    configs: blank.configs,
  };
}

export { resolveInitialState, metadataFromBlank, metadataFromDefinition };
