"use client";

import { useEffect, useRef } from "react";
import { core } from "../core";
import { useIsAuthenticated } from "./useIsAuthenticated";

/**
 * Syncs local-only recipes to cloud on auth state change (unauth→auth).
 *
 * Watches for sign-in / sign-up transition, then calls
 * core.recipes.syncToCloud() once per session.
 *
 * Must be rendered once inside BntoCoreProvider.
 */
export function useRecipeSync() {
  const isAuthenticated = useIsAuthenticated();
  const prevAuthRef = useRef(isAuthenticated);
  const syncedRef = useRef(false);

  useEffect(() => {
    const wasUnauth = !prevAuthRef.current;
    const isNowAuth = isAuthenticated;
    prevAuthRef.current = isAuthenticated;

    if (wasUnauth && isNowAuth && !syncedRef.current) {
      syncedRef.current = true;
      core.recipes.syncToCloud().catch(() => {});
    }
  }, [isAuthenticated]);
}
