"use client";

import { useState, useRef, useMemo, useEffect } from "react";
import { core, getRecipeBySlug } from "@bnto/core";
import type { ExecutionInstance, BrowserFileResult } from "@bnto/core";
import type { StoreApi } from "zustand/vanilla";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { createRecipeFlowStore } from "../../_stores/recipeFlowStore";
import type { RecipeFlowState } from "../../_stores/recipeFlowStore";
import { createRecipeFlowActions } from "../../_stores/recipeFlowActions";
import type { RecipeFlowRefs } from "../../_stores/recipeFlowActions";
import { buildPerNodeDefaults } from "../../_utils/buildPerNodeDefaults";

interface RecipeFlowSetup {
  storeApi: StoreApi<RecipeFlowState>;
  instance: ExecutionInstance;
  resultsRef: React.RefObject<BrowserFileResult[]>;
}

/** Create store + instance once per mount (StrictMode-safe). */
export function useRecipeFlowSetup(entry: BntoEntry): RecipeFlowSetup {
  const [storeApi] = useState(() => {
    const recipe = getRecipeBySlug(entry.slug);
    const defaults = recipe ? buildPerNodeDefaults(recipe.definition) : {};
    return createRecipeFlowStore(defaults);
  });
  const [instance] = useState(() => core.executions.createExecution());
  const resultsRef = useRef<BrowserFileResult[]>([]);

  useEffect(() => () => instance.reset(), [instance]);

  return { storeApi, instance, resultsRef };
}

/** Create actions once — stable for the lifetime of the provider. */
export function useRecipeFlowActions(
  storeApi: StoreApi<RecipeFlowState>,
  instance: ExecutionInstance,
  refs: RecipeFlowRefs,
  entry: BntoEntry,
) {
  return useMemo(
    () => createRecipeFlowActions(storeApi, instance, refs, entry.slug),
    [storeApi, instance, refs, entry.slug],
  );
}
