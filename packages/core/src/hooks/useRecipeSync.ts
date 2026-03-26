"use client";

import { useEffect, useRef } from "react";
import { core } from "../core";
import { useIsAuthenticated } from "./useIsAuthenticated";

/**
 * Syncs recipes between local and cloud on auth state change.
 *
 * Triggers on:
 * 1. unauth→auth transition (sign-in)
 * 2. Mount while already authenticated (page reload with empty local store)
 *
 * Pull-first strategy: pull cloud recipes into local store (so cloudId dedup
 * prevents re-upload), then push any remaining local-only recipes to cloud.
 *
 * Must be rendered once inside BntoCoreProvider.
 */
export function useRecipeSync() {
  const isAuthenticated = useIsAuthenticated();
  const syncedRef = useRef(false);

  useEffect(() => {
    if (!isAuthenticated) {
      syncedRef.current = false;
      return;
    }

    if (syncedRef.current) return;
    syncedRef.current = true;

    core.recipes
      .pullFromCloud()
      .then(() => core.recipes.syncToCloud())
      .catch(() => {});
  }, [isAuthenticated]);
}
