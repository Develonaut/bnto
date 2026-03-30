"use client";

import { useState, useMemo, useEffect } from "react";
import { core, getRecipeBySlug } from "@bnto/core";
import type { ExecutionInstance } from "@bnto/core";
import type { StoreApi } from "zustand/vanilla";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { createRecipeFlowStore } from "../../_stores/recipeFlowStore";
import type { RecipeFlowState } from "../../_stores/recipeFlowStore";
import { createRecipeFlowActions } from "../../_stores/recipeFlowActions";
import { createMutableCloudRefs } from "../../_stores/createMutableCloudRefs";
import type { MutableCloudRefs } from "../../_stores/createMutableCloudRefs";
import type { RecipeDefn } from "../../_stores/recipeFlowContext";
import { buildPerNodeDefaults } from "../../_utils/buildPerNodeDefaults";

interface RecipeFlowSetup {
  storeApi: StoreApi<RecipeFlowState>;
  instance: ExecutionInstance;
  cloudRefs: MutableCloudRefs;
}

/** Create store + instance + cloud refs once per mount (StrictMode-safe). */
export function useRecipeFlowSetup(entry: BntoEntry): RecipeFlowSetup {
  const [storeApi] = useState(() => {
    const recipe = getRecipeBySlug(entry.slug);
    const defaults = recipe ? buildPerNodeDefaults(recipe.definition) : {};
    return createRecipeFlowStore(defaults);
  });
  const [instance] = useState(() => core.executions.createExecution());
  const [cloudRefs] = useState(() => createMutableCloudRefs());

  useEffect(() => () => instance.reset(), [instance]);

  return { storeApi, instance, cloudRefs };
}

/** Create actions once — stable for the lifetime of the provider. */
export function useRecipeFlowActions(
  storeApi: StoreApi<RecipeFlowState>,
  instance: ExecutionInstance,
  cloudRefs: MutableCloudRefs,
  entry: BntoEntry,
  defn: RecipeDefn,
) {
  return useMemo(
    () =>
      createRecipeFlowActions(
        storeApi,
        instance,
        cloudRefs,
        entry.slug,
        defn.isBrowserPath,
        defn.definition,
      ),
    [storeApi, instance, cloudRefs, entry.slug, defn.isBrowserPath, defn.definition],
  );
}
