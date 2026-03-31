"use client";

import { type ReactNode, useMemo, useEffect } from "react";
import { core } from "@bnto/core";
import type { BntoEntry } from "@/lib/bntoRegistry";
import type { RecipeStepperRefs } from "../../_stores/recipeStepperActions";
import {
  RecipeStepperStoreProvider,
  RecipeStepperRefsContext,
} from "../../_stores/recipeStepperContext";
import { useRecipeDefinitionFromSlug } from "../../_hooks/useRecipeDefinitionFromSlug";
import { useRecipeStepperSetup, useRecipeStepperActions } from "./useRecipeStepperSetup";
import { useSyncDerivedStep } from "./useSyncDerivedStep";

interface RecipeStepperProviderProps {
  entry: BntoEntry;
  children: ReactNode;
}

/** Wires store, refs, execution sync, and context providers. */
export function RecipeStepperProvider({ entry, children }: RecipeStepperProviderProps) {
  const defn = useRecipeDefinitionFromSlug(entry.slug);
  const { storeApi, instance, resultsRef } = useRecipeStepperSetup(entry);

  const refs: RecipeStepperRefs = useMemo(
    () => ({ getBrowserResults: () => resultsRef.current }),
    [resultsRef],
  );
  const actions = useRecipeStepperActions(storeApi, instance, refs, entry);

  const execution = core.executions.useExecutionState(instance);
  useEffect(() => {
    resultsRef.current = execution.results;
  }, [execution.results, resultsRef]);

  useSyncDerivedStep(storeApi, instance);

  const refsValue = useMemo(
    () => ({ actions, instance, defn, entry }),
    [actions, instance, defn, entry],
  );

  return (
    <RecipeStepperStoreProvider store={storeApi}>
      <RecipeStepperRefsContext value={refsValue}>{children}</RecipeStepperRefsContext>
    </RecipeStepperStoreProvider>
  );
}
