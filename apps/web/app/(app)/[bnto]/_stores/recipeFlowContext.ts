"use client";

import { createContext, useContext } from "react";
import { createStoreContext } from "@bnto/core";
import type { ExecutionInstance, Definition } from "@bnto/core";
import type { BntoEntry } from "@/lib/bntoRegistry";
import type { RecipeFlowState } from "./recipeFlowStore";
import type { RecipeFlowActions } from "./recipeFlowActions";

// ---------------------------------------------------------------------------
// Recipe definition — static metadata derived from the slug
// ---------------------------------------------------------------------------

export interface RecipeDefn {
  definition: Definition | undefined;
  acceptLabel: string;
  dropzoneAccept: Record<string, string[]> | undefined;
}

// ---------------------------------------------------------------------------
// Zustand store context (reactive selectors)
// ---------------------------------------------------------------------------

const { Provider: RecipeFlowStoreProvider, useStore: useRecipeFlowStore } =
  createStoreContext<RecipeFlowState>("RecipeFlow");

// ---------------------------------------------------------------------------
// Companion context for non-store values (actions, instance, metadata)
// ---------------------------------------------------------------------------

interface RecipeFlowRefs {
  actions: RecipeFlowActions;
  instance: ExecutionInstance;
  defn: RecipeDefn;
  entry: BntoEntry;
}

const RecipeFlowRefsContext = createContext<RecipeFlowRefs | null>(null);
RecipeFlowRefsContext.displayName = "RecipeFlowRefsContext";

function useRecipeFlowRefs(): RecipeFlowRefs {
  const refs = useContext(RecipeFlowRefsContext);
  if (!refs) throw new Error("useRecipeFlowRefs must be used within <RecipeFlow>");
  return refs;
}

// ---------------------------------------------------------------------------
// Convenience hooks for consumers
// ---------------------------------------------------------------------------

function useRecipeFlowActions() {
  return useRecipeFlowRefs().actions;
}

function useRecipeFlowInstance() {
  return useRecipeFlowRefs().instance;
}

function useRecipeFlowDefn() {
  return useRecipeFlowRefs().defn;
}

export {
  RecipeFlowStoreProvider,
  RecipeFlowRefsContext,
  useRecipeFlowStore,
  useRecipeFlowActions,
  useRecipeFlowInstance,
  useRecipeFlowDefn,
};
