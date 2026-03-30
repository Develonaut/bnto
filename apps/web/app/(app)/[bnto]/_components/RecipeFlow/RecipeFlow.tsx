"use client";

import { type ReactNode, useMemo } from "react";
import type { BntoEntry } from "@/lib/bntoRegistry";
import { RecipeFlowStoreProvider, RecipeFlowRefsContext } from "../../_stores/recipeFlowContext";
import { useRecipeDefnFromSlug } from "../../_hooks/useRecipeDefnFromSlug";
import { useRecipeFlowSetup, useRecipeFlowActions } from "./useRecipeFlowSetup";
import { useSyncCloud } from "./useSyncCloud";
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
  const { storeApi, instance, cloudRefs } = useRecipeFlowSetup(entry);
  const actions = useRecipeFlowActions(storeApi, instance, cloudRefs, entry, defn);

  useSyncCloud(storeApi, cloudRefs, instance);
  useSyncDerivedStep(storeApi, instance, defn);

  const refsValue = useMemo(
    () => ({ actions, instance, defn, entry, cloudRefs }),
    [actions, instance, defn, entry, cloudRefs],
  );

  return (
    <RecipeFlowStoreProvider store={storeApi}>
      <RecipeFlowRefsContext value={refsValue}>{children}</RecipeFlowRefsContext>
    </RecipeFlowStoreProvider>
  );
}
