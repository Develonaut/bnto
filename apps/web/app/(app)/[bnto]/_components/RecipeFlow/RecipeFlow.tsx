"use client";

import { type ReactNode, useMemo, useEffect } from "react";
import { core } from "@bnto/core";
import type { BntoEntry } from "@/lib/bntoRegistry";
import type { RecipeFlowRefs } from "../../_stores/recipeFlowActions";
import { RecipeFlowStoreProvider, RecipeFlowRefsContext } from "../../_stores/recipeFlowContext";
import { useRecipeDefnFromSlug } from "../../_hooks/useRecipeDefnFromSlug";
import { useRecipeFlowSetup, useRecipeFlowActions } from "./useRecipeFlowSetup";
import { useSyncDerivedStep } from "./useSyncDerivedStep";

interface RecipeFlowProps {
  entry: BntoEntry;
  children: ReactNode;
}

/**
 * Recipe flow provider — client island.
 *
 * Creates a per-mount store (StrictMode-safe), wires context providers,
 * and syncs derived step state. Compose with
 * RecipeFlowStepper and RecipeFlowFileUpload in the page.
 */
export function RecipeFlow({ entry, children }: RecipeFlowProps) {
  const defn = useRecipeDefnFromSlug(entry.slug);
  const { storeApi, instance, resultsRef } = useRecipeFlowSetup(entry);

  const refs: RecipeFlowRefs = useMemo(
    () => ({ getBrowserResults: () => resultsRef.current }),
    [resultsRef],
  );
  const actions = useRecipeFlowActions(storeApi, instance, refs, entry);

  // Sync browser results into the mutable ref for lazy action access.
  const browserExec = core.executions.useExecutionState(instance);
  useEffect(() => {
    resultsRef.current = browserExec.results;
  }, [browserExec.results, resultsRef]);

  useSyncDerivedStep(storeApi, instance);

  const refsValue = useMemo(
    () => ({ actions, instance, defn, entry }),
    [actions, instance, defn, entry],
  );

  return (
    <RecipeFlowStoreProvider store={storeApi}>
      <RecipeFlowRefsContext value={refsValue}>{children}</RecipeFlowRefsContext>
    </RecipeFlowStoreProvider>
  );
}
