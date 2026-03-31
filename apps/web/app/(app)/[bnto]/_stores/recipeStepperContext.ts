"use client";

import { createContext, useContext } from "react";
import { createStoreContext } from "@bnto/core";
import type { ExecutionInstance, Definition } from "@bnto/core";
import type { BntoEntry } from "@/lib/bntoRegistry";
import type { RecipeStepperState } from "./recipeStepperStore";
import type { RecipeStepperActions } from "./recipeStepperActions";

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

const { Provider: RecipeStepperStoreProvider, useStore: useRecipeStepperStore } =
  createStoreContext<RecipeStepperState>("RecipeStepper");

// ---------------------------------------------------------------------------
// Companion context for non-store values (actions, instance, metadata)
// ---------------------------------------------------------------------------

interface RecipeStepperRefsValue {
  actions: RecipeStepperActions;
  instance: ExecutionInstance;
  defn: RecipeDefn;
  entry: BntoEntry;
}

const RecipeStepperRefsContext = createContext<RecipeStepperRefsValue | null>(null);
RecipeStepperRefsContext.displayName = "RecipeStepperRefsContext";

function useRecipeStepperRefs(): RecipeStepperRefsValue {
  const refs = useContext(RecipeStepperRefsContext);
  if (!refs) throw new Error("useRecipeStepperRefs must be used within <RecipeStepper>");
  return refs;
}

// ---------------------------------------------------------------------------
// Convenience hooks for consumers
// ---------------------------------------------------------------------------

function useRecipeStepperActions() {
  return useRecipeStepperRefs().actions;
}

function useRecipeStepperInstance() {
  return useRecipeStepperRefs().instance;
}

function useRecipeStepperDefn() {
  return useRecipeStepperRefs().defn;
}

export {
  RecipeStepperStoreProvider,
  RecipeStepperRefsContext,
  useRecipeStepperStore,
  useRecipeStepperActions,
  useRecipeStepperInstance,
  useRecipeStepperDefn,
};
