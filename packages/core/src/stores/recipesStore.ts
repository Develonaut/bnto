/** recipesStore — Zustand store with localStorage persistence. */

import { createEnhancedStore } from "./createEnhancedStore";
import type { StoredRecipe } from "../types/recipe";

interface RecipesStoreState {
  recipes: Record<string, StoredRecipe>;
  upsert: (recipe: StoredRecipe) => void;
  remove: (id: string) => void;
  hydrateFromCloud: (recipes: StoredRecipe[]) => void;
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
        state.recipes[recipe.metadata.id] = recipe;
      }),

    remove: (id) =>
      set((state) => {
        delete state.recipes[id];
      }),

    hydrateFromCloud: (recipes) =>
      set((state) => {
        for (const r of recipes) {
          if (!state.recipes[r.metadata.id]) {
            state.recipes[r.metadata.id] = r;
          }
        }
      }),
  }));

export type { RecipesStoreState };
