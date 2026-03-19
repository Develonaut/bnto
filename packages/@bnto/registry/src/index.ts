/**
 * @bnto/registry — Curation and discovery layer for recipes and node types.
 *
 * Stateless lookups over the engine-generated catalog. Sits between
 * @bnto/nodes (raw engine data) and @bnto/core (reactive store + hooks).
 *
 * No React, no Zustand — purely stateless functions.
 */

// Types
export type { RegistryData } from "./types";

// Recipes
export { getAllRecipes, getRecipeBySlug, getRecipesByCategory } from "./recipes";

// Node types
export { getAllNodeTypes, getBrowserNodeTypes } from "./nodeTypes";

// Categories
export { getAllCategories } from "./categories";

// Processors
export { getAllProcessors } from "./processors";
