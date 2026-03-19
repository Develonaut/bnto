/** recipesStore — Zustand store with localStorage persistence. */

import { createEnhancedStore } from "./createEnhancedStore";
import type { UserRecipe } from "../types/recipe";

interface RecipesStoreState {
  recipes: Record<string, UserRecipe>;
  upsert: (recipe: UserRecipe) => void;
  remove: (id: string) => void;
  hydrateFromCloud: (recipes: UserRecipe[]) => void;
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
          const existing = state.recipes[r.id];
          if (!existing) {
            // New recipe — add to local store
            state.recipes[r.id] = r;
          } else if (!existing.cloudId && r.cloudId) {
            // Local-only recipe matched by ID — merge cloud link
            state.recipes[r.id] = {
              ...existing,
              cloudId: r.cloudId,
              syncedAt: r.syncedAt ?? Date.now(),
            };
          }
        }
      }),
  }));

export type { RecipesStoreState };
