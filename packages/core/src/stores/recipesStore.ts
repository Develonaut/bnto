/** recipesStore — Zustand store with localStorage persistence. */

import type { StoreApi } from "zustand/vanilla";
import { createEnhancedStore } from "./createEnhancedStore";
import { mergeCloudRecipes } from "./mergeCloudRecipes";
import type { UserRecipe } from "../types/recipe";

interface RecipesStoreState {
  recipes: Record<string, UserRecipe>;
  upsert: (recipe: UserRecipe) => void;
  remove: (id: string) => void;
  hydrateFromCloud: (recipes: UserRecipe[]) => void;
}

export const recipesStore: StoreApi<RecipesStoreState> = createEnhancedStore<RecipesStoreState>({
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
      mergeCloudRecipes(state.recipes, recipes);
    }),
}));

export type { RecipesStoreState };
