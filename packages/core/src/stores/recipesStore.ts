/** recipesStore — Zustand store with localStorage persistence. */

import { createEnhancedStore } from "./createEnhancedStore";
import type { Recipe } from "../types/recipe";

interface RecipesStoreState {
  recipes: Record<string, Recipe>;
  upsert: (recipe: Recipe) => void;
  remove: (id: string) => void;
  hydrateFromCloud: (recipes: Recipe[]) => void;
}

export const recipesStore: import("zustand/vanilla").StoreApi<RecipesStoreState> =
  createEnhancedStore<RecipesStoreState>({
    persist: {
      name: "bnto-recipes",
      partialize: (state) => ({ recipes: state.recipes }) as RecipesStoreState,
    },
  })((set) => ({
    recipes: {},

    upsert: (recipe) =>
      set((state) => {
        state.recipes[recipe.id] = recipe;
      }),

    remove: (id) =>
      set((state) => {
        delete state.recipes[id];
      }),

    hydrateFromCloud: (recipes) =>
      set((state) => {
        for (const r of recipes) {
          if (!state.recipes[r.id]) {
            state.recipes[r.id] = r;
          }
        }
      }),
  }));

export type { RecipesStoreState };
