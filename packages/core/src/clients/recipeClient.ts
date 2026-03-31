"use client";

import type { Definition } from "@bnto/registry";
import { definitionToRecipe } from "@bnto/registry";
import { recipesStore } from "../stores/recipesStore";
import type { UserRecipe } from "../types/recipe";

/** Input shape for save — definition + recipe metadata. */
export interface SaveInput {
  id: string;
  name: string;
  slug: string;
}

function upsert(recipe: UserRecipe) {
  recipesStore.getState().upsert(recipe);
}

/** Build a UserRecipe from a Recipe. */
function toUserRecipe(recipe: ReturnType<typeof definitionToRecipe>): UserRecipe {
  return { ...recipe, savedAt: Date.now() };
}

/** Create a personal recipe from any Definition. Returns the new recipe ID. */
function createFromDefinition(definition: Definition): string {
  const id = crypto.randomUUID();
  const recipe = definitionToRecipe({ ...definition, id }, { id });
  upsert(toUserRecipe(recipe));
  return id;
}

/** Save a recipe locally (sessionStorage). */
function saveLocally(definition: Definition, metadata: SaveInput): void {
  const recipe = definitionToRecipe(definition, {
    id: metadata.id,
    slug: metadata.slug,
    name: metadata.name,
  });
  upsert(toUserRecipe(recipe));
}

/** Remove a recipe from the store. */
function removeRecipe(id: string): void {
  recipesStore.getState().remove(id);
}

/** Recipe client — store-backed CRUD. */
export function createRecipeClient() {
  return {
    get: (id: string): UserRecipe | undefined => recipesStore.getState().recipes[id],
    upsert,
    createFromDefinition,
    save: (definition: Definition, metadata: SaveInput) => saveLocally(definition, metadata),
    remove: (id: string) => removeRecipe(id),
    count: () => Object.keys(recipesStore.getState().recipes).length,
  } as const;
}
