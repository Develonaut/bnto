"use client";

import { useState, useRef, useMemo, useEffect } from "react";
import { core, getRecipeBySlug } from "@bnto/core";
import type { ExecutionInstance, BrowserFileResult } from "@bnto/core";
import type { StoreApi } from "zustand/vanilla";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { createRecipeStepperStore } from "../../_stores/recipeStepperStore";
import type { RecipeStepperState } from "../../_stores/recipeStepperStore";
import { createRecipeStepperActions } from "../../_stores/recipeStepperActions";
import type { RecipeStepperRefs } from "../../_stores/recipeStepperActions";
import { buildPerNodeDefaults } from "../../_utils/buildPerNodeDefaults";

interface RecipeStepperSetup {
  storeApi: StoreApi<RecipeStepperState>;
  instance: ExecutionInstance;
  resultsRef: React.RefObject<BrowserFileResult[]>;
}

/** Create store + instance once per mount (StrictMode-safe). */
export function useRecipeStepperSetup(entry: BntoEntry): RecipeStepperSetup {
  const [storeApi] = useState(() => {
    const recipe = getRecipeBySlug(entry.slug);
    const defaults = recipe ? buildPerNodeDefaults(recipe.definition) : {};
    return createRecipeStepperStore(defaults);
  });
  const [instance] = useState(() => core.executions.createExecution());
  const resultsRef = useRef<BrowserFileResult[]>([]);

  useEffect(() => () => instance.reset(), [instance]);

  return { storeApi, instance, resultsRef };
}

/** Create actions once — stable for the lifetime of the provider. */
export function useRecipeStepperActions(
  storeApi: StoreApi<RecipeStepperState>,
  instance: ExecutionInstance,
  refs: RecipeStepperRefs,
  entry: BntoEntry,
) {
  return useMemo(
    () => createRecipeStepperActions(storeApi, instance, refs, entry.slug),
    [storeApi, instance, refs, entry.slug],
  );
}
