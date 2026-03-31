/**
 * resolveInitialState — resolves a Definition (or blank) into initial editor state.
 *
 * Pure function. Converts the definition to bento nodes/configs and
 * extracts metadata. Falls back to a blank canvas when no definition
 * is provided.
 *
 * The editor accepts a Definition directly — slug lookup belongs at
 * the app layer (e.g., the editor page does getRecipeBySlug before
 * passing the definition here).
 */

import { type Definition, createBlankDefinition, isIoNodeType } from "@bnto/core";
import { definitionToGraph } from "../adapters/definitionToGraph";
import type { NodeConfigs } from "../adapters/types";
import type { RecipeMetadata } from "./types";

// ---------------------------------------------------------------------------
// Metadata helpers
// ---------------------------------------------------------------------------

/** Converts a name to a URL-safe slug (lowercase, hyphenated). */
function toSlug(name: string): string {
  const slug = name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
  return slug || "untitled";
}

function metadataFromBlank(): RecipeMetadata {
  const def = createBlankDefinition();
  return { id: def.id, name: def.name, slug: toSlug(def.name) };
}

function metadataFromDefinition(def: { id: string; name: string }): RecipeMetadata {
  return { id: def.id, name: def.name, slug: toSlug(def.name) };
}

// ---------------------------------------------------------------------------
// Pre-selection helper
// ---------------------------------------------------------------------------

/** Returns the ID of the first processing (non-I/O) node, or null. */
function findPrimaryNodeId(configs: NodeConfigs): string | null {
  for (const [id, config] of Object.entries(configs)) {
    if (!isIoNodeType(config.nodeType)) return id;
  }
  return null;
}

// ---------------------------------------------------------------------------
// Resolve initial state from definition or blank
// ---------------------------------------------------------------------------

function resolveInitialState(definition?: Definition) {
  if (definition) {
    const { nodes, configs } = definitionToGraph(definition);
    return {
      definition,
      metadata: metadataFromDefinition(definition),
      nodes,
      configs,
      selectedNodeId: findPrimaryNodeId(configs),
    };
  }
  const blankDef = createBlankDefinition();
  const blank = definitionToGraph(blankDef);
  return {
    definition: blankDef as Definition,
    metadata: metadataFromBlank(),
    nodes: blank.nodes,
    configs: blank.configs,
    selectedNodeId: null,
  };
}

export { resolveInitialState, metadataFromBlank, metadataFromDefinition };
