"use client";

import { createRecipeFlowStore } from "../stores/recipeFlowStore";

/**
 * Recipe flow client — public API for recipe page state management.
 *
 * Unlike the browser execution client (global singleton), recipe flow
 * stores are page-scoped: each [bnto] page mount creates its own
 * store via `createStore()`. The client exposes the factory — the
 * React hook layer manages store lifecycle (create on mount, dispose
 * on unmount).
 */
export function createRecipeClient() {
  return {
    /**
     * Create a page-scoped recipe flow store.
     *
     * @param defaultConfig - Default config for the recipe slug.
     * @returns A vanilla Zustand store instance.
     */
    createStore: (defaultConfig: Record<string, unknown> = {}) =>
      createRecipeFlowStore(defaultConfig),
  } as const;
}

export type RecipeClient = ReturnType<typeof createRecipeClient>;
