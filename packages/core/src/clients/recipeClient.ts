"use client";

import type { Definition } from "@bnto/registry";
import { definitionToRecipe } from "@bnto/registry";
import { recipesStore } from "../stores/recipesStore";
import { fetchCloudRecipes } from "../adapters/convex/recipeAdapter";
import { cloudRecipeToUserRecipe } from "../transforms/cloudRecipeToUserRecipe";
import type { RecipeService } from "../services/recipeService";
import type { ExecutionService } from "../services/executionService";
import type { AuthClient } from "./authClient";
import type { StartExecutionInput } from "../types";
import type { UserRecipe } from "../types/recipe";
import type { RawRecipeDoc } from "../types/raw";

/** Input shape for save — definition + recipe metadata. */
export interface SaveInput {
  id: string;
  name: string;
  slug: string;
  cloudId?: string | null;
}

function upsert(recipe: UserRecipe) {
  recipesStore.getState().upsert(recipe);
}

/** Build a UserRecipe from a Recipe with persistence fields. */
function toUserRecipe(
  recipe: ReturnType<typeof definitionToRecipe>,
  cloudId: string | null,
): UserRecipe {
  return { ...recipe, cloudId, savedAt: Date.now(), syncedAt: null };
}

/** Create a personal recipe from any Definition. Returns the new recipe ID. */
function createFromDefinition(definition: Definition): string {
  const id = crypto.randomUUID();
  const recipe = definitionToRecipe({ ...definition, id }, { id });
  upsert(toUserRecipe(recipe, null));
  return id;
}

/** Save locally, then sync to cloud if authenticated. */
function saveRecipeLocally(
  recipes: RecipeService,
  auth: AuthClient,
  definition: Definition,
  metadata: SaveInput,
): void {
  const cloudId = metadata.cloudId ?? undefined;
  const recipe = definitionToRecipe(definition, {
    id: metadata.id,
    slug: metadata.slug,
    name: metadata.name,
  });
  const userRecipe = toUserRecipe(recipe, cloudId ?? null);

  upsert(userRecipe);
  if (!auth.isAuthenticated()) return;

  recipes
    .save({ ...(cloudId ? { id: cloudId } : {}), name: metadata.name, definition })
    .then((resultId) => {
      upsert({ ...userRecipe, cloudId: String(resultId), syncedAt: Date.now() });
      recipes.invalidateList();
    })
    .catch(() => {});
}

/** Pull all recipes from cloud into local store. Called on sign-in. */
async function pullFromCloud(): Promise<void> {
  const docs = (await fetchCloudRecipes()) as RawRecipeDoc[];
  if (docs.length === 0) return;
  const userRecipes = docs.map(cloudRecipeToUserRecipe);
  recipesStore.getState().hydrateFromCloud(userRecipes);
}

/** Sync all local-only recipes to cloud. Called on sign-in. */
async function syncToCloud(recipes: RecipeService, auth: AuthClient): Promise<void> {
  if (!auth.isAuthenticated()) return;

  const unsynced = Object.values(recipesStore.getState().recipes).filter((r) => !r.cloudId);
  if (unsynced.length === 0) return;

  await Promise.allSettled(
    unsynced.map(async (recipe) => {
      const cloudId = await recipes.save({ name: recipe.name, definition: recipe.definition });
      if (!recipesStore.getState().recipes[recipe.id]) return;
      upsert({ ...recipe, cloudId: String(cloudId), syncedAt: Date.now() });
    }),
  );
  recipes.invalidateList();
}

/** Remove a recipe locally and from cloud if authenticated. */
function removeRecipe(recipes: RecipeService, auth: AuthClient, id: string): void {
  const recipe = recipesStore.getState().recipes[id];
  recipesStore.getState().remove(id);
  if (recipe?.cloudId && auth.isAuthenticated()) recipes.remove(recipe.cloudId).catch(() => {});
}

/** Recipe client — store-backed CRUD with cloud sync on top. */
export function createRecipeClient(
  recipes: RecipeService,
  executions: ExecutionService,
  auth: AuthClient,
) {
  return {
    get: (id: string): UserRecipe | undefined => recipesStore.getState().recipes[id],
    upsert,
    createFromDefinition,
    save: (definition: Definition, metadata: SaveInput) =>
      saveRecipeLocally(recipes, auth, definition, metadata),
    pullFromCloud: () => pullFromCloud(),
    syncToCloud: () => syncToCloud(recipes, auth),
    remove: (id: string) => removeRecipe(recipes, auth, id),
    count: () => Object.keys(recipesStore.getState().recipes).length,
    hydrateFromCloud: (cloudRecipes: UserRecipe[]) =>
      recipesStore.getState().hydrateFromCloud(cloudRecipes),
    listQueryOptions: () => recipes.listQueryOptions(),
    getQueryOptions: (id: string) => recipes.getQueryOptions(id),
    run: async (input: StartExecutionInput) => {
      const executionId = await executions.start(input);
      executions.invalidateExecutions(input.recipeId);
      return executionId;
    },
    invalidateList: () => recipes.invalidateList(),
    invalidateRecipe: (id: string) => recipes.invalidateRecipe(id),
  } as const;
}
